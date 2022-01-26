#!/usr/bin/env bash

# set -e
# set -x

echo "*** Installing the dev cargo packages ***"

rustup component add rls &&
  rustup component add llvm-tools-preview &&
  rustup component add rust-src &&
  cargo install cargo-cache

# cargo install grcov &&
# cargo install cargo-audit --features=fix &&
