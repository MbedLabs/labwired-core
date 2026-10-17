// SPDX-License-Identifier: MIT
//! Authored blocking-SPI guest and controller-memory pixel proof. NOT native
//! Arcade, DMA, IRQ, actual module glass, BGR or reset/backlight qualification.
use labwired_config::{ChipDescriptor, DeviceDescriptor, SystemManifest};
use labwired_core::{
    bus::SystemBus,
    cpu::cortex_m::CortexM,
    inspect::{artifact_format, artifact_region_ink, Artifact, InspectOpts, PixelRegion},
    memory::ProgramImage,
    peripherals::{
        components::declarative_display::{GenericDisplay, GlassWindow},
        spi::SpiDevice,
    },
    Arch, Bus, Machine,
};

const SPI: u64 = 0x43000000;
const PORT: u64 = 0x41008080;
const MCLK: u64 = 0x40000820;
const GCLK: u64 = 0x40001c00 + 0x80 + 34 * 4;
const MASTER: u32 = (3 << 2) | (2 << 16);
const MARKER: u32 = 0x51c0ffee;

#[derive(Clone, Copy, Debug)]
enum Wiring {
    Correct,
    MissingMclk,
    MissingGclk,
    WrongMux,
    CsHigh,
    DcHigh,
}

fn board(legacy: bool) -> SystemBus {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let chip = ChipDescriptor::from_file(root.join("configs/chips/atsamd51-pybadge.yaml")).unwrap();
    let manifest = SystemManifest::from_file(root.join("configs/systems/pybadge.yaml")).unwrap();
    let mut bus = SystemBus::from_config(&chip, &manifest).unwrap();
    if legacy {
        assert!(bus
            .peripherals
            .iter_mut()
            .find(|p| p.name == "sercom4")
            .unwrap()
            .dev
            .force_legacy_walk());
        bus.recompute_walk_deletable();
        bus.refresh_peripheral_index();
    }
    let descriptor =
        DeviceDescriptor::from_yaml(include_str!("../../../configs/devices/st7735r.yaml")).unwrap();
    let mut display = GenericDisplay::from_descriptor(&descriptor).unwrap();
    display.set_cs_pin("PB7");
    display.set_dc_pin("PB5");
    display.set_dc_source(PORT + 0x10, 5);
    // Fixture-only crop of real controller RAM. NOT an asserted module crop.
    display.set_glass_window(GlassWindow {
        col_offset: 0,
        row_offset: 0,
        cols: 2,
        rows: 2,
    });
    bus.attach_spi_device("sercom4", Box::new(display)).unwrap();
    bus
}

/// Emit short aligned Thumb blocks with their own literal islands, so a full
/// RGBSET upload cannot exceed the 16-bit LDR's 1 KiB literal reach. Nothing
/// executes on the host; these bytes are the guest's actual instructions.
#[derive(Default)]
struct Guest {
    code: Vec<u8>,
}
impl Guest {
    fn halfwords(&mut self, ops: &[u16]) {
        for op in ops {
            self.code.extend(op.to_le_bytes());
        }
    }

    fn store(&mut self, addr: u64, value: u32, byte: bool) {
        assert_eq!(self.code.len() & 3, 0);
        // LDR r0,[PC,#4]; LDR r1,[PC,#8]; STR[B] r1,[r0]; B past literals.
        self.halfwords(&[0x4801, 0x4902, if byte { 0x7001 } else { 0x6001 }, 0xe003]);
        self.code.extend((addr as u32).to_le_bytes());
        self.code.extend(value.to_le_bytes());
    }

    fn txc(&mut self) {
        // LDR r0,[PC,#8]; LDRB r1,[r0]; MOVS r2,#2; TST; BEQ LDRB;
        // B past the INTFLAG address literal. TXC is guest-polled, not seeded.
        self.halfwords(&[0x4802, 0x7801, 0x2202, 0x4211, 0xd0fb, 0xe001]);
        self.code.extend(((SPI + 0x18) as u32).to_le_bytes());
    }

    fn send(&mut self, byte: u8, data: bool, wiring: Wiring) {
        let data = data || matches!(wiring, Wiring::DcHigh);
        self.store(PORT + if data { 0x18 } else { 0x14 }, 1 << 5, false);
        if !matches!(wiring, Wiring::CsHigh) {
            self.store(PORT + 0x14, 1 << 7, false);
        }
        self.store(SPI + 0x28, u32::from(byte), true);
        self.txc();
        self.store(PORT + 0x18, 1 << 7, false);
    }

    fn command(&mut self, op: u8, args: &[u8], wiring: Wiring) {
        self.send(op, false, wiring);
        for &byte in args {
            self.send(byte, true, wiring);
        }
    }

    fn image(mut self) -> ProgramImage {
        self.halfwords(&[0xe7fe]); // park, never fall into a data island
        let mut bytes = vec![0u8; 0x100];
        bytes[0..4].copy_from_slice(&0x20004000u32.to_le_bytes());
        bytes[4..8].copy_from_slice(&0x4101u32.to_le_bytes());
        bytes.extend(self.code);
        let mut image = ProgramImage::new(0x4101, Arch::Arm);
        image.add_segment(0x4000, bytes);
        image
    }
}

