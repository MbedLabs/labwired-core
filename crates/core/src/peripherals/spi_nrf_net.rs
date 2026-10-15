// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
// SPDX-License-Identifier: MIT

//! The nRF52 SPIM (EasyDMA SPI master) on a world `gpio_net`: a bit-level
//! master between chips, on the shared wire engine
//! ([`crate::peripherals::spi_bitbus`]).
//!
//! Off a net `do_nrf52_easydma` exchanges the whole buffer with attached
//! device models inside one call. When a pad the SPIM's `PSEL.SCK`,
//! `PSEL.MOSI` or `PSEL.MISO` names is on a net (and `ENABLE = 7`), the wire
//! belongs to several chips and `TASKS_START` runs this engine instead:
//!
//! * EasyDMA reads `TXD.MAXCNT` bytes at `TXD.PTR` when the transfer starts
//!   (`EVENTS_STARTED`). The transfer is `max(TXD.MAXCNT, RXD.MAXCNT)` bytes;
//!   past the TX buffer `ORC` goes out.
//! * Each byte is clocked at the `FREQUENCY` rate (64 MHz core clock taken
//!   as the engine clock) in the `CONFIG` shape (CPOL, CPHA, bit order),
//!   back to back. MISO is sampled at the level the net delivers to its pad.
//! * At the end the received bytes land at `RXD.PTR` (up to `RXD.MAXCNT`),
//!   `TXD.AMOUNT` / `RXD.AMOUNT` are set and `EVENTS_ENDTX`, `EVENTS_ENDRX`
//!   and `EVENTS_END` fire, with the interrupt where `INTEN` asks for it.
//! * `TASKS_STOP` ends the transfer after the byte on the wire:
//!   `EVENTS_STOPPED`, with the amounts of what crossed.
//!
//! Chip select is the firmware's GPIO, as on SPIM0..2 (`PSEL.CSN`, the
//! SPIM3-only hardware CSN, is stored and not driven). SCK and MOSI are
//! driven push-pull while the SPIM is enabled; MISO is an input.

use super::{Spi, SpiRegs, SpiSignal, INTEN_END, INTEN_ENDRX, INTEN_ENDTX, INTEN_STOPPED};
use crate::peripherals::pad_lines::{LineDrive, PadLines};
use crate::peripherals::spi_bitbus::{SpiBitBus, SpiEvent, SpiLines, SpiShape};

/// Scheduler token for the nRF net engine.
pub(super) const NRF_NET_TOKEN: u32 = 3;

/// nRF52 SPIM `INTEN.STARTED` (bit 19, PS v1.11 §6.25.6.9).
pub(super) const INTEN_STARTED: u32 = 1 << 19;

