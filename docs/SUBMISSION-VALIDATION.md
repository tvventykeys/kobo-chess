# Beta submission validation — 17 November 2026

**Ready for owner review and a later approved upstream contribution. Not published
or stock-runtime installable.** These are new runs of the final submission source,
not the historical v0.1/v0.2 totals. Version remains 0.2.0 as requested.

Base: own main `a4cd875`; Cobalt Beta
`1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`.
Prepared isolated branch `feat/polgar-pocket-beta`, commit
`b8da05efa0495a26ec36fa6ead71c4946b65864c`. Its upstream push URL is disabled.
The original pinned SDK/main at `5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`
remains clean. Both contexts use Rust/Cargo 1.85.1 and Cobalt 0.3.37.

## Executed checks

| Check | Exact result |
| --- | --- |
| Standalone `cargo +1.85.1 fmt --check` | Passed |
| Standalone locked all-target strict Clippy, including pedantic | Passed |
| Standalone locked tests | 59 passed, 0 failed, 0 ignored |
| Standalone locked host build and bundled pack validation | Passed; twenty records verified |
| Standalone locked ARM release build | Passed; static ARM EABI5, no ELF interpreter |
| Beta `cargo fmt --all -- --check` | Passed through official contributor command |
| Beta locked app tests | 59 passed, 0 failed, 0 ignored; same suite, not 59 additional distinct tests |
| Beta locked all-target/all-feature strict app Clippy | Passed through official contributor command |
| Beta locked host app/CLI build | Passed |
| Beta locked ARM app-check/build | Passed; static binary verified by official app-check |
| `node tools/app-contribute.mjs --manifest apps/polgar-pocket/cobalt-app.json --dry-run` | Passed on committed candidate; generated fixture package and both signed catalog previews |
| `node --test tools/*.test.mjs` | 129 passed, 0 failed, 0 skipped |
| Official generator freshness, tracked and untracked page check | Passed after committed candidate |
| Official `scripts/check-apps-sim.py polgar-pocket --all-routes` | Passed, 1 app/1 submitted route, candidate SHA and clean tree recorded |
| Submitted drive.kobo, unchanged, additional Clara runs | Passed at normal and 170% |
| Complete Libra Colour real-input simulator suite | Passed at normal and 170%; 42 puzzle solves, 4 progress process restarts |
| Complete Clara BW real-input simulator suite | Passed at normal and 170%; 42 puzzle solves, 4 progress process restarts |
| Storage recovery/exit/idle suite, both profiles and scales | All four combinations passed |
| New full preparation script on fresh clean Beta clone | Reproduced all 47 contribution files byte for byte |
| Complete Git binary patch on separate clean exact-base clone | Applied cleanly; all 47 files byte-identical to committed candidate |
| Fixture package and both catalog verification | 3 successes under upstream's public fixture key; 3 expected signature failures under the actual release public key |

The simulator uses the actual Beta workspace application, Cobalt hit testing,
touch endpoints, page callbacks, filesystem store acknowledgements and process
restarts. Across four profile/scale combinations it completes every bundled
puzzle, checks Hint/Retry/Reveal, wrong legal move rejection, collection choice,
flip/coordinates, navigation confirmation, scores and both complete/partial
progress restoration. Totals are **84 puzzle solves and 8 progress restarts**;
recovery-suite restarts are not added to that count. Recovery also covers missing,
corrupt/preserved and denied data, read-only study, retry, failed save, explicit
unsaved exit/sleep acknowledgement, clean shutdown and Home/Continue.

The new test `contributor_clara_completion_and_storage_dialog_fit_at_170_percent`
validates both normal and largest Clara metrics and retains all original 58 tests.
Two genuine smaller-panel layout failures were fixed: completion control rows
and storage notices. Final captures were visually inspected for clipped text,
overlapping controls and piece contrast; completion and storage actions fit.
The recovery 170% runs emit the upstream **five-ink warning**; no layout error is
suppressed, and the routes pass. Optional WebP thumbnails are absent: the official
generator reports its supported PNG fallback. No warning-free claim is made.

The first broader policy run passed 107/108 tests, failing the active protocol's
exact lockfile inventory assertion for the newly added workspace app. The final
patch refreshes only that record's Cargo.lock output hash and explanatory reason;
it leaves its reviewed base, every SDK blob, test and enforcement code unchanged.
The lockfile itself adds exactly one 13-line app package entry. No existing
package identity, dependency edge or third-party version changes. This proposed
review-record refresh is visible for maintainer approval. Final 129/129 includes
all Node tools tests, not just that initial subset.

## Rendering, storage and host idle evidence

Six unedited real simulator PNGs, their sidecars and SHA-256 values are in
[the capture inventory](../submission/CAPTURES.json). Main listing: actual Clara
BW board. Additional captures show normal/170% Clara and Libra Colour. These are
ideal simulator frames with fresh public-demo state, not photographs or proof of
physical contrast, ghosting or refresh behavior. Capture metadata honestly records
the dirty Beta-base checkout used at capture time; the compiled source matches
the final candidate. It has not been rewritten to pretend an earlier capture was
made from the later commit.

Four two-second host idle samples recorded **0 CPU ticks, 4→4 paints, one app
thread and 9,028–9,228 KiB RSS**. Unit tests still reject networking/polling commands.
Writes remain event-driven, atomic and acknowledgement-based; no background task
or idle refresh was added. These short host observations do not measure Kobo
power, whole-runtime CPU, or battery-life improvement.

Standalone ARM binary: 3,108,448 bytes, SHA-256
`922a37f4a9224d0bada01a222c64e79bb69d37f45bc9789f3597d5d96ba794d1`.
Beta package binary SHA-256:
`4ff1c79f2849a88a22d2b8662bff66068a865e1b763decdd179d83449697fe2b`.
The local fixture `.cobalt-app` is 2,474,008 bytes and uses `example.invalid`
download URLs. It is kept outside Git and explicitly labelled LOCAL-ONLY.

The downloaded published Beta baseline was cryptographically bound to its
provenance by the official checker. It has no polgar-pocket entry. Source SHA:
`716edd9d25b17a0cb056e87069dc2182d7e5106a`; catalog SHA-256:
`497e267f6f30f3fe8f7af444493f33e600ed90dd1ce9a305ab453967fced55ba`.
Future submissions must recheck the current upstream ref and published baseline.

## Evidence and limits

[Machine results and recorded logs](../submission/checks/RESULTS.json) accompany
the packet. The full selected command logs, baseline/provenance and CI route
result are in `submission/checks/`; raw simulator state and binaries remain under
`/workspace/verification/cobalt-submission` and `/workspace/build/cobalt-beta`.
The six reviewed Store captures are the only screenshots redistributed here.
No signing seed, personal database or raw progress snapshot is included.

Not run: the unrelated full Rust monorepo suite, every other app's simulator
routes, physical Kobo installation, real suspend/resume, page-key electrical
mapping, actual e-ink refresh/colour, battery measurements and compatibility with
the owner's Zen OS/KOReader/NickelMenu/Focus Reader stack. The official CI remains
responsible for its full workspace matrix after an authorized upstream PR.
No fork, upstream PR, Store publication, maintainer credential, trust change,
firmware change, SSH activation or device write occurred.

Code/data have no identified redistribution-license blocker. Naming review and
explicit permission for upstream submission/publication remain outstanding;
trusted signing is exclusively a later protected maintainer workflow. See
[NAMING-LICENSE-PRIVACY.md](../submission/NAMING-LICENSE-PRIVACY.md),
[submission steps](../submission/SUBMIT.md) and
[hardware stop gates](HARDWARE-TEST.md).
