# Validation Report: imxrt1064

**Architecture:** ARMv7E-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:12.155631Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'lpspi1' (type 'imxrt_lpspi')

```text
[2m2026-10-18T00:53:12.148876Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:12.149032Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/imxrt1064.yaml
[2m2026-10-18T00:53:12.155631Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'lpspi1' (type 'imxrt_lpspi') has no `config.profile`, and the default SPI register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `wdog1` | `0x400b8000` | `pythonperipheral` |
| `trng` | `0x400cc000` | `imx_trng` |
| `wdog2` | `0x400d0000` | `pythonperipheral` |
| `wdog3` | `0x400bc000` | `pythonperipheral` |
| `iomuxc` | `0x401f8000` | `pythonperipheral` |
| `analog01` | `0x400d8000` | `pythonperipheral` |
| `analog03` | `0x400d8034` | `pythonperipheral` |
| `dcdc` | `0x40080000` | `pythonperipheral` |
| `gpTimer1` | `0x401ec000` | `imx_gptimer` |
| `lpuart1` | `0x40184000` | `nxp_lpuart` |
| `lpuart2` | `0x40188000` | `nxp_lpuart` |
| `lpuart3` | `0x4018c000` | `nxp_lpuart` |
| `lpuart4` | `0x40190000` | `nxp_lpuart` |
| `lpuart5` | `0x40194000` | `nxp_lpuart` |
| `lpuart6` | `0x40198000` | `nxp_lpuart` |
| `lpuart7` | `0x4019c000` | `nxp_lpuart` |
| `lpuart8` | `0x401a0000` | `nxp_lpuart` |
| `gpio1` | `0x401b8000` | `imxrt_gpio` |
| `gpio2` | `0x401bc000` | `imxrt_gpio` |
| `gpio3` | `0x401c0000` | `imxrt_gpio` |
| `gpio4` | `0x401c4000` | `imxrt_gpio` |
| `gpio5` | `0x400c0000` | `imxrt_gpio` |
| `gpio6` | `0x42000000` | `imxrt_gpio` |
| `gpio7` | `0x42004000` | `imxrt_gpio` |
| `gpio8` | `0x42008000` | `imxrt_gpio` |
| `gpio9` | `0x4200c000` | `imxrt_gpio` |
| `gpio10` | `0x401c8000` | `imxrt_gpio` |
| `enet` | `0x402d8000` | `k6xf_ethernet` |
| `enet2` | `0x402d4000` | `k6xf_ethernet` |
| `flex_spi` | `0x402a8000` | `imxrt_flexspi` |
| `flex_spi2` | `0x402a4000` | `imxrt_flexspi` |
| `lpspi1` | `0x40394000` | `imxrt_lpspi` |
| `lpspi2` | `0x40398000` | `imxrt_lpspi` |
| `lpspi3` | `0x4039c000` | `imxrt_lpspi` |
| `lpspi4` | `0x403a0000` | `imxrt_lpspi` |
| `adc1` | `0x400c4000` | `imxrt_adc` |
| `adc2` | `0x400c8000` | `imxrt_adc` |
| `pwm1` | `0x403dc000` | `imxrt_pwm` |
| `pwm2` | `0x403e0000` | `imxrt_pwm` |
| `pwm3` | `0x403e4000` | `imxrt_pwm` |
| `pwm4` | `0x403e8000` | `imxrt_pwm` |

**Total Peripherals:** 41