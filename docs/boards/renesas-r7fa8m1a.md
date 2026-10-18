# Validation Report: renesas-r7fa8m1a

**Architecture:** ARM Cortex-M85.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:31.681251Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'iic0' (type 'renesasra_iic')

```text
[2m2026-10-18T00:53:31.670986Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:31.671150Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-r7fa8m1a.yaml
[2m2026-10-18T00:53:31.680707Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680747Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680760Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680771Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680782Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680802Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680815Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680826Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680836Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680847Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680857Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680868Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'portA') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.680878Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'portB') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.681251Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'iic0' (type 'renesasra_iic') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `option_setting` | `0x27030050` | `arraymemory` |
| `gpt` | `0x40322000` | `renesasra_gpt` |
| `gpt_ns` | `0x50322000` | `renesasra_gpt` |
| `agt0` | `0x40221000` | `renesasra_agt` |
| `agt1` | `0x40221100` | `renesasra_agt` |
| `sci0` | `0x40358000` | `renesasra8m1_sci` |
| `sci1` | `0x40358100` | `renesasra8m1_sci` |
| `sci2` | `0x40358200` | `renesasra8m1_sci` |
| `sci3` | `0x40358300` | `renesasra8m1_sci` |
| `sci4` | `0x40358400` | `renesasra8m1_sci` |
| `sci9` | `0x40358900` | `renesasra8m1_sci` |
| `portMisc` | `0x40400d00` | `renesasra_gpiomisc` |
| `port0` | `0x40400000` | `renesasra8_gpio` |
| `port1` | `0x40400020` | `renesasra8_gpio` |
| `port2` | `0x40400040` | `renesasra8_gpio` |
| `port3` | `0x40400060` | `renesasra8_gpio` |
| `port4` | `0x40400080` | `renesasra8_gpio` |
| `port5` | `0x404000a0` | `renesasra8_gpio` |
| `port6` | `0x404000c0` | `renesasra8_gpio` |
| `port7` | `0x404000e0` | `renesasra8_gpio` |
| `port8` | `0x40400100` | `renesasra8_gpio` |
| `port9` | `0x40400120` | `renesasra8_gpio` |
| `portA` | `0x40400140` | `renesasra8_gpio` |
| `portB` | `0x40400160` | `renesasra8_gpio` |
| `SYSC_SCICKCR` | `0x4001e055` | `pythonperipheral` |
| `iic0` | `0x4025e000` | `renesasra_iic` |
| `iic1` | `0x4025e100` | `renesasra_iic` |

**Total Peripherals:** 27