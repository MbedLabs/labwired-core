# Validation Report: renesas-ck_ra6m5

**Architecture:** ARM Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.259500Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.259674Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-ck_ra6m5.yaml
[2m2026-10-18T11:37:31.263232Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263278Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263289Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263298Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263307Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263318Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263328Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263340Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263351Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263363Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263383Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263394Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'portA') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263406Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra6_gpio' (peripheral 'portB') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.263586Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.263678Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.263732Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.263770Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.265508Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.265526Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.265538Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.268216Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Progress: 10000 steps, current IPS: 2263762.77
[2m2026-10-18T11:37:31.270359Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.270367Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x466
[2m2026-10-18T11:37:31.270374Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 19999
[2m2026-10-18T11:37:31.270378Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 20002
[2m2026-10-18T11:37:31.270382Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 3037284.07
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `led_red` | `0x0000000a` | `led` |
| `led_green` | `0x00000009` | `led` |
| `led_blue` | `0x00000001` | `led` |
| `button` | `0x00000004` | `button` |
| `hs3001` | `0x00000044` | `hs3001` |
| `iaq` | `0x00000032` | `zmod4xxx` |
| `oaq` | `0x00000033` | `zmod4xxx` |
| `barometer` | `0x00000063` | `icp_101xx` |
| `icm` | `0x00000068` | `icm20948` |
| `magnetometer` | `0x0000000c` | `ak09916` |
| `sysc_oscsf` | `0x4001e03c` | `pythonperipheral` |
| `gpt` | `0x40169000` | `renesasra_gpt` |
| `agt0` | `0x400e8000` | `renesasra_agt` |
| `agt1` | `0x400e8100` | `renesasra_agt` |
| `agt2` | `0x400e8200` | `renesasra_agt` |
| `agt3` | `0x400e8300` | `renesasra_agt` |
| `agt4` | `0x400e8400` | `renesasra_agt` |
| `agt5` | `0x400e8500` | `renesasra_agt` |
| `sci0` | `0x40118000` | `renesasra6m5_sci` |
| `sci1` | `0x40118100` | `renesasra6m5_sci` |
| `sci2` | `0x40118200` | `renesasra6m5_sci` |
| `sci3` | `0x40118300` | `renesasra6m5_sci` |
| `sci4` | `0x40118400` | `renesasra6m5_sci` |
| `sci5` | `0x40118500` | `renesasra6m5_sci` |
| `sci6` | `0x40118600` | `renesasra6m5_sci` |
| `sci7` | `0x40118700` | `renesasra6m5_sci` |
| `sci8` | `0x40118800` | `renesasra6m5_sci` |
| `sci9` | `0x40118900` | `renesasra6m5_sci` |
| `portMisc` | `0x40080d00` | `renesasra_gpiomisc` |
| `port0` | `0x40080000` | `renesasra6_gpio` |
| `port1` | `0x40080020` | `renesasra6_gpio` |
| `port2` | `0x40080040` | `renesasra6_gpio` |
| `port3` | `0x40080060` | `renesasra6_gpio` |
| `port4` | `0x40080080` | `renesasra6_gpio` |
| `port5` | `0x400800a0` | `renesasra6_gpio` |
| `port6` | `0x400800c0` | `renesasra6_gpio` |
| `port7` | `0x400800e0` | `renesasra6_gpio` |
| `port8` | `0x40080100` | `renesasra6_gpio` |
| `port9` | `0x40080120` | `renesasra6_gpio` |
| `portA` | `0x40080140` | `renesasra6_gpio` |
| `portB` | `0x40080160` | `renesasra6_gpio` |
| `iic0` | `0x4009f000` | `stub` |
| `iic1` | `0x4009f100` | `stub` |
| `iic2` | `0x4009f200` | `stub` |

**Total Peripherals:** 44