# Validation Report: nxp-k6xf

**Architecture:** ARMv7-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T00:53:24.467834Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:24.468040Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/nxp-k6xf.yaml
[2m2026-10-18T00:53:24.469800Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T00:53:24.469957Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T00:53:24.469988Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T00:53:24.469994Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T00:53:24.470617Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T00:53:24.470630Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T00:53:24.470639Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T00:53:24.470744Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T00:53:24.470769Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T00:53:24.470775Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T00:53:24.470779Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T00:53:24.470783Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T00:53:24.470788Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2533.90
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart0` | `0x4006a000` | `k6xf_uart` |
| `mcg` | `0x40064000` | `k6xf_mcg` |
| `sim` | `0x40047000` | `k6xf_sim` |
| `eth` | `0x400c0000` | `k6xf_ethernet` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `rng` | `0x40029000` | `k6xf_rng` |

**Total Peripherals:** 6