#!/usr/bin/env bash

echo "***** INSTALLING COMMON CARGO PACKAGES *****"

EXTRA_ARGS=${1:-""}

echo $EXTRA_ARGS
cargo install $EXTRA_ARGS \
  wasm-bindgen-cli \
  taplo-cli \
  cargo-make \
  wasm-pack \
  cargo-chef \
  cargo-audit \
  cargo-tarpaulin \
  cargo-nextest
