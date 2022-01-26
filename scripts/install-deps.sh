#!/usr/bin/env bash
# THIS IS THE MAIN DEPS FILE, all other are crap

RUST_TOOLCHAIN="${1:-nightly-2021-06-29}"

apt-get update \
&& apt-get install -y --no-install-recommends \
        libssl-dev \
        clang \
        cmake \
        libclang-dev \
        musl-tools \
        libffi-dev 

echo "*** Initializing WASM build environment for toolchain $RUST_TOOLCHAIN ***"

if [ -z $CI_PROJECT_NAME ]; then
  rustup update nightly
  rustup update stable
fi

rustup toolchain install $RUST_TOOLCHAIN &&
  rustup target add wasm32-unknown-unknown --toolchain $RUST_TOOLCHAIN

cargo install cargo-make
