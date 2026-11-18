<!-- Prepared template only. No upstream PR has been opened. Target beta only
     after owner approval of upstream contribution and publication. -->

## The app

App id: `polgar-pocket` (display name Pocket Chess Studies, version 0.2.0)

What it lets a Kobo owner do: practise twenty genuine CC0 Lichess demonstration
puzzles offline on a large chessboard, with legal touch moves, deterministic
solution replies, Hint/Retry/Reveal, physical page navigation, acknowledged local
progress, restart recovery and clear storage-failure handling. No complete Polgár
book collection is included.

## Review points

- Purpose and who it is for: quiet offline chess study, primarily Kobo Libra
  Colour. Clara BW is also simulator-tested at normal and 170% text. Physical
  Kobo behavior and coexistence with existing readers remain unverified.
- Licence of the code, and of any content it bundles: AGPL-3.0-only source with
  LICENSE/THIRD-PARTY; shakmaty GPL-3.0-or-later, serde/serde_json MIT OR Apache-2.0;
  twenty public CC0-1.0 Lichess records with exact public rows/provenance. No
  private trainer, chess-book library, credential, custom piece art or personal
  history. Uses Cobalt's existing book glyph, vectors and fonts. Root attribution
  is updated. The owner selected Pocket Chess Studies, short label Pocket Chess. The
  former app ID/progress identity is retained for compatibility. No endorsement
  or exhaustive trademark clearance is claimed.
- Capabilities it declares, and why it needs each: `[]`. No network, engine,
  polling, account, Wi-Fi hold or background task. Progress uses app-local store.
- Accounts or self-hosted services it needs, as `setup.steps` describes them:
  none; `setup.steps` contains only a no-account/Wi-Fi demonstration notice.
  Choose the bundled demo collection and Continue.
- Screenshots, from a Clara BW or its simulator: unedited actual simulator images
  in `apps/polgar-pocket/screenshots/clara-home.png`, `clara-board.png`,
  `clara-board-170.png`, `clara-revealed-170.png`, with explicit captions in the
  website gallery and capture sidecars/hashes in the app's CAPTURES.json.
  Two additional Libra Colour captures are included. No device photos claimed.

The contribution adds one app and its workspace/lock registration. It adds only
that app's root attribution, generator category/screenshot mapping and generated
website outputs. The active protocol-18 review record refreshes its lockfile
output hash because the new workspace package changes the inventory. Its base,
all SDK hashes, dependency versions, enforcement code, keys, workflows and other
apps remain unchanged. Please review this proposed lockfile attestation refresh.

## Contributor check

The final lines of
`node tools/app-contribute.mjs --manifest apps/polgar-pocket/cobalt-app.json --dry-run`:

```
verified polgar-pocket (kobo-polgar-pocket)
created /workspace/work/cobalt-beta-submission/target/app-contribute/polgar-pocket/preview/polgar-pocket-0.2.0.cobalt-app
created /workspace/work/cobalt-beta-submission/target/app-contribute/polgar-pocket/preview/cobalt-app-catalog-v2.json
created /workspace/work/cobalt-beta-submission/target/app-contribute/polgar-pocket/preview/cobalt-app-catalog-v2.json.sig
created /workspace/work/cobalt-beta-submission/target/app-contribute/polgar-pocket/preview/cobalt-app-catalog.json
created /workspace/work/cobalt-beta-submission/target/app-contribute/polgar-pocket/preview/cobalt-app-catalog.json.sig
contribution check passed for polgar-pocket; local Beta-parity preview: target/app-contribute/polgar-pocket/preview
```

Prepared against Beta `1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`, local candidate
`ba5ad80d0e2f2151ec29e610f81867a4b3bb763a`. Rust 1.85.1; app tests 59/59,
0 failed/ignored; strict all-target/all-feature Clippy and formatting passed;
host and static ARM release passed. All 129 Node tools tests passed. Official
all-routes simulator runner passed for this app. Broader offline input suites
passed on Clara BW and Libra Colour at normal/170%, completing 84 puzzle lines
with 8 complete/partial progress restarts plus corruption/write-failure recovery.
The recovery routes report the existing five-ink layout warning; no
layout errors were ignored. The full unrelated Rust workspace matrix has not
been run locally.

The official fixture package and both catalogs verify only under the existing
public test key and fail under the actual trusted release public key. Maintainer
acceptance and protected Beta publishing must rebuild/sign the package and both
catalogs. No trusted release package, stock-runtime private installation path,
publication or hardware acceptance is claimed. Stable promotion and hardware
installation remain separate approval steps.
