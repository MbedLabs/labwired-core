# Validation Report: renesas-r7fa2l1a

**Architecture:** ARM Cortex-M23.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.462417Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.462556Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-r7fa2l1a.yaml
[2m2026-10-18T11:37:31.463202Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463227Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463233Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463264Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463271Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463277Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463282Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463289Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463295Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463300Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463306Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra2_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.463428Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.463490Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.463499Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.463501Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.464082Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.464088Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.464092Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.464151Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:31.464165Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.464168Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:31.464170Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:31.464172Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:31.464175Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 2981.66
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `port6` | `0x400400c0` | `renesasra2_gpio` |
| `port7` | `0x400400e0` | `renesasra2_gpio` |
| `port8` | `0x40040100` | `renesasra2_gpio` |
| `sci3` | `0x40070060` | `renesasra6m5_sci` |
| `iic1` | `0x40053100` | `stub` |
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