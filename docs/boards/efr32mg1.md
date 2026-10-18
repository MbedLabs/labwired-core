# Validation Report: efr32mg1

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:20.930436Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:20.930587Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/efr32mg1.yaml
[2m2026-10-18T11:37:20.931468Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'efr32_gpioport' (peripheral 'gpioPort') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:20.931619Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:20.931697Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:20.931703Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:20.931705Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:20.932208Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:20.932214Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:20.932218Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:20.932290Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:20.932305Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:20.932308Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:20.932310Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:20.932312Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:20.932315Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3296.10
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `i2c0` | `0x4000c000` | `stub` |
| `usart0` | `0x40010000` | `efr32_usart` |
| `usart1` | `0x40010400` | `efr32_usart` |
| `leUart0` | `0x4004a000` | `leuart` |
| `gpioPort` | `0x4000a000` | `efr32_gpioport` |
| `bitband_peripherals` | `0x42000000` | `bitbanding` |
| `bitclear` | `0x44000000` | `stub` |
| `bitset` | `0x46000000` | `stub` |
| `bitband_sram` | `0x22000000` | `bitbanding` |
| `timer0` | `0x40018000` | `efr32_timer` |
| `timer1` | `0x40018400` | `efr32_timer` |
| `cmu` | `0x400e4000` | `stub` |
| `ldma` | `0x400e2000` | `efr32mg12_ldma` |

**Total Peripherals:** 13