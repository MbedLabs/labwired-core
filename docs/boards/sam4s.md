# Validation Report: sam4s

**Architecture:** ARMv7E-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:32.503866Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'sam4s_adc') has)

```text
[2m2026-10-18T00:53:32.492368Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:32.492563Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/sam4s.yaml
[2m2026-10-18T00:53:32.503866Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'sam4s_adc') has no `config.profile`, and the default ADC register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `dwt` | `0xe0001000` | `dwt` |
| `sram_bb` | `0x22000000` | `bitbanding` |
| `tc0_1_2` | `0x40010000` | `sam_tc` |
| `tc3_4_5` | `0x40014000` | `sam_tc` |
| `usart0` | `0x40024000` | `sam_usart` |
| `usart1` | `0x40028000` | `sam_usart` |
| `adc` | `0x40038000` | `sam4s_adc` |
| `dacc` | `0x4003c000` | `sam4s_dacc` |
| `PMC_SR` | `0x400e0468` | `pythonperipheral` |
| `uart0` | `0x400e0600` | `sam_usart` |
| `uart1` | `0x400e0800` | `sam_usart` |
| `eefc` | `0x400e0a00` | `sam4s_eefc` |
| `rstc` | `0x400e1400` | `sam4s_rstc` |
| `wdt` | `0x400e1450` | `sam4s_wdt` |
| `peripheral_bb` | `0x42000000` | `bitbanding` |
| `crc` | `0x40044000` | `sam4s_crccu` |
| `pioA` | `0x400e0e00` | `sam4s_pio` |
| `pioB` | `0x400e1000` | `sam4s_pio` |
| `pioC` | `0x400e1200` | `sam4s_pio` |
| `spi` | `0x40008000` | `sam_spi` |
| `twi1` | `0x4001c000` | `sam4s_twi` |
| `twi0` | `0x40018000` | `sam4s_twi` |

**Total Peripherals:** 22