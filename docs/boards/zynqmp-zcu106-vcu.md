# Validation Report: zynqmp-zcu106-vcu

**Architecture:** ARM multicore cluster.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:54:02.077488Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'allegro_e310' )

```text
[2m2026-10-18T00:54:00.698469Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:54:00.698635Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/zynqmp-zcu106-vcu.yaml
[2m2026-10-18T00:54:02.077488Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'allegro_e310' for peripheral 'allegro' in chip 'zynqmp-zcu106-vcu' (chip file '/home/andrii/Projects/labwired/core/configs/systems/onboarding/../../chips/onboarding/zynqmp-zcu106-vcu.yaml'): this engine has no model for it, no plugin claimed it, and it is not on the known-stub allowlist. Refusing to answer it with a zero-filled stub — firmware would read zeros from silicon that was never modelled and the run would still report success. Fix it one of three ways: model the peripheral, describe it with `type: declarative` and a descriptor `path`, or — if answering zeros really is right — declare `type: stub` in the chip YAML (or add 'allegro_e310' to KNOWN_STUBBED_PERIPHERAL_TYPES in crates/core/src/bus/known_stubs.rs with a written reason).
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `allegro` | `0xa0100000` | `allegro_e310` |
| `phy3` | `0x0000000c` | `ethernetphysicallayer` |
| `i2c_mux` | `0x00000074` | `pca9548` |
| `tca6416_u97` | `0x00000020` | `tca6416` |
| `gdma` | `0xfd500000` | `zynqmp_dma` |
| `adma` | `0xffa80000` | `zynqmp_dma` |
| `uart0` | `0xff000000` | `cadence_uart` |
| `uart1` | `0xff010000` | `cadence_uart` |
| `i2c0` | `0xff020000` | `cadence_i2c` |
| `i2c1` | `0xff030000` | `cadence_i2c` |
| `ttc0` | `0xff110000` | `cadence_ttc` |
| `ttc1` | `0xff120000` | `cadence_ttc` |
| `ttc2` | `0xff130000` | `cadence_ttc` |
| `ttc3` | `0xff140000` | `cadence_ttc` |
| `gem0` | `0xff0b0000` | `cadencegem` |
| `gem1` | `0xff0c0000` | `cadencegem` |
| `gem2` | `0xff0d0000` | `cadencegem` |
| `gem3` | `0xff0e0000` | `cadencegem` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `gpio` | `0xff0a0000` | `xilinxgpiops` |
| `ipi` | `0xff300000` | `zynqmp_ipi` |
| `rtc` | `0xffa60000` | `zynqmp_rtc` |
| `L1_PLL_STATUS_READ_1_SERDES_REGISTER` | `0xfd4063e4` | `arraymemory` |
| `L2_PLL_STATUS_READ_1_SERDES_REGISTER` | `0xfd40a3e4` | `arraymemory` |
| `L3_PLL_STATUS_READ_1_SERDES_REGISTER` | `0xfd40e3e4` | `arraymemory` |
| `PS_CTRL_STATUS_AMS_REGISTER` | `0xffa50040` | `pythonperipheral` |
| `qspi` | `0xff0f0000` | `zynqmp_gqspi` |

**Total Peripherals:** 27