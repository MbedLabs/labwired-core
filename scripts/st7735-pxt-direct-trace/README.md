# Pinned PXT direct RGB444 fragment capture — execution pending

This is a proposed hosted-only source-component control, not a working PyBadge
panel or full PXT runtime. No compiler/guest passing result is claimed yet.

`run.py` downloads and SHA256-checks `screen.cpp` and its Microsoft MIT license
at pxt-common-packages commit `31abf23d118f35010fb75122e60a1eb2b6dffe9f`.
It extracts the original `sendIndexedImage444` method and board-ID predicate
as exact byte fragments, recording their separate hashes. It compiles those
fragments in an authored host class/transport boundary, without source fixes or
substitutions. The original full class, constructor, palette setter, Image type,
`updateScreen` dispatch and runtime are **not** compiled or executed.

The artificial boundary supplies literal palette nibbles, source buffers and
synchronous SPI/pins. Predicate cases cover all six legacy IDs, three VID
boundary/example IDs, four unknown/absent IDs and missing SPI. Those controls
test the actual extracted predicate, not generated compiler macros, real CF2
loading or original branch dispatch.

The proposed four capture cases check command `0x2C`, every data byte, CS/D/C
levels, CS release and exact transfer sizes. A named host-capture mutant flips
the first data byte and must fail with `RAMWR mismatch`.

- Small 4×2: eight distinct literal palette colours, one 12-byte transfer.
- Odd 3×2: the original loop emits eight pixel slots for six declared image pixels. Both
  padded nibbles remain in the 12-byte capture; they are not stripped or fixed.
- Width 160, height 5: expected transfers 480, 480, 240 bytes. The original
  lookahead `3*(W+1)/2` flushes after two rows despite the three-row buffer.
- Width 159, height 4: expected transfers 720, 240 bytes, with each row's padded
  pixel retained. This does not establish correct production Image layout.

These are independently tabulated source-review expectations pending hosted
execution. They establish no panel RGB444 transfer function, valid pixels,
module GM/aperture, actual PXT Image storage, ARM/SAM/DMA/IRQ/fiber behavior,
physical timing, performance or consumer adoption. Do not seed an identity LUT
to make the existing ST7735R prototype render these bytes. Follow the
[panel contract](../../docs/boards/pybadge-native.md) for those separate gaps.

The workflow is intended to produce only the capture JSON and exact downloaded
Microsoft license. It does not upload binaries or restricted firmware. Wait for
the current orientation qualification before opening the next source PR; require
the actual capture/mutant and all enabled exact-head checks before landing.

## Referenced source and notice

- [Original screen.cpp](https://github.com/microsoft/pxt-common-packages/blob/31abf23d118f35010fb75122e60a1eb2b6dffe9f/libs/screen---st7735/screen.cpp)
- [Original license](https://github.com/microsoft/pxt-common-packages/blob/31abf23d118f35010fb75122e60a1eb2b6dffe9f/LICENSE)

MIT License

Copyright (c) Microsoft Corporation. All rights reserved.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
