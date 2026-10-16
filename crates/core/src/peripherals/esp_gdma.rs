// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
// SPDX-License-Identifier: MIT

//! What the ESP32-C3 and ESP32-C6 GDMA models share: the Espressif linked-list
//! descriptor walk, and the coupling of a channel bound to the GP-SPI2
//! (`PERI_SEL` = 0) to the controller model
//! ([`crate::peripherals::esp32c3::spi::Esp32c3Spi`], which both chips use).
//!
//! ## Descriptors (C3 TRM §2.4.3, C6 TRM §4.4.3)
//!
//! Three words in RAM: `dw0` owner (bit 31, 1 = DMA), `suc_eof` (bit 30),
//! `length` (bits 23:12, bytes valid), `size` (bits 11:0, buffer capacity);
//! `dw1` the buffer address; `dw2` the next descriptor, 0 at the end.
//!
//! * **OUT** (memory → peripheral): the walk reads `length` bytes from each
//!   buffer and stops after the descriptor carrying `suc_eof` (the end of the
//!   frame) or at `next = 0`. With `OUT_AUTO_WRBACK` it hands each descriptor
//!   back to the CPU (owner cleared).
//! * **IN** (peripheral → memory): the walk fills each buffer up to `size`,
//!   then writes `dw0` back with the owner cleared, `length` = the bytes it
//!   received, and `suc_eof` on the descriptor that took the last byte.
//! * With `*_CHECK_OWNER` set, a descriptor the CPU owns ends the walk.
//!
//! ## The GP-SPI2 coupling
//!
//! The channel moves data between memory and the controller's DMA FIFOs, not
//! through MMIO: the GP-SPI has no data-port register. When a channel bound
//! to SPI2 starts:
//!
//! * its **OUT** link is walked at once and the bytes are pushed into the
//!   controller's transmit FIFO ([`crate::Peripheral::gdma_out_load`]) — the
//!   prefetch silicon does before `SPI_CMD.USR` — and `OUT_EOF`,
//!   `OUT_TOTAL_EOF` and `OUT_DONE` latch;
//! * its **IN** link waits for a transaction with `DMA_RX_ENA` to end, then
//!   receives its bytes ([`crate::Peripheral::gdma_in_take`]), latches
//!   `IN_DONE` and `IN_SUC_EOF`, and lets the controller complete the
//!   transaction ([`crate::Peripheral::gdma_in_delivered`]: `USR` clears,
//!   `SPI_TRANS_DONE`), so the data is in memory when firmware sees the
//!   transaction done.
//!
//! The controller is reached through [`crate::Bus::with_peripheral_mut`] by
//! its bus id, `spi2` on both chips.
//!
//! Not modelled: `IN_DSCR_EMPTY` / `*_DSCR_ERR` (an IN chain too short drops
//! the excess; a malformed chain just ends the walk), the FIFO status
//! registers' live levels, and burst / priority settings.

use crate::Bus;

/// `dw0` owner bit: 1 = the DMA owns the descriptor.
pub(crate) const DESC_OWNER: u32 = 1 << 31;
/// `dw0` `suc_eof`.
pub(crate) const DESC_SUC_EOF: u32 = 1 << 30;
/// `dw0` `length` field (bits 23:12).
const DESC_LEN_MASK: u32 = 0xFFF << 12;
/// Runaway-chain guard: a list that loops back on itself is walked at most
/// this many descriptors.
const MAX_CHAIN: usize = 4096;

/// A finished OUT walk.
#[derive(Debug, Default)]
pub(crate) struct OutWalk {
    pub bytes: Vec<u8>,
    /// Address of the last descriptor consumed (`OUT_EOF_DES_ADDR`).
    pub last_desc: u32,
}

/// Walk an OUT (transmit) list from `first`, collecting its bytes.
pub(crate) fn walk_out(
    bus: &mut dyn Bus,
    first: u32,
    check_owner: bool,
    auto_wrback: bool,
) -> OutWalk {
    let mut out = OutWalk::default();
    let mut desc = first;
    for _ in 0..MAX_CHAIN {
        if desc == 0 {
            break;
        }
        let Ok(dw0) = bus.read_u32(u64::from(desc)) else {
            break;
        };
        if check_owner && dw0 & DESC_OWNER == 0 {
            break;
        }
        let buf = bus.read_u32(u64::from(desc) + 4).unwrap_or(0);
        let next = bus.read_u32(u64::from(desc) + 8).unwrap_or(0);
        let len = (dw0 & DESC_LEN_MASK) >> 12;
        for k in 0..len {
            out.bytes.push(bus.read_u8(u64::from(buf + k)).unwrap_or(0));
        }
        if auto_wrback {
            let _ = bus.write_u32(u64::from(desc), dw0 & !DESC_OWNER);
        }
        out.last_desc = desc;
        if dw0 & DESC_SUC_EOF != 0 {
            break;
        }
        desc = next;
    }
    out
}

