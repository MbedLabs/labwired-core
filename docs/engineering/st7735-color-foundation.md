# ST7735 colour foundation — integration still pending

`crates/core/src/peripherals/components/st7735_color.rs` is a pure byte codec,
not a connected display, shipped descriptor, guest pixel proof or performance
result. It does not change the existing ST7789 model or PyBadge configuration.
The full acceptance boundary remains [P3](target-next-lanes.md) and the
[PyBadge panel contract](../boards/pybadge-native.md).

The implementation uses the [ST7735R v0.2 datasheet](https://cdn-shop.adafruit.com/datasheets/ST7735R_V0.2.pdf),
particularly sections 9.8.20–22, 9.15, 9.18 and 10.1.22. It decodes serial
RGB444/RGB565/RGB666 into six-bit memory channels. RGBSET has 128 entries;
undefined lookup values produce an explicit unknown pixel, not invented black.
Complete pixels remain distinguishable from incomplete bytes. Software reset
preserves depth/table; hardware reset restores depth and invalidates the table.
Reserved COLMOD rejection is an emulator API policy, not silicon qualification.

Unit controls cover all 4,096 RGB444 and 65,536 RGB565 inputs with a non-linear,
channel-distinct table, RGB666 low-bit exclusion, delivery chunk splits, partial
pixel discard, table invalidation/replacement and reset/depth changes. These
controls are not evidence of a working native screen. Hosted checks must pass
on the exact source before adopting the codec.

## Next implementation slice

1. Add an opt-in serial-colour extension to the generic display descriptor and
   command parser, preserving existing fixed-format panels. Support the full
   128-byte RGBSET payload; invalidate the table at upload start and expose
   incomplete upload/unknown pixels honestly. Exact partial-table silicon
   contents are outside the codec's current boundary.
2. Feed only RAMWR data into the codec. Every completed pixel, including an
   unknown one, consumes one address step; incomplete pixels consume none.
   Preserve state across whole-byte CS pauses (datasheet section 9.6), discard
   partial pixels at command/reset boundaries, and qualify these decisions.
3. Store decoded memory values independently of later LUT updates; preserve
   frame RAM across both resets. Implement MADCTL/BGR and orientation-aware
   address/window reset separately from colour decoding. Determine module
   GM/crop before claiming physical four-corner orientation parity.
4. Add generic reset/backlight GPIO observation and SAM SPI attachment without
   controller-specific bus downcasts or new idle polling. Prove mux/clock/CS/DC
   negatives and reset/backlight effects using an owned permissive MMIO guest
   with pixel artifacts on hosted CI. Retain the existing scheduler controls.
5. Only then complete P3 qualification, followed by the separate IRQ/DMA and
   production-runtime lanes. No app pin, hardware acknowledgement or benchmark
   baseline should move on codec unit tests alone.

The pinned [CODAL stream](https://github.com/lancaster-university/codal-core/blob/312ae57e0b31f5b9df07a81e9d846945828e3c5a/source/drivers/ST7735.cpp)
is the later integration input. Independent datasheet vectors and actual driver
bytes must agree before a production-stream claim; no such guest run exists for
this foundation yet.
