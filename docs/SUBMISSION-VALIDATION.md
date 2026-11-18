# Pocket Chess Studies Beta validation — 18 November 2026

Repository main was `417ad5e18dc9f9083c748300d9ad21e3819305db`. Cobalt Beta
was `1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`, independently confirmed
against the remote, and the standalone SDK pin was
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`. The Beta candidate and command
logs are recorded in [RESULTS.json](../submission/checks/RESULTS.json).
Both Rust contexts used 1.85.1 and Cobalt 0.3.37.

| Check | Result |
| --- | --- |
| Standalone formatting, locked host build and public pack validation | Passed. All twenty bundled records validated. |
| Standalone locked tests and strict all-target/all-feature Clippy | 59 passed, 0 failed, 0 ignored. Clippy passed with pedantic lints enabled. |
| Standalone locked ARM release | Passed. Static ARM EABI5, hard-float ABI and no ELF interpreter. |
| Beta locked host app and CLI build | Passed. |
| Official `node tools/app-contribute.mjs --manifest apps/polgar-pocket/cobalt-app.json --dry-run` | Passed. Workspace formatting, 59 app tests (0 failed or ignored), strict all-target/all-feature Clippy, static ARM app-check and local package/catalog previews. |
| Upstream Node policy suite | 129 passed, 0 failed, 0 skipped. |
| Official generated-page freshness and submitted drive route | Passed. One app and one route on the clean Beta candidate. The route also passed on Clara BW at 170%. |
| Full Libra Colour and Clara BW offline touch suites | Passed at normal and 170% on each profile. 84 puzzle solves and 8 progress restarts across both profiles. |
| Storage, exit and idle suite | Passed on both profiles at both scales. |
| Patch and preparation script | Both reproduced the same reviewed contribution from the exact Beta base. |
| Published Beta and local fixture verification | Both published catalogs verified under the release key. The local fixture package and both fixture catalogs passed under the fixture key and each failed with a signature error under the release key. |

The first simulator attempts raced an asynchronously rendered page-button
callback. The Clara failure screenshot already showed the expected second
puzzle. The test now waits for the exact puzzle number after both page-button
events. Initial proxy, loopback and subprocess permission failures were resolved by
running the relevant commands with the required environment access. No verification checks were removed.

The offline suites use Cobalt touch hit testing, page callbacks, storage
acknowledgements and actual app process restarts. They exercise every puzzle,
Hint/Retry/Reveal, wrong legal moves, flip, coordinates, navigation confirmation
and complete/partial progress recovery. Recovery runs exercise corrupt and
unwritable data, write retry, explicit loss acknowledgements and clean exit.
At 170%, the recovery route emits the upstream five-ink warning. The website
generator uses its supported PNG fallback because WebP thumbnails are absent.

The six [simulator captures](../submission/CAPTURES.json) use fresh public demo
state. Capture sidecars retain the actual source revision, dirty state and
executable hash. The source was already final when it was captured, and the
candidate commit also records those images. The renamed home screen was
visually checked on Clara BW at both text sizes. Frames show simulated colour,
layout and input, with physical panel behavior outside their scope.

The official checker fetched published Beta source
`716edd9d25b17a0cb056e87069dc2182d7e5106a`. Its v2 catalog SHA-256 is
`497e267f6f30f3fe8f7af444493f33e600ed90dd1ce9a305ab453967fced55ba`.
The signed catalogs verified against the published Gallery package and contain
no `polgar-pocket` entry. Version 0.2.0 is therefore still the initial
proposed Store version. The registered minimum is 0.3.37, above the protocol 18
floor of 0.3.34. The app's zero capabilities, internal ID, collection, puzzle
pack and on-disk progress format are preserved. The proposed Beta lockfile
adds only the app package and refreshes the active lockfile review hash, which
requires upstream maintainer review.

The full unrelated Cobalt Rust workspace matrix, other apps' simulator routes,
physical installation, real sleep/resume, physical keys, e-ink refresh, battery
life and the owner's existing reader stack were outside the completed checks.
Trusted installation and hardware testing remain gated on official delivery,
owner approval and a compatible reader handoff. See the
[signing investigation](COBALT-SIGNING.md) and [hardware guide](HARDWARE-TEST.md).
