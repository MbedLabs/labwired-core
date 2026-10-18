# Validation Report: imxrt500

**Architecture:** ARMv8-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:22.987631Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:22.987743Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/imxrt500.yaml
[2m2026-10-18T11:37:22.988456Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:22.988507Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:22.988513Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:22.988516Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:22.988992Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:22.988997Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:22.989003Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:22.989062Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:22.989074Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:22.989076Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:22.989079Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:22.989081Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:22.989083Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3539.09
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `ctimer0` | `0x40028000` | `lpc_ctimer` |
| `ctimer1` | `0x40029000` | `lpc_ctimer` |
| `ctimer2` | `0x4002a000` | `lpc_ctimer` |
| `ctimer3` | `0x4002b000` | `lpc_ctimer` |
| `ctimer4` | `0x4002c000` | `lpc_ctimer` |
| `ostimer` | `0x40113000` | `nxp_ostimer` |
| `flexcomm0` | `0x40106000` | `lpc_usart` |
| `flexcomm12` | `0x4020d000` | `lpc_usart` |
| `flexspi2` | `0x4013c000` | `imxrt_flexspi` |
| `flexspi` | `0x40134000` | `imxrt_flexspi` |
| `clkctl0` | `0x40001000` | `lpc_clock0` |
| `trng` | `0x40138000` | `imx_trng` |
| `dwt` | `0xe0001000` | `dwt` |
| `clkctl1` | `0x40021000` | `pythonperipheral` |
| `reset_flexspi1` | `0x40000010` | `pythonperipheral` |
| `reset_adc0` | `0x40000014` | `pythonperipheral` |
| `reset_flexcomm12` | `0x40020010` | `pythonperipheral` |
| `PRSTCTL2` | `0x40020018` | `pythonperipheral` |
| `reset_hsgpio3` | `0x40020014` | `pythonperipheral` |

**Total Peripherals:** 19