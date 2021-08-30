#!/usr/bin/env bash

# set -e
# set -x

echo "*** Installing the cargo packages ***"

# enable this if you are reinstalling cachepot
# unset RUSTC_WRAPPER

rustup component add clippy &&
  rustup component add rustfmt &&
  cargo install --force cargo-make &&
  cargo install rusty-hook
