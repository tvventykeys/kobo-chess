#!/usr/bin/env bash
# Debian 13 x86_64, unprivileged; no device operations.
set -euo pipefail
cd /workspace/kobo-chess
mkdir -p /workspace/toolchains /workspace/deps
export RUSTUP_HOME=/workspace/toolchains/rustup CARGO_HOME=/workspace/toolchains/cargo
export PATH="$CARGO_HOME/bin:$PATH"
if ! command -v rustup >/dev/null; then
  curl --fail --show-error --location --proto '=https' --tlsv1.2 https://sh.rustup.rs -o /workspace/toolchains/rustup-init.sh
  sh /workspace/toolchains/rustup-init.sh -y --no-modify-path --default-toolchain 1.85.1 --profile minimal --component rustfmt --component clippy
fi
rustup toolchain install 1.85.1 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.85.1 armv7-unknown-linux-musleabihf
revision=5ac30a46efce23d7fa0d1cb12a499c9b57b816b0
if [[ ! -e /workspace/deps/Cobalt ]]; then
  git clone https://github.com/BandarLabs/Cobalt.git /workspace/deps/Cobalt
  git -C /workspace/deps/Cobalt checkout --detach "$revision"
fi
[[ $(git -C /workspace/deps/Cobalt rev-parse HEAD) == "$revision" ]] || { echo 'Cobalt revision differs; preserve it and inspect before continuing.' >&2; exit 1; }
[[ -z $(git -C /workspace/deps/Cobalt status --porcelain) ]] || { echo 'Cobalt has local changes; preserve and inspect them.' >&2; exit 1; }
# APT verifies the Debian archive signature and package hashes. Download only;
# extract into a user-owned prefix rather than installing into the host OS.
mkdir -p /workspace/toolchains/apt/lists/partial /workspace/toolchains/apt/cache/archives/partial /workspace/toolchains/cross
cat > /workspace/toolchains/apt/sources.list <<'SOURCES'
deb [signed-by=/usr/share/keyrings/debian-archive-keyring.gpg] https://deb.debian.org/debian trixie main
SOURCES
apt_options=(-o Debug::NoLocking=1 -o Dir::Etc::parts=- -o Dir::Etc::main=-
  -o Dir::Etc::sourcelist=/workspace/toolchains/apt/sources.list -o Dir::Etc::sourceparts=-
  -o Dir::State::lists=/workspace/toolchains/apt/lists -o Dir::Cache=/workspace/toolchains/apt/cache)
/usr/bin/apt-get "${apt_options[@]}" update
/usr/bin/apt-get "${apt_options[@]}" --download-only --no-install-recommends -y install gcc-arm-linux-gnueabihf libc6-dev-armhf-cross
for archive in /workspace/toolchains/apt/cache/archives/*.deb; do
  dpkg-deb -x "$archive" /workspace/toolchains/cross
done
source scripts/env.sh
rustc +1.85.1 --version
cargo +1.85.1 --version
cargo +1.85.1 fmt --version
cargo +1.85.1 clippy --version
arm-linux-gnueabihf-gcc --version
arm-linux-gnueabihf-ar --version
cd "$COBALT_SOURCE"
cargo +1.85.1 fetch --locked
cargo +1.85.1 build --locked -p kobo-hello -p kobo-lichess -p kobo-cli
cargo +1.85.1 build --locked --release --target armv7-unknown-linux-musleabihf -p kobo-hello -p kobo-lichess
