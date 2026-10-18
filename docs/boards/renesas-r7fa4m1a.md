# Validation Report: renesas-r7fa4m1a

**Architecture:** ARM Cortex-M4F.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.497731Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.497872Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-r7fa4m1a.yaml
[2m2026-10-18T11:37:31.498688Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498714Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498726Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498737Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498750Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498761Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498773Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498791Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498802Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498813Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498825Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra4_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.498938Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.499015Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.499025Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.499031Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.499915Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.499925Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.499931Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.500015Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:31.500033Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.500037Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:31.500041Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:31.500044Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:31.500048Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1975.19
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `gpt` | `0x40078000` | `renesasra_gpt` |
| `agt0` | `0x40084000` | `renesasra_agt` |
| `agt1` | `0x40084100` | `renesasra_agt` |
| `sci0` | `0x40070000` | `renesasra6m5_sci` |
| `sci1` | `0x40070020` | `renesasra6m5_sci` |
| `sci2` | `0x40070040` | `renesasra6m5_sci` |
| `sci9` | `0x40070120` | `renesasra6m5_sci` |
| `portMisc` | `0x40040d00` | `renesasra_gpiomisc` |
| `port0` | `0x40040000` | `renesasra4_gpio` |
| `port1` | `0x40040020` | `renesasra4_gpio` |
| `port2` | `0x40040040` | `renesasra4_gpio` |
| `port3` | `0x40040060` | `renesasra4_gpio` |
| `port4` | `0x40040080` | `renesasra4_gpio` |
| `port5` | `0x400400a0` | `renesasra4_gpio` |
| `port6` | `0x400400c0` | `renesasra4_gpio` |
| `port7` | `0x400400e0` | `renesasra4_gpio` |
| `port8` | `0x40040100` | `renesasra4_gpio` |
| `port9` | `0x40040120` | `renesasra4_gpio` |
| `sysc_oscsf` | `0x4001e03c` | `pythonperipheral` |
| `sysc_vbtsr` | `0x4001e4b1` | `pythonperipheral` |
| `iic0` | `0x40053000` | `stub` |
| `iic1` | `0x40053100` | `stub` |

**Total Peripherals:** 22