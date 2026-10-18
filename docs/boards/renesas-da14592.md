# Validation Report: renesas-da14592

**Architecture:** ARM Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:30.820203Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'gpadc' (type 'renesasda14_g)

```text
[2m2026-10-18T00:53:30.816810Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:30.819295Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-da14592.yaml
[2m2026-10-18T00:53:30.820203Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'gpadc' (type 'renesasda14_gpadc') has no `config.profile`, and the default ADC register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `timer1` | `0x50010300` | `renesasda14_gpt` |
| `timer2` | `0x50010400` | `renesasda14_gpt` |
| `timer3` | `0x50010500` | `renesasda14_gpt` |
| `timer4` | `0x50020a00` | `renesasda14_gpt` |
| `uart0` | `0x50020000` | `renesasda14_uart` |
| `uart1` | `0x50020100` | `renesasda14_uart` |
| `gpadc` | `0x50040900` | `renesasda14_gpadc` |
| `gpio` | `0x50020600` | `renesasda14_gpio` |
| `dma` | `0x50060200` | `renesasda14_dma` |
| `i2c` | `0x50020300` | `designware_apb_i2c` |
| `clock_gen` | `0x50000000` | `renesasda14_clockgenerationcontroller` |
| `xtal32m_regs` | `0x50010000` | `renesasda14_xtal32mregisters` |
| `spi` | `0x50020200` | `renesasda_spi` |
| `gp_regs` | `0x50050300` | `renesasda14_generalpurposeregisters` |

**Total Peripherals:** 14