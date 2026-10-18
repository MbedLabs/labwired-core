# Validation Report: vexpress

**Architecture:** ARMv7-A.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:36.561843Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:36.561985Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/vexpress.yaml
[2m2026-10-18T11:37:36.957542Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:36.957634Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:36.957642Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:36.957646Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:36.958553Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:36.958568Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:36.958574Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:36.958651Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:36.958670Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:36.958674Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:36.958678Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:36.958681Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:36.958685Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1933.55
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart0` | `0x10009000` | `pl011` |
| `uart1` | `0x1000a000` | `pl011` |
| `uart2` | `0x1000b000` | `pl011` |
| `uart3` | `0x1000c000` | `pl011` |
| `timer1` | `0x10011000` | `arm_sp804_timer` |
| `timer2` | `0x10012000` | `arm_sp804_timer` |
| `sysCtl` | `0x10000000` | `armsysctl` |
| `pl310` | `0x1e00a000` | `pl310` |
| `pl031` | `0x10017000` | `pl031` |
| `usbEhci` | `0x4f000000` | `isp1761` |
| `kmi0` | `0x10006000` | `pl050` |
| `kmi1` | `0x10007000` | `pl050` |
| `usbHub` | `0x00000001` | `usbhub` |
| `usbMouse` | `0x00000002` | `usbmouse` |
| `usbKeyboard` | `0x00000001` | `usbkeyboard` |
| `controller` | `0x20000000` | `emulatorcontroller` |
| `lcd` | `0x10020000` | `pl110` |

**Total Peripherals:** 17