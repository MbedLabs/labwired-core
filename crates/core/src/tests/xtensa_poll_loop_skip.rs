// LabWired - Firmware Simulation Platform
// Copyright (C) 2026 Andrii Shylenko
//
// This software is released under the MIT License.
// See the LICENSE file in the project root for full license information.

//! The CCOUNT poll-loop skip (`cpu/xtensa_lx7/poll_loop.rs`) must fire, and it
//! must not change what the firmware does.
//!
//! The program is the shape of the four-servo trajectory sketch (share
//! d26Wk2eXNiFS): a pulse loop that drops four outputs at four CCOUNT
//! deadlines (it records `dt` where a real sketch writes `GPIO_OUT_W1TC`),
//! then a frame-tail busy-wait, then two loops the skip must REFUSE: one that
//! carries a CCOUNT sum from iteration to iteration, and one that tests a bit
//! of CCOUNT (not an ordered compare). Each test runs the program twice — idle
//! fast-forward on and off — and requires identical records, CCOUNT and
//! machine cycles, and a skip that covered most of the busy-waiting.
//!
//! Single core: the core runs the program on the primary. Dual core: the
//! secondary (an ESP32 APP_CPU running Arduino `loopTask`) runs it while the
//! primary sits in WAITI. Before the skip existed, the dual-core machine
//! refused any skip while the secondary was active and the single-core one
//! had no WAITI to skip, so both `skipped` assertions fail without it.

#[cfg(all(test, feature = "event-scheduler"))]
mod tests {
    use crate::bus::SystemBus;
    use crate::cpu::xtensa_lx7::XtensaLx7;
    use crate::cpu::xtensa_sr::CCOUNT;
    use crate::machine::AdvanceRequest;
    use crate::system::xtensa::configure_xtensa_esp32;
    use crate::{Bus, Cpu, Machine};

    const CODE: u64 = 0x4008_1000;
    const IDLE: u64 = 0x4008_2000;
    const DATA: u32 = 0x3ffb_1000;

    /// `frame.S`, assembled with xtensa-esp32-elf-as (esp-2021r2):
    ///
    /// ```text
    ///  0: rsr.ccount a2            ; t0
    ///     movi.n  a10, 15          ; four outputs pending
    ///  5: rsr.ccount a13           ; pulse: anchor
    ///     sub     a13, a13, a2     ; dt
    ///     movi a9, 0; movi a12, 4
    ///     loop    a12, 36          ; for i in 0..4
    ///     bbc     a10, a9, skip    ;   if pending & (1 << i)
    ///     l32i    a8, a6+4*i+64    ;     && dt >= end[i]
    ///     bltu    a13, a8, skip
    ///     s32i.n  a13, a6+4*i      ;   record dt, clear the bit
    ///     ...
    /// 36: bnez    a10, pulse
    ///     l32i    a4, a6, 80       ; tail deadline
    /// 3c: rsr.ccount a3; sub a3, a3, a2; bgeu a4, a3, 3c
    ///     rsr.ccount a5; s32i a5, a6, 32
    /// 50: rsr.ccount a3; add.n a7, a7, a3     ; accumulates: refuse
    ///     sub a3, a3, a2; bgeu a4, a3, 50     ; (a4 = a6[84])
    ///     s32i a7, a6, 36; rsr.ccount a5; s32i a5, a6, 40
    /// 64: rsr.ccount a3; bbci a3, 12, 64      ; bit test: refuse
    ///     rsr.ccount a5; s32i.n a5, a6, 44
    /// 6f: waiti 0; j 6f
    /// ```
    const FRAME: [u8; 117] = [
        0x20, 0xea, 0x03, 0x0c, 0xfa, 0xd0, 0xea, 0x03, 0x20, 0xdd, 0xc0, 0x92, 0xa0, 0x00, 0xc2,
        0xa0, 0x04, 0x76, 0x8c, 0x21, 0x97, 0x5a, 0x1c, 0xe0, 0x89, 0x11, 0x6a, 0x88, 0x82, 0x28,
        0x10, 0x87, 0x3d, 0x11, 0xe0, 0x89, 0x11, 0x6a, 0x88, 0xd9, 0x08, 0x0c, 0x1b, 0x00, 0x19,
        0x40, 0x00, 0xbb, 0xa1, 0xb0, 0xaa, 0x30, 0x1b, 0x99, 0x56, 0xba, 0xfc, 0x42, 0x26, 0x14,
        0x30, 0xea, 0x03, 0x20, 0x33, 0xc0, 0x37, 0xb4, 0xf6, 0x50, 0xea, 0x03, 0x52, 0x66, 0x08,
        0x0c, 0x07, 0x42, 0x26, 0x15, 0x30, 0xea, 0x03, 0x3a, 0x77, 0x20, 0x33, 0xc0, 0x37, 0xb4,
        0xf4, 0x72, 0x66, 0x09, 0x50, 0xea, 0x03, 0x52, 0x66, 0x0a, 0x30, 0xea, 0x03, 0xc7, 0x63,
        0xf9, 0x50, 0xea, 0x03, 0x59, 0xb6, 0x00, 0x70, 0x00, 0x46, 0xfe, 0xff,
    ];
    /// `waiti 0; j .-3` — a core with nothing to do.
    const PARK: [u8; 6] = [0x00, 0x70, 0x00, 0x46, 0xfe, 0xff];

