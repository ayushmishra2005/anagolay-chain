#!/usr/bin/env bash

# set -e
# set -x

echo "*** Installing the cargo packages ***"

# enable this if you are reinstalling cachepot
# unset RUSTC_WRAPPER

cargo install cargo-make
# cargo install rusty-hook
# rustup component add llvm-tools-preview 
# cargo install grcov 
# cargo install cargo-audit --features=fix
