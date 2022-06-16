#!/usr/bin/env bash

set -eux

PROJECT_ROOT=$(git rev-parse --show-toplevel)
cd $PROJECT_ROOT

chain="${1:-dev}"
pallet=$2
build_and_run="${3:-true}"
output=./pallets/${pallet}/src/weights.rs
build_command="cargo run --release --features runtime-benchmarks -- benchmark"
run_command="./target/release/anagolay benchmark"

echo "Benchmark: ${pallet} ⚒⚒"

if $build_and_run; then

  $build_command \
    --chain="${chain}" \
    --steps=50 \
    --repeat=100 \
    --pallet="${pallet}" \
    --extrinsic=* \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --output="${output}" \
    --template=./templates/module-weight-template.hbs
else
  $run_command \
    --chain="${chain}" \
    --steps=50 \
    --repeat=100 \
    --pallet="${pallet}" \
    --extrinsic=* \
    --execution=wasm \
    --wasm-execution=compiled \
    --heap-pages=4096 \
    --output="${output}" \
    --template=./templates/module-weight-template.hbs
fi
# # since benchmark generates a weight.rs file that may or may not cargo fmt'ed.
# # so do cargo fmt here.
makers format
