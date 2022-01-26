#!/usr/bin/env bash

# set -e
# set -x

RUST_TOOLCHAIN="${1:-nightly-2021-06-29}"

echo "*** Initializing WASM build environment for toolchain $RUST_TOOLCHAIN ***"

if [ -z $CI_PROJECT_NAME ]; then
  rustup update nightly
  rustup update stable
fi

rustup toolchain install $RUST_TOOLCHAIN &&
  rustup target add wasm32-unknown-unknown --toolchain $RUST_TOOLCHAIN

# rustup default stable
