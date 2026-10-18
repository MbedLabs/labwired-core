# Validation Report: max32652-evkit

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:15.431955Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'max32650_i2c'))

```text
[2m2026-10-18T00:53:15.424294Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:15.424457Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/max32652-evkit.yaml
[2m2026-10-18T00:53:15.431955Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'max32650_i2c') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `led0` | `0x00000019` | `led` |
| `led1` | `0x0000001a` | `led` |
| `gcr` | `0x40000000` | `max32650_gcr` |
| `wdt0` | `0x40003000` | `max32650_wdt` |
| `wdt1` | `0x40003400` | `max32650_wdt` |
| `timer0` | `0x40010000` | `max32650_timer` |
| `timer1` | `0x40011000` | `max32650_timer` |
| `tpu` | `0x40001000` | `max32650_tpu` |
| `timer2` | `0x40012000` | `max32650_timer` |
| `timer3` | `0x40013000` | `max32650_timer` |
| `timer4` | `0x40014000` | `max32650_timer` |
| `timer5` | `0x40015000` | `max32650_timer` |
| `i2c0` | `0x4001d000` | `max32650_i2c` |
| `i2c1` | `0x4001e000` | `max32650_i2c` |
| `adc` | `0x40034000` | `max32650_adc` |
| `uart0` | `0x40042000` | `max32650_uart` |
| `uart1` | `0x40043000` | `max32650_uart` |
| `uart2` | `0x40044000` | `max32650_uart` |
| `spi0` | `0x40046000` | `max32650_spi` |
| `spi1` | `0x40047000` | `max32650_spi` |
| `spi2` | `0x40048000` | `max32650_spi` |
| `gpio0` | `0x40008000` | `max32650_gpio` |
| `gpio1` | `0x40009000` | `max32650_gpio` |
| `gpio2` | `0x4000a000` | `max32650_gpio` |
| `gpio3` | `0x4000b000` | `max32650_gpio` |
| `spi3` | `0x400be000` | `max32650_spi` |
| `rtc` | `0x40006000` | `max32650_rtc` |
| `pwrseq` | `0x40006800` | `max32650_pwrseq` |
| `dwt` | `0xe0001000` | `dwt` |
| `bitbandPeripherals` | `0x42000000` | `bitbanding` |
| `bitbandSram` | `0x22000000` | `bitbanding` |

**Total Peripherals:** 31