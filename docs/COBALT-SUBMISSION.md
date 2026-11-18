# Cobalt Beta Store submission preparation

This packet records the previous demo-only candidate. The CC0 starter preview has
its own [roadmap](ROADMAP.md) and [validation](LICHESS-STARTER-VALIDATION.md).
Refresh the packet before proposing a new upstream submission.

This is a **local, reviewable submission packet**, not an upstream submission or
Store release. The application stays at version **0.2.0**. The owner must approve
naming and an upstream PR/publication step separately; no fork was created,
upstream branch pushed, maintainer contacted, release published or device touched.

## Current upstream authority

The official destination is `BandarLabs/Cobalt:beta`, verified again on 18 November 2026 at
`1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`. Main/pinned SDK remains
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0`. Both use Rust 1.85.1 and workspace
0.3.37, but Beta is the actual contribution base. The original SDK checkout is
unchanged. An isolated clone lives at `/workspace/work/cobalt-beta-refresh`, with upstream pushes disabled.

The actual [contributor guide](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/docs/CONTRIBUTING_APPS.md), registry normalizer, page generator,
package verification code, public fixture seed and app PR template were read.
The official contribution has `apps/polgar-pocket/{src,packs,Cargo.toml,
cobalt-app.json,README.md,LICENSE,THIRD-PARTY.md,drive.kobo,CAPTURES.json,
screenshots,capture-metadata}` plus workspace/lock registration. Package name is
`kobo-polgar-pocket`; Cargo inherits the platform workspace version and the app
release version belongs only to cobalt-app.json. The explicit minimum is 0.3.37,
above the protocol-derived 0.3.34 floor. Capabilities remain empty.

The current generator also requires a category entry and needs a screenshot
mapping to avoid displaying a generic Store screen. Only those two app-specific
entries are added to its existing tables: Games and the real Clara BW screenshot.
The official generator then writes the app page, index, sitemap and README table.
The active protocol-18 review record also refreshes only its Cargo.lock output
hash and explains the new workspace package. The existing review base and every
SDK blob stay fixed; this proposed review-record update needs maintainer review.
No catalog, signing key/workflow, other app, Lichess source or manual website
page content is edited. The root attribution receives the app-specific CC0 notice.

## Required compatibility changes

Upstream workspace Clippy includes pedantic rules; this app now checks against
those same rules in both repositories. Literal separators, string conversions
and if-let handling were adjusted. Three local, reasoned lint allowances keep
independent UI/storage flags and the existing long screen/action descriptions
rather than forcing an unrelated architecture rewrite. serde_json is aligned
with upstream's existing 1.0.151; the proposed upstream Cargo.lock adds only the
new package, with no dependency downgrade. Original 58 tests are retained.

The contributor template asks for Clara BW captures. Actual simulation found
revealed-solution controls and storage warnings clipping on that smaller panel.
Completion now uses Retry/Home/Next in one row with Previous in the top bar;
storage recovery uses an explicit dismissible dialog and a persistent Storage
control. The puzzle, Hint/Retry/Reveal behavior, physical buttons, legal rules,
progress identity/schema and offline execution remain intact. A new normal/170%
Clara regression test covers these layouts. Store assets are copied only after
successful real-input runs. See SUBMISSION-VALIDATION.md for exact results.

## Packet and reproduction

`submission/` contains:

* `upstream.json`: repository, exact Beta base, app/package/version, compatibility,
  zero capabilities and explicit no-publication/no-device authorization state.
* `cobalt-beta.patch`: complete Git binary patch against that base, including new
  source, required registration, reviewed PNGs and generated website changes.
* `README-upstream.md`, `drive.kobo`, `upstream-third-party.md`: actual app-facing
  readme, interactive smoke script and root attribution addition.
* `screenshots/`, `capture-metadata/`, `CAPTURES.json`: authentic unedited simulator
  images and hashes/profiles/text scales, not images drawn for the listing.
* `icon.md`: the canonical runtime-rendered `book` glyph specification; no custom
  icon file or unsupported manifest field is required.
* `NAMING-LICENSE-PRIVACY.md`: code/data/art audit and unresolved name decision.
* `UPSTREAM-PR.md`: filled official PR template for owner review only.
* `SUBMIT.md`: exact future submission/acceptance/signing/hardware approval steps.

`scripts/prepare-cobalt-beta.py CLEAN_CHECKOUT` recreates the complete contribution
locally in a separate clean checkout at the pinned base. It refuses the pinned
SDK, wrong revision, dirty checkout, occupied app identity, altered PNG hashes or mismatched capture metadata
or an unapproved pack. It copies explicit public file allowlists and calls only
the local page generator. It does not fetch, push, sign, publish or install.

Preparation requires Rust 1.85.1 and the already-cached upstream dependencies;
offline Cargo metadata adds only the app lock entry and rejects any other lock
change. The independent script and binary patch were both checked against the
exact base. After preparation, use upstream's documented command:

```bash
node tools/app-contribute.mjs --manifest apps/polgar-pocket/cobalt-app.json --dry-run
```

That command reads the current public Beta catalog/provenance, validates app
versions and minimums, generates pages, formats, tests, lints, builds/verifies
static ARM and creates deterministic package/catalog previews with the existing
**public test-only `[42; 32]` fixture seed**. Its download URLs are `example.invalid`.
The unsigned source/patch and fixture preview are eligible inputs for maintainer
review, not proof of stock-runtime installation.

## Signing and approval boundary

The documented [release train](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/docs/RELEASE-TRAIN.md) and [publisher workflow](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/.github/workflows/apps.yml) show the supported Store route requires a maintainer-approved merge to
Beta. Protected automation builds and signs both package and catalogs with the
actual trusted release key, uploads assets and then moves the catalog pointer.
No signing credential is needed or available to a contributor. A local fixture
signature must not be enrolled on a reader. No documented stock-runtime private
owner-key/sideload route was found. Stable promotion is separate.

The proposed public name is **Pocket Chess Studies**, with **Pocket Chess** as
the short label. The owner should confirm the name before public submission.
The existing `polgar-pocket` runtime/Store ID, collection ID and progress format
are preserved. The packet contains twenty CC0 Lichess demonstration puzzles.
See the [naming and licensing review](../submission/NAMING-LICENSE-PRIVACY.md).

Opening the owner's app-repository PR is authorized by this task. Opening the
public upstream PR, creating/pushing a fork, triggering Beta Store publication,
merging either PR or installing on the Kobo are not authorized. KOReader,
NickelMenu, Zen OS, Focus Reader, firmware, SSH and startup configuration remain
untouched. The existing HARDWARE-TEST.md gates still apply after any future
trusted availability; no physical compatibility or battery measurement is claimed.
