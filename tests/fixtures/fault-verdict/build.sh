#!/usr/bin/env bash
# Rebuild the fault-verdict fixture ELFs. Needs arm-none-eabi-gcc.
# The ELFs are committed so CI needs no cross toolchain.
set -euo pipefail
cd "$(dirname "$0")"
build() {
  arm-none-eabi-gcc -mcpu=cortex-m4 -mthumb -Og -g -ffreestanding -nostdlib \
    -fno-builtin -fdebug-prefix-map="$PWD"=tests/fixtures/fault-verdict -Wall -Wextra -Werror -DFAULT_KIND="$2" \
    -T link.ld -o "fault-$1.elf" fault_fixture.c
}
build hardfault-forced 1
build busfault 2
build undef 3
build div0 4
build lockup 5
build readonly-flash 6
