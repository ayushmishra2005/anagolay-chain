#!/usr/bin/env bash

echo "***** INSTALLING RUST RELATED PACKAGES *****"

set -x
set -o errexit

rustup default nightly-2022-05-28
rustup target add wasm32-unknown-unknown --toolchain nightly-2022-05-28
rustup target add x86_64-unknown-linux-gnu --toolchain nightly-2022-05-28

if [ -z $CI_PROJECT_NAME ]; then
  rustup update nightly
  rustup update stable
fi

if [[ "${1}" =~ "dev" ]]; then
  echo "$2"
  rustup component add rls
  rustup component add llvm-tools-preview
  rustup component add rustfmt
  rustup component add rustc-dev
  rustup component add rust-std
  rustup component add rust-analysis
  rustup component add rust-src
  rustup component add rust-docs
fi

#### make this optional and with the flag fo the dev contianer
# rustup component add rls
# rustup component add llvm-tools-preview
# cargo install taplo-cli
#### make this optional and with the flag fo the dev contianer
