#!/usr/bin/env bash

echo "***** INSTALLING COMMON CARGO PACKAGES *****"

cargo install wasm-bindgen-cli taplo-cli

# this below failed
# cargo install cargo-audit cargo-tarpaulin
