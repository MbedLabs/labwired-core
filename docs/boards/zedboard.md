# Validation Report: zedboard

**Architecture:** Dual ARM Cortex-A9.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:37.107614Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:37.107741Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/zedboard.yaml
[2m2026-10-18T11:37:37.861289Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'xilinxgpiops' (peripheral 'gpio') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:37.861450Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:37.861522Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:37.861529Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:37.861532Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:37.862282Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:37.862291Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x0, SP: 0x0
[2m2026-10-18T11:37:37.862299Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:37.865646Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2434910.40
[2m2026-10-18T11:37:37.869066Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:37.869080Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x9c40
[2m2026-10-18T11:37:37.869084Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 20000
[2m2026-10-18T11:37:37.869088Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20000
[2m2026-10-18T11:37:37.869091Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2646848.13
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `usbEhci2` | `0xe0003000` | `ehcihostcontroller` |
| `usbEhci` | `0xe0002000` | `ehcihostcontroller` |
| `pl310` | `0xf8f02000` | `pl310` |
| `gem0` | `0xe000b000` | `cadencegem` |
| `gem1` | `0xe000c000` | `cadencegem` |
| `uart0` | `0xe0000000` | `cadence_uart` |
| `uart1` | `0xe0001000` | `cadence_uart` |
| `i2c1` | `0xe0005000` | `stub` |
| `i2c0` | `0xe0004000` | `stub` |
| `spi0` | `0xe0006000` | `stub` |
| `spi1` | `0xe0007000` | `stub` |
| `sdhci0` | `0xe0100000` | `stub` |
| `sdhci1` | `0xe0101000` | `stub` |
| `ttc0` | `0xf8001000` | `cadence_ttc` |
| `ttc1` | `0xf8002000` | `cadence_ttc` |
| `watchdog0` | `0xf8005000` | `stub` |
| `globalTimer` | `0xf8f00200` | `arm_globaltimer` |
| `scu` | `0xf8f00000` | `armsnoopcontrolunit` |
| `qspi` | `0xe000d000` | `xilinxqspi` |
| `gpio` | `0xe000a000` | `xilinxgpiops` |
| `xadc` | `0xf8007100` | `stub` |
| `slcr` | `0xf8000000` | `stub` |
| `nand` | `0xe000e000` | `pythonperipheral` |
| `dma_pl330` | `0xf8003000` | `pl330_dma` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `led0` | `0x0000003d` | `led` |
| `led1` | `0x0000003e` | `led` |
| `led2` | `0x0000003f` | `led` |
| `led3` | `0x00000040` | `led` |
| `led4` | `0x00000041` | `led` |
| `led5` | `0x00000042` | `led` |
| `led6` | `0x00000043` | `led` |
| `led7` | `0x00000044` | `led` |

**Total Peripherals:** 33