# Development and architecture

Read AGENTS.md and the environment section of README first. Use the restored
setup without reinstalling it. The separate app manifest references the pinned
SDK at `../deps/Cobalt`; upstream remains read-only and is not a workspace member.
`scripts/setup-cloud.sh` is the existing recovery path for a missing toolchain.

```bash
cd /workspace/kobo-chess
source scripts/env.sh
bash scripts/verify-pocket.sh
```

The verification script uses locked Cargo resolution and Rust 1.85.1, formats,
lints all targets with warnings denied, tests, builds host and ARM release,
validates the pack, checks static ELF identity and drives the real Cobalt
simulator at normal/170% text with Libra Colour. Build outputs and simulator
state/screenshots stay outside Git under `/workspace/build` and
`/workspace/verification`. No device or account is required. The SDK CLI's
`dev` internally calls Cargo without `--locked`; run the locked host build first
and keep Cargo.lock unchanged. App build/check commands always use `--locked`.

Modules:

* `main.rs`: lifecycle, declarative screens, touch routing, promotion/leave
  dialogs, page-button callback, acknowledgement-aware persistence.
* `board.rs`: oriented 64-square actions and SDK chess glyphs; selected source,
  legal dots, capture outlines and checked king. No custom pixel drawing.
* `rules.rs`: standard shakmaty positions and legal UCI moves, including special
  moves. No substitute movement engine and no runtime chess search.
* `pack.rs`, `import.rs`: bounded versioned packs, lazy puzzle decoding and
  explicit local converters. No production database integration.
* `session.rs`: ordered player solution, deterministic immediate replies,
  wrong-move retention, source-only hint, retry and reveal.
* `progress.rs`: aggregate counters and active-session snapshot, version
  validation and reconstruction. `tests.rs`: focused engine/application tests.

Selection and legal destinations never write storage. Real progress events and
navigation write one versioned snapshot through `Context::store()`. The SDK
runtime performs atomic replacement, including power-loss handling; application
writes serialize and coalesce while an acknowledgement is pending. No successful
save is claimed before the result. Storage failures stay visible with explicit
retry. Corrupt/incompatible values remain untouched; study becomes read-only.
The normal suspend gate waits for pending writes; resume redraws saved in-memory
state. Process restart reconstructs the exact played prefix and assistance flags.
Clock timestamps are recorded only for plausible system dates (2020 onward);
there is no trusted-time or sync claim for an incorrectly set reader clock.

Battery measures implemented: no networking capabilities or tasks, Wi-Fi hold,
clock timer, event polling, animation, continuous engine, keep-awake request or
background worker. The SDK blocks on runtime input while idle. Declarative
screens use the runtime's diff/targeted e-ink refresh policy; the app never forces
full refreshes per tap. Small selected packs and one decoded puzzle bound work.
Storage activity follows meaningful actions, not square selection or idle time.
No battery-life measurement, ghosting accuracy or hardware refresh timing is
claimed from the simulator.

At 170% text the renderer reserves controls before sizing the root-level board.
Keep it at screen root. Short Prev/Home/Next labels retain comfortable targets.
The Coords control labels empty squares on request; occupied squares retain their
absolute action names. Conventional outer file/rank rails are not exposed by this
SDK, and no unsupported drawing API is assumed. A board flip preserves absolute
square actions. Hint highlights only its source until the owner selects a piece.

The simulator suite exercises collection/settings screens, wrong legal moves,
hint/retry/reveal, navigation confirmation, forward/back page events, every
published line including promotion, all completion counters, durable process
restart and a partly played assisted attempt after restart. `clean` checks layout
errors on each important screen. Ideal screenshots inspect readability; they
are not physical panel captures. Physical buttons and suspend/resume remain
hardware acceptance items even though their SDK callbacks are tested.
