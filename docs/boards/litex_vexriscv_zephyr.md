# Validation Report: litex_vexriscv_zephyr

**Architecture:** RISC-V RV32IMAC.

## 1. Dynamic Simulation Validation
**Status:** ✅ Structural Check Passed (structural-ok)

```json
{
  "system_manifest": true,
  "chip_descriptor": true,
  "flash_base_present": true,
  "ram_base_present": true
}
```


## Hardware Coverage Report

| Peripheral ID | Base Address | Type |
|---|---|---|
| `uart` | `0x60001800` | `litex_uart` |
| `spi` | `0x60002000` | `stub` |
| `timer0` | `0x60002800` | `litex_timer` |
| `gpio_out` | `0x60005800` | `litex_gpio` |
| `gpio_in` | `0x60006000` | `litex_gpio` |
| `eth` | `0x60009800` | `litex_ethernet` |

**Total Peripherals:** 6