/// Fill an IN (receive) list from `first` with `data`, writing each used
/// descriptor back. Returns the address of the descriptor that took the
/// last byte (`IN_SUC_EOF_DES_ADDR`).
pub(crate) fn fill_in(bus: &mut dyn Bus, first: u32, data: &[u8], check_owner: bool) -> u32 {
    let mut desc = first;
    let mut at = 0usize;
    let mut last = first;
    for _ in 0..MAX_CHAIN {
        if desc == 0 {
            break;
        }
        let Ok(dw0) = bus.read_u32(u64::from(desc)) else {
            break;
        };
        if check_owner && dw0 & DESC_OWNER == 0 {
            break;
        }
        let buf = bus.read_u32(u64::from(desc) + 4).unwrap_or(0);
        let next = bus.read_u32(u64::from(desc) + 8).unwrap_or(0);
        let size = (dw0 & 0xFFF) as usize;
        let n = size.min(data.len() - at);
        for (k, b) in data[at..at + n].iter().enumerate() {
            let _ = bus.write_u8(u64::from(buf) + k as u64, *b);
        }
        at += n;
        let done = at == data.len();
        let mut wb = (dw0 & !(DESC_OWNER | DESC_SUC_EOF | DESC_LEN_MASK)) | ((n as u32) << 12);
        if done {
            wb |= DESC_SUC_EOF;
        }
        let _ = bus.write_u32(u64::from(desc), wb);
        last = desc;
        if done {
            break;
        }
        desc = next;
    }
    last
}

/// The GP-SPI2's bus id on both chips.
const GP_SPI2: &str = "spi2";

/// Push an OUT link's bytes into the GP-SPI2's transmit FIFO.
pub(crate) fn spi2_out_load(bus: &mut dyn Bus, bytes: &[u8]) {
    bus.with_peripheral_mut(GP_SPI2, &mut |spi| spi.gdma_out_load(bytes));
}

/// The bytes a finished GP-SPI2 DMA transaction holds for an IN link.
pub(crate) fn spi2_in_take(bus: &mut dyn Bus) -> Option<Vec<u8>> {
    let mut out = None;
    bus.with_peripheral_mut(GP_SPI2, &mut |spi| out = spi.gdma_in_take());
    out
}

/// Tell the GP-SPI2 its received bytes are in memory.
pub(crate) fn spi2_in_delivered(bus: &mut dyn Bus) {
    bus.with_peripheral_mut(GP_SPI2, &mut |spi| spi.gdma_in_delivered());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bus::SystemBus;

    fn desc(bus: &mut SystemBus, at: u32, dw0: u32, buf: u32, next: u32) {
        bus.write_u32(u64::from(at), dw0).unwrap();
        bus.write_u32(u64::from(at) + 4, buf).unwrap();
        bus.write_u32(u64::from(at) + 8, next).unwrap();
    }

    #[test]
    fn an_out_list_is_read_up_to_its_eof_descriptor() {
        let mut bus = SystemBus::new();
        let base = 0x2000_0000u32;
        for (i, b) in [1u8, 2, 3, 4, 5].iter().enumerate() {
            bus.write_u8(u64::from(base + 0x100) + i as u64, *b)
                .unwrap();
        }
        desc(
            &mut bus,
            base,
            DESC_OWNER | (3 << 12) | 8,
            base + 0x100,
            base + 0x10,
        );
        desc(
            &mut bus,
            base + 0x10,
            DESC_OWNER | DESC_SUC_EOF | (2 << 12) | 8,
            base + 0x103,
            base + 0x20,
        );
        desc(
            &mut bus,
            base + 0x20,
            DESC_OWNER | (1 << 12) | 8,
            base + 0x100,
            0,
        );
        let w = walk_out(&mut bus, base, true, true);
        assert_eq!(w.bytes, vec![1, 2, 3, 4, 5], "stops after suc_eof");
        assert_eq!(w.last_desc, base + 0x10);
        assert_eq!(
            bus.read_u32(u64::from(base)).unwrap() & DESC_OWNER,
            0,
            "auto write-back returns it to the CPU"
        );
        // A CPU-owned first descriptor with CHECK_OWNER: nothing.
        assert!(walk_out(&mut bus, base, true, false).bytes.is_empty());
    }

    #[test]
    fn an_in_list_is_filled_and_written_back_with_eof_on_the_last() {
        let mut bus = SystemBus::new();
        let base = 0x2000_0000u32;
        desc(&mut bus, base, DESC_OWNER | 4, base + 0x100, base + 0x10);
        desc(&mut bus, base + 0x10, DESC_OWNER | 4, base + 0x104, 0);
        let last = fill_in(&mut bus, base, &[9, 8, 7, 6, 5, 4], true);
        assert_eq!(last, base + 0x10);
        let dw0a = bus.read_u32(u64::from(base)).unwrap();
        let dw0b = bus.read_u32(u64::from(base) + 0x10).unwrap();
        assert_eq!(dw0a, (4 << 12) | 4, "full, CPU-owned, no EOF");
        assert_eq!(dw0b, DESC_SUC_EOF | (2 << 12) | 4, "2 bytes, EOF");
        let got: Vec<u8> = (0..6)
            .map(|i| bus.read_u8(u64::from(base + 0x100) + i).unwrap())
            .collect();
        assert_eq!(got, vec![9, 8, 7, 6, 5, 4]);
    }
}
