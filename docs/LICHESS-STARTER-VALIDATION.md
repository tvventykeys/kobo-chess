# CC0 starter preview validation

## Data and reproduction

Lichess listed **6,193,530 puzzles** on the 2026-11-11 export. Its download host
returned proxy HTTP 403 in this environment. The 20,000-puzzle test therefore
uses the puzzle objects from a pinned public derivative of the Lichess export.
This measures a real 20,000-puzzle collection and does not verify the current
official snapshot byte for byte. [Official listing and CC0 declaration](https://database.lichess.org/#puzzles) · [Pinned public derivative](https://github.com/mcognetta/lichess-combined-puzzle-game-db/tree/a0a8c3c2ce3cf56923072a44fa2f2e0a8d8f2935)

| Input or output | Recorded value |
| --- | --- |
| Public derivative file | `combined_puzzle_db_first_50k.ndjson.bz2` at the pinned commit |
| Download SHA-256 | `21c7ec0d68bbccebccd6c85b2e307cac8eec1d7dec0c72043fd26266b816ad6c` |
| Derived CSV | 51,200 puzzle rows, UTF-8 and LF line endings |
| Derived CSV SHA-256 | `d60a6ec0202c169e8b17450f117f97946648a6a4551f8286bca779e5980997aa` |
| Valid output SHA-256 | `975d5ed5b495147fcb702d92e07fe6d2a7751482554072521ac6d92c60dc55ac` |
| Indexed records | 20,000 distinct IDs, 5,000 per rating band, zero demo ID overlaps |
| Indexed bytes | 3,149,487 total, 240,000 entry bytes |
| Compressed size probe | 1,175,937 bytes with `zstd -3` over the whole file |
| Example composed filter | Rating under 1200 and theme `mate` matched 2,220 puzzles |

CSV extraction used only `PuzzleId,FEN,Moves,Rating,Themes`, in that column
order. The mirror's game objects and annotations were excluded. The archive,
CSV, generated index, screenshots and temporary progress data stay outside Git.

```python
import bz2
import csv
import json

with bz2.open("combined_puzzle_db_first_50k.ndjson.bz2", "rt", encoding="utf-8") as source:
    with open("public-mirror.csv", "w", encoding="utf-8", newline="") as target:
        writer = csv.DictWriter(
            target, fieldnames=["PuzzleId", "FEN", "Moves", "Rating", "Themes"],
            lineterminator="\n", extrasaction="ignore",
        )
        writer.writeheader()
        for line in source:
            writer.writerow(json.loads(line)["puzzle"])
```

Run the [import commands](PUZZLE-PACKS.md) on `public-mirror.csv`. The
independent imports emitted identical bytes. All imported setup and solution
moves replayed in shakmaty. This validates legality and the source's move
sequence, while mate strength and alternate winning lines remain Lichess's
analysis.

## Host benchmarks

Linux x86-64, Intel Xeon Platinum 8573C, three logical CPUs available, Rust
1.85.1 with release optimization. Measurements used a warm build while other
checks were running. No numbers below are Kobo ARM timings.

| Operation | Observed |
| --- | --- |
| Validate complete existing index | 0.635 s median over 7 processes, range 0.506–1.364 s |
| Import CSV and independently validate the result | 2.267 s median over 3 processes, range 1.751–3.714 s |
| Peak RSS observed across benchmark children | 12,160 KiB |
| Starter index extrapolation for 6.2M at measured mean | Approximately 976 MB (931 MiB), excluding update staging |

Validation timing uses the host release executable with
`--validate-lichess-index starter.idx`. Import timing uses
`scripts/import-lichess-starter.py public-mirror.csv starter.idx --binary RELEASE_BINARY`.
Elapsed time is measured around the entire process, including startup. RSS is
Linux `getrusage(RUSAGE_CHILDREN).ru_maxrss`. In the app, the two result
vectors together use up to 160,000 bytes of element storage on ARM, while
embedded bytes are immutable and read directly.

The static unstripped ARMv7 hard-float ELF was 3,142,144 bytes for the demo
and 6,293,464 bytes for the preview. Both had no ELF interpreter. The preview
adds 3,151,320 bytes, close to the index size and below the Cobalt 64 MiB
executable limit.

| Preview simulator run | App RSS after interaction | Time to initial Home |
| --- | ---: | ---: |
| Clara BW, normal | 12,404 KiB | 20.761 s |
| Clara BW, 170% | 12,344 KiB | 8.051 s |
| Libra Colour, normal | 12,396 KiB | 2.685 s |
| Libra Colour, 170% | 12,340 KiB | 7.352 s |

The simulator's ready timer starts before the CLI launches. It includes Cargo
work, simulator startup, and concurrent CPU load. The first Clara run included
a rebuild. These timings include more than app startup.

## Verification

- Standalone demo and 20,000-puzzle preview passed locked host and static ARM
  builds, formatting, strict all-target/all-feature Clippy, and 60 Rust tests
  each with zero failed or ignored. The legacy demo CSV reproduces the
  checked-in 20-record pack byte for byte. Cargo.lock is unchanged.
- The real-input demo suites completed 84 solves and eight progress restarts
  across Clara BW and Libra Colour at normal and 170% text. They include
  physical-page callback simulation, wrong and legal move handling, partial
  attempt restoration and clean completion. The four storage recovery suites
  passed, with Cobalt's five-ink diagnostic emitted during recovery. One
  initial run lost the driver's follow-up connection after a successful exit.
  The test now posts the exit touch to the visible control and independently
  checks the simulator's exit code and persisted state.
- The starter simulator matched the combined rating/theme query, opened indexed
  results, paginated them, checked cancel and confirm for the page-key
  transition, revealed the solution, used Cobalt Back, and restored the same
  puzzle by ID on restart in all four profile/scale combinations. It began
  with a real demo snapshot from the default executable and verified that the
  demo bytes stayed identical. A separate Clara run preserved a deliberately
  corrupted preview snapshot and kept the demo readable. The initial reveal
  route exposed the existing named-action mismatch with Cobalt's reserved
  Back ID; the final route passed with the corrected callback.
- The official contributor dry-run passed using the demo configuration on the
  isolated Cobalt Beta candidate `785b0327d29f4bd58c778013aeb4dc5138d413d9`
  based on `1cc5b6412c4229b85e9960dd956c3d26c4ffd66e`. Its 60 locked Rust
  tests, formatting, strict Clippy, ARM app check and fixture package preview
  passed. All 129 Node policy tests passed. Cobalt's `check-apps-sim.py
  polgar-pocket --all-routes` passed at normal and 170% on Clara BW.
- No official signing, publishing, hardware test, external pack installation,
  direct official snapshot import, unrelated Cobalt Rust workspace matrix, or
  full database import was performed.

To repeat the full starter simulator integration, build the demo and preview
as described in [Local Lichess packs](PUZZLE-PACKS.md), using separate target
directories, and run:

```bash
POCKET_DEMO_TARGET=/absolute/demo-target \
POCKET_CLI=/absolute/path/to/kobo \
CARGO_TARGET_DIR=/absolute/preview-target \
POCKET_STARTER_PACK=/absolute/path/to/starter.idx \
POCKET_EVIDENCE=/absolute/evidence-directory \
  python3 scripts/check-pocket-starter-sim.py
```

Exact raw local logs and captures from this review were kept outside Git under
`/workspace/verification/lichess-starter`. The Libra demo run's per-screen
captures are under the existing `/workspace/verification/pocket-v02` directory.
