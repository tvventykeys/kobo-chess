# Pocket pack v1

UTF-8 JSON Lines: one header followed by one puzzle object per line. No comments,
blank records or unknown fields. `packs/lichess-20.jsonl` is the reference.

Header fields: `schema_version` (1), `collection_id`, `title`, `source`, `license`.
Puzzle fields: `id`, `fen`, `side_to_move` (`white` or `black`), `solution` (array
of standard UCI moves), `explanation`, optional `difficulty`, `tags`, `source_url`.
The starting FEN is **after** any setup move; solution index zero belongs to the
solver. Lines alternate solver/opponent and end after a solver move. Castling
uses king destinations (`e1g1`); promotions include the piece (`a7a8n`).

Loading verifies standard-chess FEN validity, side, every move through shakmaty,
odd nonzero line length, unique IDs, version, fields and bounds. v0.2 compares
against the supplied ordered line, so another legal winning move is reported
as outside this solution. It does not claim engine verification of alternate wins.

Bounds: 4 MiB per pack, 8 KiB per line, 4,096 puzzles, 63 plies, 48 title
characters and 180 explanation characters. Bounds reject rather than truncate.
The initial pack is about 12 KiB. A byte-offset index keeps one decoded puzzle
in memory at a time; loading validates each record sequentially. Raw bytes of a
selected pack remain resident. Future library browsing should index files on
the runtime shelf instead of increasing this bound or importing a whole database.

Host commands, after `source scripts/env.sh` and a locked host build:

```bash
"$CARGO_TARGET_DIR/debug/polgar-pocket" --validate-pack packs/lichess-20.jsonl
python3 scripts/import-lichess-csv.py packs/lichess-20-source.csv /workspace/work/reproduced.jsonl
"$CARGO_TARGET_DIR/debug/polgar-pocket" --import-lichess /private/selected-rows.json /private/study.jsonl
"$CARGO_TARGET_DIR/debug/polgar-pocket" --import-trainer /private/book.json /private/selected.jsonl 1,2,3
```

Lichess imports accept the inspected trainer export shape: an array containing
`puzzleId`, `fen`, `moves` (space-separated UCI including setup), `rating`,
`themes` and optional `gameUrl`. Setup is applied exactly once. Collection ID
is `lichess-tactics`; original puzzle IDs remain unchanged. CSV conversion
streams only until the requested count, default 20 at ratings 800–1500.

The private trainer importer accepts `bookId`, `bookTitle`, `problems`, and
problem fields `problemid`, `fen`, `moves` (semicolon-separated `c3-c8` syntax),
optional `explanation` and `difficulty`. It preserves IDs and FEN, normalizes
moves to UCI, and additionally requires the line to end in checkmate, matching
the inspected interactive trainer. IDs are stored as decimal strings; a future
sync adapter must convert book IDs back to numbers. Missing, repeated, illegal
or self-assessed problems fail the import. No answer reconstruction is performed.

Imported private packs belong outside the repository and public artifacts. Book
ownership does not grant redistribution permission. The v0.2 application bundles
one selected pack at build time; external shelf installation and a multi-pack
catalog are future work. The collection screen lists only that functioning pack.
