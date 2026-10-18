# Validation Report: cortex-a53-gicv3_smp

**Architecture:** ARMv8-A (AArch64).

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:19.738727Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:19.738907Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/cortex-a53-gicv3_smp.yaml
[2m2026-10-18T11:37:19.836965Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:19.837106Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:19.837118Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:19.837125Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:19.837565Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:19.837571Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:19.837579Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:19.837647Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:19.837670Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:19.837677Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:19.837683Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:19.837688Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:19.837695Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3567.88
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart0` | `0x09000000` | `pl011` |
| `rtc` | `0x09010000` | `pl031` |

**Total Peripherals:** 2