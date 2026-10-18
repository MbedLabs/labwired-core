# Validation Report: efm32pg12

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T00:53:06.951074Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:06.951232Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/efm32pg12.yaml
[2m2026-10-18T00:53:06.960964Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T00:53:06.961071Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T00:53:06.961081Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T00:53:06.961085Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T00:53:06.961756Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T00:53:06.961771Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T00:53:06.961776Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T00:53:06.971684Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 944193.09
[2m2026-10-18T00:53:06.985602Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T00:53:06.985622Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x466
[2m2026-10-18T00:53:06.985628Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 19999
[2m2026-10-18T00:53:06.985632Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20002
[2m2026-10-18T00:53:06.985637Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 814725.76
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `usart0` | `0x40010000` | `efm32_uart` |
| `usart1` | `0x40010400` | `efm32_uart` |
| `usart2` | `0x40010800` | `efm32_uart` |
| `usart3` | `0x40010c00` | `efm32_uart` |
| `leUart0` | `0x4004a000` | `leuart` |
| `bitband` | `0x42000000` | `bitbanding` |
| `cmuStatusOscillator` | `0x400c802c` | `pythonperipheral` |
| `vcmpStatus` | `0x40000008` | `pythonperipheral` |

**Total Peripherals:** 8