const LINES: SpiLines = SpiLines {
    sck: SpiSignal::Sck as usize,
    mosi: SpiSignal::Mosi as usize,
    miso: SpiSignal::Miso as usize,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Phase {
    #[default]
    Idle,
    /// `TASKS_START` was written: EasyDMA reads the TX buffer next.
    Starting,
    /// Bytes are on the wire.
    Running,
    /// The last byte is in: EasyDMA writes the RX buffer next.
    Finishing,
}

/// The net engine's state, kept beside the register file.
#[derive(Debug, Default)]
pub(super) struct SpimNet {
    bus: SpiBitBus,
    phase: Phase,
    tx: Vec<u8>,
    rx: Vec<u8>,
    /// `TASKS_STOP` came while a byte was on the wire.
    stop: bool,
    armed: Vec<u64>,
    irq_armed: bool,
}

impl Spi {
    fn nrf_net_pads(&self) -> Option<std::sync::Arc<PadLines>> {
        self.lines.as_ref().map(|l| l.pad_lines().clone())
    }

    /// `true` when this nRF52 SPIM is enabled on pads that sit on a world
    /// `gpio_net`.
    pub(super) fn nrf_net_mode(&self) -> bool {
        matches!(&self.regs, SpiRegs::Nrf52(r) if r.map == super::NrfSpimMap::Nrf52)
            && (self.nrf52_claim_sck.on_net()
                || self.nrf52_claim_mosi.on_net()
                || self.nrf52_claim_miso.on_net())
    }

    fn nrf_net_now(&self) -> u64 {
        self.h5_clock.as_ref().map_or(0, crate::CycleClock::now)
    }

    fn nrf_net_shape(&self) -> (SpiShape, u64) {
        let config = match &self.regs {
            SpiRegs::Nrf52(r) => r.config,
            _ => 0,
        };
        let shape = SpiShape {
            cpol: config & super::NRF52_CONFIG_CPOL != 0,
            cpha: config & super::NRF52_CONFIG_CPHA != 0,
            lsb_first: config & super::NRF52_CONFIG_ORDER_LSB != 0,
            bits: 8,
        };
        (shape, (self.nrf52_sck_bit_cycles() / 2).max(1))
    }

    /// Re-state the line drives after a register write in net mode: SCK at
    /// CPOL and MOSI driven while enabled, MISO an input.
    pub(super) fn nrf_net_apply_drive(&mut self) {
        let Some(pads) = self.nrf_net_pads() else {
            return;
        };
        if !self.nrf_net_mode() {
            return;
        }
        let (shape, half) = self.nrf_net_shape();
        if !self.nrf_net.bus.master_busy() {
            self.nrf_net.bus.shape = shape;
            self.nrf_net.bus.half = half;
        }
        pads.set_stamp(Some(self.nrf_net_now()));
        self.nrf_net.bus.m_idle_drive(LINES, &pads);
        pads.set_stamp(None);
    }

    /// `TASKS_START` in net mode.
    pub(super) fn nrf_net_start(&mut self) {
        if self.nrf_net.phase == Phase::Idle {
            self.nrf_net.phase = Phase::Starting;
            self.nrf_net.stop = false;
        }
    }

    /// `TASKS_STOP` in net mode: after the byte on the wire, or now.
    pub(super) fn nrf_net_stop(&mut self) {
        match self.nrf_net.phase {
            Phase::Idle => {
                if let SpiRegs::Nrf52(r) = &mut self.regs {
                    r.events_stopped = 1;
                }
            }
            Phase::Starting => {
                self.nrf_net.phase = Phase::Idle;
                if let SpiRegs::Nrf52(r) = &mut self.regs {
                    r.events_stopped = 1;
                }
            }
            Phase::Running | Phase::Finishing => self.nrf_net.stop = true,
        }
    }

    /// `true` while EasyDMA has RAM to move.
    pub(super) fn nrf_net_needs_bus(&self) -> bool {
        matches!(self.nrf_net.phase, Phase::Starting | Phase::Finishing)
    }

    /// `true` while a transfer is anywhere between `TASKS_START` and its end.
    pub(super) fn nrf_net_busy(&self) -> bool {
        self.nrf_net.phase != Phase::Idle || self.nrf_net.bus.master_busy()
    }

    /// The interrupt level: an event whose `INTEN` bit is set.
    pub(super) fn nrf_net_irq(&self) -> bool {
        match &self.regs {
            SpiRegs::Nrf52(r) => {
                (r.events_stopped != 0 && r.inten & INTEN_STOPPED != 0)
                    || (r.events_endrx != 0 && r.inten & INTEN_ENDRX != 0)
                    || (r.events_end != 0 && r.inten & INTEN_END != 0)
                    || (r.events_endtx != 0 && r.inten & INTEN_ENDTX != 0)
                    || (r.events_started != 0 && r.inten & INTEN_STARTED != 0)
            }
            _ => false,
        }
    }

    /// EasyDMA: read the TX buffer at the start, write the RX buffer at the
    /// end. Called from `tick_with_bus` and from the scheduler event.
    pub(super) fn nrf_net_dma(&mut self, bus: &mut dyn crate::Bus) {
        let Some(pads) = self.nrf_net_pads() else {
            return;
        };
        let SpiRegs::Nrf52(r) = &self.regs else {
            return;
        };
        let (txd_ptr, txd_max, rxd_ptr, rxd_max, orc) = (
            u64::from(r.txd_ptr),
            r.txd_maxcnt as usize & 0xFFFF,
            u64::from(r.rxd_ptr),
            r.rxd_maxcnt as usize & 0xFFFF,
            (r.orc & 0xFF) as u8,
        );
        let now = self.nrf_net_now();
        match self.nrf_net.phase {
            Phase::Starting => {
                let n = txd_max.max(rxd_max);
                self.nrf_net.tx = (0..n)
                    .map(|i| {
                        if i < txd_max {
                            bus.read_u8(txd_ptr + i as u64).unwrap_or(0)
                        } else {
                            orc
                        }
                    })
                    .collect();
                self.nrf_net.rx.clear();
                if let SpiRegs::Nrf52(r) = &mut self.regs {
                    r.events_started = 1;
                }
                let (shape, half) = self.nrf_net_shape();
                self.nrf_net.bus.shape = shape;
                self.nrf_net.bus.half = half;
                if n == 0 {
                    self.nrf_net.phase = Phase::Finishing;
                    self.nrf_net_dma(bus);
                    return;
                }
                self.nrf_net.phase = Phase::Running;
                pads.set_stamp(Some(now));
                let first = u16::from(self.nrf_net.tx[0]);
                self.nrf_net.bus.m_begin(first, now, LINES, &pads);
                pads.set_stamp(None);
            }
            Phase::Finishing => {
                let done = self.nrf_net.rx.len();
                for (i, &b) in self.nrf_net.rx.iter().take(rxd_max).enumerate() {
                    let _ = bus.write_u8(rxd_ptr + i as u64, b);
                }
                let stopped = std::mem::take(&mut self.nrf_net.stop);
                let complete = done >= self.nrf_net.tx.len();
                if let SpiRegs::Nrf52(r) = &mut self.regs {
                    r.txd_amount = done.min(txd_max) as u32;
                    r.rxd_amount = done.min(rxd_max) as u32;
                    if done >= txd_max {
                        r.events_endtx = 1;
                    }
                    if done >= rxd_max {
                        r.events_endrx = 1;
                    }
                    if complete {
                        r.events_end = 1;
                    }
                    if stopped {
                        r.events_stopped = 1;
                    }
                }
                self.nrf_net.phase = Phase::Idle;
            }
            _ => {}
        }
    }

    fn nrf_net_step(&mut self, now: u64) {
        let Some(pads) = self.nrf_net_pads() else {
            return;
        };
        while let Some(due) = self.nrf_net.bus.deadline() {
            if due > now {
                break;
            }
            self.nrf_net.bus.step(due, LINES, &pads);
            for event in self.nrf_net.bus.take_events() {
                let SpiEvent::MDone(word) = event else {
                    continue;
                };
                self.nrf_net.rx.push(word as u8);
                let next = self.nrf_net.rx.len();
                if next < self.nrf_net.tx.len() && !self.nrf_net.stop {
                    let byte = u16::from(self.nrf_net.tx[next]);
                    self.nrf_net.bus.m_begin(byte, due, LINES, &pads);
                } else {
                    self.nrf_net.phase = Phase::Finishing;
                }
            }
        }
        pads.set_stamp(None);
    }

    /// Walk path: one tick.
    pub(super) fn nrf_net_tick(&mut self) {
        let now = self.nrf_net_now();
        if self.nrf_net.bus.deadline().is_some_and(|d| d <= now) {
            self.nrf_net_step(now);
        }
    }

    /// Scheduler path: events to arm (delays from now).
    pub(super) fn nrf_net_take_events(&mut self) -> Vec<(u64, u32)> {
        let now = self.nrf_net_now();
        let due = if self.nrf_net_needs_bus() {
            Some(now + 1)
        } else {
            self.nrf_net.bus.deadline()
        };
        let mut events = Vec::new();
        if let Some(due) = due {
            let due = due.max(now + 1);
            if !self.nrf_net.armed.contains(&due) {
                self.nrf_net.armed.push(due);
                events.push((due - now - 1, NRF_NET_TOKEN));
            }
        } else if self.nrf_net_irq() && !self.nrf_net.irq_armed {
            self.nrf_net.irq_armed = true;
            events.push((0, NRF_NET_TOKEN));
        }
        events
    }

    /// Scheduler path: an armed event fired at `now`.
    pub(super) fn nrf_net_on_event(
        &mut self,
        now: u64,
        bus: &mut dyn crate::Bus,
    ) -> crate::sched::EventResult {
        self.nrf_net.armed.retain(|&t| t > now);
        self.nrf_net.irq_armed = false;
        self.nrf_net_step(now);
        if self.nrf_net_needs_bus() {
            self.nrf_net_dma(bus);
        }
        let irq = self.nrf_net_irq();
        let mut res = crate::sched::EventResult {
            raise_own_irq: irq,
            ..Default::default()
        };
        let next = if self.nrf_net_needs_bus() {
            Some(now + 1)
        } else {
            self.nrf_net.bus.deadline()
        };
        match next {
            Some(due) => {
                let due = due.max(now + 1);
                if !self.nrf_net.armed.contains(&due) {
                    self.nrf_net.armed.push(due);
                    res.reschedule_delay = Some(due - now);
                }
            }
            None if irq => {
                // A level: re-raised while the event and its enable hold.
                self.nrf_net.irq_armed = true;
                res.reschedule_delay = Some(1);
            }
            None => {}
        }
        res
    }

    /// The engine forgets a transfer when the SPIM is disabled.
    pub(super) fn nrf_net_disable(&mut self) {
        self.nrf_net.bus.reset();
        self.nrf_net.phase = Phase::Idle;
        self.nrf_net.stop = false;
        if let Some(pads) = self.nrf_net_pads() {
            for line in [LINES.sck, LINES.mosi, LINES.miso] {
                pads.set_mode(line, LineDrive::Input);
            }
        }
    }
}
