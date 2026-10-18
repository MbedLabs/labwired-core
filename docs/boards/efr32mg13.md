# Validation Report: efr32mg13

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:07.454608Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c1' (type 'efr32_i2ccontr)

```text
[2m2026-10-18T00:53:07.452889Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:07.453152Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/efr32mg13.yaml
[2m2026-10-18T00:53:07.454608Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c1' (type 'efr32_i2ccontroller') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `rtcc` | `0x40042000` | `efr32_rtcc` |
| `i2c1` | `0x4000c400` | `efr32_i2ccontroller` |
| `usart2` | `0x40010800` | `efr32_usart` |
| `usart3` | `0x40010c00` | `efr32_usart` |
| `flashCtrl` | `0x400e0000` | `efr32xg13flashcontroller` |
| `wtimer0` | `0x4001a000` | `efr32_timer` |
| `wtimer1` | `0x4001a400` | `efr32_timer` |
| `emu_pwrcfg` | `0x400e3038` | `pythonperipheral` |
| `emu_if` | `0x400e3024` | `pythonperipheral` |
| `i2c0` | `0x4000c000` | `efr32_i2ccontroller` |
| `usart0` | `0x40010000` | `efr32_usart` |
| `usart1` | `0x40010400` | `efr32_usart` |
| `leUart0` | `0x4004a000` | `leuart` |
| `gpioPort` | `0x4000a000` | `efr32_gpioport` |
| `bitband_peripherals` | `0x42000000` | `bitbanding` |
| `bitclear` | `0x44000000` | `bitaccess` |
| `bitset` | `0x46000000` | `bitaccess` |
| `bitband_sram` | `0x22000000` | `bitbanding` |
| `timer0` | `0x40018000` | `efr32_timer` |
| `timer1` | `0x40018400` | `efr32_timer` |
| `cmu` | `0x400e4000` | `efr32_cmu` |
| `ldma` | `0x400e2000` | `efr32mg12_ldma` |

**Total Peripherals:** 22