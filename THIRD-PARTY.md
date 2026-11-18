# Licensing and attribution

Pocket Chess Studies is AGPL-3.0-only; see LICENSE. Cobalt requires this license
for linked apps (no linking exception).

* Cobalt 0.3.37, BandarLabs and contributors, AGPL-3.0-only:
  https://github.com/BandarLabs/Cobalt/tree/5ac30a46efce23d7fa0d1cb12a499c9b57b816b0 .
  The app uses its SDK, renderer, chess glyphs, font/layout, storage and simulator.
  Hello informed lifecycle handling; Lichess informed the small board-cell/glyph
  mapping and shakmaty integration. The Lichess app was not copied as a whole.
  Upstream and its assets remain unmodified. See Cobalt THIRD-PARTY.md for its
  fonts, glyphs, vendored libraries and transitive notices.
* shakmaty 0.27.3, Niklas Fiekas and contributors, GPL-3.0-or-later:
  https://github.com/niklasf/shakmaty . Standard legal chess and FEN/UCI validation.
* serde and serde_json, MIT OR Apache-2.0: https://serde.rs/ and
  https://github.com/serde-rs/json . Serialization only.
* Lichess puzzle records, CC0-1.0: https://database.lichess.org/#puzzles .
  Exact subset and mirror provenance: packs/SOURCE.md.

Cargo.lock pins transitive versions. `cargo metadata --locked` lists each
crate's license and source. Runtime artwork is supplied by the linked SDK;
no private trainer piece SVGs, logos or book pages are copied into this app.
Keep the application source, this notice, upstream notices and dependency license
texts available alongside any redistributed binary. No warranty is provided.
