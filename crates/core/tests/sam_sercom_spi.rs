// SPDX-License-Identifier: MIT
//! Actual board/MMIO tests for the blocking controller slice. Not display or
//! DMA/native Arcade qualification; the slave below is a transport observer.
use labwired_config::{ChipDescriptor, SystemManifest};
use labwired_core::{
    bus::SystemBus,
    cpu::cortex_m::CortexM,
    memory::ProgramImage,
    peripherals::sam::sercom_spi::SamSercomSpi,
    peripherals::spi::{SpiDevice, SpiSampling},
    AdvanceRequest, Arch, Bus, Machine,
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
    board_with_drive(true)
}

fn board_with_drive(legacy: bool) -> (SystemBus, Arc<Mutex<Seen>>) {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let chip = ChipDescriptor::from_file(root.join("configs/chips/atsamd51-pybadge.yaml")).unwrap();
    let manifest = SystemManifest::from_file(root.join("configs/systems/pybadge.yaml")).unwrap();
    let mut bus = SystemBus::from_config(&chip, &manifest).unwrap();
    if legacy {
        bus.peripherals
            .iter_mut()
            .find(|p| p.name == "sercom4")
            .unwrap()
            .dev
            .as_any_mut()
            .unwrap()
            .downcast_mut::<SamSercomSpi>()
            .unwrap()
            .force_legacy_walk();
        bus.recompute_walk_deletable();
        bus.refresh_peripheral_index();
    }
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
    bus.write_u8(PORT + 0x30 + 6, 0x20).unwrap();
    bus.write_u8(PORT + 0x30 + 7, 0x20).unwrap();
    bus.write_u8(PORT + 0x40 + 13, 1).unwrap();
    bus.write_u8(PORT + 0x40 + 15, 1).unwrap();
    bus.write_u32(PORT, (1 << 7) | (1 << 5)).unwrap();
    bus.write_u32(SPI, MASTER).unwrap();
    bus.write_u32(SPI + 4, 1 << 17).unwrap();
    bus.write_u8(SPI + 0x0c, 1).unwrap();
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
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 7, 1);
    bus.write_u32(SPI + 0x28, 0x81).unwrap();
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 3, 0);
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x81]); // 32-bit access sends ONE byte
    assert_eq!(seen.lock().unwrap().dc, [false]);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 7, 7);
    let controller = bus
        .peripherals
        .iter()
        .find(|p| p.name == "sercom4")
        .unwrap();
    assert_eq!(controller.dev.peek(0x28), Some(0xdb));
    assert_eq!(bus.read_u8(SPI + 0x29).unwrap(), 0);
    assert_eq!(
        bus.read_u8(SPI + 0x18).unwrap() & 4,
        4,
        "debug peeks and upper DATA lanes must not consume RX"
    );
    bus.write_u8(SPI + 0x18, 7).unwrap(); // RXC/DRE are not W1C
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 7, 5);
    assert_eq!(bus.read_u32(SPI + 0x28).unwrap(), 0xdb);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 4, 0);
    bus.write_u32(PORT + 0x18, 1 << 5).unwrap();
    bus.write_u16(SPI + 0x28, 0x42).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().dc, [false, true]);
    bus.write_u32(PORT + 0x18, 1 << 7).unwrap();
    settle(&mut bus);
    bus.write_u8(SPI + 0x28, 0x19).unwrap();
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
    bus.write_u8(SPI + 0x28, 0x11).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(MCLK, 1).unwrap();
    bus.write_u32(GCLK, 0).unwrap();
    bus.write_u8(SPI + 0x28, 0x22).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(GCLK, 1 << 6).unwrap();
    bus.write_u8(PORT + 0x40 + 13, 0).unwrap();
    bus.write_u8(SPI + 0x28, 0x33).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u8(PORT + 0x40 + 13, 1).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x33]);
    bus.write_u32(SPI, MASTER).unwrap();
    bus.write_u32(SPI, (1 << 2) | 2).unwrap();
    bus.write_u8(SPI + 0x28, 0x44).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x33]);
}

#[test]
fn pending_completion_freezes_when_clock_is_removed() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    bus.write_u8(SPI + 0x28, 0x51).unwrap();
    bus.tick_peripherals();
    bus.write_u32(MCLK, 0).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(MCLK, 1).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x51]);
}

