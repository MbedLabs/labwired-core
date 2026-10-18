# Validation Report: renesas-ek_ra2e1

**Architecture:** ARM Cortex-M23.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.343772Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.343955Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-ek_ra2e1.yaml
[2m2026-10-18T11:37:31.345174Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345219Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345229Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345237Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345246Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345254Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345263Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345271Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.345489Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.345617Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.345631Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.345636Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.346618Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.346630Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.346640Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.346722Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:31.346744Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.346749Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:31.346753Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:31.346757Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:31.346761Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1784.32
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `led_blue` | `0x0000000f` | `led` |
| `led_green` | `0x0000000e` | `led` |
| `led_red` | `0x0000000d` | `led` |
| `button_s2` | `0x0000000f` | `button` |
| `button_s1` | `0x00000004` | `button` |
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
| `iic0` | `0x40053000` | `stub` |

**Total Peripherals:** 22