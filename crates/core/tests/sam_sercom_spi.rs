// SPDX-License-Identifier: MIT
//! Actual board/MMIO tests for the blocking controller slice. Not display or
//! DMA/native Arcade qualification; the slave below is a transport observer.
use labwired_config::{ChipDescriptor, SystemManifest};
use labwired_core::{
    bus::SystemBus,
    peripherals::spi::{SpiDevice, SpiSampling},
    Bus,
};
use std::sync::{Arc, Mutex};

const SPI: u64 = 0x43000000;
const PORT: u64 = 0x41008080;
const MCLK: u64 = 0x40000820;
const GCLK: u64 = 0x40001c00 + 0x80 + 34 * 4;
const MASTER: u32 = (3 << 2) | (2 << 16);

#[derive(Default, Debug)]
struct Seen {
    bytes: Vec<u8>,
    dc: Vec<bool>,
    selects: usize,
    releases: usize,
}
struct Slave {
    seen: Arc<Mutex<Seen>>,
    dc: bool,
    dc_source: Option<(u64, u8)>,
    edge: bool,
}
impl SpiDevice for Slave {
    fn cs_pin(&self) -> &str {
        "PB7"
    }
    fn dc_pin(&self) -> Option<&str> {
        Some("PB5")
    }
    fn dc_source(&self) -> Option<(u64, u8)> {
        self.dc_source
    }
    fn set_dc_source(&mut self, addr: u64, bit: u8) {
        self.dc_source = Some((addr, bit));
    }
    fn set_dc_level(&mut self, level: bool) {
        self.dc = level;
    }
    fn sampling(&self) -> SpiSampling {
        if self.edge {
            SpiSampling::edge_mode(0)
        } else {
            SpiSampling::Byte
        }
    }
    fn cs_select(&mut self) {
        self.seen.lock().unwrap().selects += 1;
    }
    fn cs_release(&mut self) {
        self.seen.lock().unwrap().releases += 1;
    }
    fn transfer(&mut self, byte: u8) -> u8 {
        let mut seen = self.seen.lock().unwrap();
        seen.bytes.push(byte);
        seen.dc.push(self.dc);
        byte ^ 0x5a
    }
}
fn board() -> (SystemBus, Arc<Mutex<Seen>>) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let chip = ChipDescriptor::from_file(root.join("configs/chips/atsamd51-pybadge.yaml")).unwrap();
    let manifest = SystemManifest::from_file(root.join("configs/systems/pybadge.yaml")).unwrap();
    let mut bus = SystemBus::from_config(&chip, &manifest).unwrap();
    let seen = Arc::new(Mutex::new(Seen::default()));
    bus.attach_spi_device(
        "sercom4",
        Box::new(Slave {
            seen: seen.clone(),
            dc: false,
            dc_source: Some((PORT + 0x10, 5)),
            edge: false,
        }),
    )
    .unwrap();
    (bus, seen)
}
fn enable(bus: &mut SystemBus) {
    bus.write_u32(MCLK, 1).unwrap();
    bus.write_u32(GCLK, 1 << 6).unwrap();
    // Odd pins use the high mux nibble; PINCFG.PMUXEN is independent of DIR.
    bus.write(PORT + 0x30 + 6, 0x20).unwrap();
    bus.write(PORT + 0x30 + 7, 0x20).unwrap();
    bus.write(PORT + 0x40 + 13, 1).unwrap();
    bus.write(PORT + 0x40 + 15, 1).unwrap();
    bus.write_u32(PORT, (1 << 7) | (1 << 5)).unwrap();
    bus.write_u32(SPI, MASTER).unwrap();
    bus.write_u32(SPI + 4, 1 << 17).unwrap();
    bus.write(SPI + 0x0c, 1).unwrap();
    bus.write_u32(SPI, MASTER | 2).unwrap();
}
fn settle(bus: &mut SystemBus) {
    for _ in 0..100 {
        bus.tick_peripherals();
    }
}

#[test]
fn blocking_mmio_frames_cs_dc_fifo_and_w1c() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    assert_eq!(bus.read(SPI + 0x18).unwrap() & 7, 1);
    bus.write_u32(SPI + 0x28, 0x81).unwrap();
    assert_eq!(bus.read(SPI + 0x18).unwrap() & 3, 0);
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x81]); // 32-bit access sends ONE byte
    assert_eq!(seen.lock().unwrap().dc, [false]);
    assert_eq!(bus.read(SPI + 0x18).unwrap() & 7, 7);
    bus.write(SPI + 0x18, 7).unwrap(); // RXC/DRE are not W1C
    assert_eq!(bus.read(SPI + 0x18).unwrap() & 7, 5);
    assert_eq!(bus.read_u32(SPI + 0x28).unwrap(), 0xdb);
    assert_eq!(bus.read(SPI + 0x18).unwrap() & 4, 0);
    bus.write_u32(PORT + 0x18, 1 << 5).unwrap();
    bus.write_u16(SPI + 0x28, 0x42).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().dc, [false, true]);
    bus.write_u32(PORT + 0x18, 1 << 7).unwrap();
    settle(&mut bus);
    bus.write(SPI + 0x28, 0x19).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x81, 0x42]);
    assert_eq!(seen.lock().unwrap().selects, 1);
    assert_eq!(seen.lock().unwrap().releases, 1);
}

#[test]
fn missing_clock_mux_or_master_mode_never_reaches_slave() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    bus.write_u32(MCLK, 0).unwrap();
    bus.write(SPI + 0x28, 0x11).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(MCLK, 1).unwrap();
    bus.write_u32(GCLK, 0).unwrap();
    bus.write(SPI + 0x28, 0x22).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(GCLK, 1 << 6).unwrap();
    bus.write(PORT + 0x40 + 13, 0).unwrap();
    bus.write(SPI + 0x28, 0x33).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write(PORT + 0x40 + 13, 1).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x33]);
    bus.write_u32(SPI, MASTER).unwrap();
    bus.write_u32(SPI, (1 << 2) | 2).unwrap();
    bus.write(SPI + 0x28, 0x44).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x33]);
}

#[test]
fn pending_completion_freezes_when_clock_is_removed() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    bus.write(SPI + 0x28, 0x51).unwrap();
    bus.tick_peripherals();
    bus.write_u32(MCLK, 0).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(MCLK, 1).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x51]);
}

#[test]
fn reset_cancels_inflight_and_preserves_attachment_and_edges_are_rejected() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    let err = bus.attach_spi_device(
        "sercom4",
        Box::new(Slave {
            seen: seen.clone(),
            dc: false,
            dc_source: None,
            edge: true,
        }),
    );
    assert!(err.is_err());
    bus.write(SPI + 0x28, 0x99).unwrap();
    bus.tick_peripherals();
    bus.write_u32(SPI, 1).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    assert_eq!(bus.read_u32(SPI).unwrap(), 0);
    assert_eq!(bus.read(SPI + 0x18).unwrap(), 0);
    enable(&mut bus);
    bus.write(SPI + 0x28, 0x73).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x73]);
}
