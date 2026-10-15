// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
//
// This software is released under the MIT License.
// See the LICENSE file in the project root for full license information.

//! Runtime program-counter coverage.
//!
//! This records which instruction addresses a firmware run actually executed,
//! plus the control-flow edges between them, by observing the per-instruction
//! step hook. It is the raw execution-coverage source that higher layers map
//! back to source statements and branches via DWARF debug info.
//!
//! This is distinct from the [`crate::coverage`] module: that one measures, by
//! observable behaviour, whether a chip model implements the registers a part's
//! SVD declares (model faithfulness). This one measures which addresses of the
//! firmware under test ran (firmware coverage).

use crate::SimulationObserver;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// Taken / not-taken counts observed for the instruction at one source address.
///
/// `taken` counts the times execution left via a non-fall-through path (a taken
/// conditional branch, an unconditional jump, a call, a return or a trap);
/// `not_taken` counts the times it fell through to the next instruction. A
/// source address with `taken > 0` is a branch site.
///
/// This is the *dynamic* view: it only knows about instructions that diverged
/// at least once, and it guesses the fall-through address from a Thumb-only
/// length rule. Reports should prefer [`PcCoverageObserver::edge_count`]
/// against a static list of conditional branches (see
/// `labwired_loader::branch_sites`), which is ISA-correct and also counts the
/// branches that never ran.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BranchCounts {
    pub taken: u64,
    pub not_taken: u64,
    pub target: Option<u32>,
}

/// Everything the observer accumulates, behind one lock so the per-instruction
/// hook takes a single mutex instead of three.
#[derive(Debug, Default)]
struct State {
    /// Aligned instruction address -> times executed.
    hits: HashMap<u32, u64>,
    /// `(from, to)` aligned address pair -> times execution went from one
    /// instruction straight to the next. Recorded for every consecutive pair,
    /// so it carries no ISA assumption: whoever knows the instruction set
    /// (the static branch decoder) decides which pairs are "taken" and which
    /// are "fall-through".
    edges: HashMap<(u32, u32), u64>,
    /// Legacy dynamic branch view (Thumb length rule), see [`BranchCounts`].
    branches: BTreeMap<u32, BranchCounts>,
    /// Previous instruction: (aligned_pc, Thumb-rule length in bytes).
    last: Option<(u32, u32)>,
}

/// Observer that accumulates, for one run: how many times each instruction
/// address executed, and how many times each `(from, to)` step between two
/// consecutive instructions happened.
///
/// Addresses are stored half-word aligned (the Thumb bit is masked off) so a PC
/// and its interworking alias collapse to one entry. The edge counts are what
/// branch coverage is built from: for a conditional branch at `S` with target
/// `T` and fall-through `F`, "taken" is `edge_count(S, T)` and "not taken" is
/// `edge_count(S, F)`. An interrupt that arrives right after `S` records
/// `S -> handler` instead, so that one outcome is not counted. The error is
/// always towards *less* coverage, never more.
#[derive(Debug, Default)]
pub struct PcCoverageObserver {
    state: Mutex<State>,
    total: AtomicU64,
}

impl PcCoverageObserver {
    pub fn new() -> Self {
        Self::default()
    }

    /// Total instructions observed (including repeats of the same address).
    pub fn total_instructions(&self) -> u64 {
        self.total.load(Ordering::SeqCst)
    }

