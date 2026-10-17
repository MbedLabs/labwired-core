// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
//
// This software is released under the MIT License.
// See the LICENSE file in the project root for full license information.

use crate::SimResult;
use serde::{Deserialize, Serialize};

pub use labwired_config::CpuWritePolicy;

/// One recorded policy violation (the first per region is kept in full).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionViolation {
    pub addr: u64,
    pub cycle: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc: Option<u32>,
}

/// Per-region stats for `result.json`'s `memory_policy` block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegionPolicyStats {
    pub mode: CpuWritePolicy,
    pub violations: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<RegionViolation>,
}

/// What `labwired test` writes under `result.json.memory_policy`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPolicyReport {
    pub flash: RegionPolicyStats,
    pub ram: RegionPolicyStats,
}

/// Live policy state carried by [`crate::bus::SystemBus`].
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct MemoryPolicyState {
    pub flash: CpuWritePolicy,
    pub ram: CpuWritePolicy,
    pub violations_flash: u64,
    pub violations_ram: u64,
    pub first_flash: Option<RegionViolation>,
    pub first_ram: Option<RegionViolation>,
    pub flash_warning_emitted: bool,
    pub ram_warning_emitted: bool,
    pub last_cpu_pc: u32,
    pub cycle: u64,
}

#[allow(dead_code)]
impl MemoryPolicyState {
    /// The policies a chip declares — used by `from_config` and unit tests.
    pub fn for_chip(chip: &labwired_config::ChipDescriptor) -> Self {
        Self {
            flash: chip.flash.cpu_writes,
            ram: chip.ram.cpu_writes,
            ..Default::default()
        }
    }

    pub fn active(&self) -> bool {
        !self.flash.is_allow() || !self.ram.is_allow()
    }

    pub fn note_pc(&mut self, pc: u32) {
        self.last_cpu_pc = pc;
    }

    /// True when a CPU store at `addr` must skip the direct-to-RAM fast path
    /// (the policy gate in `accessors.rs` then handles it).
    pub fn ram_blocked(&self, addr: u64, ram_base: u64, ram_len: u64) -> bool {
        self.ram != CpuWritePolicy::Allow && addr >= ram_base && addr < ram_base + ram_len
    }

    fn record(
        region_first: &mut Option<RegionViolation>,
        count: &mut u64,
        addr: u64,
        pc: u32,
        cycle: u64,
    ) {
        *count += 1;
        if region_first.is_none() {
            *region_first = Some(RegionViolation {
                addr,
                cycle,
                pc: Some(pc),
            });
        }
    }

    /// The gate `bus/accessors.rs` calls before any store that would land in
    /// the flash region, its boot alias, the ram region, or an extra memory
    /// window. `flash_alias_active` is `bus.flash_boot_alias && flash_base != 0`.
    /// Returns `None` when the store may proceed, or `Some(Err(..))` when it
    /// must raise a precise data fault.
    pub fn gate(
        &mut self,
        addr: u64,
        ram_base: u64,
        ram_len: u64,
        flash_base: u64,
        flash_len: u64,
        flash_alias_active: bool,
        extra: &[(u64, u64)],
    ) -> Option<SimResult<()>> {
        let in_flash = (addr >= flash_base && addr < flash_base.saturating_add(flash_len))
            || (flash_alias_active && addr < flash_len);
        let in_ram = addr >= ram_base && addr < ram_base.saturating_add(ram_len);
        let in_extra = extra
            .iter()
            .any(|(b, l)| addr >= *b && addr < b.saturating_add(*l));
        if !in_flash && !in_ram && !in_extra {
            return None;
        }
        let pc = self.last_cpu_pc;
        let cycle = self.cycle;
        if in_flash {
            match self.flash {
                CpuWritePolicy::Allow => {}
                CpuWritePolicy::Warn => {
                    Self::record(
                        &mut self.first_flash,
                        &mut self.violations_flash,
                        addr,
                        pc,
                        cycle,
                    );
                    if !self.flash_warning_emitted {
                        self.flash_warning_emitted = true;
                        tracing::warn!(
                            addr = format_args!("{addr:#010X}"),
                            pc = format_args!("{pc:#010X}"),
                            "CPU store into flash with cpu_writes: warn — store committed"
                        );
                    }
                }
                CpuWritePolicy::Fault => {
                    Self::record(
                        &mut self.first_flash,
                        &mut self.violations_flash,
                        addr,
                        pc,
                        cycle,
                    );
                    return Some(Err(crate::SimulationError::MemoryViolation(addr)));
                }
            }
        }
        if in_ram {
            match self.ram {
                CpuWritePolicy::Allow => {}
                CpuWritePolicy::Warn => {
                    Self::record(
                        &mut self.first_ram,
                        &mut self.violations_ram,
                        addr,
                        pc,
                        cycle,
                    );
                    if !self.ram_warning_emitted {
                        self.ram_warning_emitted = true;
                        tracing::warn!(
                            addr = format_args!("{addr:#010X}"),
                            pc = format_args!("{pc:#010X}"),
                            "CPU store into ram with cpu_writes: warn — store committed"
                        );
                    }
                }
                CpuWritePolicy::Fault => {
                    Self::record(
                        &mut self.first_ram,
                        &mut self.violations_ram,
                        addr,
                        pc,
                        cycle,
                    );
                    return Some(Err(crate::SimulationError::MemoryViolation(addr)));
                }
            }
        }
        None
    }

