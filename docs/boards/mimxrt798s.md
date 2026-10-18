# Validation Report: mimxrt798s

**Architecture:** ARMv8-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:16.160081Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'imxrt700_clock)

```text
[2m2026-10-18T00:53:15.989333Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:15.989500Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/mimxrt798s.yaml
[2m2026-10-18T00:53:16.160081Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'imxrt700_clockcontrol' for peripheral 'clkctl0_fro1' in chip 'mimxrt798s' (chip file '/home/andrii/Projects/labwired/core/configs/systems/onboarding/../../chips/onboarding/mimxrt798s.yaml'): this engine has no model for it, no plugin claimed it, and it is not on the known-stub allowlist. Refusing to answer it with a zero-filled stub — firmware would read zeros from silicon that was never modelled and the run would still report success. Fix it one of three ways: model the peripheral, describe it with `type: declarative` and a descriptor `path`, or — if answering zeros really is right — declare `type: stub` in the chip YAML (or add 'imxrt700_clockcontrol' to KNOWN_STUBBED_PERIPHERAL_TYPES in crates/core/src/bus/known_stubs.rs with a written reason).
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