# Validation Report: versatile

**Architecture:** ARMv7-A.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:36.358214Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:36.358322Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/versatile.yaml
[2m2026-10-18T11:37:36.512119Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:36.512212Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:36.512218Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:36.512221Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:36.512822Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:36.512831Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x0, SP: 0x0
[2m2026-10-18T11:37:36.512835Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:36.516748Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2211760.66
[2m2026-10-18T11:37:36.520669Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:36.520682Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x9c40
[2m2026-10-18T11:37:36.520686Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 20000
[2m2026-10-18T11:37:36.520689Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20000
[2m2026-10-18T11:37:36.520693Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2361240.80
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `usbEhci` | `0x00000001` | `isp1761` |
| `usbHub` | `0x00000001` | `usbhub` |
| `lcd` | `0x10120000` | `pl110` |
| `controller` | `0x20000000` | `emulatorcontroller` |
| `usbMouse` | `0x00000002` | `usbmouse` |
| `usbKeyboard` | `0x00000001` | `usbkeyboard` |
| `uart0` | `0x101f1000` | `pl011` |
| `uart1` | `0x101f2000` | `pl011` |
| `sysctl` | `0x10000000` | `armsysctl` |
| `timer1` | `0x101e2000` | `arm_sp804_timer` |
| `timer2` | `0x101e3000` | `arm_sp804_timer` |
| `smc91x` | `0x10010000` | `smc91x` |
| `kmi0` | `0x10006000` | `pl050` |
| `kmi1` | `0x10007000` | `pl050` |

**Total Peripherals:** 14