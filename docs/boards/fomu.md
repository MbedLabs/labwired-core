# Validation Report: fomu

**Architecture:** RISC-V RV32I.

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
| `valenty` | `0x60004800` | `valentyusb` |
| `spi` | `0x60005000` | `picorv_spi` |

**Total Peripherals:** 3