# Pocket Chess Studies roadmap

The public library uses the official [CC0 Lichess puzzle database](https://database.lichess.org/#puzzles).
Keep the game-derived puzzles and the Lichess IDs. Copyrighted book studies,
private trainer material and book import/sync are outside the public roadmap.

| Stage | Delivery and acceptance |
| --- | --- |
| Pocket Twenty | Default offline build. The 20 demo records, numbering, app ID and existing progress key remain available. |
| 20,000-puzzle starter preview | Generate an indexed collection on a host from a local Lichess CSV. Opt in at build time. Browse by rating and theme, open a result, and navigate matching puzzles by touch or page key. Test both Kobo simulator profiles, progress recovery, binary size and startup memory. |
| Starter release | Verify and pin a direct official CSV snapshot, agree on the distribution route, and design a durable progress migration and capacity plan. Review startup, saves, suspend/exit and memory on owner-approved hardware. Any Cobalt contribution or publishing decision is separate. |
| Full database research | Treat the roughly 6.2M-puzzle scale as a planning input. Measure a compact partitioned format with bounded reads, filters and transactional updates. Evaluate storage reservations, offline delivery, ARM read/decode latency and interrupted imports before proposing a size or release date. |

The preview picks 5,000 puzzles in each of four rating bands from the first
eligible rows of the input. This gives useful coverage across difficulty. It is
an ordered starter sample, so there is no claim of statistically representative
theme distribution or fresh engine analysis. All theme tags are preserved, and
five commonly used tags have on-reader filters. The browser displays three
results at a time. Picked results navigate within their filter until restart.
After restart the attempt is restored and navigation uses the unfiltered
starter order. The rating/theme choice is intentionally session-local.

## Cobalt limits verified for this milestone

References are pinned to Cobalt Beta
`1cc5b6412c4229b85e9960dd956c3d26c4ffd66e` (0.3.37). The standalone SDK is
pinned to `5ac30a46efce23d7fa0d1cb12a499c9b57b816b0` (0.3.37).

- A signed app package carries one executable, a canonical manifest and a
  signature. The limits are 64 MiB for the executable and 128 MiB for the
  package. A build-time data section can fit the starter without relying on
  extra package files. [Bundle format](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-app-store/src/bundle.rs) · [Size limits](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-app-store/src/lib.rs)
- Durable store limits are 256 KiB per value and 256 keys per application.
  Preview history uses its own atomic key and refuses a new history entry
  after 1,000 distinct puzzles. It can still save practice for existing
  entries. First launch copies the existing demo snapshot in memory, and
  preview saves leave the original demo key byte-identical. The demo and
  preview histories diverge after that first load. Use the same indexed pack
  on subsequent preview builds. A new snapshot or a production-grade larger
  history requires an explicit ID-based migration and sharding decision.
  [Protocol bounds](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobo-protocol/src/lib.rs)
- The SDK shelf is app-private, with 256 KiB transfers, 256 MiB total, 4,096
  blobs and a 64 MiB free-space floor. The reserved `shared-files` capability
  has no SDK call in this revision. The shelf can serve bounded reads of data
  already delivered to an app. A supported external puzzle pack installer,
  USB picker, or offline companion ingress would require separate evidence.
  Host CSV import uses no device, network capability, filesystem workaround or
  changes to Cobalt's signing model. [SDK APIs](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/SDK.md)
- Cobalt keeps up to four hosted apps and uses least-recently-used eviction.
  Its runtime source explicitly describes the limit as a count of hosted
  apps. Treat host RSS and startup results as sizing evidence. The retained
  reader memory and latency budget still needs physical measurement.
  [Hosted app limit](https://github.com/BandarLabs/Cobalt/blob/1cc5b6412c4229b85e9960dd956c3d26c4ffd66e/crates/kobod/src/device.rs#L2115)

The measured starter index is 3,149,487 bytes (about 3.00 MiB, of which 240,000
bytes are fixed entries). At the same average size, 6.2M rows would occupy
about 976 MB (931 MiB), before allowing for staging, migration or progress.
The 12-byte entries alone would take 74.4 MB at that scale. A single zstd
level-3 stream was 1,175,937 bytes for the starter. Extrapolating that ratio
gives about 365 MB before random-access block overhead. The current format
caps itself at 20,000 puzzles and 8 MiB. Full coverage would need smaller
encoded blocks and a different storage/delivery strategy, even if compressing
blocks proved sufficiently fast.

See [pack format and import](PUZZLE-PACKS.md) and
[verification and measurements](LICHESS-STARTER-VALIDATION.md).
