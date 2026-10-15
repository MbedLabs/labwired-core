# SPI and I²C between two chips, over gpio nets

Pairs of chips in one world whose hardware SPI or I²C are wired pin to pin
with `gpio_net` interconnects (see
[GPIO nets](../../docs/howto/gpio-nets.md#buses-over-nets)). Nothing is
exchanged byte by byte behind the scenes: every bit crosses the nets at the
pads, so the net reports and a logic analyzer on either board see the real
waveform, and the bus behaves like wires: ACKs are SDA held low by the other
chip, a target that is slow to answer holds SCL low, a missing target is the
pull-up reading 1.

| Environment | Node `a` (or `rp`) | Node `b` (or `rp`) | Nets |
|-------------|----------|----------|------|
| `env-spi.yaml` | STM32G071 SPI1 master, mode 0, SCK = f/128, CS on a GPIO (PA4) | G071 SPI1 slave, hardware NSS (PA4), RXNE interrupt | `cs` (pull-up), `sck`, `miso`, `mosi` (pull-down) |
| `env-i2c.yaml` | G071 I2C1 controller (PB6/PB7, open drain) | G071 I2C1 target at 0x42, interrupt driven | `scl`, `sda` (pull-up) |
| `env-spi16.yaml` | STM32F401 SPI1 master, 16-bit frames | F401 SPI1 slave, software select | `sck`, `miso`, `mosi` |
| `env-spi16-nss.yaml` | F401 SPI1 master, 16-bit frames, NSS driven by the SPI (`SSOE=1`, PA4 AF5) | F401 SPI1 slave, hardware NSS (`SSM=0`, PA4 AF5) | `nss` (pull-up), `sck`, `miso`, `mosi` |
| `env-i2c-legacy.yaml` | STM32F103 legacy I2C1 master (PB6/PB7) | F401 legacy I2C1 slave at 0x42 | `scl`, `sda` (pull-up) |
| `env-spi-rp2040.yaml` | RP2040 SPI0 master (GP2-GP5, hardware CSn) | G071 SPI1 slave | `cs`, `sck`, `miso`, `mosi` |
| `env-spi-rp2040-slave.yaml` | G071 SPI1 master, mode 3 | RP2040 SPI0 slave | `cs`, `sck`, `miso`, `mosi` |
| `env-i2c-rp2040.yaml` | RP2040 I2C0 controller (GP4 SDA, GP5 SCL) | G071 I2C1 target at 0x42 | `scl`, `sda` (pull-up) |
| `env-i2c-rp2040-target.yaml` | G071 I2C1 controller | RP2040 I2C0 target at 0x42 | `scl`, `sda` (pull-up) |
| `env-spi-nrf52.yaml` | nRF52840 SPIM0 master (P0.03 SCK, P0.04 MOSI, P0.05 MISO, CS on GPIO P0.02), EasyDMA, END interrupt | G071 SPI1 slave | `cs`, `sck`, `miso`, `mosi` |
| `env-spi-nrf52-slave.yaml` | G071 SPI1 master, mode 0 | nRF52840 SPIS0 slave (CSN P0.02), semaphore + EasyDMA | `cs`, `sck`, `miso`, `mosi` |
| `env-i2c-nrf52.yaml` | nRF52840 TWIM0 controller (P0.27 SCL, P0.26 SDA), K250, shortcuts | G071 I2C1 target at 0x42 | `scl`, `sda` (pull-up) |
| `env-i2c-nrf52-target.yaml` | G071 I2C1 controller | nRF52840 TWIS0 target at 0x42 (polled `WRITE`/`READ`, `PREPARERX`/`TX`) | `scl`, `sda` (pull-up) |
| `env-spi-esp32c3.yaml` | ESP32-C3 GP-SPI2 master (GPIO4 SCK, GPIO5 MOSI, GPIO3 MISO, GPIO1 CS0 through the GPIO matrix), mode 0, 100 kHz | G071 SPI1 slave | `cs`, `sck`, `miso`, `mosi` |
| `env-spi-esp32c3-slave.yaml` | ESP32-C3 GP-SPI2 slave (W buffer) | G071 SPI1 master | `cs`, `sck`, `miso`, `mosi` |
| `env-i2c-esp32c3.yaml` | ESP32-C3 I2C0 controller (GPIO8 SDA, GPIO9 SCL), 100 kHz | G071 I2C1 target at 0x42 | `scl`, `sda` (pull-up) |
| `env-i2c-esp32c3-target.yaml` | ESP32-C3 I2C0 target at 0x42, stretching SCL until its firmware answers | G071 I2C1 controller | `scl`, `sda` (pull-up) |

SPI: the master sends `A5 3C 5A C3` and receives `81 82 83 84`, which the
slave queues from its SPI interrupt (or preloads in its FIFO, on the RP2040).
The tests also swap in the frame-shape variants (`spi-*-mode1/2/3`,
`spi-*-lsb`), the G071's other `CR2.DS` frame sizes (`spi-*-ds16/12/4`, each
chip as master in turn), a slave that feeds every answer from its TXE
interrupt (`spi-slave-txe`), one that underruns (`spi-slave-underrun`) and a
master that leaves `CR2.FRXTH` at reset (`spi-master-nofrxth`), which the
G0's FIFO SPI stalls after one 8-bit frame as silicon does.

I²C: the controller writes `10 DE AD` to 0x42 (register pointer 0x10, then two
bytes), writes the pointer again and reads two bytes back through a repeated
START (`DE AD`), then writes to 0x50, where nobody answers: NACKF and STOP.
The legacy pair adds a single-byte read. `i2c-bulk-*` (on the `env-i2c.yaml`
wiring) moves 300 bytes each way over the 10-bit address 0x2A5 with
`RELOAD`, then a general call and a write to a masked `OAR2` address. The
tests also run the legacy pair the other way round (F401 master, F103 slave)
and a G071 controller against the F103 slave, and swap in targets built with
`-DSLOW=1` (`*-target-slow.elf`), whose firmware answers several SCL low
periods late, so the controller is held by clock stretching. The RP2040 runs
the same register-file exchange on its I2C0, as controller and as target. The ESP32-C3
runs both exchanges as well (`src/esp32c_*.c`, register level), and so does
the ESP32-C6 from the same sources (`-DCHIP_C6`, `firmware/esp32c6-*.elf`;
the tests swap the system and firmware into the `esp32c3` environments).
`esp32c3-esp-hal/` holds esp-hal 1.1 SPI and I2C master applications that run
the same exchanges through esp-hal's own drivers.

Results are left in SRAM at 0x20000100 (see the comment at the top of each
source). The tests are
`cargo test -p labwired-core --test world_gpio_net_buses`.

## Rebuild the firmware

```bash
examples/gpio-net-buses/build.sh   # clang + lld, no vendor SDK
```

The ELFs are committed so the tests run without a toolchain.
