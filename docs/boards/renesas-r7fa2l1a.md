# Validation Report: renesas-r7fa2l1a

**Architecture:** ARM Cortex-M23.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:31.268164Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'iic1' (type 'renesasra_iic')

```text
[2m2026-10-18T00:53:31.265588Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:31.266924Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-r7fa2l1a.yaml
[2m2026-10-18T00:53:31.267990Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.268014Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.268024Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T00:53:31.268164Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'iic1' (type 'renesasra_iic') has no `config.profile`, and the default I2C register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `port6` | `0x400400c0` | `renesasra2_gpio` |
| `port7` | `0x400400e0` | `renesasra2_gpio` |
| `port8` | `0x40040100` | `renesasra2_gpio` |
| `sci3` | `0x40070060` | `renesasra6m5_sci` |
| `iic1` | `0x40053100` | `renesasra_iic` |
| `portMisc` | `0x40040d00` | `renesasra_gpiomisc` |
| `port0` | `0x40040000` | `renesasra2_gpio` |
| `port1` | `0x40040020` | `renesasra2_gpio` |
| `port2` | `0x40040040` | `renesasra2_gpio` |
| `port3` | `0x40040060` | `renesasra2_gpio` |
| `port4` | `0x40040080` | `renesasra2_gpio` |
| `port5` | `0x400400a0` | `renesasra2_gpio` |
| `port9` | `0x40040120` | `renesasra2_gpio` |
| `gpt` | `0x40078000` | `renesasra_gpt` |
| `agt0` | `0x40084000` | `renesasra_agt` |
| `agt1` | `0x40084100` | `renesasra_agt` |
| `sci0` | `0x40070000` | `renesasra6m5_sci` |
| `sci1` | `0x40070020` | `renesasra6m5_sci` |
| `sci2` | `0x40070040` | `renesasra6m5_sci` |
| `sci9` | `0x40070120` | `renesasra6m5_sci` |
| `sysc_oscsf` | `0x4001e03c` | `pythonperipheral` |
| `iic0` | `0x40053000` | `renesasra_iic` |

**Total Peripherals:** 22