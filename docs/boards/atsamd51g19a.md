# Validation Report: atsamd51g19a

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:19.059112Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:19.059286Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/atsamd51g19a.yaml
[2m2026-10-18T11:37:19.060487Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:19.060658Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:19.060668Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:19.060672Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:19.061206Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:19.061218Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:19.061223Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:19.064968Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2331903.57
[2m2026-10-18T11:37:19.068422Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:19.068451Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x466
[2m2026-10-18T11:37:19.068455Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 19999
[2m2026-10-18T11:37:19.068459Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20002
[2m2026-10-18T11:37:19.068462Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2568644.35
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `sercom3` | `0x41014000` | `samd5_uart` |
| `dwt` | `0xe0001000` | `dwt` |
| `gclk_phctrl1` | `0x40001c84` | `pythonperipheral` |
| `pac_intflag` | `0x40001010` | `pythonperipheral` |
| `pac_status` | `0x40001040` | `pythonperipheral` |

**Total Peripherals:** 5