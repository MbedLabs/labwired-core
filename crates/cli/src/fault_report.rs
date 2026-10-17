// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
//
// This software is released under the MIT License.
// See the LICENSE file in the project root for full license information.

//! The Cortex-M fault verdict on the CLI: decode what the CPU recorded, with
//! the firmware's own symbols, and print it for a human.
//!
//! The decode itself is `labwired_core::fault_verdict::decode_fault`; this
//! module only supplies the ELF symbolizer and the terminal rendering.

use labwired_core::fault_verdict::{decode_fault, FaultSymbolizer, FaultVerdict};

/// The verdict for `cpu`, symbolized against `elf` when it parses. `None`
/// when nothing faulted (or the CPU models no ARM fault registers).
///
/// `policy` is the run's memory-policy report: when the fault address matches
/// a recorded read-only-region violation, the verdict names the region.
pub fn fault_verdict<C: labwired_core::Cpu + ?Sized>(
    cpu: &C,
    elf: Option<&[u8]>,
    policy: Option<&labwired_core::bus::MemoryPolicyReport>,
) -> Option<FaultVerdict> {
    let capture = cpu.fault_capture()?;
    let symbols = elf.and_then(|b| labwired_loader::SymbolProvider::from_bytes(b.to_vec()).ok());
    let symbolizer = symbols.as_ref().map(|s| s as &dyn FaultSymbolizer);
    let mut verdict = decode_fault(&capture, symbolizer, None)?;
    if let (Some(report), Some(addr_hex)) = (policy, verdict.fault_address.as_deref()) {
        // `hex` renders `0x` + 8 fixed-width uppercase digits, no separators.
        if let Ok(addr) = u64::from_str_radix(addr_hex.trim_start_matches("0x"), 16) {
            let region = if report.flash.first.as_ref().is_some_and(|v| v.addr == addr) {
                Some("flash")
            } else if report.ram.first.as_ref().is_some_and(|v| v.addr == addr) {
                Some("ram")
            } else {
                None
            };
            if let Some(region) = region {
                verdict = decode_fault(
                    &capture,
                    symbolizer,
                    Some(&labwired_core::fault_verdict::DataFaultReason::ReadOnlyRegion { region }),
                )
                .expect("capture is a fault");
            }
        }
    }
    Some(verdict)
}

/// The two human lines: the sentence, then the raw registers it came from.
pub fn render(v: &FaultVerdict) -> String {
    let flags: Vec<&str> = v.causes.iter().map(|c| c.flag.as_str()).collect();
    let mut regs = format!("HFSR={} CFSR={}", v.registers.hfsr, v.registers.cfsr);
    if !flags.is_empty() {
        regs.push_str(&format!(" [{}]", flags.join(" ")));
    }
    if let (Some(addr), Some(src)) = (&v.fault_address, &v.fault_address_source) {
        regs.push_str(&format!(" {src}={addr}"));
    }
    format!("FAULT  {}\n       {}", v.summary, regs)
}

/// Print the verdict to stderr (stdout carries firmware UART and `--json`).
pub fn eprint(v: &FaultVerdict) {
    eprintln!("{}", render(v));
}
