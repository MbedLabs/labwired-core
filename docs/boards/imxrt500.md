# Validation Report: imxrt500

**Architecture:** ARMv8-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:12.309418Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m UART type 'lpc_usart' (peripheral 'flex)

```text
[2m2026-10-18T00:53:12.305091Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:12.305248Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/imxrt500.yaml
[2m2026-10-18T00:53:12.309418Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m UART type 'lpc_usart' (peripheral 'flexcomm0') has no register layout modelled yet and no `config.profile` set; it will NOT be silently mapped onto an STM32. Choose a layout explicitly with `config: { profile: <one of the supported layouts> }`, or add a dedicated model for it.
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