#[test]
fn overflow_preserves_queued_rx_and_status_flags_are_w1c() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    for byte in [0x10, 0x20, 0x30] {
        bus.write_u8(SPI + 0x28, byte).unwrap();
        settle(&mut bus);
    }
    assert_eq!(seen.lock().unwrap().bytes, [0x10, 0x20, 0x30]);
    assert_eq!(bus.read_u16(SPI + 0x1a).unwrap() & 4, 4);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 128, 128);
    bus.write_u16(SPI + 0x1a, 4).unwrap();
    bus.write_u8(SPI + 0x18, 128).unwrap();
    assert_eq!(bus.read_u16(SPI + 0x1a).unwrap(), 0);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 128, 0);
    assert_eq!(bus.read_u8(SPI + 0x28).unwrap(), 0x4a);
    assert_eq!(bus.read_u16(SPI + 0x28).unwrap(), 0x7a);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap() & 4, 0);
}

#[test]
fn wrong_dopo_and_character_size_block_transfers_and_configuration_is_protected() {
    let (mut bus, seen) = board();
    enable(&mut bus);
    bus.write_u32(SPI, 3 << 2 | 2).unwrap();
    assert_eq!(
        bus.read_u32(SPI).unwrap(),
        MASTER | 2,
        "enabled DOPO is protected"
    );
    bus.write_u32(SPI, MASTER).unwrap(); // disable
    bus.write_u32(SPI, 3 << 2 | 2).unwrap(); // wrong DOPO=0
    bus.write_u8(SPI + 0x28, 0x61).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    bus.write_u32(SPI, 3 << 2).unwrap(); // disable, cancelling pending data
    bus.write_u32(SPI + 4, 1).unwrap(); // unsupported 9-bit frame
    bus.write_u32(SPI, MASTER | 2).unwrap();
    bus.write_u8(SPI + 0x28, 0x62).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    enable(&mut bus);
    bus.write_u8(SPI + 0x28, 0x63).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x63]);
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
    bus.write_u8(SPI + 0x28, 0x99).unwrap();
    bus.tick_peripherals();
    bus.write_u32(SPI, 1).unwrap();
    settle(&mut bus);
    assert!(seen.lock().unwrap().bytes.is_empty());
    assert_eq!(bus.read_u32(SPI).unwrap(), 0);
    assert_eq!(bus.read_u8(SPI + 0x18).unwrap(), 0);
    enable(&mut bus);
    bus.write_u8(SPI + 0x28, 0x73).unwrap();
    settle(&mut bus);
    assert_eq!(seen.lock().unwrap().bytes, [0x73]);
}

/// Small independently authored Thumb guest: MMIO setup, DATA writes, TXC
/// polling, then an SRAM completion marker. Only code/vector bytes are loaded;
/// the host does not configure the controller or complete a guest transfer.
fn polling_guest(correct_mux: bool) -> ProgramImage {
    let mut ops = Vec::<u16>::new();
    let mut literals = Vec::<(usize, u8, u32)>::new();
    let load = |ops: &mut Vec<u16>, literals: &mut Vec<(usize, u8, u32)>, rt, value| {
        literals.push((ops.len(), rt, value));
        ops.push(0);
    };
    let stores = [
        (MCLK, 1, false),
        (GCLK, 1 << 6, false),
        (PORT + 0x36, if correct_mux { 0x20 } else { 0x30 }, true),
        (PORT + 0x37, 0x20, true),
        (PORT + 0x4d, 1, true),
        (PORT + 0x4f, 1, true),
        (PORT, (1 << 7) | (1 << 5), false),
        (SPI, MASTER, false),
        (SPI + 4, 1 << 17, false),
        (SPI + 0x0c, 1, true),
        (SPI, MASTER | 2, false),
        (SPI + 0x28, 0x51, true),
        (PORT + 0x18, 1 << 5, false),
        (SPI + 0x28, 0x42, true),
        (0x20000000, 0xcafebabe, false),
    ];
    for (addr, value, byte) in stores {
        load(&mut ops, &mut literals, 0, addr as u32);
        load(&mut ops, &mut literals, 1, value);
        ops.push(if byte { 0x7001 } else { 0x6001 }); // STRB/STR r1,[r0]
        if addr == SPI + 0x28 {
            load(&mut ops, &mut literals, 0, (SPI + 0x18) as u32);
            // LDRB r1,[r0]; MOVS r2,#2; TST r1,r2; BEQ -10 (poll TXC).
            ops.extend([0x7801, 0x2202, 0x4211, 0xd0fb]);
        }
    }
    ops.push(0xe7fe); // firmware parks after completion
    if ops.len() & 1 != 0 {
        ops.push(0xbf00);
    }
    let mut code: Vec<u8> = ops.iter().flat_map(|op| op.to_le_bytes()).collect();
    for (index, rt, value) in literals {
        let pc = ((index * 2 + 4) & !3) as u32;
        let delta = code.len() as u32 - pc;
        assert_eq!(delta & 3, 0);
        assert!(delta <= 1020);
        let op = 0x4800 | (u16::from(rt) << 8) | (delta / 4) as u16;
        code[index * 2..index * 2 + 2].copy_from_slice(&op.to_le_bytes());
        code.extend(value.to_le_bytes());
    }
    let mut application = vec![0u8; 0x100];
    application[0..4].copy_from_slice(&0x20004000u32.to_le_bytes());
    application[4..8].copy_from_slice(&0x4101u32.to_le_bytes());
    application.extend(code);
    let mut image = ProgramImage::new(0x4101, Arch::Arm);
    image.add_segment(0x4000, application);
    image
}

