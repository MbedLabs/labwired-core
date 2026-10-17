// SPDX-License-Identifier: MIT
//! SAMD51 SERCOM SPI master, byte-level controller qualification slice.
//!
//! Register facts: DS60001507 / CMSIS SERCOM SPI map. Transfers require MODE=3,
//! ENABLE, 8-bit CHSIZE, and the configured PORT mux/DOPO. The bus owns live
//! MCLK/GCLK gating. No wire-edge fidelity, dynamic GCLK frequency derivation,
//! DMA, split-vector IRQ delivery, or complete native Arcade qualification is
//! claimed here. INTEN/INTFLAG are modelled for polling only. The delay is
//! in nominal SERCOM core-clock cycles; CPU/kernel clock scaling remains owed.

use crate::peripherals::spi::SpiDevice;
use crate::{Bus, Peripheral, PeripheralTickResult, SimResult};
use std::cell::{Cell, RefCell};
use std::collections::VecDeque;

const DRE: u8 = 1;
const TXC: u8 = 2;
const RXC: u8 = 4;
const ERROR: u8 = 128;
const BUFOVF: u16 = 4;

/// Board-declared output pads; no hard-coded PyBadge pins in the controller.
#[derive(Debug, Clone, Copy)]
pub struct SamSpiPads {
    pub port_base: u64,
    pub sck: u8,
    pub mosi: u8,
    pub mux: u8,
    pub dopo: u8,
}

pub struct SamSercomSpi {
    ctrla: u32,
    ctrlb: u32,
    baud: u8,
    inten: u8,
    flags: u8,
    status: u16,
    rx: RefCell<VecDeque<u8>>,
    last_rx: Cell<u8>,
    holding: Option<u8>,
    active: Option<(u8, u64)>,
    pads: SamSpiPads,
    devices: Vec<Box<dyn SpiDevice>>,
    selected: Vec<bool>,
}

impl std::fmt::Debug for SamSercomSpi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SamSercomSpi")
            .field("ctrla", &self.ctrla)
            .field("flags", &self.effective_flags())
            .finish()
    }
}

impl SamSercomSpi {
    pub fn new(pads: SamSpiPads) -> Self {
        Self {
            ctrla: 0,
            ctrlb: 0,
            baud: 0,
            inten: 0,
            flags: 0,
            status: 0,
            rx: RefCell::new(VecDeque::new()),
            last_rx: Cell::new(0),
            holding: None,
            active: None,
            pads,
            devices: Vec::new(),
            selected: Vec::new(),
        }
    }

    pub fn push_device(&mut self, device: Box<dyn SpiDevice>) {
        self.devices.push(device);
        self.selected.push(false);
    }

    fn enabled(&self) -> bool {
        self.ctrla & 2 != 0 && (self.ctrla >> 2) & 7 == 3 && self.ctrlb & 7 == 0
    }

    fn effective_flags(&self) -> u8 {
        self.flags
            | if self.enabled() && self.holding.is_none() {
                DRE
            } else {
                0
            }
            | if !self.rx.borrow().is_empty() { RXC } else { 0 }
    }

    fn reg(offset: u64) -> (u64, u32) {
        let base = match offset {
            0..=3 => 0,
            4..=7 => 4,
            0x0c => 0x0c,
            0x14 => 0x14,
            0x16 => 0x16,
            0x18 => 0x18,
            0x1a..=0x1b => 0x1a,
            0x1c..=0x1f => 0x1c,
            0x28..=0x2b => 0x28,
            _ => offset,
        };
        (base, ((offset - base) * 8) as u32)
    }

    fn value(&self, base: u64, consume: bool) -> u32 {
        match base {
            0 => self.ctrla,
            4 => self.ctrlb,
            0x0c => self.baud.into(),
            0x14 | 0x16 => self.inten.into(),
            0x18 => self.effective_flags().into(),
            0x1a => self.status.into(),
            0x1c => 0,
            0x28 => {
                if consume {
                    if let Some(v) = self.rx.borrow_mut().pop_front() {
                        self.last_rx.set(v);
                    }
                }
                self.last_rx.get().into()
            }
            _ => 0,
        }
    }

