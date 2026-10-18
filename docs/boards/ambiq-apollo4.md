# Validation Report: ambiq-apollo4

**Architecture:** ARM Cortex-M4F. Ultra-low power MCU with SPOT technology.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:04.026938Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'ambiqapollo4_ad)

```text
[2m2026-10-18T00:53:04.025627Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:04.025859Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/ambiq-apollo4.yaml
[2m2026-10-18T00:53:04.026938Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'ambiqapollo4_adc') has no `config.profile`, and the default ADC register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `bootrom_logger` | `0x07fffffc` | `ambiqapollo4_bootromlogger` |
| `adc` | `0x40038000` | `ambiqapollo4_adc` |
| `gpio` | `0x40010000` | `ambiqapollo4_gpio` |
| `iom0` | `0x40050000` | `ambiqapollo4_iomaster` |
| `iom1` | `0x40051000` | `ambiqapollo4_iomaster` |
| `iom2` | `0x40052000` | `ambiqapollo4_iomaster` |
| `iom3` | `0x40053000` | `ambiqapollo4_iomaster` |
| `iom4` | `0x40054000` | `ambiqapollo4_iomaster` |
| `iom5` | `0x40055000` | `ambiqapollo4_iomaster` |
| `iom6` | `0x40056000` | `ambiqapollo4_iomaster` |
| `iom7` | `0x40057000` | `ambiqapollo4_iomaster` |
| `rtc` | `0x40004800` | `ambiqapollo4_rtc` |
| `timer` | `0x40008000` | `ambiqapollo4_timer` |
| `stimer` | `0x40008800` | `ambiqapollo4_systemtimer` |
| `uart0` | `0x4001c000` | `pl011` |
| `uart1` | `0x4001d000` | `pl011` |
| `uart2` | `0x4001e000` | `pl011` |
| `uart3` | `0x4001f000` | `pl011` |
| `pwrctrl` | `0x40021000` | `ambiqapollo4_powercontroller` |
| `security` | `0x40030000` | `ambiqapollo4_security` |
| `wdt` | `0x40024000` | `ambiqapollo4_watchdog` |
| `cpu_complex` | `0x48000000` | `pythonperipheral` |

**Total Peripherals:** 22