    pub fn report(&self) -> Option<MemoryPolicyReport> {
        if !self.active() {
            return None;
        }
        Some(MemoryPolicyReport {
            flash: RegionPolicyStats {
                mode: self.flash,
                violations: self.violations_flash,
                first: self.first_flash.clone(),
            },
            ram: RegionPolicyStats {
                mode: self.ram,
                violations: self.violations_ram,
                first: self.first_ram.clone(),
            },
        })
    }
}

#[allow(dead_code)]
impl crate::bus::SystemBus {
    /// Policy gate for one CPU store. See [`MemoryPolicyState::gate`].
    pub(crate) fn memory_policy_gate_store(&mut self, addr: u64) -> Option<SimResult<()>> {
        if !self.memory_policy.active() {
            return None;
        }
        let extra: Vec<(u64, u64)> = self
            .extra_mem
            .iter()
            .map(|m| (m.base_addr, m.data.len() as u64))
            .collect();
        self.memory_policy.gate(
            addr,
            self.ram.base_addr,
            self.ram.data.len() as u64,
            self.flash.base_addr,
            self.flash.data.len() as u64,
            self.flash_boot_alias && self.flash.base_addr != 0,
            &extra,
        )
    }

    /// True when a CPU store at `addr` must skip the direct-RAM fast paths in
    /// the Cortex-M core (policy is non-default for RAM).
    #[inline(always)]
    pub(crate) fn ram_cpu_writes_blocked(&self, addr: u64) -> bool {
        self.memory_policy
            .ram_blocked(addr, self.ram.base_addr, self.ram.data.len() as u64)
    }

    /// The `result.json` block; `None` on default-allow buses.
    pub fn memory_policy_report(&self) -> Option<MemoryPolicyReport> {
        self.memory_policy.report()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state_flash(mode: CpuWritePolicy) -> MemoryPolicyState {
        MemoryPolicyState {
            flash: mode,
            ..Default::default()
        }
    }

    #[test]
    fn allow_does_not_gate() {
        let mut s = state_flash(CpuWritePolicy::Allow);
        assert!(s
            .gate(
                0x0800_1000,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1_0000,
                false,
                &[]
            )
            .is_none());
        assert_eq!(s.violations_flash, 0);
    }

    #[test]
    fn fault_gates_and_records_first() {
        let mut s = state_flash(CpuWritePolicy::Fault);
        s.note_pc(0x0800_006C);
        assert!(s
            .gate(
                0x0800_1000,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1_0000,
                false,
                &[]
            )
            .is_some());
        assert!(s
            .gate(
                0x0800_1004,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1_0000,
                false,
                &[]
            )
            .is_some());
        assert_eq!(s.violations_flash, 2);
        let first = s.first_flash.clone().unwrap();
        assert_eq!(first.addr, 0x0800_1000);
        assert_eq!(first.pc, Some(0x0800_006C));
    }

    #[test]
    fn warn_commits_and_dedups_the_log() {
        let mut s = state_flash(CpuWritePolicy::Warn);
        assert!(s
            .gate(
                0x0800_1000,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1_0000,
                false,
                &[]
            )
            .is_none());
        assert!(s
            .gate(
                0x0800_1004,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1_0000,
                false,
                &[]
            )
            .is_none());
        assert_eq!(s.violations_flash, 2);
        assert!(s.flash_warning_emitted);
    }

    #[test]
    fn ram_fault_gates_ram_only() {
        let mut s = MemoryPolicyState {
            ram: CpuWritePolicy::Fault,
            ..Default::default()
        };
        assert!(s
            .gate(
                0x2000_0010,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1000,
                false,
                &[]
            )
            .is_some());
        assert!(s
            .gate(
                0x0800_0010,
                0x2000_0000,
                0x1000,
                0x0800_0000,
                0x1000,
                false,
                &[]
            )
            .is_none());
        assert_eq!(s.violations_ram, 1);
    }

    #[test]
    fn boot_alias_writes_are_gated_as_flash() {
        let mut s = state_flash(CpuWritePolicy::Fault);
        assert!(s
            .gate(0x40, 0x2000_0000, 0x1000, 0x0800_0000, 0x1000, true, &[])
            .is_some());
        assert_eq!(s.violations_flash, 1);
    }

    #[test]
    fn report_is_none_until_a_policy_is_set() {
        let s = MemoryPolicyState::default();
        assert!(s.report().is_none());
        let s = state_flash(CpuWritePolicy::Fault);
        assert!(s.report().is_some());
    }
}