    /// Distinct instruction addresses executed, ascending.
    pub fn covered_addresses(&self) -> Vec<u32> {
        self.state
            .lock()
            .map(|s| {
                s.hits
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Number of distinct instruction addresses executed.
    pub fn covered_count(&self) -> usize {
        self.state.lock().map(|s| s.hits.len()).unwrap_or(0)
    }

    /// Times the instruction at `addr` executed (0 if never). The Thumb bit is
    /// ignored.
    pub fn hit_count(&self, addr: u32) -> u64 {
        let addr = addr & !1;
        self.state
            .lock()
            .map(|s| s.hits.get(&addr).copied().unwrap_or(0))
            .unwrap_or(0)
    }

    /// Every executed address with its execution count, ascending by address.
    pub fn hits(&self) -> Vec<(u32, u64)> {
        self.state
            .lock()
            .map(|s| {
                let mut v: Vec<(u32, u64)> = s.hits.iter().map(|(a, n)| (*a, *n)).collect();
                v.sort_unstable();
                v
            })
            .unwrap_or_default()
    }

    /// Times execution stepped from the instruction at `from` directly to the
    /// instruction at `to` (both Thumb bits ignored). 0 if never.
    pub fn edge_count(&self, from: u32, to: u32) -> u64 {
        let key = (from & !1, to & !1);
        self.state
            .lock()
            .map(|s| s.edges.get(&key).copied().unwrap_or(0))
            .unwrap_or(0)
    }

    /// A consistent copy of all `(from, to) -> count` steps, ascending.
    pub fn edge_counts(&self) -> Vec<((u32, u32), u64)> {
        self.state
            .lock()
            .map(|s| {
                let mut v: Vec<((u32, u32), u64)> = s.edges.iter().map(|(k, n)| (*k, *n)).collect();
                v.sort_unstable();
                v
            })
            .unwrap_or_default()
    }

    /// Control-flow edges taken, as `(from, to)` aligned address pairs,
    /// ascending (legacy dynamic view, Thumb length rule).
    pub fn edges(&self) -> Vec<(u32, u32)> {
        self.state
            .lock()
            .map(|s| {
                s.branches
                    .iter()
                    .filter_map(|(src, c)| c.target.filter(|_| c.taken > 0).map(|t| (*src, t)))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Branch sites — source addresses that diverged at least once — with their
    /// taken/not-taken counts, ascending by address (legacy dynamic view).
    pub fn branch_sites(&self) -> Vec<(u32, BranchCounts)> {
        self.state
            .lock()
            .map(|s| {
                s.branches
                    .iter()
                    .filter(|(_, c)| c.taken > 0)
                    .map(|(src, c)| (*src, *c))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Keep the counts but forget the previous instruction, so the next step
    /// starts a new chain and records no edge. Call it when execution jumps
    /// for a reason that is not the program's (a snapshot restore).
    pub fn break_chain(&self) {
        if let Ok(mut s) = self.state.lock() {
            s.last = None;
        }
    }

    /// Forget everything recorded so far, as if the observer were new.
    pub fn reset(&self) {
        if let Ok(mut s) = self.state.lock() {
            *s = State::default();
        }
        self.total.store(0, Ordering::SeqCst);
    }

    /// True if the given (aligned) address was executed at least once.
    pub fn was_executed(&self, addr: u32) -> bool {
        self.hit_count(addr) > 0
    }
}

impl SimulationObserver for PcCoverageObserver {
    fn on_step_start(&self, pc: u32, opcode: u32) {
        let aligned = pc & !1;
        self.total.fetch_add(1, Ordering::SeqCst);

        // Thumb instructions are 16 or 32 bits. A 32-bit instruction has its
        // top 5 opcode bits in 0b11101/0b11110/0b11111, encoded so the high
        // half-word leads; treat anything with those high bits set as 4 bytes.
        // Only the legacy dynamic view uses this; edge counts need no length.
        let len = if (opcode >> 11) & 0b11111 >= 0b11101 && (opcode >> 16) != 0 {
            4
        } else {
            2
        };

        let Ok(mut s) = self.state.lock() else {
            return;
        };
        *s.hits.entry(aligned).or_insert(0) += 1;
        if let Some((prev_pc, prev_len)) = s.last {
            *s.edges.entry((prev_pc, aligned)).or_insert(0) += 1;
            let fallthrough = prev_pc.wrapping_add(prev_len);
            let counts = s.branches.entry(prev_pc).or_default();
            if aligned != fallthrough {
                counts.taken += 1;
                counts.target = Some(aligned);
            } else {
                counts.not_taken += 1;
            }
        }
        s.last = Some((aligned, len));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_distinct_addresses_and_count() {
        let cov = PcCoverageObserver::new();
        // Three 16-bit instructions, the middle one executed twice.
        cov.on_step_start(0x0800_0000, 0x4600);
        cov.on_step_start(0x0800_0002, 0x4600);
        cov.on_step_start(0x0800_0002, 0x4600);
        cov.on_step_start(0x0800_0004, 0x4600);

        assert_eq!(cov.total_instructions(), 4);
        assert_eq!(cov.covered_count(), 3);
        assert_eq!(
            cov.covered_addresses(),
            vec![0x0800_0000, 0x0800_0002, 0x0800_0004]
        );
        assert!(cov.was_executed(0x0800_0002));
        assert!(!cov.was_executed(0x0800_0006));
    }

    #[test]
    fn masks_thumb_bit() {
        let cov = PcCoverageObserver::new();
        cov.on_step_start(0x0800_0001, 0x4600); // interworking alias
        assert!(cov.was_executed(0x0800_0000));
        assert_eq!(cov.covered_count(), 1);
    }

    #[test]
    fn records_branch_edges_not_fallthrough() {
        let cov = PcCoverageObserver::new();
        // Sequential fall-through: no edges.
        cov.on_step_start(0x0800_0000, 0x4600);
        cov.on_step_start(0x0800_0002, 0x4600);
        assert!(cov.edges().is_empty());

        // A jump backwards to 0x0800_0000: one edge from 0x0800_0002.
        cov.on_step_start(0x0800_0000, 0x4600);
        assert_eq!(cov.edges(), vec![(0x0800_0002, 0x0800_0000)]);
    }

    #[test]
    fn records_branch_taken_and_not_taken_counts() {
        let cov = PcCoverageObserver::new();
        // Branch at 0x0800_0100: once falls through to 0x0800_0102, once taken
        // to 0x0800_0200. (Re-entering the branch is itself a divergence from
        // 0x0800_0102, so assert on 0x0800_0100's own counts, not the site
        // total.)
        cov.on_step_start(0x0800_0100, 0x4600);
        cov.on_step_start(0x0800_0102, 0x4600); // 0x...0100 fell through
        cov.on_step_start(0x0800_0100, 0x4600); // re-enter the branch
        cov.on_step_start(0x0800_0200, 0x4600); // 0x...0100 taken to 0x...0200

        let sites = cov.branch_sites();
        let (_, counts) = sites
            .iter()
            .find(|(src, _)| *src == 0x0800_0100)
            .expect("0x0800_0100 is a branch site");
        assert_eq!(counts.not_taken, 1);
        assert_eq!(counts.taken, 1);
        assert_eq!(counts.target, Some(0x0800_0200));
    }

    #[test]
    fn counts_hits_per_address() {
        let cov = PcCoverageObserver::new();
        for _ in 0..3 {
            cov.on_step_start(0x0800_0010, 0x4600);
        }
        cov.on_step_start(0x0800_0013, 0x4600); // Thumb alias of 0x...12
        assert_eq!(cov.hit_count(0x0800_0010), 3);
        assert_eq!(cov.hit_count(0x0800_0012), 1);
        assert_eq!(cov.hit_count(0x0800_0014), 0);
        assert_eq!(cov.hits(), vec![(0x0800_0010, 3), (0x0800_0012, 1)]);
    }

    /// Edge counts carry no ISA assumption: a 4-byte RISC-V step is recorded
    /// exactly like a 2-byte Thumb one, which the legacy length rule gets wrong.
    #[test]
    fn edge_counts_are_isa_agnostic() {
        let cov = PcCoverageObserver::new();
        // RISC-V: beq at 0x100 (4 bytes), falls through to 0x104 once...
        cov.on_step_start(0x100, 0x0000_0463);
        cov.on_step_start(0x104, 0x0000_0013);
        // ...and is taken to 0x108 once.
        cov.on_step_start(0x100, 0x0000_0463);
        cov.on_step_start(0x108, 0x0000_0013);
        assert_eq!(cov.edge_count(0x100, 0x104), 1, "fall-through");
        assert_eq!(cov.edge_count(0x100, 0x108), 1, "taken");
        assert_eq!(cov.edge_count(0x104, 0x100), 1, "loop back");
        assert_eq!(cov.edge_count(0x100, 0x10c), 0);
        assert_eq!(cov.edge_counts().len(), 3);
    }

    #[test]
    fn reset_forgets_everything() {
        let cov = PcCoverageObserver::new();
        cov.on_step_start(0x100, 0x4600);
        cov.on_step_start(0x200, 0x4600);
        cov.reset();
        assert_eq!(cov.total_instructions(), 0);
        assert_eq!(cov.covered_count(), 0);
        assert!(cov.edge_counts().is_empty());
        // No stale "previous instruction": the first step after a reset
        // records no edge.
        cov.on_step_start(0x300, 0x4600);
        assert!(cov.edge_counts().is_empty());
    }

    #[test]
    fn break_chain_keeps_counts_but_records_no_edge() {
        let cov = PcCoverageObserver::new();
        cov.on_step_start(0x100, 0x4600);
        cov.break_chain();
        cov.on_step_start(0x500, 0x4600);
        assert_eq!(cov.hit_count(0x100), 1);
        assert_eq!(cov.edge_count(0x100, 0x500), 0, "the rewind is not a jump");
        assert!(cov.edge_counts().is_empty());
    }
}
