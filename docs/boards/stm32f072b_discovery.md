# Validation Report: stm32f072b_discovery

**Architecture:** ARMv6-M.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:32.922195Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:32.922325Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/stm32f072b_discovery.yaml
[2m2026-10-18T11:37:32.923295Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:32.923359Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:32.923368Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:32.923374Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:32.924424Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:32.924439Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20020000
[2m2026-10-18T11:37:32.924449Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:32.924519Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 2: Memory access violation at 0x2001ffe0
[2m2026-10-18T11:37:32.924537Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:32.924541Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x45c
[2m2026-10-18T11:37:32.924544Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 2
[2m2026-10-18T11:37:32.924548Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 2
[2m2026-10-18T11:37:32.924551Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 1703.83
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `led` | `0x00000006` | `led` |
| `usart1` | `0x40013800` | `stm32f7_usart` |
| `usart2` | `0x40004400` | `stm32f7_usart` |
| `usart3` | `0x40004800` | `stm32f7_usart` |
| `usart4` | `0x40004c00` | `stm32f7_usart` |
| `usart5` | `0x40005000` | `stm32f7_usart` |
| `usart6` | `0x40011400` | `stm32f7_usart` |
| `usart7` | `0x40011800` | `stm32f7_usart` |
| `gpioPortA` | `0x48000000` | `stm32_gpioport` |
| `gpioPortB` | `0x48000400` | `stm32_gpioport` |
| `gpioPortC` | `0x48000800` | `stm32_gpioport` |
| `gpioPortD` | `0x48000c00` | `stm32_gpioport` |
| `gpioPortE` | `0x48001000` | `stm32_gpioport` |
| `gpioPortF` | `0x48001400` | `stm32_gpioport` |
| `i2c1` | `0x40005400` | `stm32f7_i2c` |
| `i2c2` | `0x40005800` | `stm32f7_i2c` |
| `spi1` | `0x40013000` | `stm32spi` |
| `spi2` | `0x40003800` | `stm32spi` |
| `timer1` | `0x40012c00` | `stm32_timer` |
| `timer2` | `0x40000000` | `stm32_timer` |
| `timer3` | `0x40000400` | `stm32_timer` |
| `timer6` | `0x40001000` | `stm32_timer` |
| `timer7` | `0x40001400` | `stm32_timer` |
| `timer14` | `0x40002000` | `stm32_timer` |
| `timer15` | `0x40014000` | `stm32_timer` |
| `timer16` | `0x40014400` | `stm32_timer` |
| `timer17` | `0x40014800` | `stm32_timer` |
| `can` | `0x40006400` | `stmcan` |
| `rtc` | `0x40002800` | `stm32f4_rtc` |
| `rcc` | `0x40021000` | `stm32f0_rcc` |
| `dma` | `0x40020000` | `stm32g0dma` |
| `adc` | `0x40012400` | `stm32f0_adc` |
| `crc` | `0x40023000` | `stm32_crc` |

**Total Peripherals:** 33