# Naming, licensing and privacy review

## Name decision before publication

The draft proposes **Pocket Chess Studies** (short label **Pocket Chess**) at
version 0.2.0. Confirm the public name before upstream submission or publication.
The previous display name could suggest an association with László Polgár or
his published collections. The CC0 puzzle license does not grant naming rights.
No exhaustive trademark search or legal clearance is claimed.

The internal app ID remains `polgar-pocket` and the collection ID remains
`lichess-tactics`. Both identities and the progress format are preserved.

## Redistributed material

| Material | Actual source/license | Submission treatment |
| --- | --- | --- |
| App source | This public repository, AGPL-3.0-only | Complete Rust source and LICENSE included |
| SDK, renderer, vectors and fonts | Cobalt Beta workspace, its LICENSE/THIRD-PARTY and asset notices | Linked as workspace dependencies; original renderer/artwork unchanged |
| Chess rules | shakmaty 0.27.3, GPL-3.0-or-later | Existing workspace dependency; compatible GPLv3/AGPLv3 combination |
| Serialization | serde 1.0.229, serde_json 1.0.151, MIT OR Apache-2.0 | Existing workspace versions; no upstream dependency downgrade |
| Twenty puzzle records | Lichess public database, CC0-1.0 | Exact public source rows, IDs, FENs, moves and provenance retained |
| Store mark | Cobalt `book` glyph | Runtime-rendered AGPL source; no new raster/SVG icon |
| Store screenshots | Actual Cobalt simulator output | Unedited PNGs with capture sidecars, profiles, scales, hashes and scenario captions |

The pack is not the Polgár, Nunn or Woodpecker library. No chess-book scans,
excerpts, private puzzle exports, Cburnett SVGs or commercial logos are copied.
No new engine-authored or invented demonstration solutions are included. The
source file list is intentionally allowlisted; local fixture databases and raw
simulator progress are not part of the proposed contribution. Upstream's root
THIRD-PARTY.md receives an app-specific CC0 notice as required by its guide.

## Privacy boundary

The submission contains source, public CC0 records, documentation and reviewed
screenshots only. All screenshots come from fresh temporary simulator state;
the displayed completion is a test, not personal study history. No production
trainer, Cloudflare, Fishy Home or Lichess account is accessed. No reader is
connected, scanned or installed. No credential, token, personal database,
`.env`, signing seed, app progress record or system backup is included.

Cobalt's deliberately public `[42; 32]` fixture seed remains in upstream's
existing tools; its local preview packages/catalogs are not release credentials
or installable stock-runtime artifacts. The reviewed patch does not alter the
release key, seed handling, workflow, network policy or other applications.

Code and data have no identified redistribution-license blocker under these
licenses. Naming approval is unresolved. This report is an evidence inventory,
not a warranty of trademark clearance or a claim of hardware compatibility.
