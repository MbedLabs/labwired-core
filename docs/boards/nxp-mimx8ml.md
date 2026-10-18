# Validation Report: nxp-mimx8ml

**Architecture:** ARM multicore cluster.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:29.424748Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m UART type 'mxcuart' (peripheral 'uart1')

```text
[2m2026-10-18T00:53:24.523322Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:24.523541Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/nxp-mimx8ml.yaml
[2m2026-10-18T00:53:28.837192Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'nxp_imx_gpio' (peripheral 'gpio1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:28.837303Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'nxp_imx_gpio' (peripheral 'gpio2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:28.837342Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'nxp_imx_gpio' (peripheral 'gpio3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:28.837375Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'nxp_imx_gpio' (peripheral 'gpio4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:28.837407Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'nxp_imx_gpio' (peripheral 'gpio5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:29.424748Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m UART type 'mxcuart' (peripheral 'uart1') has no register layout modelled yet and no `config.profile` set; it will NOT be silently mapped onto an STM32. Choose a layout explicitly with `config: { profile: <one of the supported layouts> }`, or add a dedicated model for it.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `gpt1` | `0x302d0000` | `imx_gptimer` |
| `gpt2` | `0x302e0000` | `imx_gptimer` |
| `gpt3` | `0x302f0000` | `imx_gptimer` |
| `gpt4` | `0x30700000` | `imx_gptimer` |
| `gpt5` | `0x306f0000` | `imx_gptimer` |
| `gpt6` | `0x306e0000` | `imx_gptimer` |
| `gpio1` | `0x30200000` | `nxp_imx_gpio` |
| `gpio2` | `0x30210000` | `nxp_imx_gpio` |
| `gpio3` | `0x30220000` | `nxp_imx_gpio` |
| `gpio4` | `0x30230000` | `nxp_imx_gpio` |
| `gpio5` | `0x30240000` | `nxp_imx_gpio` |
| `uart1` | `0x30860000` | `mxcuart` |
| `uart2` | `0x30890000` | `mxcuart` |
| `uart3` | `0x30880000` | `mxcuart` |
| `uart4` | `0x30a60000` | `mxcuart` |
| `usdhc1` | `0x30b40000` | `nxp_usdhc` |
| `usdhc2` | `0x30b50000` | `nxp_usdhc` |
| `usdhc3` | `0x30b60000` | `nxp_usdhc` |
| `wdog1` | `0x30280000` | `nxp_imx_watchdog` |
| `wdog2` | `0x30290000` | `nxp_imx_watchdog` |
| `wdog3` | `0x302a0000` | `nxp_imx_watchdog` |
| `src` | `0x30390000` | `nxp_imx8mp_systemresetcontroller` |
| `flexcan1` | `0x308c0000` | `nxp_flexcan` |
| `flexcan2` | `0x308d0000` | `nxp_flexcan` |
| `eth_mac` | `0x30be0000` | `k6xf_ethernet` |
| `eth_qos` | `0x30bf0000` | `synopsysdwcethernetqualityofservice` |
| `i2c1` | `0x30a20000` | `nxp_imx_i2c` |
| `i2c2` | `0x30a30000` | `nxp_imx_i2c` |
| `i2c3` | `0x30a40000` | `nxp_imx_i2c` |
| `i2c4` | `0x30a50000` | `nxp_imx_i2c` |
| `i2c5` | `0x30ad0000` | `nxp_imx_i2c` |
| `i2c6` | `0x30ae0000` | `nxp_imx_i2c` |
| `apis1_gpc` | `0x303a0000` | `pythonperipheral` |
| `ecspi1` | `0x30820000` | `nxp_ecspi` |
| `ecspi2` | `0x30830000` | `nxp_ecspi` |
| `ecspi3` | `0x30840000` | `nxp_ecspi` |

**Total Peripherals:** 36