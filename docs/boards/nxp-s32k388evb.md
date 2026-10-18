# Validation Report: nxp-s32k388evb

**Architecture:** ARMv7E-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:29.811982Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'sema42' for pe)

```text
[2m2026-10-18T00:53:29.801085Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:29.801391Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/nxp-s32k388evb.yaml
[2m2026-10-18T00:53:29.811982Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'sema42' for peripheral 'sema4' in chip 'nxp-s32k388evb' (chip file '/home/andrii/Projects/labwired/core/configs/systems/onboarding/../../chips/onboarding/nxp-s32k388evb.yaml'): this engine has no model for it, no plugin claimed it, and it is not on the known-stub allowlist. Refusing to answer it with a zero-filled stub — firmware would read zeros from silicon that was never modelled and the run would still report success. Fix it one of three ways: model the peripheral, describe it with `type: declarative` and a descriptor `path`, or — if answering zeros really is right — declare `type: stub` in the chip YAML (or add 'sema42' to KNOWN_STUBBED_PERIPHERAL_TYPES in crates/core/src/bus/known_stubs.rs with a written reason).
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `userButton0` | `0x00000001` | `button` |
| `userButton1` | `0x0000001c` | `button` |
| `led2_red` | `0x000000b1` | `led` |
| `led2_green` | `0x000000b2` | `led` |
| `led2_blue` | `0x000000b3` | `led` |
| `led1_red` | `0x000000b6` | `led` |
| `led1_green` | `0x000000b7` | `led` |
| `led1_blue` | `0x000000b8` | `led` |
| `dwt` | `0xe0001000` | `dwt` |
| `mscm` | `0x40260000` | `s32k3xx_miscellaneoussystemcontrolmodule` |
| `siul2` | `0x40290000` | `s32k3xx_systemintegrationunitlite2` |
| `mc_me` | `0x402dc000` | `s32kxx_modeentrymodule` |
| `sema4` | `0x40460000` | `sema42` |
| `swt0` | `0x40270000` | `s32k3xx_softwarewatchdogtimer` |
| `swt1` | `0x4046c000` | `s32k3xx_softwarewatchdogtimer` |
| `swt2` | `0x40470000` | `s32k3xx_softwarewatchdogtimer` |
| `swt3` | `0x40070000` | `s32k3xx_softwarewatchdogtimer` |
| `stm0` | `0x40274000` | `s32k3xx_systemtimermodule` |
| `stm1` | `0x40474000` | `s32k3xx_systemtimermodule` |
| `stm2` | `0x40478000` | `s32k3xx_systemtimermodule` |
| `stm3` | `0x4047c000` | `s32k3xx_systemtimermodule` |
| `pit0` | `0x400b0000` | `s32k3xx_periodicinterrupttimer` |
| `pit1` | `0x400b4000` | `s32k3xx_periodicinterrupttimer` |
| `pit2` | `0x402fc000` | `s32k3xx_periodicinterrupttimer` |
| `pit3` | `0x40300000` | `s32k3xx_periodicinterrupttimer` |
| `rtc` | `0x40288000` | `s32k3xx_realtimeclock` |
| `lpi2c0` | `0x40350000` | `s32k3xx_lowpowerinterintegratedcircuit` |
| `lpi2c1` | `0x40354000` | `s32k3xx_lowpowerinterintegratedcircuit` |
| `lpspi0` | `0x40358000` | `imxrt_lpspi` |
| `lpspi1` | `0x4035c000` | `imxrt_lpspi` |
| `lpspi2` | `0x40360000` | `imxrt_lpspi` |
| `lpspi3` | `0x40364000` | `imxrt_lpspi` |
| `lpspi4` | `0x404bc000` | `imxrt_lpspi` |
| `lpspi5` | `0x404c0000` | `imxrt_lpspi` |
| `flex_io` | `0x40324000` | `s32k3xx_flexio` |
| `lpuart0` | `0x40328000` | `nxp_lpuart` |
| `lpuart1` | `0x4032c000` | `nxp_lpuart` |
| `lpuart2` | `0x40330000` | `nxp_lpuart` |
| `lpuart3` | `0x40334000` | `nxp_lpuart` |
| `lpuart4` | `0x40338000` | `nxp_lpuart` |
| `lpuart5` | `0x4033c000` | `nxp_lpuart` |
| `lpuart6` | `0x40340000` | `nxp_lpuart` |
| `lpuart7` | `0x40344000` | `nxp_lpuart` |
| `lpuart8` | `0x4048c000` | `nxp_lpuart` |
| `lpuart9` | `0x40490000` | `nxp_lpuart` |
| `lpuart10` | `0x40494000` | `nxp_lpuart` |
| `lpuart11` | `0x40498000` | `nxp_lpuart` |
| `lpuart12` | `0x4049c000` | `nxp_lpuart` |
| `lpuart13` | `0x404a0000` | `nxp_lpuart` |
| `lpuart14` | `0x404a4000` | `nxp_lpuart` |
| `lpuart15` | `0x404a8000` | `nxp_lpuart` |
| `can0` | `0x40304000` | `nxp_flexcan` |
| `can1` | `0x40308000` | `nxp_flexcan` |
| `can2` | `0x4030c000` | `nxp_flexcan` |
| `can3` | `0x40310000` | `nxp_flexcan` |
| `can4` | `0x40314000` | `nxp_flexcan` |
| `can5` | `0x40318000` | `nxp_flexcan` |
| `can6` | `0x4031c000` | `nxp_flexcan` |
| `can7` | `0x40320000` | `nxp_flexcan` |
| `gmac0` | `0x40484000` | `s32k3xx_gmac` |
| `gmac1` | `0x40488000` | `s32k3xx_gmac` |
| `dma` | `0x4020c000` | `nxp_edma` |
| `dma_channels0_11` | `0x40210000` | `nxp_edma_channels` |
| `dma_channels12_32` | `0x40410000` | `nxp_edma_channels` |
| `dma_mux0` | `0x40280000` | `s32k3xx_dmamux` |
| `dma_mux1` | `0x40284000` | `s32k3xx_dmamux` |
| `xrdc` | `0x40278000` | `nxp_xrdc` |
| `emios0` | `0x40088000` | `s32k3xx_emios` |
| `emios1` | `0x4008c000` | `s32k3xx_emios` |
| `emios2` | `0x40090000` | `s32k3xx_emios` |
| `mc_cgm` | `0x402d8000` | `s32k3xx_mc_cgm` |
| `plldig` | `0x402e0000` | `s32k3xx_plldig` |

**Total Peripherals:** 72