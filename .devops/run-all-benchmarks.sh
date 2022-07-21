#!/usr/bin/env bash

set -eux

PROJECT_ROOT=$(git rev-parse --show-toplevel)
cd $PROJECT_ROOT

chain="${1:-dev}"

echo "1. Building the Anagolay with release and benchmarks..."

makers --profile=production build-release-benchmarks

echo "2. Running the benchmarks for all pallets individually."
for i in $(ls pallets); do
  echo "Calculating weights for [$i] ..."
  ./scripts/run-benchmarks.sh $chain $i false
done
echo "Done!"

echo "3. Benchmarking dependencies ⚒⚒"
./scripts/run-benchmarks.sh $chain pallet_vesting false ./runtime/src/weights/pallet_vesting.rs ./templates/dep-weight-template.hbs

# ./target/release/anagolay benchmark \
#   --chain="${chain}" \
#   --steps=50 \
#   --repeat=100 \
#   --pallet="*" \
#   --extrinsic=* \
#   --execution=wasm \
#   --wasm-execution=compiled \
#   --heap-pages=4096 \
#   --output=./runtime/src/weights/ \
#   --template=./templates/module-weight-template.hbs

# # since benchmark generates a weight.rs file that may or may not cargo fmt'ed.
# # so do cargo fmt here.
# makers format
