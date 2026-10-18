# Validation Report: alif_ae822fa0e5597bs0

**Architecture:** ARM Cortex-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:03.917606Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'designware_apb)

```text
[2m2026-10-18T00:53:03.915318Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:03.915489Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/alif_ae822fa0e5597bs0.yaml
[2m2026-10-18T00:53:03.916956Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917015Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917048Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917079Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917111Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917141Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917171Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917202Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917238Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917269Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917299Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio10') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917329Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio11') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917359Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio12') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917389Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio13') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917419Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'gpio14') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917449Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'designware_apb_gpio' (peripheral 'lpgpio') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:03.917606Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'i2c0' (type 'designware_apb_i2c') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `gpio0` | `0x49000000` | `designware_apb_gpio` |
| `gpio1` | `0x49001000` | `designware_apb_gpio` |
| `gpio2` | `0x49002000` | `designware_apb_gpio` |
| `gpio3` | `0x49003000` | `designware_apb_gpio` |
| `gpio4` | `0x49004000` | `designware_apb_gpio` |
| `gpio5` | `0x49005000` | `designware_apb_gpio` |
| `gpio6` | `0x49006000` | `designware_apb_gpio` |
| `gpio7` | `0x49007000` | `designware_apb_gpio` |
| `gpio8` | `0x49008000` | `designware_apb_gpio` |
| `gpio9` | `0x49009000` | `designware_apb_gpio` |
| `gpio10` | `0x4900a000` | `designware_apb_gpio` |
| `gpio11` | `0x4900b000` | `designware_apb_gpio` |
| `gpio12` | `0x4900c000` | `designware_apb_gpio` |
| `gpio13` | `0x4900d000` | `designware_apb_gpio` |
| `gpio14` | `0x4900e000` | `designware_apb_gpio` |
| `lpgpio` | `0x42002000` | `designware_apb_gpio` |
| `i2c0` | `0x49010000` | `designware_apb_i2c` |
| `i2c1` | `0x49011000` | `designware_apb_i2c` |
| `i2c2` | `0x49012000` | `designware_apb_i2c` |
| `i2c3` | `0x49013000` | `designware_apb_i2c` |
| `uart0` | `0x49018000` | `ns16550` |
| `uart1` | `0x49019000` | `ns16550` |
| `uart2` | `0x4901a000` | `ns16550` |
| `uart3` | `0x4901b000` | `ns16550` |
| `uart4` | `0x4901c000` | `ns16550` |
| `uart5` | `0x4901d000` | `ns16550` |
| `uart6` | `0x4901e000` | `ns16550` |
| `uart7` | `0x4901f000` | `ns16550` |
| `fdcan` | `0x49036000` | `cast_canctrl` |
| `dwt` | `0xe0001000` | `dwt` |

**Total Peripherals:** 30