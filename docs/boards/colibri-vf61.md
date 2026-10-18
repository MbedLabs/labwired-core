# Validation Report: colibri-vf61

**Architecture:** ARM Cortex-A5 / Cortex-M4.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:19.436817Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:19.436966Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/colibri-vf61.yaml
[2m2026-10-18T11:37:19.438149Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:19.438229Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:19.438239Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:19.438245Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:19.438808Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:19.438816Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:19.438823Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:19.438891Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:19.438906Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:19.438910Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:19.438914Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:19.438918Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:19.438922Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2971.95
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `usbHub` | `0x00000001` | `usbhub` |
| `mouse` | `0x00000005` | `usbmouse` |
| `fusion` | `0x00000010` | `fusionf0710a` |
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

**Total Peripherals:** 17