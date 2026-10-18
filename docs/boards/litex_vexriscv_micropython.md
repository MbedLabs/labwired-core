# Validation Report: litex_vexriscv_micropython

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
| `clock0` | `0xe0004800` | `litex_mmcm_csr32` |
| `timer` | `0x60002800` | `litex_timer_csr32` |
| `uart` | `0x60001800` | `litex_uart` |
| `eth` | `0x60007800` | `litex_ethernet` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `spi` | `0x60005000` | `litex_spi_flash` |

**Total Peripherals:** 6