# micro:bit v2 Validation Runbook

Run all commands from `core/`.

## 1) Optional: ensure target installed

```bash
rustup target add thumbv7em-none-eabi
```

## 2) Build smoke firmware

```bash
cargo build -p firmware-nrf52833-demo --release --target thumbv7em-none-eabi
```

## 3) Run deterministic UART smoke

```bash
cargo run -q -p labwired-cli -- test \
  --script examples/microbit-v2/uart-smoke.yaml \
  --output-dir out/microbit-v2/uart-smoke \
  --no-uart-stdout
```

Pass criteria:

1. exit code is `0`
2. UART contains `OK`

## 4) Run firmware survival gate

```bash
cargo test -p labwired-core --test firmware_survival test_nrf52833_microbit_v2_smoke_survival -- --nocapture
```

Pass criteria:

1. test passes
2. UART sink contains `OK` (fixture: `tests/fixtures/microbit-v2-smoke.elf`)

## 5) Run direct simulation for PC/SP evidence

```bash
cargo run -q -p labwired-cli -- \
  --firmware target/thumbv7em-none-eabi/release/firmware-nrf52833-demo \
  --system configs/systems/microbit-v2.yaml \
  --max-steps 32 \
  --json
```

## 6) Run unsupported-instruction audit

Audited against the **tier-1 fixture** (the deepest firmware committed for this
board), exact command:

```bash
./scripts/unsupported_instruction_audit.sh \
  --firmware tests/fixtures/tier1/nrf52833.elf \
  --system configs/systems/microbit-v2.yaml \
  --max-steps 200000 \
  --out-dir out/unsupported-audit/microbit-v2
```

Observed result (2026-09-20):

```text
unknown_thumb16: 0
unhandled_thumb32: 0
unknown_riscv: 0
unsupported_total: 0
report: out/unsupported-audit/microbit-v2/report.md
```

The report records `instructions_executed = 199999`, `sim_exit_code = 0`,
`instruction_support_percent = 100.0000`, and the full TIER1 transcript was
already emitted by step 199999 (see `run.json` in the audit directory).

Pass criteria:

1. script exits `0`
2. audit report exists at `out/unsupported-audit/microbit-v2/report.md`
3. `unsupported_total` is `0` over the audited steps

## 7) Tier-1 peripheral self-tests (raw-register depth)

The fixture is a standalone crate (own `[workspace]`), so it builds with its
own target dir rather than the workspace root's.

```bash
cd examples/tier1-fixture/nrf52833
cargo build --release --target thumbv7em-none-eabi
cd ../../..

# Only when re-cutting the committed blob:
cp examples/tier1-fixture/nrf52833/target/thumbv7em-none-eabi/release/tier1-fixture-nrf52833 \
   tests/fixtures/tier1/nrf52833.elf

labwired run --chip configs/chips/nrf52833.yaml \
  --firmware tests/fixtures/tier1/nrf52833.elf --max-steps 8000000 \
  2>&1 | grep -a TIER1
```

Observed transcript (verbatim, blob built from source_rev
`3d458ca36adc1292af1fdbf2391286a94586b6c2`):

```text
TIER1 gpio PASS
TIER1 clock PASS
TIER1 timer PASS
TIER1 irq PASS
TIER1 rtc PASS
TIER1 i2c PASS
TIER1 spi PASS
TIER1 adc PASS
TIER1 dma PASS
TIER1 wdt PASS
TIER1 pwm PASS
TIER1 done
```

Pass criteria:

1. all eleven printed classes report `PASS` (UART is implicit via `TIER1 done`)
2. `TIER1 done` is present — the fixture completed its whole sequence
3. `irq` is a real peripheral-sourced NVIC delivery: TIMER0 COMPARE0 pends
   NVIC IRQ 8 and the `DefaultHandler` counts the vector actually running
4. `dma` proves EasyDMA descriptor semantics (two pointers, `MAXCNT` 4 then 2,
   sentinels + exact payload) on the SAADC RESULT channel; the class is
   declared by the chip YAML opt-in `tier1_classes: ["dma"]` — nRF52 has no
   central DMA controller
5. the live full-matrix run with the updated CLI shows nrf52833 at
   `adc=pass clock=pass dma=pass gpio=pass i2c=pass irq=pass pwm=pass
   rtc=pass spi=pass timer=pass uart=pass wdt=pass`, and no other chip's row
   changed (diff against `docs/coverage/tier1-matrix.json`: exactly
   `nrf52833/dma: na -> pass` and `nrf52833/irq: na -> pass`)

## A10 transmit stack-lifetime regression

`uarte-stack-buffer.S` is original MIT-licensed Cortex-M4 assembly, with no
Nordic/CODAL/MakeCode implementation copied. Its `stack_putc` arms a one-byte
UARTE transfer from its stack, returns, and a separate function overwrites that
same slot before the caller waits for ENDTX. It emits `A10 stack reuse OK\n`.
RAM counters prove that every transmitted slot was reused and the guest reached
its terminal success state rather than timing out.

The Rust integration test compiles that source with ARM GCC and `board-io.ld`,
prints the ELF SHA-256, executes it on the micro:bit V2 system at tick interval 1 in both feature
configurations, additionally at interval 64 with the scheduler, and checks
exact ordered UART bytes and all guest counters:

```sh
cargo test -p labwired-core --features event-scheduler --test microbit_v2_uarte_stack_guest -- --nocapture
LABWIRED_UARTE_STACK_INTERVALS=1 cargo test -p labwired-core --no-default-features --test microbit_v2_uarte_stack_guest -- --nocapture
```

The model controls separately require that early capture publishes no bytes,
AMOUNT, completion event or interrupt, while the original frame-count completion
deadline is retained. This is a bounded simulated UART/stack-lifetime proof;
MakeCode firmware on the released WASM engine and Lite adoption remain separate.

## A10 interrupt-chained completion regression

`uarte-irq-chain.S` is separately authored MIT assembly; no Nordic or CODAL
implementation is copied. UARTE0's real interrupt vector chains each next
stack-backed byte from ENDTX, then inspects TXSTOPPED in that same handler.
An unsolicited stop records failure before the new transfer can be mistaken
for idle. After the last byte, the guest explicitly requests STOPTX and requires
its TXSTOPPED event to finish. The exact expected stream is `A10 IRQ chain OK\n`.

The same `microbit_v2_uarte_stack_guest` target runs both guests. The new guest
requires done=1, every byte completed in order, zero unsolicited stops, idle
state only after the final explicit stop, and an exact serial-byte comparison.
Scheduler runs cover intervals 1, 64 and 1024; feature-off runs select interval 1.
The earlier stack-reuse guest remains unchanged. These tests do not substitute
for Lite's actual MakeCode CLI and browser complete-line qualification.

The event distinction follows the [nRF52833 UARTE transmission specification](https://docs.nordicsemi.com/r/bundle/ps_nrf52833/page/uarte.html):
ENDTX marks a completed buffer; TXSTOPPED acknowledges STOPTX. Existing framing,
completion-cycle bounds, descriptor capture and explicit-stop IRQ checks stay
required. No physical timing or silicon re-capture result is implied.
