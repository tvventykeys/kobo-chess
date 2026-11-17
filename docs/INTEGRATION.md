# Future trainer integration (research only)

The private repository was accessible and inspected locally at revision
`bfbc180b93b9d9d998a9ae055705f59775258551`. No production service, database,
Cloudflare settings, registry or sync endpoint was modified or contacted.
No private source/data is copied into this repository. These are interoperability
observations, not a claim of already working synchronization.

## What exists

* `book-registry.js`: stable IDs including `polgar-4462`, `nunn-1001`,
  `lichess-tactics`, `woodpecker-method`, `nunn-endgame-444` and others; entries
  name assets and optional local data. Availability/playability differs by book.
* `board.js`, `puzzle-rules.js`: interactive book records use numeric
  `problemid`, starting `fen`, and semicolon-separated source/destination moves
  (`c3-c8`, optional promotion suffix). Actual exported book objects contain
  `bookId`, `bookTitle`, `problems` and provenance metadata. Some collections
  are self-assessed or reconstructed and must not be silently converted into
  verified tactical exercises. The importer supports only legal published
  mating lines, preserving IDs/FEN/explanations when present.
* `tactics-catalog.mjs`: separate, read-only `lichess-tactics.sqlite`; tables
  `meta` and `puzzles`, schema `lichess-tactics-1`, rating band 800–1500.
  Selected exports carry `puzzleId`, `fen`, `moves`, `rating`, themes,
  opening tags and game URL. `tactics-model.mjs` applies the first move as
  setup; the remaining odd-length line starts and ends with the solver.
* `progress-event.js`: append-only events have `id`, `ts`, `kind`, `deviceId`,
  optional `bookId`, and `puzzleId`. Missing `bookId` means legacy
  `polgar-4462`. Book IDs are numeric; Lichess IDs are alphanumeric strings.
  Attempts carry `result` (`clean`, `assisted`, `skipped`), `wrong`, `hints`,
  optional duration/rating/themes. Favorites, cursors and reading use distinct
  kinds. A revealed solution maps to skipped, never clean.
* `db.js`: IndexedDB history plus pending outbox and sync checkpoint updated
  together. `progress-store.mjs`: SQLite events keyed by unique ID, WAL and
  full synchronous transactions, paginated sequence cursor, epoch, backups.
  `sync.js`: POST exchange of pending events plus `after`/`epoch`, validating
  returned cursor and `more`; bounded batches and replay after epoch changes.
  The browser uses periodic polling. Pocket should use explicit sync instead.

## First safe pack path

An owner-selected local export produces a small pack, validates every FEN and
line, retains collection/puzzle IDs and source/license, and excludes private
book data from public commits. Both Lichess and the inspected book schema have
working host converters in `src/import.rs`. A local 20-problem book subset was
successfully converted and validated outside the repository. It is not bundled.
The public app uses only CC0 Lichess data with the existing collection ID.

Next add an immutable shelf catalog: one pack identity/version/hash and a byte
index per installed subset. Download into a temporary slot; validate fully;
atomically publish the catalog pointer. Keep the previous valid pack after
interruption. Select among genuinely installed packs. Do not load a whole book
library or a multi-million-row database onto the reader.

## Progress schema migration before sync

v1 stores stable collection/puzzle IDs, monotonic local revision, cumulative
counters, best completion, reveal history and the active attempt. It has no
per-attempt UUID journal, so **do not synthesize individual historic events
from these aggregates**. Preserve v1 as a local baseline during a v2 migration.

A v2 needs a durable installation UUID, monotonic event sequence and append-only
attempt/cursor outbox. Allocate event ID and payload once, atomically with local
progress. Keep the same ID on retry. Use the trainer's expected numeric/string
puzzle types, timestamp units (milliseconds), and result semantics; omit events
requiring a reliable time if the device clock is unknown. Map revealed attempts
to skipped; assisted attempts must have actually played the required line.

On an explicit Wi-Fi sync: snapshot a bounded outbox batch, exchange it with
`after`/`epoch`, validate response, then atomically merge remote IDs, advance the
checkpoint and acknowledge only that exact batch. Never clear newer pending
items. Keep acknowledged IDs to deduplicate. An epoch change restarts download
at zero while retaining local history. Interrupted requests resend the same IDs.
Download with `more` until caught up, with a finite session budget and visible
retry. No reset/overwrite endpoint and no automatic migration of production data.

Stockfish/Fishy analysis, Lichess accounts/games and spaced repetition should be
separate optional services consuming a position or immutable progress events.
They must not add work to idle puzzle sessions. None is implemented in v0.2.