    fn store(&mut self, offset: u64, value: u32, width: u32) {
        let (base, shift) = Self::reg(offset);
        let mask = (if width == 32 {
            u32::MAX
        } else {
            (1u32 << width) - 1
        }) << shift;
        let bits = (value << shift) & mask;
        match base {
            0 => {
                let next = (self.ctrla & !mask) | bits;
                if next & 1 != 0 {
                    for (dev, selected) in self.devices.iter_mut().zip(&mut self.selected) {
                        if *selected {
                            dev.cs_release();
                            *selected = false;
                        }
                    }
                    self.ctrla = 0;
                    self.ctrlb = 0;
                    self.baud = 0;
                    self.inten = 0;
                    self.flags = 0;
                    self.status = 0;
                    self.rx.borrow_mut().clear();
                    self.last_rx.set(0);
                    self.holding = None;
                    self.active = None;
                } else {
                    // Configuration is enable-protected; ENABLE itself is writable.
                    self.ctrla = if self.ctrla & 2 != 0 {
                        (self.ctrla & !2) | (next & 2)
                    } else {
                        next & !1
                    };
                    if self.ctrla & 2 == 0 {
                        self.holding = None;
                        self.active = None;
                    }
                }
            }
            4 if self.ctrla & 2 == 0 => self.ctrlb = (self.ctrlb & !mask) | bits,
            0x0c if self.ctrla & 2 == 0 => self.baud = bits as u8,
            0x14 => self.inten &= !(bits as u8),
            0x16 => self.inten |= bits as u8 & (DRE | TXC | RXC | ERROR),
            0x18 => self.flags &= !(bits as u8 & (TXC | ERROR)),
            0x1a => self.status &= !(bits as u16 & BUFOVF),
            0x28 if shift == 0 && self.enabled() && self.holding.is_none() => {
                self.holding = Some(value as u8);
                self.flags &= !TXC;
            }
            _ => {}
        }
    }

    fn pads_ready(&self, bus: &dyn Bus) -> bool {
        if (self.ctrla >> 16) & 3 != self.pads.dopo as u32 {
            return false;
        }
        [self.pads.sck, self.pads.mosi].iter().all(|&pin| {
            let cfg = bus
                .read_u8(self.pads.port_base + 0x40 + u64::from(pin))
                .ok();
            let mux = bus
                .read_u8(self.pads.port_base + 0x30 + u64::from(pin / 2))
                .ok();
            cfg.is_some_and(|v| v & 1 != 0)
                && mux.is_some_and(|v| (v >> ((pin & 1) * 4)) & 15 == self.pads.mux)
        })
    }
}

