# Validation Report: ice40up5k-mdp-evn

**Architecture:** Soft RISC-V / FPGA.

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
| `i2c` | `0x60007800` | `litex_i2c` |
| `bmp180` | `0x00000077` | `bmp180` |
| `lsm330_a` | `0x00000018` | `lsm330_accelerometer` |
| `lsm330_g` | `0x0000006b` | `lsm330_gyroscope` |
| `lis2ds12` | `0x0000001d` | `lis2ds12` |
| `lsm303dlhc_a` | `0x00000019` | `lsm303dlhc_accelerometer` |
| `lsm303dlhc_g` | `0x0000001e` | `lsm303dlhc_gyroscope` |
| `timer0` | `0x60002800` | `litex_timer` |
| `valenty` | `0x60004800` | `valentyusb` |
| `spi` | `0x60005000` | `picorv_spi` |

**Total Peripherals:** 10