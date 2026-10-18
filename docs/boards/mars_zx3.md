# Validation Report: mars_zx3

**Architecture:** Dual ARM Cortex-A9.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:23.324277Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:23.324454Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/mars_zx3.yaml
[2m2026-10-18T11:37:23.968384Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'xilinxgpiops' (peripheral 'gpio') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:23.968616Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:23.968737Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:23.968746Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:23.968752Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:23.969454Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:23.969461Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x0, SP: 0x0
[2m2026-10-18T11:37:23.969468Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:23.971410Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 3772520.16
[2m2026-10-18T11:37:23.974126Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:23.974136Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x9c40
[2m2026-10-18T11:37:23.974139Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 20000
[2m2026-10-18T11:37:23.974142Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20000
[2m2026-10-18T11:37:23.974144Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3712478.21
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
| `phy` | `0x00000003` | `ethernetphysicallayer` |
| `usbHub` | `0x00000001` | `usbhub` |
| `usbMouse` | `0x00000002` | `usbmouse` |
| `usbKeyboard` | `0x00000001` | `usbkeyboard` |

**Total Peripherals:** 28