#[test]
fn cortex_m_guest_polls_real_spi_completion_and_wrong_mux_cannot_complete() {
    for correct_mux in [true, false] {
        let (bus, seen) = board();
        let mut machine = Machine::new(CortexM::new(), bus);
        machine.load_firmware(&polling_guest(correct_mux)).unwrap();
        for _ in 0..2000 {
            machine.step().unwrap();
        }
        let marker = machine.bus.read_u32(0x20000000).unwrap();
        if correct_mux {
            assert_eq!(marker, 0xcafebabe);
            assert_eq!(seen.lock().unwrap().bytes, [0x51, 0x42]);
            assert_eq!(seen.lock().unwrap().dc, [false, true]);
        } else {
            assert_ne!(marker, 0xcafebabe);
            assert!(seen.lock().unwrap().bytes.is_empty());
        }
    }
}

#[test]
fn polling_guest_matches_legacy_with_auto_batches_and_scheduler() {
    for legacy in [true, false] {
        for interval in [1, 64, 1024] {
            if interval > 1 && !cfg!(feature = "event-scheduler") {
                continue;
            }
            for correct_mux in [true, false] {
                let (mut bus, seen) = board_with_drive(legacy);
                bus.config.peripheral_tick_interval = if legacy { 1 } else { interval };
                let mut machine = Machine::new(CortexM::new(), bus);
                machine.config.peripheral_tick_interval = if legacy { 1 } else { interval };
                machine.load_firmware(&polling_guest(correct_mux)).unwrap();
                machine.advance(AdvanceRequest::run(Some(2000))).unwrap();
                let marker = machine.bus.read_u32(0x20000000).unwrap();
                let seen = seen.lock().unwrap();
                if correct_mux {
                    assert_eq!(marker, 0xcafebabe, "legacy={legacy}, interval={interval}");
                    assert_eq!(seen.bytes, [0x51, 0x42]);
                    assert_eq!(seen.dc, [false, true]);
                    assert_eq!(machine.bus.read_u8(SPI + 0x18).unwrap() & 7, 7);
                    assert_eq!(machine.bus.read_u8(SPI + 0x28).unwrap(), 0x51 ^ 0x5a);
                    assert_eq!(machine.bus.read_u8(SPI + 0x28).unwrap(), 0x42 ^ 0x5a);
                } else {
                    assert_ne!(marker, 0xcafebabe);
                    assert!(seen.bytes.is_empty());
                }
            }
        }
    }
}

