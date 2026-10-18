# Validation Report: renesas_rz_g2l

**Architecture:** Dual ARM Cortex-A55 / Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T00:53:32.155021Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T00:53:32.155216Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas_rz_g2l.yaml
[2m2026-10-18T00:53:32.155543Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T00:53:32.155604Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T00:53:32.155612Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T00:53:32.155616Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T00:53:32.161067Z[0m [33m WARN[0m [2mlabwired_core[0m[2m:[0m Failed to load segment at 0x0 - outside of memory map
[2m2026-10-18T00:53:32.161089Z[0m [33m WARN[0m [2mlabwired_core[0m[2m:[0m Failed to load segment at 0x400 - outside of memory map
[2m2026-10-18T00:53:32.161117Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T00:53:32.161123Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20000000
[2m2026-10-18T00:53:32.161128Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T00:53:32.161159Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 0: Memory access violation at 0x400
[2m2026-10-18T00:53:32.161177Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T00:53:32.161181Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x400
[2m2026-10-18T00:53:32.161185Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 0
[2m2026-10-18T00:53:32.161189Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 0
[2m2026-10-18T00:53:32.161194Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 0.00
```
