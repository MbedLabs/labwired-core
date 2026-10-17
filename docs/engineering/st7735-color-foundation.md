# ST7735 colour pipeline — board integration still pending

`crates/core/src/peripherals/components/st7735_color.rs` is a pure byte codec.
The draft generic-display integration opts into it using `serial_color` and
`load_rgb_lut`; it is not a shipped PyBadge display, guest pixel proof or
performance result. Existing fixed-format panels do not opt in, and the
ST7789 descriptor and PyBadge configuration remain unchanged.
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

`.github/workflows/st7735-color-controls.yml` compiles this dependency-free
module directly with Rust 1.95.0 and requires all eleven controls to be listed
before execution. This focused hosted check supplements, not replaces, the
normal crate/workspace checks; it does not execute a guest or measure RTx.

## Draft parser and memory integration

`configs/devices/st7735r.yaml` is an **unregistered prototype** using GM00's
132×162 controller extent, not a claim about PyBadge module GM/crop. It must be
loaded explicitly. RGB888 artifact bytes contain bit-expanded RGB666 memory
channels (`(v << 2) | (v >> 4)`), not original wire bytes or analogue glass output.
BGR, gamma, scan order and inversion rendering remain unimplemented. MADCTL
address mapping reuses the generic engine; physical module parity is still owed.

The opt-in parser collects the complete 128-byte RGBSET payload, invalidates
the previous table at upload start, and leaves an interrupted upload undefined.
Whole-byte CS pauses preserve both parameter and pixel progress. A new command
discards an incomplete pixel; each completed pixel consumes one address step,
including a pixel whose lookup value is unknown. Later LUT changes do not
recolour stored RAM. Partial parameter-register updates on silicon are not
claimed by this all-or-nothing command parser.

RAM starts unknown. A validity map follows the same orientation/crop as frame
bytes; artifacts publish `known_pixels` and `unknown_pixels` and withhold bytes
if any visible pixel is unknown, even when bytes were requested. `painted_bytes`
is then null. Raw `framebuffer()` users must consult `known_pixels()` rather than
interpret storage placeholders as valid black. Generation includes validity,
so unknown-to-known black is observable. This policy avoids a fabricated frame,
but downstream RGB888 rendering/verification still needs integration.

Software reset retains COLMOD, MADCTL, LUT, RAM and pixel validity; hardware
reset restores COLMOD/MADCTL defaults and invalidates the LUT, retaining RAM
and validity. Reset cancels parser fragments and restores orientation-aware
full windows. The explicit hardware-reset method is **not GPIO-connected yet**.
Reserved COLMOD values are rejected without changing the selected format and
counted in artifact metadata. Clear-RAM/refresh and incompatible descriptor
formats are rejected for this profile.

Colour snapshots use a separate tagged shape containing RAM and validity.
Existing fixed-format v1 snapshots are unchanged. Cross-profile/wrong-size
restores are refused before mutation. As with existing display snapshots,
control/parser state is rebuilt by bus replay, not restored by this RAM snapshot.

Fifteen `st7735_stream_*` controls in
`crates/core/src/peripherals/components/declarative_display.rs` exercise this
prototype through the SPI door, plus descriptor/validity/snapshot checks.
They run in the normal hosted crate/workspace checks, not the standalone codec
job. Until exact-head checks pass these are proposed controls, not a passing
qualification receipt. No guest or performance result is implied.

## Next implementation slice

1. Review and qualify the opt-in parser/memory integration on hosted CI,
   including the existing fixed-format display controls. Keep partial-table
   silicon contents outside the declared boundary unless independently modelled.
2. Add RGB888 downstream artifact decoding/verification and independently check
   BGR semantics without confusing controller memory with glass colour. Match
   independent vectors to the pinned driver's packed stream.
3. Determine module
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
