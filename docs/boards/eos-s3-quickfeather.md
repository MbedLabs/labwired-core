# Validation Report: eos-s3-quickfeather

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:21.161442Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:21.161618Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/eos-s3-quickfeather.yaml
[2m2026-10-18T11:37:21.162693Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:21.162786Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:21.162795Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:21.162802Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:21.163680Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:21.163696Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:21.163709Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:21.163791Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:21.163816Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:21.163823Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:21.163828Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:21.163833Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:21.163840Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1935.33
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `user_button` | `0x00000001` | `button` |
| `ledB` | `0x00000004` | `led` |
| `ledG` | `0x00000005` | `led` |
| `ledR` | `0x00000006` | `led` |
| `mc3635` | `0x0000004c` | `mc3635` |
| `uart` | `0x40010000` | `pl011` |
| `dwt` | `0xe0001000` | `dwt` |
| `spt` | `0x40005c00` | `eoss3_simpleperiodictimer` |
| `adc` | `0x40005a00` | `stub` |
| `spi` | `0x40007000` | `stub` |
| `dmaSpi` | `0x40007400` | `stub` |
| `packetFifo` | `0x40002000` | `stub` |
| `systemDma` | `0x4000c000` | `udma` |
| `systemDmaBridge` | `0x4000d000` | `eoss3_systemdmabridge` |
| `ffe` | `0x4004a000` | `stub` |
| `i2cMaster0` | `0x00000000` | `opencoresi2c` |
| `i2cMaster1` | `0x00000001` | `opencoresi2c` |
| `voice` | `0x40015000` | `stub` |
| `powerMgmt` | `0x40004400` | `pythonperipheral` |

**Total Peripherals:** 19