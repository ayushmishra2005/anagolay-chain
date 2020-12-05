#!/usr/bin/env bash

set -e
set -x

echo "*** Initializing WASM build environment"

rustup update nightly
rustup update stable

rustup toolchain install nightly-2020-06-01
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-06-01

rustup default nightly-2020-06-01
# cargo +nightly-2020-06-01 build --release

# rustup target add wasm32-unknown-unknown --toolchain nightly
