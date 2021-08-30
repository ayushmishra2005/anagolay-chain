#!/usr/bin/env bash

# set -e
# set -x

echo "*** Installing the cargo packages ***"

rustup component add clippy &&
    rustup component add rustfmt &&
    rustup component add llvm-tools-preview &&
    cargo install grcov &&
    cargo install cargo-audit --features=fix