fn guest(wiring: Wiring) -> ProgramImage {
    let mut guest = Guest::default();
    for (addr, value, byte) in [
        (
            MCLK,
            if matches!(wiring, Wiring::MissingMclk) {
                0
            } else {
                1
            },
            false,
        ),
        (
            GCLK,
            if matches!(wiring, Wiring::MissingGclk) {
                0
            } else {
                1 << 6
            },
            false,
        ),
        (
            PORT + 0x36,
            if matches!(wiring, Wiring::WrongMux) {
                0x30
            } else {
                0x20
            },
            true,
        ),
        (PORT + 0x37, 0x20, true),
        (PORT + 0x4d, 1, true),
        (PORT + 0x4f, 1, true),
        (PORT, (1 << 7) | (1 << 5), false),
        (PORT + 0x18, 1 << 7, false), // CS inactive before enabling SPI
        (SPI, MASTER, false),
        (SPI + 4, 0, false), // receive disabled: no unused RX FIFO overflow
        (SPI + 0x0c, 1, true),
        (SPI, MASTER | 2, false),
    ] {
        guest.store(addr, value, byte);
    }
    guest.command(0x3a, &[3], wiring);
    // Deliberately non-linear/channel-distinct six-bit LUT, not a linear ramp.
    let lut: [u8; 128] = std::array::from_fn(|i| ((i * 19 + i / 32 * 7 + 11) & 63) as u8);
    guest.command(0x2d, &lut, wiring);
    guest.command(0x2a, &[0, 0, 0, 1], wiring);
    guest.command(0x2b, &[0, 0, 0, 1], wiring);
    guest.command(0x2c, &[0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc], wiring);
    guest.store(0x20000000, MARKER, false);
    guest.image()
}

fn artifact(bus: &SystemBus) -> Artifact {
    bus.device_artifact_at(
        "sercom4",
        None,
        &[artifact_format::RGB888],
        "panel",
        &InspectOpts {
            include_bytes: true,
            peripheral: None,
        },
    )
    .expect("attached prototype evidence")
}

fn run(wiring: Wiring, legacy: bool) -> (u32, Artifact) {
    let mut machine = Machine::new(CortexM::new(), board(legacy));
    machine.load_firmware(&guest(wiring)).unwrap();
    for _ in 0..50000 {
        machine.step().unwrap();
        if machine.bus.read_u32(0x20000000).unwrap() == MARKER {
            break;
        }
    }
    (
        machine.bus.read_u32(0x20000000).unwrap(),
        artifact(&machine.bus),
    )
}

#[test]
fn st7735_guest_paints_packed_rgb444_through_sam_spi_and_real_gpio_cs_dc() {
    for legacy in [true, false] {
        let (marker, frame) = run(Wiring::Correct, legacy);
        assert_eq!(marker, MARKER, "legacy={legacy}: guest did not finish");
        assert_eq!(frame.meta["known_pixels"], 4);
        assert_eq!(frame.meta["unknown_pixels"], 0);
        // Independent expected table addresses for the authored packed stream.
        let addresses = [1, 34, 99, 4, 37, 102, 7, 40, 105, 10, 43, 108];
        let expected: Vec<u8> = addresses
            .into_iter()
            .map(|i| {
                let v = ((i * 19 + i / 32 * 7 + 11) & 63) as u8;
                (v << 2) | (v >> 4)
            })
            .collect();
        assert_eq!(
            frame.bytes.as_deref(),
            Some(expected.as_slice()),
            "legacy={legacy}"
        );
        assert_eq!(
            artifact_region_ink(
                artifact_format::RGB888,
                &frame.meta,
                frame.bytes.as_deref().unwrap(),
                PixelRegion {
                    x: 0,
                    y: 0,
                    w: 2,
                    h: 2
                }
            )
            .unwrap(),
            (4, 4)
        );
    }
}

#[test]
fn st7735_guest_clock_mux_cs_and_dc_negatives_do_not_paint_placeholder_pixels() {
    for legacy in [true, false] {
        for wiring in [
            Wiring::MissingMclk,
            Wiring::MissingGclk,
            Wiring::WrongMux,
            Wiring::CsHigh,
            Wiring::DcHigh,
        ] {
            let (marker, frame) = run(wiring, legacy);
            assert_eq!(frame.meta["known_pixels"], 0, "{wiring:?}, legacy={legacy}");
            assert_eq!(frame.meta["unknown_pixels"], 4);
            assert!(frame.bytes.is_none());
            if matches!(wiring, Wiring::CsHigh | Wiring::DcHigh) {
                assert_eq!(
                    marker, MARKER,
                    "wire negative must not fake a controller stall"
                );
            } else {
                assert_ne!(marker, MARKER, "clock/mux negative cannot pass TXC polling");
            }
        }
    }
}
