# kobo-chess agent instructions

This is an independent native Cobalt chess application repository targeting the
Kobo Libra Colour (N428 / code 390). Polgár Trainer Pocket is implemented in
`src/`; the bundled CC0 pack is in `packs/`. Run `bash scripts/verify-pocket.sh` for application validation.
`scripts/verify-cloud.sh` validates the preserved environment and official examples.
Keep this repository separate from BandarLabs/Cobalt. Do not add it to the
upstream workspace or copy upstream code into this repository as setup work.

## Environment and verification

- Use the existing checkout at `/workspace/kobo-chess`. Each cloud task is already
  isolated; do not create Git worktrees unless the user requests one.
- Read README.md, then `source scripts/env.sh` in each shell. Rust is pinned to
  Cobalt's 1.85.1; use `cargo +1.85.1` explicitly outside the upstream checkout.
- `bash scripts/setup-cloud.sh` installs/refreshes the unprivileged Debian 13
  x86_64 environment and builds official examples. It preserves unexpected
  upstream revisions/changes by stopping rather than resetting them.
- Upstream is `/workspace/deps/Cobalt`, pinned to the revision in README.md.
  Treat it as a read-only dependency: do not edit its source, manifests or lockfile.
- Device target: `armv7-unknown-linux-musleabihf`, static musl, hard-float ARMv7.
  Use Rust's bundled rust-lld and the ARM C compiler only for C dependencies.
  Never substitute glibc device linking or Kobo system shared libraries.
- Build and test commands, simulator startup and limitations are in README.md.
  Use locked Cargo resolution. Do not disable checksum, TLS or signature checks.
- Keep caches, dependencies, simulator data, screenshots and binaries outside
  this repository. Never commit credentials. Inspect git status before/after work.
- Run checks appropriate to changes; report exact passed, failed, ignored and
  unrun outcomes. Do not describe a build or an open port as functional validation.

## Device and installation safety

- Cloud setup and simulator tests require no reader connection or credentials.
- Do not run `kobo setup`, deploy, package installation, device taps, display
  probes, firmware updates or SSH/device writes without specific user direction.
- Do not modify existing KOReader, Focus Reader or Polgár Trainer installations,
  device files, Nickel, bootloader, partitions, system libraries or startup hooks.
- Simulator colour, ghosting and timing are models, not hardware acceptance.
  Report hardware, suspend/resume and live Lichess checks as unverified until run.
- Do not request tokens for offline chess tests. Any future credentials belong
  in secure environment settings or Cobalt's documented secret flow, never chat.
