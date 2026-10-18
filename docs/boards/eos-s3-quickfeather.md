# Validation Report: eos-s3-quickfeather

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:07.841332Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'eoss3_adc') has)

```text
[2m2026-10-18T00:53:07.839526Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:07.839683Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/eos-s3-quickfeather.yaml
[2m2026-10-18T00:53:07.841332Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m Peripheral 'adc' (type 'eoss3_adc') has no `config.profile`, and the default ADC register layout is STM32's. Refusing to model this silicon with another vendor's register map: an unmodelled peripheral is a gap you can see, a wrongly modelled one answers reads with registers that do not exist on this part. Set `config: { profile: <layout> }` explicitly.
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `user_button` | `0x00000001` | `button` |
| `ledB` | `0x00000004` | `led` |
| `ledG` | `0x00000005` | `led` |
| `ledR` | `0x00000006` | `led` |
| `mc3635` | `0x0000004c` | `mc3635` |
| `uart` | `0x40010000` | `pl011` |
| `dwt` | `0xe0001000` | `dwt` |
| `spt` | `0x40005c00` | `eoss3_simpleperiodictimer` |
| `adc` | `0x40005a00` | `eoss3_adc` |
| `spi` | `0x40007000` | `designware_spi` |
| `dmaSpi` | `0x40007400` | `eoss3_spi_dma` |
| `packetFifo` | `0x40002000` | `eoss3_packetfifo` |
| `systemDma` | `0x4000c000` | `udma` |
| `systemDmaBridge` | `0x4000d000` | `eoss3_systemdmabridge` |
| `ffe` | `0x4004a000` | `eoss3_flexiblefusionengine` |
| `i2cMaster0` | `0x00000000` | `opencoresi2c` |
| `i2cMaster1` | `0x00000001` | `opencoresi2c` |
| `voice` | `0x40015000` | `eoss3_voice` |
| `powerMgmt` | `0x40004400` | `pythonperipheral` |

**Total Peripherals:** 19