# Pocket Chess Studies roadmap

The public library uses [CC0 Lichess puzzles](https://database.lichess.org/#puzzles)
rated **80–1500 inclusive**, for White and Black. The collection target is
approximately **1.95 million** puzzles. The owner reports 1,947,528 rows in
their previously filtered snapshot. Counts may vary by snapshot. Copyrighted
book studies and private trainer data are outside the public library.

| Stage | Delivery and acceptance |
| --- | --- |
| Pocket Twenty | Default offline build. Original records, ordering and progress key stay available. |
| 20,000-puzzle starter | Opt-in build from a local public CSV. Retain original IDs, FEN and UCI, ratings, themes and CC0 attribution. Validate each setup and solution with shakmaty. Expose combined rating/theme browsing and keep saves offline. |
| Full 80–1500 collection | Validate a published, pinned source or a reviewed CC0-only extraction of the owner's existing filtered rows. Build independent bounded-read shards. Confirm supported delivery, validate quotas and replacement, and measure actual ARM latency before release. |

The starter takes the first eligible rows in source order, excludes the demo
IDs, and sorts by ID. There is no solver-colour filter during import. Both
colours were present in the measured starter. All theme tags are retained,
and five commonly used tags have on-reader filters. The browser displays
three results at a time. Picked results navigate within the filter until
restart. Restart restores the attempt and uses unfiltered starter navigation.

## Private import reuse

The public importer already uses the Lichess CSV column contract. It can
consume a separately reviewed host export of the existing filtered data with
only `PuzzleId,FEN,Moves,Rating,Themes`. Prefer the retained original
Lichess rows or an export routine that reproduces them. Request the source
snapshot identity and checksum or equivalent trusted provenance. Verify
inclusive rating bounds, unique IDs, both solver sides, and every setup and
solution. Confirm FEN is pre-setup and Moves still starts with the setup
move. A normalized post-setup table cannot be supplied to that importer
without recovering the original representation.

Reuse of the **procedure** needs no database connection. Reuse of the **rows**
needs an owner-provided or explicitly authorized, independently reviewed
CC0-only export. Generate it beside the data under the owner's control,
using explicit column projection and a Lichess-only source relation. Exclude
book or trainer rows, notes, private annotations, accounts, game history,
progress, credentials, server metadata and database dumps. Review the result
and manifest before copying data or publishing anything. No private database,
code, export or credentials were accessed for this work.

If provenance or source semantics cannot be recovered, use the official source
and stream it once on the host. Reject rows outside the range before chess
validation or storage. Keep only eligible rows in bounded staging buckets.
A prefiltered export avoids reading the unfiltered stream. Neither process
requires a full uncompressed CSV on disk or reader.

## Proposed large collection format

- **Shard and block layout.** Sort by original five-byte ID. Start with
  up to 64 independently compressed blocks per shard. A block contains up
  to 256 records and at most 65,535 uncompressed bytes. A 24-byte directory
  entry stores first and last ID, shard-local byte offset, compressed and
  raw lengths, and record count. Keep a small top-level manifest with version,
  source and license, rating bounds, shard ID ranges, counts and digests.
  An import uses bounded host staging buckets or external sort. Validate
  before committing a build and reject duplicate IDs across shards.
- **Lookups and filters.** Locate the candidate block by ID ranges. Read
  only its compressed bytes, expand into a fixed-size buffer, and decode
  the selected row. Keep a separate paged 3-byte-per-record facet table
  with rating and bit flags for the five UI themes and solver colour,
  anchored by ordinal to the directory. Stream facet pages on demand and
  retain the selected page or a bounded cursor, rather than building a
  resident match vector for the whole collection. Keep all source tags
  in each block. Extra theme filters may scan blocks or use a later
  measured posting-list design.
- **Publication and recovery.** Assemble the next generation on the host,
  record file digests and size, and activate only after a complete verified
  import through a supported path. Keep the prior usable generation until
  activation. Budget for staged data, previous data, progress and the
  runtime free-space reserve. Reject an update that exceeds quota. If two
  complete generations and progress do not fit, measure a resumable
  shard-reuse update before enabling in-place updates.
- **Progress.** Anchor records to source ID, never a row number or shard.
  Use a separate atomic active-session key and sparse paged histories.
  Design the commit protocol so an interrupted attempt update cannot erase
  prior history or be counted twice. Preserve schema versions and retain
  progress for temporarily absent IDs when replacing source snapshots.
  Budget 32 bytes per record as a sizing example (about 59 MiB if every ID
  has history), plus framing and recovery overhead. Exact progress encoding,
  migration and crash tests are release work.

The starter is implemented today. The large format is a design with a
host block benchmark, not a reader implementation. The measured projected
index, payload and directory total is about **98 MiB** for the filtered
collection. A second generation plus dense progress could exhaust the
256 MiB shelf quota after overhead. See [measurements](LICHESS-STARTER-VALIDATION.md).

## Cobalt constraints

Cobalt Beta inspected at `1cc5b6412c4229b85e9960dd956c3d26c4ffd66e` (0.3.37).

- Signed bundles contain one executable. Limits are 64 MiB for the executable
  and 128 MiB for the package. The starter can embed its small index. The
  proposed full filtered collection exceeds the executable limit.
  [Store limits](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-app-store/src/lib.rs)
- Static limits are 256 KiB per store value and 256 keys. The app-private shelf
  supports 256 KiB chunks and 256 MiB total, capped at 4,096 blobs with a 64 MiB
  free-space reserve. The reserved user-visible `shared-files` capability
  has no SDK call. We found no supported external pack installation route
  for this app. Offline use after legitimate delivery is feasible in design.
  [Protocol bounds](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-protocol/src/lib.rs) · [SDK](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/SDK.md)
- The starter uses a separate, rating-scoped progress key and reads the
  existing demo snapshot on first launch. The older unrestricted preview
  key is preserved on disk. The revised preview does not merge it, since
  puzzles may be absent or have moved. Starter history caps at 1,000 distinct
  IDs per snapshot, with no history eviction. Keep the same starter file
  across builds until an ID-based migration is implemented.
- Cold reads, resume, battery behaviour and the physical memory budget
  need an authorized hardware test. The app and docs ship no data beyond
  the existing Pocket Twenty by default.
