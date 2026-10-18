# Validation Report: cortex-r52

**Architecture:** ARMv8-R.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:19.995224Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:19.995400Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/cortex-r52.yaml
[2m2026-10-18T11:37:20.046911Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:20.047027Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:20.047045Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:20.047050Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:20.047587Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:20.047602Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:20.047607Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:20.051266Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2375548.55
[2m2026-10-18T11:37:20.054402Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:20.054416Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x454
[2m2026-10-18T11:37:20.054420Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 20000
[2m2026-10-18T11:37:20.054422Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20004
[2m2026-10-18T11:37:20.054425Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2713362.18
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart0` | `0x9c090000` | `pl011` |

**Total Peripherals:** 1