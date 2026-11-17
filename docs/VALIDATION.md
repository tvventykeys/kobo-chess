# Application validation — v0.2, 17 November 2026

Branch `feat/polgar-pocket-v0.2`, based on merged main `85be813`. Rust 1.85.1;
Cobalt 0.3.37 at `5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`. Remote upstream
main was checked and matches that pinned SDK. Existing setup/env/upstream
verification scripts and the Cobalt working tree remain unchanged.

## Results

| Check | Actual result |
| --- | --- |
| Formatting | `cargo +1.85.1 fmt --check` passed |
| Clippy | Locked, all targets, `-D warnings`: passed |
| Application unit/regression tests | **58 passed, 0 failed, 0 ignored**; includes all original 41 |
| Host build | Locked Rust 1.85.1 build passed |
| ARM release build | Locked `armv7-unknown-linux-musleabihf` release build passed |
| ARM ELF | Static 32-bit ARM EABI5, no ELF interpreter, 3,104,312 bytes |
| Pack/solution checks | All 20 original CC0 Lichess IDs, FENs and complete legal lines passed; source records unchanged |
| Normal/170% Libra Colour functional simulation | 42 completed solves, four process restarts; promotion, hints, retry, reveal, page events and partial-session restoration passed |
| Recovery simulation at both text sizes | Corrupt file preservation, safe missing-value reload, failed writes, retry, explicit loss warning, sleep waiver, restored writes and clean app exit passed |
| Idle observation | App process: zero additional CPU ticks and unchanged paint count over two seconds at each text size; one sleeping thread |
| Progress upgrade | v0.1-shaped records retain counters, assistance, reveal history, orientation and active session; no reset |
| Cobalt signing tests | `kobo-app-store`: 30 passed, 0 failed, 0 ignored; 0 doc tests |
| Verification-only bundle | Creation and exact manifest/binary/signature verification under its fixture key passed |
| Actual release-key check | Fixture correctly refused with Ed25519 signature verification failure |
| Python/shell syntax and Git whitespace | Passed |
| First physical test / installed device package / measured battery | **Not performed / not available / not measured** |

The last full `bash scripts/verify-pocket.sh` command runs formatting, strict
Clippy, unit tests, host/ARM builds, pack/static-ELF checks and both simulator
suites. Its final log is `/workspace/verification/pocket-v02/final-verification.log`.
No host build, open server port or successful developer signature is counted as
physical-device validation.

## What the simulator actually exercised

Both runs asserted profile `libra-colour-390`, 1264 × 1680, colour panel, with
`KOBO_SIM_OFFLINE=1`. The driver resolves actions to rendered hit targets and
sends actual taps. All twenty solutions were played at each text size, including
promotion; two restored partly played assisted attempts account for the other
two solves. Four full process restarts check progress restoration. Separate
recovery runs restart against a deliberately damaged **temporary host fixture**,
confirm those bytes remain intact, then remove only that fixture to reproduce
the missing-value reload crash path. No user/production/device data is involved.

Denied writes are induced by removing write permission on the temporary store
directory, then restored. The warning, retry, explicit Allow sleep and exit-loss
screen must fit and respond at 170%. Restoring permission permits a real atomic
save, and the app/CLI must exit with status 0 after Exit to launcher. The upstream
driver races with app shutdown: a final tap may be acknowledged or reported as
disconnected; only those outcomes plus observed process exit 0 and durable saved
state pass. Other driver errors remain failures.

Repeated hints are checked against actual simulator paint counts. The dedicated
app child's `/proc` CPU and memory statistics are sampled separately from the
CLI/renderer. In the reviewed host observations RSS was approximately 9 MiB,
with one sleeping thread and zero new CPU ticks over two seconds. Those values
are not device RAM, battery lifetime, refresh timing or long-session measurements.
See each run's `recovery/{default,170}/idle.json` for exact observations.

## Failures found and fixed

Six regressions first failed against v0.1: storage reload panic, misleading Home
confirmation, repeated identical redraws, accepted castling alias, fabricated
completion and mismatched restored results. New tests also cover save-aware exit,
write-loss/sleep acknowledgement, restored completion feedback, SAN, promotion
navigation, bounded import streams and v0.1 progress compatibility.

A 170% recovery screen initially put the Allow sleep control below the panel.
The compact action row fixed the renderer failure; both recovery layouts are
checked again. The exit driver initially treated successful disconnect as an
error; its expectation now requires a normal acknowledgement or the specific
shutdown message, plus independent clean process exit. No application failure
was waived. [AUDIT-V02.md](AUDIT-V02.md) explains the actual fixes.

## Artifacts and remaining limits

Cloud artifacts are outside Git:

* ARM executable: `/workspace/build/cobalt/armv7-unknown-linux-musleabihf/release/polgar-pocket`
* Host executable: `/workspace/build/cobalt/debug/polgar-pocket`
* Screenshots, layouts, public saved progress, logs and idle records:
  `/workspace/verification/pocket-v02/` (`default`, `170`, `recovery`).
* Verification-only bundle/public manifest and key:
  `/workspace/verification/pocket-v02/bundle/`.
* Prepared, unsubmitted upstream source tree:
  `/workspace/verification/pocket-v02/contribution-review-final/`.
* Delivery archive:
  `/workspace/verification/polgar-pocket-0.2.0-cloud-delivery.zip`.

Source, binaries, public evidence and license notices are packaged without private
book data or any signing seed. Screenshots were inspected for board dominance,
piece distinction, contrast, readable feedback, clipping and overlap, including
storage errors and enlarged text. Cobalt vector artwork is retained; Cburnett
SVG integration is an SDK capability limitation, documented with its actual
license in the audit. Conventional outer coordinate rails remain unsupported.

There is **no stock-device-ready package**. A trusted package and trusted catalog
are required; owner key enrollment/non-Store sideloading is not exposed in the
inspected stock implementation. The public contributor route triggers maintainer
Beta Store publication, which was not authorized or performed. The signing
investigation and conditional hardware/rollback guide state the exact external
requirements. Zen OS coexistence, existing-reader handoff, actual page mapping,
physical touch, suspend/resume, real colour, ghosting and battery still need an
owner-attended test after those requirements are satisfied.

No Kobo was connected or changed. KOReader, NickelMenu, Zen OS, Focus Reader and
production Polgár Trainer were untouched. No firmware, boot, partition, system
library, SSH or Store publication changes were made. Fresh engine analysis,
alternate winning-line validation, online play and production sync remain outside
scope. The earlier 507 platform tests and v0.1's 41 tests are historical results,
not added to this version's 58-test total.
