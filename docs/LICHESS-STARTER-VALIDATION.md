# Validation and sizing for Lichess puzzles rated 80–1500

## Source and reproducibility

The starter was generated from the same pinned public derivative already on
this host. Earlier access to the official download returned HTTP 403. The
revised work used the small local mirror and did not fetch, process or package
the complete Lichess database. The mirror contains 51,200 public puzzle rows
after projecting only `PuzzleId,FEN,Moves,Rating,Themes`. No game objects,
annotations or private data were used. [Official CC0 source](https://database.lichess.org/#puzzles) · [Pinned mirror](https://github.com/mcognetta/lichess-combined-puzzle-game-db/tree/a0a8c3c2ce3cf56923072a44fa2f2e0a8d8f2935)

| Evidence | Result |
| --- | --- |
| Mirror archive SHA-256 | `21c7ec0d68bbccebccd6c85b2e307cac8eec1d7dec0c72043fd26266b816ad6c` |
| Projected CSV SHA-256 | `d60a6ec0202c169e8b17450f117f97946648a6a4551f8286bca779e5980997aa` |
| CSV prefix consumed by the starter importer, SHA-256 | `250a1590d5d09f0ae85831fab4e13d8da2921343baabfb1bb7d372b3e97e2e53` |
| Indexed pack SHA-256 | `d93cb3e2dca316470ef24e020ff71d7bc1fc6d2c91d63653b52a063a2d516cce` |
| Validated starter count and solver sides | 20,000 unique IDs, 10,158 White and 9,842 Black |
| Ratings observed in this sample | 590–1500. The selector and validator allow 80–1500 inclusive. The mirror supplies no evidence of puzzles near the lower boundary. |
| Example combined filter | 10,934 puzzles under 1200, including 4,753 with the `mate` theme |
| Starter size | 3,036,524 bytes including 240,000 index entry bytes |
| Repeat import and independent legality validation | 1.61 s elapsed on the x86-64 host using the debug binary. Output byte-identical. |

To reproduce the local input, unpack the mirror file
`combined_puzzle_db_first_50k.ndjson.bz2`. Read the `puzzle` object from each
JSON line and pass those five columns in the stated order to Python
`csv.DictWriter`, with UTF-8 and LF line endings. Run the
[starter importer](PUZZLE-PACKS.md) on the resulting CSV. Every selected
setup and solution replays through shakmaty. Legality checks use the supplied
Lichess answer line.

## Large collection benchmark

```bash
python3 scripts/benchmark-lichess-blocks.py /path/to/starter-80-1500.idx
```

The benchmark groups the real 20,000 indexed rows into independent zlib
level-6 blocks and measures 1,000 seeded warm file lookups. Each lookup
searches the block directory, reads and decompresses one block, and finds
and decodes the matching row. It uses Linux x86-64 and Python's bundled
zlib. The sample does not measure native ARM, cold storage reads, facet
scan latency at full scale or official delivery.

| Metric | Measured on 20,000 |
| --- | ---: |
| Independent blocks | 79 |
| Compressed payload | 993,045 bytes |
| Directory and facet table | 1,896 + 60,000 bytes |
| Largest raw and compressed blocks | 36,789 and 12,953 bytes |
| Warm random lookup | 178 µs median, 242 µs p95 |

Applying those measured bytes per row to **1,947,528** gives approximately
**102.7 MB (98.0 MiB)**, about 7,700 blocks and 121 shards at 64 blocks per
shard. The extrapolation includes the directory and filter facets and
excludes manifest, update staging, filesystem overhead and progress. The
same rows at the measured uncompressed starter density would occupy about
296 MB (282 MiB). Actual size depends on the filtered snapshot and block
composition. Two projected compressed generations plus 32 bytes of history
for every row would consume about 255 MiB before overhead, almost the full
Cobalt shelf quota. The starter itself uses the existing uncompressed
indexed format. [Large collection design](ROADMAP.md)

## Verification

- The default and filtered preview configurations passed locked builds,
  strict Clippy and 60 Rust tests each, with zero failed or ignored. The
  static ARM binaries measured 3,142,012 bytes for the demo and 6,180,424
  bytes for the filtered preview. Demo files and the progress key are preserved.
- The filtered preview passed real-input sessions on Clara BW and Libra
  Colour at normal and 170% text. The runs exercised rating/theme filtering,
  paging, page-key confirmation, reveal, and restart. Demo snapshot bytes
  stayed unchanged. Clara BW also passed deliberately corrupt preview
  recovery with the original bytes preserved. App RSS after interaction was
  12,256–12,416 KiB. CLI-to-Home took 1.23–3.47 s, including Cargo and
  simulator startup.
- On the isolated Cobalt Beta tree, the official contributor dry-run and
  official demo simulator route both passed with the revised source.
- No private export, full filtered import, full archive implementation,
  actual ARM performance test, Kobo change, or upstream submission occurred.

Run `scripts/check-pocket-starter-sim.py` with
`POCKET_DEMO_TARGET`, `POCKET_CLI`, `CARGO_TARGET_DIR`,
`POCKET_STARTER_PACK` and `POCKET_EVIDENCE` set to the corresponding
demo target, simulator CLI, preview target, pack and external evidence
directory. Raw logs from this review are under
`/workspace/verification/lichess-starter`.
