# Validation Report: ramn

**Architecture:** ARM Cortex-M.

## 1. Dynamic Simulation Validation
**Status:** ❌ Failed ([2m2026-10-18T00:53:30.591492Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'stm32_fdcan' f)

```text
[2m2026-10-18T00:53:30.589752Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:30.589941Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/ramn.yaml
[2m2026-10-18T00:53:30.591492Z[0m [31mERROR[0m [2mlabwired_cli[0m[2m:[0m unknown peripheral type 'stm32_fdcan' for peripheral 'fdcan1' in chip 'ramn' (chip file '/home/andrii/Projects/labwired/core/configs/systems/onboarding/../../chips/onboarding/ramn.yaml'): this engine has no model for it, no plugin claimed it, and it is not on the known-stub allowlist. Refusing to answer it with a zero-filled stub — firmware would read zeros from silicon that was never modelled and the run would still report success. Fix it one of three ways: model the peripheral, describe it with `type: declarative` and a descriptor `path`, or — if answering zeros really is right — declare `type: stub` in the chip YAML (or add 'stm32_fdcan' to KNOWN_STUBBED_PERIPHERAL_TYPES in crates/core/src/bus/known_stubs.rs with a written reason).
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `pwr` | `0x40007000` | `pythonperipheral` |
| `otp` | `0x0bfa0000` | `pythonperipheral` |
| `spi1` | `0x40013000` | `stm32spi` |
| `spi2` | `0x40003800` | `stm32spi` |
| `spi3` | `0x40003c00` | `stm32spi` |
| `usart1` | `0x40013800` | `stm32f7_usart` |
| `usart2` | `0x40004400` | `stm32f7_usart` |
| `usart3` | `0x40004800` | `stm32f7_usart` |
| `uart4` | `0x40004c00` | `stm32f7_usart` |
| `uart5` | `0x40005000` | `stm32f7_usart` |
| `lpuart1` | `0x40008000` | `stm32f7_usart` |
| `gpioPortA` | `0x42020000` | `stm32_gpioport` |
| `gpioPortB` | `0x42020400` | `stm32_gpioport` |
| `gpioPortC` | `0x42020800` | `stm32_gpioport` |
| `gpioPortD` | `0x42020c00` | `stm32_gpioport` |
| `gpioPortE` | `0x42021000` | `stm32_gpioport` |
| `gpioPortF` | `0x42021400` | `stm32_gpioport` |
| `gpioPortG` | `0x42021800` | `stm32_gpioport` |
| `gpioPortH` | `0x42021c00` | `stm32_gpioport` |
| `dma1` | `0x40020000` | `stm32ldma` |
| `dma2` | `0x40020400` | `stm32ldma` |
| `dmamux` | `0x40020800` | `stm32_dmamux` |
| `i2c1` | `0x40005400` | `stm32f1_i2c` |
| `fdcanRAM` | `0x4000ac00` | `arraymemory` |
| `fdcan1` | `0x4000a400` | `stm32_fdcan` |
| `timer1` | `0x40012c00` | `stm32_timer` |
| `timer2` | `0x40000000` | `stm32_timer` |
| `timer3` | `0x40000400` | `stm32_timer` |
| `timer4` | `0x40000800` | `stm32_timer` |
| `timer5` | `0x40000c00` | `stm32_timer` |
| `timer6` | `0x40001000` | `stm32_timer` |
| `timer7` | `0x40001400` | `stm32_timer` |
| `timer8` | `0x40013400` | `stm32_timer` |
| `timer15` | `0x40014000` | `stm32_timer` |
| `timer16` | `0x40014400` | `stm32_timer` |
| `timer17` | `0x40014800` | `stm32_timer` |
| `rcc` | `0x40021000` | `stm32l5_rcc` |
| `flashController` | `0x40022000` | `stm32wba_flashcontroller` |
| `rng` | `0x420c0800` | `stm32_rng` |
| `adc1` | `0x42028000` | `stm32l5_adc` |
| `iwdg` | `0x40003000` | `stm32_independentwatchdog` |
| `usb` | `0x4000d400` | `stm32_usb` |
| `usb_sram` | `0x4000d800` | `arraymemory` |

**Total Peripherals:** 43