    /// Pulse deadlines (cycles after t0), the frame-tail deadline, and the
    /// accumulating loop's deadline.
    const PULSE_END: [u32; 4] = [12_000, 24_000, 18_000, 30_000];
    const TAIL_END: u32 = 480_000;
    const ACC_END: u32 = 500_000;
    const RUN_CYCLES: u64 = 600_000;

    struct Outcome {
        records: Vec<u32>,
        ccount: u32,
        total_cycles: u64,
        skipped: u64,
        poll_skipped: u64,
    }

    fn load(bus: &mut SystemBus, at: u64, code: &[u8]) {
        for (i, b) in code.iter().enumerate() {
            bus.write_u8(at + i as u64, *b).unwrap();
        }
    }

    fn arm(cpu: &mut XtensaLx7, pc: u64) {
        cpu.pc = pc as u32;
        cpu.regs.write_logical(6, DATA);
        cpu.sr.write(CCOUNT, 0);
    }

    fn run(dual: bool, fast_forward: bool) -> Outcome {
        let mut bus = SystemBus::new();
        let mut primary = configure_xtensa_esp32(&mut bus);
        load(&mut bus, CODE, &FRAME);
        load(&mut bus, IDLE, &PARK);
        for (i, end) in PULSE_END.iter().enumerate() {
            bus.write_u32(u64::from(DATA) + 64 + 4 * i as u64, *end).unwrap();
        }
        bus.write_u32(u64::from(DATA) + 80, TAIL_END).unwrap();
        bus.write_u32(u64::from(DATA) + 84, ACC_END).unwrap();
        let mut machine = if dual {
            arm(&mut primary, IDLE);
            let mut app = XtensaLx7::new_app_cpu();
            arm(&mut app, CODE);
            app.unhalt();
            Machine::new(primary, bus).with_secondary_cpu(app)
        } else {
            arm(&mut primary, CODE);
            Machine::new(primary, bus)
        };
        machine.config.idle_fast_forward_enabled = fast_forward;
        while machine.total_cycles < RUN_CYCLES {
            machine
                .advance(AdvanceRequest::run(Some(RUN_CYCLES - machine.total_cycles)))
                .unwrap();
        }
        let records = (0..12)
            .map(|i| machine.bus.read_u32(u64::from(DATA) + 4 * i).unwrap())
            .collect();
        let runner: &XtensaLx7 = if dual {
            machine.cpu_secondary.as_ref().unwrap()
        } else {
            &machine.cpu
        };
        Outcome {
            records,
            ccount: runner.sr.read(CCOUNT),
            total_cycles: machine.total_cycles,
            skipped: machine.idle_fast_forward_cycles_skipped,
            poll_skipped: machine.poll_loop_cycles_skipped,
        }
    }

    fn check(dual: bool) {
        let fast = run(dual, true);
        let reference = run(dual, false);
        assert_eq!(reference.skipped, 0);
        // The program ran to the end in the reference run.
        assert!(reference.records[11] != 0, "reference never reached the end");
        assert_eq!(
            fast.records, reference.records,
            "the skip changed what the firmware recorded"
        );
        assert_eq!(fast.ccount, reference.ccount);
        assert_eq!(fast.total_cycles, reference.total_cycles);
        // Each output dropped at the first iteration with dt >= its deadline.
        for (i, end) in PULSE_END.iter().enumerate() {
            let dt = fast.records[i];
            assert!(dt >= *end && dt < end + 64, "output {i}: dt {dt} vs end {end}");
        }
        // The skip covered most of the pulse and tail waiting (~480 k
        // cycles), and none of the two loops it must refuse (another ~24 k).
        assert!(
            fast.poll_skipped > 440_000,
            "poll-loop skip did not fire: skipped {} of the ~480 k busy cycles",
            fast.poll_skipped
        );
        assert!(
            fast.poll_skipped < u64::from(TAIL_END),
            "the skip ran through a loop it must refuse ({} cycles)",
            fast.poll_skipped
        );
    }

    #[test]
    fn poll_loop_skip_on_the_only_core_changes_nothing() {
        check(false);
    }

    #[test]
    fn poll_loop_skip_on_an_active_secondary_changes_nothing() {
        check(true);
    }
}
