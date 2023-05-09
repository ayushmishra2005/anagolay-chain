#!/usr/bin/env bash

set -eux

PROJECT_ROOT=$(git rev-parse --show-toplevel)
cd $PROJECT_ROOT

chain="${1:-dev}"

echo "1. Building the Anagolay with release and benchmarks..."

makers --profile=production build-release-benchmarks

echo "2. Running the benchmarks for all pallets individually."
for i in $(find pallets -type f -name 'benchmarking.rs' | sed -r 's|pallets\/([^/]+)\/src\/benchmarking.rs$|\1|' |sort -u); do
  echo "Calculating weights for [$i] ..."
  ./.devops/run-benchmarks.sh $chain $i false
done
echo "Done!"

echo "3. Benchmarking dependencies ⚒⚒"
for i in $(find runtime/src/weights -type f -name 'pallet_*.rs' | sed -r 's|runtime\/src\/weights/pallet_([^/]+).rs$|\1|' |sort -u); do
  echo "Calculating weights for [$i] ..."
  ./.devops/run-benchmarks.sh $chain pallet_$i false ./runtime/src/weights/pallet_$i.rs ./templates/dep-weight-template.hbs
done
echo "Done!"

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