impl Peripheral for SamSercomSpi {
    fn as_any(&self) -> Option<&dyn std::any::Any> {
        Some(self)
    }
    fn as_any_mut(&mut self) -> Option<&mut dyn std::any::Any> {
        Some(self)
    }
    fn spi_attached_devices(&self) -> Option<&Vec<Box<dyn SpiDevice>>> {
        Some(&self.devices)
    }
    fn spi_attached_devices_mut(&mut self) -> Option<&mut Vec<Box<dyn SpiDevice>>> {
        Some(&mut self.devices)
    }
    fn read(&self, offset: u64) -> SimResult<u8> {
        let (base, shift) = Self::reg(offset);
        Ok((self.value(base, offset == 0x28) >> shift) as u8)
    }
    fn read_u16(&self, offset: u64) -> SimResult<u16> {
        let (base, shift) = Self::reg(offset);
        Ok((self.value(base, offset == 0x28) >> shift) as u16)
    }
    fn read_u32(&self, offset: u64) -> SimResult<u32> {
        let (base, shift) = Self::reg(offset);
        Ok(self.value(base, offset == 0x28) >> shift)
    }
    fn peek(&self, offset: u64) -> Option<u8> {
        let (base, shift) = Self::reg(offset);
        Some((self.value(base, false) >> shift) as u8)
    }
    fn write(&mut self, offset: u64, value: u8) -> SimResult<()> {
        self.store(offset, value.into(), 8);
        Ok(())
    }
    fn write_u16(&mut self, offset: u64, value: u16) -> SimResult<()> {
        self.store(offset, value.into(), 16);
        Ok(())
    }
    fn write_u32(&mut self, offset: u64, value: u32) -> SimResult<()> {
        self.store(offset, value, 32);
        Ok(())
    }
    fn tick(&mut self) -> PeripheralTickResult {
        self.tick_elapsed(1)
    }
    fn tick_elapsed(&mut self, cycles: u64) -> PeripheralTickResult {
        if self.enabled() {
            if let Some((_, left)) = &mut self.active {
                *left = left.saturating_sub(cycles);
            }
        }
        PeripheralTickResult::default()
    }
    fn needs_bus_tick(&self) -> bool {
        self.holding.is_some() || self.active.is_some() || self.selected.iter().any(|s| *s)
    }
    fn tick_with_bus(&mut self, bus: &mut dyn Bus) {
        for (dev, selected) in self.devices.iter_mut().zip(&mut self.selected) {
            let next = if dev.cs_pin().is_empty() {
                true
            } else {
                bus.read_gpio_output_by_label(dev.cs_pin()) == Some(false)
            };
            if next != *selected {
                if next {
                    dev.cs_select();
                } else {
                    dev.cs_release();
                }
            }
            *selected = next;
        }
        if !self.enabled() || !self.pads_ready(bus) {
            return;
        }
        let mut completed = false;
        if let Some((tx, 0)) = self.active {
            let tx = if self.ctrla & (1 << 30) != 0 {
                tx.reverse_bits()
            } else {
                tx
            };
            let mut rx = 0xff;
            for (dev, selected) in self.devices.iter_mut().zip(&self.selected) {
                if *selected {
                    rx &= dev.transfer(tx);
                }
            }
            if self.ctrla & (1 << 30) != 0 {
                rx = rx.reverse_bits();
            }
            if self.ctrlb & (1 << 17) != 0 {
                if self.rx.borrow().len() < 2 {
                    self.rx.borrow_mut().push_back(rx);
                } else {
                    self.status |= BUFOVF;
                    self.flags |= ERROR;
                }
            }
            self.active = None;
            completed = true;
        }
        if self.active.is_none() {
            if let Some(tx) = self.holding.take() {
                self.active = Some((tx, 16 * (u64::from(self.baud) + 1)));
            } else if completed {
                self.flags |= TXC;
            }
        }
    }
    fn drives_central_device_time(&self) -> bool {
        true
    }
    fn advance_attached_device_time_us(&mut self, us: u64) {
        for d in &mut self.devices {
            d.advance_time_us(us);
        }
    }
    fn drain_attached_pin_drives(&mut self, out: &mut Vec<(String, String, bool)>) {
        for d in &mut self.devices {
            crate::peripherals::device::drain_spi_pin_drives(&mut **d, out);
        }
    }
    fn for_each_attached_device(&self, f: &mut dyn FnMut(crate::inspect::AttachedDeviceRef<'_>)) {
        for d in &self.devices {
            crate::inspect::visit_spi_device(&**d, f);
        }
    }
    fn for_each_attached_sim_input(
        &mut self,
        f: &mut dyn FnMut(&mut dyn crate::sim_input::SimInput) -> bool,
    ) -> bool {
        for d in &mut self.devices {
            if let Some(si) = d.as_sim_input_mut() {
                if f(si) {
                    return true;
                }
            }
        }
        false
    }
}
