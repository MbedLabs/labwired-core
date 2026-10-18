# Validation Report: zynqmp

**Architecture:** Quad ARM Cortex-A53 / Dual Cortex-R5F.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:54:03.617280Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'cadence_i2c') )

```text
[2m2026-10-18T00:54:02.188599Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:54:02.190115Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/zynqmp.yaml
[2m2026-10-18T00:54:03.617280Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'cadence_i2c') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
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

**Total Peripherals:** 23