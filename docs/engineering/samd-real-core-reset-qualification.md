# Actual CODAL core-reset qualification lane

This proposed slice is not a passing guest result or complete board reset.
The actual caller is pinned to Runtime
`4ab84c54c1e9d28c6e927458ddebc3da7b753432`; its separate twenty-mode build
passes, with an independent retained-original audit showing all old0–18 ELF
hashes unchanged. Build evidence does not qualify the engine correction.

The focused `CortexM::reset` correction clears prior masks, IT/PSR and LR
according to [Arm's Cortex-M4 register summary](https://documentation-service.arm.com/static/5f2ac4ab60a93e65927bbdbf).
This was a source-review finding, not a claimed earlier guest failure. Permanent
unit controls deliberately poison the prior state. The actual target_reset
caller separately requires pre-CPSID startup register capture, real invalid-key
return versus valid AIRCR store/reset cut, two startup/data/BSS epochs, retained
NOLOAD witness, pre-reset core-state readbacks and closed post-terminal bounds.
Use actual retired store opcode/registers, not invented SCS write callbacks.
Previous-retirement registers at reset entry are OLD context; only the actual
post-first-LDR snapshot and startup's own register stores describe the new boot.

Because image loading also invokes CPU reset, the hosted workflow clean-builds
and repeats all nineteen earlier service/heap/SPI modes at unchanged Runtime
`932d476344107386d56dfd2d595d74d713780563`, under both scheduler profiles.
The new caller also runs single-step and real bounded-batch paths in both.
Every profile/caller executes despite another failure; retain all originals.
Do not merge on unit or build evidence alone. Freeze independent source/image/
provider predicates and refusal controls before sole original acquisition;
then require every enabled final-head check and exact tested/landed tree proof.

The selected engine reset is core-local: peripheral models, external pending
sources, flash commands/persistence, USB/QSPI, physical reset behavior, complete
board/API/build/image admission, native Arcade, active RTx and installed consumer
qualification remain separate. The [board/API continuation](samd-runtime-replacement-plan.md#current-continuation-after-actual-spi-callers)
and [landed lifetime checkpoint](samd-real-spi-lifetime-qualification.md) remain
authoritative. No consumer pin, ACK, physical capture or threshold change.
