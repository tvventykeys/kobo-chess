#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
source scripts/env.sh
cargo +1.85.1 fmt --check
cargo +1.85.1 clippy --locked --all-targets -- -D warnings
cargo +1.85.1 test --locked
cargo +1.85.1 build --locked
"$CARGO_TARGET_DIR/debug/polgar-pocket" --validate-pack packs/lichess-20.jsonl
cargo +1.85.1 build --locked --release --target armv7-unknown-linux-musleabihf
file "$CARGO_TARGET_DIR/armv7-unknown-linux-musleabihf/release/polgar-pocket"
if arm-linux-gnueabihf-readelf -l "$CARGO_TARGET_DIR/armv7-unknown-linux-musleabihf/release/polgar-pocket" | rg 'INTERP'; then
    echo 'Unexpected dynamic ELF interpreter' >&2
    exit 1
fi
python3 scripts/check-pocket-sim.py
