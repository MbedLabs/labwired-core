# Validation Report: litex_picorv32

**Architecture:** RISC-V.

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
| `timer0` | `0xe0002000` | `litex_timer` |
| `uart` | `0x60001800` | `litex_uart` |
| `eth` | `0x60007800` | `litex_ethernet` |
| `phy` | `0x00000000` | `ethernetphysicallayer` |
| `spi` | `0x60005000` | `stub` |

**Total Peripherals:** 5