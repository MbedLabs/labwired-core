# ST7735 GM00 address orientations — qualification pending

This test-only P3 follow-up extends the
[captured-command SAM controls](st7735-codal-sam-binding.md). It changes no
production model, panel descriptor, consumer pin, capture or performance floor.
The seven-test target has not yet been executed at this source; no passing guest
result or production-panel qualification is claimed.

## Independent address vectors

[ST7735R v0.2](https://cdn-shop.adafruit.com/datasheets/ST7735R_V0.2.pdf)
section 9.11.2 describes GM00's eight address-pointer orientations. The authored
guest targets a fixed physical-memory crop: columns 11–12, rows 7–10. Its
RGBSET and eight packed pixels are unchanged from the width-four original-driver
capture. MADCTL and address-window commands are independently authored, not a
capture of the original driver's rotated execution. Expectations call neither
the model's mapping helper nor its colour decoder.

| MADCTL | CASET | RASET | Physical row-major pixel indices |
| --- | --- | --- | --- |
| `00` | 11–12 | 7–10 | 1, 2, 3, 4, 5, 6, 7, 8 |
| `40` | 119–120 | 7–10 | 2, 1, 4, 3, 6, 5, 8, 7 |
| `80` | 11–12 | 151–154 | 7, 8, 5, 6, 3, 4, 1, 2 |
| `C0` | 119–120 | 151–154 | 8, 7, 6, 5, 4, 3, 2, 1 |
| `20` | 7–10 | 11–12 | 1, 5, 2, 6, 3, 7, 4, 8 |
| `60` | 7–10 | 119–120 | 5, 1, 6, 2, 7, 3, 8, 4 |
| `A0` | 151–154 | 11–12 | 4, 8, 3, 7, 2, 6, 1, 5 |
| `E0` | 151–154 | 119–120 | 8, 4, 7, 3, 6, 2, 5, 1 |

Literal RGB888 colours distinguish all eight indices, including the non-extreme
colour `[16, 52, 85]`. All eight orientations run in ordinary and forced-legacy
SPI modes. A named negative clears MV in the `20` case without changing its
windows: guest TXC polling must still complete, but the fixed crop must contain
zero known and eight unknown pixels, with frame bytes withheld. The host only
attaches the fixture crop; it performs no guest MMIO setup or pixel injection.

## Acceptance and exclusions

Execute all seven tests in `crates/core/tests/st7735_sam_guest.rs` in both
feature configurations, retain the live original-driver/fixture comparison, and
require every enabled exact-head check to pass before landing. Record the tested
source, guest hash and tested/landed tree identities. Static source review or
formatting is not guest execution.

These vectors cover valid GM00 controller-memory addresses only. They establish
no deployed aperture/GM, BGR/glass parity, direct Adafruit RGB444 renderer,
original CODAL ARM execution, IRQ/DMA, active-frame RTx or new-panel WASM result.
Range-ignore semantics, odd-height production packing, retained RAM/LUT controls
and effective deployed build/CF2 configuration remain separate P3 requirements.
P3 and CP14 remain open.
