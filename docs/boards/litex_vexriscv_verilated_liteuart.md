# Validation Report: litex_vexriscv_verilated_liteuart

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
| `timer0` | `0x60002800` | `litex_timer` |
| `uart` | `0xe0001000` | `cosimulateduart` |

**Total Peripherals:** 2