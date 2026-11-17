# Source this file in Bash before using Cargo or Cobalt.
export RUSTUP_HOME=/workspace/toolchains/rustup
export CARGO_HOME=/workspace/toolchains/cargo
export COBALT_SOURCE=/workspace/deps/Cobalt
export CARGO_TARGET_DIR=/workspace/build/cobalt
export PATH="$CARGO_HOME/bin:/workspace/toolchains/cross/usr/bin:$PATH"
export LD_LIBRARY_PATH="/workspace/toolchains/cross/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc
export AR_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-ar
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER="$RUSTUP_HOME/toolchains/1.85.1-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/rust-lld"
export CARGO_BUILD_JOBS=4
export CARGO_PROFILE_DEV_DEBUG=0
export CARGO_INCREMENTAL=0
export CFLAGS_armv7_unknown_linux_musleabihf=--sysroot=/workspace/toolchains/cross
