# Validation Report: mimxrt798s

**Architecture:** ARMv8-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:24.381129Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:24.381238Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/mimxrt798s.yaml
[2m2026-10-18T11:37:24.459193Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:24.459269Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:24.459274Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:24.459277Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:24.459749Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:24.459754Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:24.459757Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:24.459809Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:24.459820Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:24.459823Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:24.459825Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:24.459827Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:24.459830Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3632.69
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