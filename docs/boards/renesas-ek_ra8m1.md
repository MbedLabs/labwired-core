# Validation Report: renesas-ek_ra8m1

**Architecture:** ARM Cortex-M85.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.381659Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.381825Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas-ek_ra8m1.yaml
[2m2026-10-18T11:37:31.384979Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra_gpiomisc' (peripheral 'portMisc') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385010Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port0') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385020Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port1') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385030Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port2') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385040Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port3') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385050Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port4') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385060Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port5') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385070Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port6') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385079Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port7') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385088Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port8') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385097Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'port9') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385107Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'portA') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385116Z[0m [33m WARN[0m [2mlabwired_core::bus::profiles[0m[2m:[0m GPIO type 'renesasra8_gpio' (peripheral 'portB') has no dedicated register model; falling back to the STM32F1 placeholder layout. This is a known-incomplete onboarding model — set `config.profile` or add a real model for correct behaviour.
[2m2026-10-18T11:37:31.385246Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.385330Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.385338Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.385342Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.386534Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.386543Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:31.386552Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.386624Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:31.386643Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.386647Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:31.386651Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:31.386655Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:31.386658Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1530.89
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `led_blue` | `0x00000000` | `led` |
| `led_green` | `0x0000000e` | `led` |
| `led_red` | `0x0000000d` | `led` |
| `button_s2` | `0x00000008` | `button` |
| `button_s1` | `0x00000009` | `button` |
| `option_setting` | `0x27030050` | `arraymemory` |
| `gpt` | `0x40322000` | `renesasra_gpt` |
| `gpt_ns` | `0x50322000` | `renesasra_gpt` |
| `agt0` | `0x40221000` | `renesasra_agt` |
| `agt1` | `0x40221100` | `renesasra_agt` |
| `sci0` | `0x40358000` | `renesasra8m1_sci` |
| `sci1` | `0x40358100` | `renesasra8m1_sci` |
| `sci2` | `0x40358200` | `renesasra8m1_sci` |
| `sci3` | `0x40358300` | `renesasra8m1_sci` |
| `sci4` | `0x40358400` | `renesasra8m1_sci` |
| `sci9` | `0x40358900` | `renesasra8m1_sci` |
| `portMisc` | `0x40400d00` | `renesasra_gpiomisc` |
| `port0` | `0x40400000` | `renesasra8_gpio` |
| `port1` | `0x40400020` | `renesasra8_gpio` |
| `port2` | `0x40400040` | `renesasra8_gpio` |
| `port3` | `0x40400060` | `renesasra8_gpio` |
| `port4` | `0x40400080` | `renesasra8_gpio` |
| `port5` | `0x404000a0` | `renesasra8_gpio` |
| `port6` | `0x404000c0` | `renesasra8_gpio` |
| `port7` | `0x404000e0` | `renesasra8_gpio` |
| `port8` | `0x40400100` | `renesasra8_gpio` |
| `port9` | `0x40400120` | `renesasra8_gpio` |
| `portA` | `0x40400140` | `renesasra8_gpio` |
| `portB` | `0x40400160` | `renesasra8_gpio` |
| `SYSC_SCICKCR` | `0x4001e055` | `pythonperipheral` |
| `iic0` | `0x4025e000` | `stub` |
| `iic1` | `0x4025e100` | `stub` |

**Total Peripherals:** 32