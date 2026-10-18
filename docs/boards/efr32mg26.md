# Validation Report: efr32mg26

**Architecture:** ARM Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:21.065884Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:21.066035Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/efr32mg26.yaml
[2m2026-10-18T11:37:21.068462Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:21.068560Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:21.068573Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:21.068577Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:21.069116Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:21.069130Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:21.069134Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:21.072196Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2769975.68
[2m2026-10-18T11:37:21.074044Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:21.074057Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x466
[2m2026-10-18T11:37:21.074060Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 19999
[2m2026-10-18T11:37:21.074063Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20002
[2m2026-10-18T11:37:21.074065Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3646942.29
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `DCDC_IF` | `0x50098028` | `pythonperipheral` |

**Total Peripherals:** 1