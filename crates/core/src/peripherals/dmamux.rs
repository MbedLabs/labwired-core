// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
// SPDX-License-Identifier: MIT

//! STM32 DMA request multiplexer (DMAMUX) — G0 / G4 / WB / L4+ (RM0444 §11,
//! RM0440 §13).
//!
//! On these parts a DMA channel has no fixed peripheral: DMAMUX channel `x`
//! feeds DMA channel `x + 1`, and its `DMAMUX_CxCR.DMAREQ_ID` names the
//! peripheral request that drives it. The model is that routing table:
//! [`Peripheral::dma_route_request`] answers "which DMA channels does
//! `spi1_tx` drive right now" by scanning the `CxCR` registers for the
//! request's DMAREQ_ID (from the chip yaml's `requests:` table, the
//! datasheet's "DMAMUX: assignment of multiplexer inputs to resources"),
//! naming the DMA controller the multiplexer feeds (`dma:`).
//!
//! Register map (RM0444 §11.6, G071 SVD): `C0CR..` at `4·x`, `CSR` 0x80
//! (read-only), `CFR` 0x84 (write-only), `RG0CR..RG3CR` 0x100, `RGSR` 0x140
//! (read-only), `RGCFR` 0x144 (write-only), and the identification block at
//! the top of the window (`HWCFGR2` 0x3EC .. `SIDR` 0x3FC, constants from the
//! chip yaml's `id_registers`).
//!
//! Not modelled: request synchronisation (`SE`, `SYNC_ID`, `NBREQ`), the
//! request generators (`RGxCR` are stored, but generate nothing), and the
//! synchronisation-overrun flags (`CSR` / `RGSR` stay 0).

use crate::{DmaRoute, Peripheral, SimResult};

/// `DMAMUX_CxCR` writable bits (G071 SVD): DMAREQ_ID[7:0], SOIE, EGE, SE,
/// SPOL[1:0], NBREQ[4:0], SYNC_ID[4:0].
const CXCR_MASK: u32 = 0x1FFF_03FF;
/// `DMAREQ_ID` field.
const DMAREQ_ID_MASK: u32 = 0xFF;
/// `DMAMUX_RGxCR` writable bits: SIG_ID[4:0], OIE, GE, GPOL[1:0], GNBREQ[4:0].
const RGXCR_MASK: u32 = 0x00FF_011F;
/// Request generators on every DMAMUX this models.
const GENERATORS: usize = 4;
const CSR: u64 = 0x80;
const CFR: u64 = 0x84;
const RG_BASE: u64 = 0x100;
const RGSR: u64 = 0x140;
const RGCFR: u64 = 0x144;

#[derive(Debug, Default, serde::Serialize)]
pub struct Dmamux {
    cxcr: Vec<u32>,
    rgxcr: [u32; GENERATORS],
    /// Request name (`spi1_tx`) → DMAREQ_ID.
    #[serde(skip)]
    requests: Vec<(String, u32)>,
    /// The DMA controller (peripheral id) DMAMUX channel `x` feeds as its
    /// channel `x + 1`.
    #[serde(skip)]
    dma: String,
    /// Read-only identification registers `(offset, value)`.
    #[serde(skip)]
    id_registers: Vec<(u64, u32)>,
}

impl Dmamux {
    /// A multiplexer with `channels` request lines feeding the DMA
    /// controller `dma`.
    pub fn new(channels: usize, dma: &str) -> Self {
        Self {
            cxcr: vec![0; channels],
            dma: dma.to_string(),
            ..Default::default()
        }
    }

    /// Name the DMAREQ_ID of a peripheral request (`spi1_tx` → 17 on G0).
    pub fn with_request(mut self, request: &str, id: u32) -> Self {
        self.requests.push((request.to_string(), id));
        self
    }

    /// A read-only identification register (HWCFGRx / VERR / IPIDR / SIDR).
    pub fn with_id_register(mut self, offset: u64, value: u32) -> Self {
        self.id_registers.push((offset, value));
        self
    }

    fn read_reg(&self, offset: u64) -> u32 {
        let word = (offset / 4) as usize;
        if word < self.cxcr.len() {
            return self.cxcr[word];
        }
        if (RG_BASE..RG_BASE + 4 * GENERATORS as u64).contains(&offset) {
            return self.rgxcr[((offset - RG_BASE) / 4) as usize];
        }
        match offset {
            // No synchronisation / generator overrun is modelled; the
            // clear-flag registers are write-only.
            CSR | CFR | RGSR | RGCFR => 0,
            _ => self
                .id_registers
                .iter()
                .find(|(o, _)| *o == offset)
                .map_or(0, |(_, v)| *v),
        }
    }

