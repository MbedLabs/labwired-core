# Validation Report: efm32jg12

**Architecture:** ARM Cortex-M3.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T00:53:06.596563Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:06.596845Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/efm32jg12.yaml
[2m2026-10-18T00:53:06.598736Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T00:53:06.598884Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T00:53:06.598936Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T00:53:06.598964Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T00:53:06.599766Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T00:53:06.599826Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T00:53:06.599856Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T00:53:06.603744Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2104152.28
[2m2026-10-18T00:53:06.607610Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T00:53:06.607679Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x454
[2m2026-10-18T00:53:06.607708Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 20000
[2m2026-10-18T00:53:06.607733Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20004
[2m2026-10-18T00:53:06.607757Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2280656.44
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `i2c0` | `0x4000c000` | `efm32ggi2ccontroller` |
| `i2c1` | `0x4000c400` | `efm32ggi2ccontroller` |
| `timer0` | `0x40018000` | `efm32timer` |
| `timer1` | `0x40018400` | `efm32timer` |
| `usart0` | `0x40010000` | `efm32_uart` |
| `usart1` | `0x40010400` | `efm32_uart` |
| `usart2` | `0x40010800` | `efm32_uart` |
| `usart3` | `0x40010c00` | `efm32_uart` |
| `leUart0` | `0x4004a000` | `leuart` |
| `gpioPort` | `0x4000a000` | `efmgpioport` |
| `bitband` | `0x42000000` | `bitbanding` |
| `cmuStatusOscillator` | `0x400c802c` | `pythonperipheral` |
| `vcmpStatus` | `0x40000008` | `pythonperipheral` |

**Total Peripherals:** 13