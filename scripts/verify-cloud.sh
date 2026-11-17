#!/usr/bin/env bash
set -euo pipefail
cd /workspace/kobo-chess
source scripts/env.sh
cd "$COBALT_SOURCE"
cargo +1.85.1 build --locked -p kobo-hello -p kobo-lichess -p kobo-cli
cargo +1.85.1 build --locked --release --target armv7-unknown-linux-musleabihf -p kobo-hello -p kobo-lichess
cargo +1.85.1 test --locked -p kobo-lichess -p kobo-sdk -p kobo-profile
cargo +1.85.1 test --locked -p kobo-net --test lichess_stream_mock -- --test-threads=1
cargo +1.85.1 clippy --locked -p kobo-hello -p kobo-lichess --all-targets -- -D warnings
python3 scripts/quality/check-lichess-computer-sim.py --cli "$CARGO_TARGET_DIR/debug/kobo" --output /workspace/verification/lichess-computer
python3 /workspace/kobo-chess/scripts/check-libra-sim.py
