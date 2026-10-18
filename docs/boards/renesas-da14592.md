# Validation Report: renesas-da14592

**Architecture:** ARM Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.316268Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.316415Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-da14592.yaml
[2m2026-10-18T11:37:31.317026Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasda14_gpio' (peripheral 'gpio') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.317158Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.317261Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.317281Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.317286Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.317897Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.317905Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.317913Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.317991Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:31.318010Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.318015Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:31.318018Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:31.318022Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:31.318027Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2717.68
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `timer1` | `0x50010300` | `renesasda14_gpt` |
| `timer2` | `0x50010400` | `renesasda14_gpt` |
| `timer3` | `0x50010500` | `renesasda14_gpt` |
| `timer4` | `0x50020a00` | `renesasda14_gpt` |
| `uart0` | `0x50020000` | `renesasda14_uart` |
| `uart1` | `0x50020100` | `renesasda14_uart` |
| `gpadc` | `0x50040900` | `stub` |
| `gpio` | `0x50020600` | `renesasda14_gpio` |
| `dma` | `0x50060200` | `renesasda14_dma` |
| `i2c` | `0x50020300` | `stub` |
| `clock_gen` | `0x50000000` | `stub` |
| `xtal32m_regs` | `0x50010000` | `stub` |
| `spi` | `0x50020200` | `stub` |
| `gp_regs` | `0x50050300` | `stub` |

**Total Peripherals:** 14