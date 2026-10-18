# Validation Report: vybrid

**Architecture:** ARMv7-A / ARMv7-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T00:53:51.346536Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:51.346701Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/vybrid.yaml
[2m2026-10-18T00:53:51.351157Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T00:53:51.351268Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T00:53:51.351288Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T00:53:51.351293Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T00:53:51.352117Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T00:53:51.352131Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T00:53:51.352139Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T00:53:51.352232Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T00:53:51.352256Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T00:53:51.352260Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T00:53:51.352265Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T00:53:51.352269Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T00:53:51.352273Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2048.91
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart0` | `0x40027000` | `imxuart` |
| `uart1` | `0x40028000` | `imxuart` |
| `uart2` | `0x40029000` | `imxuart` |
| `eDma0` | `0x40018000` | `vybriddma` |
| `eDma1` | `0x40098000` | `vybriddma` |
| `fb` | `0x40058000` | `vybriddcu` |
| `usbEhci0` | `0x40034000` | `ehcihostcontroller` |
| `usbEhci1` | `0x400b4000` | `ehcihostcontroller` |
| `i2c0` | `0x40066000` | `vybridi2c` |
| `i2c1` | `0x40067000` | `vybridi2c` |
| `timers` | `0x40037000` | `periodicinterrupttimer` |
| `globalTimer` | `0x40002200` | `arm_globaltimer` |
| `sema4` | `0x4001d000` | `sema4` |
| `nand` | `0x400e0000` | `fslnand` |

**Total Peripherals:** 14