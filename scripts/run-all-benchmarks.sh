#!/usr/bin/env bash

set -eu
chain="${1:-dev}"

echo "1. Building the Anagolay with release and benchmarks..."

cargo make build-release-benchmarks

echo "2. Running the benchmarks for all pallets indivitually."
for i in $(ls pallets); do
  # exclude the template pallet
  if [ $i != "123-pallet" ]; then
    echo "Calcualting weights for [$i] ..."
    ./scripts/run-benchmarks.sh $chain $i false
  fi
done
echo "Done!"

echo "2. Benchmarking all pallets ⚒⚒"

./target/release/anagolay benchmark \
  --chain="${chain}" \
  --steps=50 \
  --repeat=100 \
  --pallet="*" \
  --extrinsic=* \
  --execution=wasm \
  --wasm-execution=compiled \
  --heap-pages=4096 \
  --output=./runtime/src/weights/ \
  --template=./templates/module-weight-template.hbs

# # since benchmark generates a weight.rs file that may or may not cargo fmt'ed.
# # so do cargo fmt here.
cargo make fmt