#[test]
fn scheduler_clock_gate_freezes_inflight_frame_and_resumes_once() {
    for gate in [MCLK, GCLK] {
        let (bus, seen) = board_with_drive(false);
        let mut machine = Machine::new(CortexM::new(), bus);
        machine.load_firmware(&polling_guest(true)).unwrap();
        // Wait for ENABLE, then execute DATA setup and the beginning of polling.
        // This also works for the walk reference, which consumes the holding
        // byte before the host can observe DRE=0 at the next boundary.
        let mut enabled = false;
        for _ in 0..200 {
            machine.step().unwrap();
            if machine.bus.read_u32(SPI).unwrap() & 2 != 0 {
                enabled = true;
                break;
            }
        }
        assert!(enabled, "guest did not enable SPI");
        for _ in 0..5 {
            machine.step().unwrap();
        }
        assert!(seen.lock().unwrap().bytes.is_empty());
        machine.bus.write_u32(gate, 0).unwrap();
        for _ in 0..100 {
            machine.step().unwrap();
        }
        assert!(
            seen.lock().unwrap().bytes.is_empty(),
            "unclocked frame escaped"
        );
        machine
            .bus
            .write_u32(gate, if gate == MCLK { 1 } else { 1 << 6 })
            .unwrap();
        for _ in 0..10 {
            machine.step().unwrap();
        }
        assert!(
            seen.lock().unwrap().bytes.is_empty(),
            "gated time consumed BAUD countdown"
        );
        for _ in 0..2000 {
            machine.step().unwrap();
        }
        assert_eq!(seen.lock().unwrap().bytes, [0x51, 0x42]);
        assert_eq!(machine.bus.read_u32(0x20000000).unwrap(), 0xcafebabe);
    }
}

#[test]
fn idle_spi_does_not_arm_events_or_force_the_legacy_walk() {
    let (bus, _) = board_with_drive(false);
    let spi = &bus
        .peripherals
        .iter()
        .find(|p| p.name == "sercom4")
        .unwrap()
        .dev;
    assert_eq!(spi.uses_scheduler(), cfg!(feature = "event-scheduler"));
    assert_eq!(spi.needs_legacy_walk(), !cfg!(feature = "event-scheduler"));
    assert!(!spi.needs_bus_tick());
    if cfg!(feature = "event-scheduler") {
        assert_eq!(bus.max_safe_tick_interval(), 1024);
    }
    let mut machine = Machine::new(CortexM::new(), bus);
    // Wrong mux guest would arm an event; idle setup must not.
    machine.load_firmware(&polling_guest(true)).unwrap();
    machine.step().unwrap();
    assert!(machine.sched.is_empty());
}

#[test]
fn reset_cancels_pending_wake_without_reselecting_slave_and_next_frame_works() {
    let (bus, seen) = board_with_drive(false);
    let mut machine = Machine::new(CortexM::new(), bus);
    let mut bytes = vec![0u8; 0x100];
    bytes[0..4].copy_from_slice(&0x20004000u32.to_le_bytes());
    bytes[4..8].copy_from_slice(&0x4101u32.to_le_bytes());
    bytes.extend(0xe7feu16.to_le_bytes()); // authored idle branch, no MMIO setup
    let mut image = ProgramImage::new(0x4101, Arch::Arm);
    image.add_segment(0x4000, bytes);
    machine.load_firmware(&image).unwrap();
    // Explicit host-MMIO cancellation adversary; not native firmware proof.
    enable(&mut machine.bus);
    machine.bus.write_u8(SPI + 0x28, 0x81).unwrap();
    for _ in 0..5 {
        machine.step().unwrap();
    }
    assert_eq!(seen.lock().unwrap().selects, 1);
    machine.bus.write_u32(SPI, 1).unwrap();
    for _ in 0..100 {
        machine.step().unwrap();
    }
    {
        let seen = seen.lock().unwrap();
        assert!(seen.bytes.is_empty());
        assert_eq!(seen.selects, 1);
        assert_eq!(seen.releases, 1);
    }
    assert!(machine.sched.is_empty(), "stale reset wake stayed armed");
    assert_eq!(machine.bus.read_u8(SPI + 0x18).unwrap(), 0);
    enable(&mut machine.bus);
    machine.bus.write_u8(SPI + 0x28, 0x42).unwrap();
    for _ in 0..100 {
        machine.step().unwrap();
    }
    assert_eq!(seen.lock().unwrap().bytes, [0x42]);
    machine.bus.write_u32(PORT + 0x18, 1 << 7).unwrap();
    for _ in 0..5 {
        machine.step().unwrap();
    }
    assert_eq!(seen.lock().unwrap().releases, 2);
    assert!(machine.sched.is_empty());
}
