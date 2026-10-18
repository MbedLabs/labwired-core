# Validation Report: litex_nexys_video_vexriscv_linux

**Architecture:** RISC-V RV32IMA.

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
| `ctrl` | `0xf0000000` | `litex_soc_controller_csr32` |
| `uart` | `0xf0001000` | `litex_uart` |
| `timer0` | `0xf0001800` | `litex_timer_csr32` |
| `mmc_controller` | `0xf0005000` | `litesdcard_csr32` |

**Total Peripherals:** 4