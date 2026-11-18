# Local Lichess packs

Pocket Twenty is the default build. Its v1 JSON Lines pack contains one header
and twenty records in their established order. Each record starts after the
opponent's setup move. The app keeps a byte-offset index, validates all moves
using shakmaty, and accepts the supplied solution line. The existing demo pack
and its persisted schema are unchanged.

## Build the starter preview

Download a CC0 puzzle snapshot from the [Lichess open database](https://database.lichess.org/#puzzles)
on a host, then work from the local decompressed CSV. Keep the input stable
during import and keep generated files outside Git.

```bash
source scripts/env.sh
cargo +1.85.1 build --locked
zstd -d /path/to/lichess_db_puzzle.csv.zst -o /path/to/lichess_db_puzzle.csv
python3 scripts/import-lichess-starter.py /path/to/lichess_db_puzzle.csv /path/to/starter.idx
"$CARGO_TARGET_DIR/debug/polgar-pocket" --validate-lichess-index /path/to/starter.idx
POCKET_STARTER_PACK=/path/to/starter.idx cargo +1.85.1 build --locked
POCKET_STARTER_PACK=/path/to/starter.idx KOBO_SIM_PROFILE=libra-colour-390 \
  KOBO_SIM_OFFLINE=1 "$CARGO_TARGET_DIR/debug/kobo" dev 127.0.0.1:8787
```

The default importer selects the first 20,000 eligible rows in source order.
Use `--count N` for a smaller test pack. Ratings 80 and 1500 are both included,
with no solver-colour restriction. Demo IDs are excluded. The output sorts
selected records by ASCII puzzle ID. The importer reads the CSV once and
stops at the requested count. A malformed selected row, illegal move, duplicate
selected ID or shortage fails the import. Final replacement is atomic on the
output filesystem.

The provenance header records the official source URL, CC0 identifier, policy
and SHA-256 of the original CSV bytes consumed through the selected row that
completed the quota. A complete source snapshot digest can be kept separately
if it is already available. Obtain input from Lichess or a traceable public
source. The build flag embeds the reviewed file and the app rejects indexed
rows outside 80–1500. Build without the variable for the demo and for default
submission preparation. See the [roadmap](ROADMAP.md) for delivery constraints.

Local demo reproduction still uses
`python3 scripts/import-lichess-csv.py packs/lichess-20-source.csv /path/to/demo.jsonl`.
The bounded legacy `--import-lichess` JSON converter only supports Lichess rows.
The public application accepts no book-study or private trainer import format.

## PCSIDX01 storage format

All integers are little-endian. The 16-byte prefix contains an 8-byte magic
(`PCSIDX01`), u32 provenance JSON length and u32 record count. Provenance JSON
is followed by sorted 12-byte entries. An entry contains the original
five-byte ASCII ID, u32 exclusive end offset into the payload, u16 rating and
u8 theme bitset. Bits 0–4 encode mate, fork, pin, endgame and middlegame.
A payload record is a compact JSON array of original ID, pre-setup FEN, full
UCI sequence including setup, rating and all theme tags. The original games
and unused CSV columns are excluded.

At validation, IDs, offsets, sizes, ratings and theme bits must match the
payload, and every setup move and solver/opponent move must replay legally
using the app's standard chess rules. Setup is applied once to construct the
solver's position. Solutions retain Lichess UCI, including underpromotion and
standard king-destination castling. The preview validates all records on
startup. It retains the embedded bytes and uses index-only scans for filtering.
Only the chosen record is decoded into a study session. This format is capped
at 8 MiB and 20,000 puzzles. See the [roadmap](ROADMAP.md) for progress and
installation constraints.
