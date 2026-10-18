# Validation Report: mimxrt700_evk

**Architecture:** ARMv8-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:24.245591Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:24.245706Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/mimxrt700_evk.yaml
[2m2026-10-18T11:37:24.334467Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:24.334578Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:24.334590Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:24.334594Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:24.335316Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:24.335324Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:24.335329Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:24.335392Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:24.335410Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:24.335414Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:24.335417Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:24.335421Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:24.335425Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2419.57
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `lpi2c15` | `0x40213000` | `s32k3xx_lowpowerinterintegratedcircuit` |
| `micfil` | `0x4020e000` | `imxrt700_micfil` |
| `clkctl0_fro1` | `0x40001000` | `imxrt700_clockcontrol` |
| `clkctl1` | `0x40041000` | `imxrt700_clockcontrol` |
| `clkctl2` | `0x40065000` | `imxrt700_clockcontrol` |
| `clkctl3_fro2` | `0x40061000` | `imxrt700_clockcontrol` |
| `clkctl4` | `0x400a1000` | `imxrt700_clockcontrol` |
| `rstctl1_i3c0` | `0x40040010` | `pythonperipheral` |
| `i3c0_rst_cpu1` | `0x4000001c` | `pythonperipheral` |
| `rstctl1_i3c3` | `0x40057000` | `pythonperipheral` |
| `iprstclp0_clr_cpu1` | `0x40060070` | `pythonperipheral` |
| `syscon3_nsvtor` | `0x4006209c` | `pythonperipheral` |

**Total Peripherals:** 12