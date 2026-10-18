# Validation Report: eos-s3-qomu

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:07.768660Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'usbserialport_)

```text
[2m2026-10-18T00:53:07.766852Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:07.767091Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/eos-s3-qomu.yaml
[2m2026-10-18T00:53:07.768660Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'usbserialport_s3b' for peripheral 'usbserial' in chip 'eos-s3-qomu' (chip file '/home/andrii/Projects/labwired/core/configs/systems/onboarding/../../chips/onboarding/eos-s3-qomu.yaml'): this engine has no model for it, no plugin claimed it, and it is not on the known-stub allowlist. Refusing to answer it with a zero-filled stub — firmware would read zeros from silicon that was never modelled and the run would still report success. Fix it one of three ways: model the peripheral, describe it with `type: declarative` and a descriptor `path`, or — if answering zeros really is right — declare `type: stub` in the chip YAML (or add 'usbserialport_s3b' to KNOWN_STUBBED_PERIPHERAL_TYPES in crates/core/src/bus/known_stubs.rs with a written reason).
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `ledB` | `0x00000003` | `led` |
| `ledG` | `0x00000001` | `led` |
| `ledR` | `0x00000000` | `led` |
| `usbserial` | `0x40020000` | `usbserialport_s3b` |
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

**Total Peripherals:** 18