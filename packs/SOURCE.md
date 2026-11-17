# Starter pack provenance

The public starter collection is **Lichess · Pocket Twenty**, collection ID
`lichess-tactics` (matching the existing trainer registry). It is not a Polgár or
Nunn book extract. Lichess publishes its puzzle database under CC0 1.0:
https://database.lichess.org/#puzzles . The official source/license declaration
was inspected in `lichess-org/database`, commit
`400f124a43200857be9f78ca781ebbbba01b4ced`, `web/index.html.tpl`.

The database download host returned a proxy HTTP 403 in this environment. The
CSV records were retrieved unchanged from the public mirror
https://github.com/Ali-Raza764/chess_puzzles_api/blob/94e65c36af8358a04d3c835984435845fc638fe3/puzzles_1000.csv .
Only CC0 database rows are reused; no mirror application code is included.

Selection: first 20 rows in source order with rating 800–1500 inclusive.
`lichess-20-source.csv` preserves all original columns (line endings normalized to LF). SHA-256 of that subset:
`38293bacf5d71e62167225507d080a4a21c67e171b8e3282097d9c0cd86d1c5c`.
The pack applies the first UCI move as setup, then keeps the remaining line,
original IDs, ratings, themes and puzzle URLs. Explanations are theme labels,
not invented book annotations. No puzzle position or solution was composed.

All 20 complete source sequences were independently replayed using chess.js;
the normalized pack and every solution are replayed by shakmaty tests and solved
through simulator touch input. This verifies legality, setup convention and
session execution, relying on Lichess for tactical authorship/analysis; no fresh
Stockfish best-move or uniqueness analysis is claimed.

Reproduce with `scripts/import-lichess-csv.py`; JSON records compare identically
to the checked-in pack. All provenance is public. Private trainer data was
inspected only outside this repository, and no private book records are shipped.
