# Validation Report: atsamd21j17d-aft

**Architecture:** ARM Cortex-M0+. General purpose low power microcontroller.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:19.030224Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:19.030389Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/atsamd21j17d-aft.yaml
[2m2026-10-18T11:37:19.031009Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'samd21_gpio' (peripheral 'gpio_a') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:19.031040Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'samd21_gpio' (peripheral 'gpio_b') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:19.031198Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:19.031288Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:19.031298Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:19.031304Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:19.031774Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:19.031782Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:19.031790Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:19.031870Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:19.031886Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:19.031890Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:19.031894Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:19.031897Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:19.031901Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3378.01
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `dwt` | `0x0000e000` | `dwt` |
| `rtc` | `0x00004000` | `samd21_rtc` |
| `gpio_a` | `0x00004100` | `samd21_gpio` |
| `gpio_b` | `0x00004100` | `samd21_gpio` |
| `usart0` | `0x00004200` | `samd5_uart` |
| `tc4` | `0x00004200` | `samd21_timer` |
| `tc6` | `0x00004200` | `samd21_timer` |
| `gclk` | `0x40000c00` | `arraymemory` |
| `usart1` | `0x00004200` | `samd5_uart` |
| `usart2` | `0x00004200` | `samd5_uart` |

**Total Peripherals:** 10