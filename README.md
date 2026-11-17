# kobo-chess

Independent native Cobalt chess application workspace for Kobo Libra Colour
(N428, code 390). This repository currently contains development setup and
verification tooling; chess application implementation has not started.

## Verified upstream and tools

Inspected [BandarLabs/Cobalt](https://github.com/BandarLabs/Cobalt) at
`5ac30a46efce23d7fa0d1cb12a499c9b57b816b0` (workspace version 0.3.37):
`SDK.md`, `docs/DEVELOPING.md`, `docs/INSTALL.md`, `docs/DEVICES.md`,
`crates/kobo-sdk`, `crates/kobo-cli`, `crates/kobo-sim`, `apps/lichess`,
its simulator scripts, toolchain pin and Cargo lockfile.

The cloud machine is Debian 13 x86_64. The installed tools are:

| Tool | Version |
| --- | --- |
| rustc | 1.85.1 (4eb161250) |
| Cargo | 1.85.1 (d73d2caf9) |
| rustfmt | 1.8.0-stable |
| Clippy | 0.1.85 |
| ARM GCC | 14.2.0 (Debian 14.2.0-19) |
| ARM binutils | 2.44 |

Device target: `armv7-unknown-linux-musleabihf`, with static musl and hard-float
ARMv7 ABI. LLVM `rust-lld` comes from the pinned Rust toolchain. The Debian ARM
compiler builds C/assembly dependencies such as ring; it does not link the final
application against Kobo's system libraries. Host simulator builds use
`x86_64-unknown-linux-gnu`; no SDL, display server or desktop GUI is required.

## Reproduce the cloud environment

```bash
cd /workspace/kobo-chess
bash scripts/setup-cloud.sh
source scripts/env.sh
bash scripts/verify-cloud.sh
```

The setup script uses the existing checkout and installs tools without sudo.
It needs the base machine's Git, curl, native C compiler, make, Python 3, APT,
dpkg-deb and Debian archive keyring. Debian packages are downloaded with archive
signature/hash verification and extracted into a user-owned prefix. Rust and
upstream source revisions are pinned; Debian package updates within trixie are
resolved from the signed repository, so this is not a byte-identical OS snapshot.
No checksum, TLS or signature bypass is used.

Locations (all outside this independent application checkout):

- `/workspace/deps/Cobalt`: unmodified upstream source dependency.
- `/workspace/toolchains`: Rust, Cargo registry/cache and extracted cross tools.
- `/workspace/build/cobalt`: host and ARM build outputs.
- `/workspace/verification`: simulator logs, screenshots and structured evidence.

Source `scripts/env.sh` in each new shell; it sets the linker, cross compiler,
local library search path and shared build cache. Four build jobs are used for
the available five CPUs; development debug information and incremental compilation
are disabled to keep cloud builds smaller. Do not run concurrent verification
jobs against a CLI binary that another build may replace.

Setup preserves an existing upstream checkout with unexpected changes/revision
by stopping. It does not reset it. Upstream sources are not vendored or committed
here. The supplied `kobo new` command generates a standalone app using a local
SDK path; future application work should use this separate repository and the
pinned SDK, without joining or editing the upstream workspace.

## Build and test commands

After sourcing `scripts/env.sh`, run from `$COBALT_SOURCE`:

```bash
cd "$COBALT_SOURCE"
cargo +1.85.1 build --locked -p kobo-hello -p kobo-lichess -p kobo-cli
cargo +1.85.1 build --locked --release --target armv7-unknown-linux-musleabihf -p kobo-hello -p kobo-lichess
cargo +1.85.1 test --locked -p kobo-lichess -p kobo-sdk -p kobo-profile
cargo +1.85.1 test --locked -p kobo-net --test lichess_stream_mock -- --test-threads=1
cargo +1.85.1 clippy --locked -p kobo-hello -p kobo-lichess --all-targets -- -D warnings
```

The stream tests use a local HTTPS mock and a test certificate. No Lichess
account, token or live game is needed. The workspace-wide suite includes many
unrelated apps and is not required for this representative chess SDK workflow.

## Simulator

The supported development simulator is Cobalt's browser simulator, served by the
`kobo` CLI. Start it in the Lichess application directory:

```bash
source /workspace/kobo-chess/scripts/env.sh
cd "$COBALT_SOURCE/apps/lichess"
KOBO_SIM_PROFILE=libra-colour-390 KOBO_SIM_OFFLINE=1 \
  KOBO_SIM_DATA=/workspace/verification/manual-libra-data \
  "$CARGO_TARGET_DIR/debug/kobo" dev 127.0.0.1:8787
```

It uses Libra Colour's 1264 × 1680, 300 PPI profile and colour renderer. The
upstream default remains Clara BW, so set the profile explicitly. The second
supported firmware profile is `libra-colour-390-4.46.23836`. The simulator is
loopback-only; use local HTTP requests and CLI driving in cloud sessions. No
cloud web preview is configured. Ctrl-C stops the foreground simulator.

In a second shell, source `scripts/env.sh` and exercise it:

```bash
"$CARGO_TARGET_DIR/debug/kobo" drive --address 127.0.0.1:8787 \
  --step 'expect Lichess' --step 'tap-id play-computer' --step 'wait-for Start game'
```

Repeatable automated checks which start and stop their own simulators:

```bash
cd "$COBALT_SOURCE"
python3 scripts/quality/check-lichess-computer-sim.py \
  --cli "$CARGO_TARGET_DIR/debug/kobo" \
  --output /workspace/verification/lichess-computer
python3 /workspace/kobo-chess/scripts/check-libra-sim.py
```

The upstream offline suite checks play, resume, resign, clocks, board orientation,
last-move outlines, undo and rendering at default and 170% text size, on its
hard-coded Clara BW profile. The local Libra check verifies the exact profile,
colour flag and geometry, then drives Lichess into an offline board and captures
screenshots. Colour appearance, ghosting, physical refresh timing, touch,
buttons, battery and suspend/resume still require owner-attended device tests.
The broader upstream simulator routes were inspected but not all executed.

## Network and safety

Actual GitHub clone/remote reads, Rust toolchain downloads, Cargo registry fetches
and signed Debian package downloads were tested. Required destinations are
`github.com` (through platform Git authentication), `sh.rustup.rs`,
`static.rust-lang.org`, `index.crates.io`, `static.crates.io`, and
`deb.debian.org`. The environment's package-manager network preset includes these;
no extra domains or secrets were needed. Access in a future session must still
be verified if policy changes.

Do not run `kobo setup`, deploy or device-write commands for cloud validation.
Do not alter KOReader, Focus Reader, Polgár Trainer, Nickel, firmware, boot files,
partitions or system libraries. No reader was connected or modified during setup.
See AGENTS.md for instructions for future agents.

## Recorded validation (17 November 2026)

| Check | Actual result |
| --- | --- |
| Locked host build: Hello, Lichess, kobo CLI | Passed |
| Locked ARM release build: Hello and Lichess | Passed; `file` confirms static ARM EABI5 executables; no ELF interpreter |
| Lichess unit tests | 144 passed |
| Device profile tests | 92 passed |
| SDK unit tests | 257 passed |
| SDK documentation tests | 1 passed, 2 ignored upstream |
| Lichess local HTTPS stream tests | 13 passed |
| Hello and Lichess Clippy, all targets, `-D warnings` | Passed |
| Upstream offline chess simulator suite | Passed at default and 170% text sizes |
| Libra Colour simulator functional check | Passed; asserted profile, 1264 × 1680 geometry and colour flag; started an offline game |
| Setup script rerun | Passed, including both host and ARM builds |

The initial ARM attempt failed because cross development headers were missing;
installing `libc6-dev-armhf-cross` and setting the relocated sysroot resolved it.
Normal system APT installation was unavailable without root, so the repeatable
script uses verified download-only extraction instead. Offline simulator logs
include denied network tasks, and the 170% upstream run emits ink-count layout
warnings; both functional suites still completed successfully. Those diagnostics
were not suppressed or presented as clean hardware acceptance.

No application source was developed, and the upstream checkout remains unchanged.
The entire upstream workspace suite, physical-reader execution and live online
Lichess play were not tested. The environment was published, then restoration
was verified in a fresh cloud instance: the documented toolchains, SDK and
dependencies were available without running setup or performing recovery. The
complete verification script passed again, including both simulator checks.
Environment installation and startup settings remain separate from this
repository; committing these files does not change or republish those settings.

The complete `bash scripts/verify-cloud.sh` command also completed with exit 0,
rerunning all listed builds, tests, Clippy and both simulator checks together.
The current-run command log is `/workspace/verify-cloud.log`; generated simulator
evidence remains under `/workspace/verification` and is not committed.