    fn write_reg(&mut self, offset: u64, value: u32) {
        let word = (offset / 4) as usize;
        if word < self.cxcr.len() {
            self.cxcr[word] = value & CXCR_MASK;
        } else if (RG_BASE..RG_BASE + 4 * GENERATORS as u64).contains(&offset) {
            self.rgxcr[((offset - RG_BASE) / 4) as usize] = value & RGXCR_MASK;
        }
    }
}

impl Peripheral for Dmamux {
    fn read(&self, offset: u64) -> SimResult<u8> {
        let v = self.read_reg(offset & !3);
        Ok((v >> ((offset & 3) * 8)) as u8)
    }

    fn write(&mut self, offset: u64, value: u8) -> SimResult<()> {
        let reg = offset & !3;
        let shift = (offset & 3) * 8;
        let v = (self.read_reg(reg) & !(0xFF << shift)) | (u32::from(value) << shift);
        self.write_reg(reg, v);
        Ok(())
    }

    fn write_u32(&mut self, offset: u64, value: u32) -> SimResult<()> {
        self.write_reg(offset & !3, value);
        Ok(())
    }

    fn dma_route_request(&self, request: &str, out: &mut Vec<DmaRoute>) {
        let Some(&(_, id)) = self.requests.iter().find(|(name, _)| name == request) else {
            return;
        };
        for (x, cr) in self.cxcr.iter().enumerate() {
            // DMAREQ_ID 0 is "no request" (RM0444 Table 59), so a channel left
            // at reset never routes.
            if id != 0 && cr & DMAREQ_ID_MASK == id {
                out.push(DmaRoute {
                    controller: Some(self.dma.clone()),
                    channel: x as u32 + 1,
                });
            }
        }
    }

    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }

    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }

    fn snapshot(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn g0() -> Dmamux {
        Dmamux::new(7, "dma1")
            .with_request("spi1_rx", 16)
            .with_request("spi1_tx", 17)
            .with_id_register(0x3FC, 0xA3C5_DD01)
    }

    fn routes(m: &Dmamux, request: &str) -> Vec<DmaRoute> {
        let mut out = Vec::new();
        m.dma_route_request(request, &mut out);
        out
    }

    #[test]
    fn a_request_drives_the_dma_channel_whose_cxcr_selects_it() {
        let mut m = g0();
        assert!(
            routes(&m, "spi1_tx").is_empty(),
            "nothing selected at reset"
        );
        m.write_u32(0x04, 17).unwrap(); // C1CR: SPI1_TX → DMA channel 2
        m.write_u32(0x00, 16).unwrap(); // C0CR: SPI1_RX → DMA channel 1
        assert_eq!(
            routes(&m, "spi1_tx"),
            vec![DmaRoute {
                controller: Some("dma1".into()),
                channel: 2
            }]
        );
        assert_eq!(routes(&m, "spi1_rx")[0].channel, 1);
        assert!(routes(&m, "spi2_tx").is_empty(), "an unknown request");
        m.write_u32(0x04, 0).unwrap();
        assert!(routes(&m, "spi1_tx").is_empty());
    }

    #[test]
    fn registers_store_under_their_masks_and_id_registers_are_constant() {
        let mut m = g0();
        m.write_u32(0x00, 0xFFFF_FFFF).unwrap();
        assert_eq!(m.read_reg(0x00), CXCR_MASK);
        m.write_u32(0x100, 0xFFFF_FFFF).unwrap();
        assert_eq!(m.read_reg(0x100), RGXCR_MASK);
        m.write_u32(0x84, 0xFFFF_FFFF).unwrap();
        assert_eq!(m.read_reg(0x84), 0, "CFR is write-only");
        m.write_u32(0x3FC, 0).unwrap();
        assert_eq!(m.read_reg(0x3FC), 0xA3C5_DD01);
        // A byte write merges into the word.
        m.write(0x01, 0x02).unwrap();
        assert_eq!(m.read_reg(0x00), (CXCR_MASK & !0xFF00) | 0x0200);
    }
}
