# Validation Report: sam_e70

**Architecture:** ARMv7E-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:32.009685Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:32.009799Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/sam_e70.yaml
[2m2026-10-18T11:37:32.192214Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:32.192307Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:32.192315Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:32.192319Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:32.192854Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:32.192863Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:32.192872Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:32.195956Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2756888.98
[2m2026-10-18T11:37:32.197705Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:32.197709Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x466
[2m2026-10-18T11:37:32.197711Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 19999
[2m2026-10-18T11:37:32.197713Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20002
[2m2026-10-18T11:37:32.197716Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3711684.22
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `usart0` | `0x40024000` | `sam_usart` |
| `usart1` | `0x40028000` | `sam_usart` |
| `usart2` | `0x4002c000` | `sam_usart` |
| `gem` | `0x40050000` | `cadencegem` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `trng` | `0x40070000` | `sam_trng` |
| `PMC_SR` | `0x400e0668` | `pythonperipheral` |

**Total Peripherals:** 7