# Application validation — 17 November 2026

Application branch: `feat/polgar-pocket-v0.1`, based on latest `origin/main`
(`9735b3a`, including the verified environment). Rust 1.85.1; pinned Cobalt
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`, version 0.3.37. The existing setup,
environment and upstream verification scripts were preserved byte-for-byte.
Upstream Cobalt's working tree remained clean.

## Actual results

| Check | Result |
| --- | --- |
| `cargo +1.85.1 fmt --check` | Passed |
| `cargo +1.85.1 clippy --locked --all-targets -- -D warnings` | Passed, no warnings |
| `cargo +1.85.1 test --locked` | **41 passed, 0 failed, 0 ignored** |
| `cargo +1.85.1 build --locked` | Passed, x86_64 host executable |
| `cargo +1.85.1 build --locked --release --target armv7-unknown-linux-musleabihf` | Passed |
| ARM ELF inspection | Static 32-bit ARM EABI5, no ELF interpreter; 3,076,372 bytes |
| Builtin pack validation | 20 unique original Lichess IDs; all FENs and complete legal lines passed |
| Independent source replay | All 20 original setup/solution lines passed chess.js replay |
| CSV import reproduction | 20 generated JSON records compare equal to the builtin pack |
| Private trainer import | 20 selected book problems locally imported/validated; no private data in Git |
| `bash scripts/verify-pocket.sh` | Exit 0, including both simulator text sizes |
| Simulator full functional suite | **42 completed solves, 4 process restarts**, normal and 170% text |
| Final-code simulator smoke | Passed normal/170% controls, updated navigation dialog, hint/retry/reveal and layout checks |
| Development bundle creation | Passed actual Cobalt `app-bundle` command |
| Development bundle verification | Passed signature, manifest and exact binary comparison with `app-verify` |
| Shell/Python script syntax | Passed |
| Physical Kobo / measured battery / real suspend-resume | **Not run** |
| Live trainer synchronization / online play / analysis | **Not implemented or run** |
| Stock-runtime application installation / Store publication | **Not performed**; trusted platform signing is still required |

The full suite used `libra-colour-390`, asserted 1264 × 1680 and `colourPanel=true`,
with `KOBO_SIM_OFFLINE=1`. Its driver resolves controls to actual display
coordinates and sends taps through hit testing, rather than directly calling
application actions. It solved all twenty puzzles at both text sizes, including
the promotion chooser, and two restored partial attempts. Deterministic opponent
replies were required for each line to complete. Simulated page-button events
went through `/input` and the SDK callback. The navigation guard was also tested.

Progress was read from the actual simulator store on disk before process
termination, not only from in-memory state. A fresh app process restored all
20 solved records, selection and clean/assistance history. A second restart at
each text size restored the partly played first puzzle and its hint flag; finishing
it was scored assisted. Retry retained previous attempts/reveal history. A reveal
alone kept the completion count at zero. Unit tests additionally cover denied
writes, coalescing, corruption preservation and malformed restored sessions.

Screenshots were inspected for the dominant board, colour/paper contrast,
readable controls and completed explanation/line at 170% text. An early version
showed excessive empty-square coordinate labels and crowded enlarged navigation
labels; these were fixed with optional Coords and short Prev/Home/Next labels.
Final screenshots show all controls on screen with no ordinary scrolling.
Layout `clean` checks passed throughout the important screens. No warnings/errors
were found in the final simulator logs. Simulator ghosting, real colour response,
physical button mapping, touch calibration, refresh timing and suspend/resume
still need hardware acceptance. No physical-reader testing is claimed.

## Artifacts in the cloud workspace

* ARM ELF: `/workspace/build/cobalt/armv7-unknown-linux-musleabihf/release/polgar-pocket`
* Host ELF: `/workspace/build/cobalt/debug/polgar-pocket`
* Development bundle, public key and manifest: `/workspace/verification/pocket-bundle/`
* Full verification log: `/workspace/verification/pocket-final-verification.log`
* Final smoke log: `/workspace/verification/pocket-final-smoke.log`
* Screenshots/layout/store evidence: `/workspace/verification/pocket/`
  (`default/`, `170/`, their restart directories, `final-smoke/`).
* Delivery archive: `/workspace/verification/polgar-pocket-0.1.0-cloud-delivery.zip`

The delivery excludes the development signing seed, private book fixtures and
production data. The `.cobaltapp` is a verified development artifact, **not a
stock-runtime trusted release**. See INSTALLATION.md for the missing platform
signing/installation step. No existing device installation was changed.

The environment-only 507-test results in README are historical platform setup
validation and are not counted as new application tests. No fresh Stockfish
best-move/uniqueness analysis was performed; source authenticity, legal replay
and interactive execution were verified. The current app accepts its ordered
published line, not all alternate wins.
