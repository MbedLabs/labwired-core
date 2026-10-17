#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
"""Exact caller/accessor fragments in authored host object boundaries."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import urllib.request

import run as direct

HEADER = "libs/base/pxtbase.h"
HEADER_SHA = "87c459c4d8fefa5bd851f862be8e83659185825c41448155caed34a22fe0bd17"
FRAGMENTS = {
    "caller.inc": "9443b0d8e64b57daf36068a0ca8a50a2c58094452573c8dabe7522a3e7a40236",
    "image-header.inc": "23056e13fc8341d15f3754024000f35f8928b2dd5ea7aa916f1fd47072cdac4b",
    "image-accessors.inc": "8198b2664d5627e90b54d477f11751c7788b06e1d9abfe0d345bf02da5f26436",
    "method.inc": direct.FRAGMENTS["sendIndexedImage444"],
}


def extract(source, header):
    fragments = {
        "caller.inc": direct.fragment(source, b"void updateScreen(Image_ img) {", b"\n\n//%"),
        "image-header.inc": direct.fragment(header, b"struct ImageHeader {", b"\n\nclass RefImage"),
        "image-accessors.inc": direct.fragment(
            header, b"    uint8_t *data() { return buffer->data; }", b"\n    uint8_t fillMask("
        ),
        "method.inc": direct.fragment(source, b"    void sendIndexedImage444(", b"\n#endif\n};"),
    }
    if {name: hashlib.sha256(data).hexdigest() for name, data in fragments.items()} != FRAGMENTS:
        raise RuntimeError("pinned caller/accessor fragment mismatch")
    return fragments


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", required=True)
    parser.add_argument("--output", required=True)
    args = parser.parse_args()
    downloaded = direct.download_inputs()
    with urllib.request.urlopen(
        f"https://raw.githubusercontent.com/microsoft/pxt-common-packages/{direct.PIN}/{HEADER}",
        timeout=30,
    ) as response:
        header = response.read(65537)
    if len(header) > 65536 or hashlib.sha256(header).hexdigest() != HEADER_SHA:
        raise RuntimeError("pinned Image header mismatch")
    fragments = extract(downloaded["libs/screen---st7735/screen.cpp"], header)
    captures = {}
    root = Path(__file__).resolve().parent
    with tempfile.TemporaryDirectory(prefix="pxt-caller-", dir=args.work_dir) as folder:
        work = Path(folder)
        for name, data in fragments.items():
            (work / name).write_bytes(data)
        for enabled in [False, True]:
            binary = work / ("caller-on" if enabled else "caller-off")
            subprocess.run([
                "g++", "-m32", "-std=c++17", "-O0", "-Wall", "-Wextra", "-pedantic",
                *(["-DUSE_RGB444=1"] if enabled else []),
                "-I", str(work), str(root / "caller.cc"), "-o", str(binary),
            ], check=True, timeout=60)
            capture = json.loads(subprocess.check_output([str(binary)], text=True, timeout=15))
            if capture["rgb444Compiled"] != enabled or len(capture["cases"]) != 11:
                raise RuntimeError("missing caller boundary cases")
            captures["macro-on" if enabled else "macro-off"] = capture
            if enabled:
                negative = subprocess.run(
                    [str(binary), "--corrupt-data-capture"], text=True,
                    capture_output=True, timeout=15,
                )
                if negative.returncode == 0 or "caller RAMWR mismatch" not in negative.stderr:
                    raise RuntimeError("caller corrupt-data negative did not fail")
    result = {
        "schema": "labwired.st7735.pxt-caller-fragments.v1",
        "sourcePin": direct.PIN,
        "inputSha256": {**direct.INPUTS, HEADER: HEADER_SHA},
        "fragmentSha256": FRAGMENTS,
        "architecture": "32-bit host x86; not ARM",
        "compiler": subprocess.check_output(["g++", "--version"], text=True).splitlines()[0],
        "capture": captures,
        "negativeControl": "corrupt first caller RAMWR byte rejected",
        "limits": [
            "exact updateScreen, ImageHeader/accessors and direct method fragments only",
            "authored object allocation, fields, palette, SPI, waits and address-window boundaries",
            "checked memcpy boundary uses original constructor allocation formula, not original constructor execution",
            "partial padding and copy-capacity rejection are component controls, not a deployed firmware defect claim",
            "macro on/off is authored build configuration, not deployed CF2 or constructor selection",
            "no full RefImage allocator/GC, full runtime, module/LUT, ARM/DMA/IRQ, RTx or app adoption",
        ],
    }
    Path(args.output).write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({
        "schema": result["schema"], "sourcePin": direct.PIN,
        "builds": {key: len(value["cases"]) for key, value in captures.items()},
        "negativeControl": result["negativeControl"],
    }))


if __name__ == "__main__":
    main()
