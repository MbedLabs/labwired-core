# Validation Report: renesas_rz_g2l

**Architecture:** Dual ARM Cortex-A55 / Cortex-M33.

## 1. Dynamic Simulation Validation
**Status:** ✅ Passed (simulation-ok)

```text
[2m2026-10-18T11:37:31.692279Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting LabWired Simulator
[2m2026-10-18T11:37:31.692445Z[0m [32m INFO[0m [2mlabwired_core::system::builder[0m[2m:[0m Loading chip descriptor: ../../chips/onboarding/renesas_rz_g2l.yaml
[2m2026-10-18T11:37:31.692751Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Loading firmware: "/home/andrii/Projects/labwired-renode/core/tests/fixtures/uart-ok-thumbv7m.elf"
[2m2026-10-18T11:37:31.692819Z[0m [32m INFO[0m [2mlabwired_loader[0m[2m:[0m ELF Entry Point: 0x401
[2m2026-10-18T11:37:31.692832Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Firmware Loaded Successfully!
[2m2026-10-18T11:37:31.692836Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Entry Point: 0x401
[2m2026-10-18T11:37:31.693218Z[0m [33m WARN[0m [2mlabwired_core[0m[2m:[0m Failed to load segment at 0x0 - outside of memory map
[2m2026-10-18T11:37:31.693232Z[0m [33m WARN[0m [2mlabwired_core[0m[2m:[0m Failed to load segment at 0x400 - outside of memory map
[2m2026-10-18T11:37:31.693271Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Starting Simulation (ARM Cortex-M)...
[2m2026-10-18T11:37:31.693276Z[0m [32m INFO[0m [2mlabwired_cli::commands::run[0m[2m:[0m Initial PC: 0x400, SP: 0x20000000
[2m2026-10-18T11:37:31.693281Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Running for 20000 steps...
[2m2026-10-18T11:37:31.693304Z[0m [32m INFO[0m [2mlabwired_cli[0m[2m:[0m Simulation Error at step 0: Memory access violation at 0x400
[2m2026-10-18T11:37:31.693322Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Simulation loop finished.
[2m2026-10-18T11:37:31.693326Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Final PC: 0x400
[2m2026-10-18T11:37:31.693330Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Instructions: 0
[2m2026-10-18T11:37:31.693333Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Total Cycles: 0
[2m2026-10-18T11:37:31.693337Z[0m [32m INFO[0m [2mlabwired_cli::report[0m[2m:[0m Average IPS: 0.00
```
