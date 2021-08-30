## Proof of Existence definition pallet

Create, replace and store PoE definitions;

Contains the storage management and PoE definition

# Benchmarking

To enable benchmarking run `cargo build --release --features runtime-benchmarks`

To generate pallet weights run

```sh
./target/release/anagolay benchmark \
   --chain=dev \
   --steps=50 \
   --repeat=20 \
   --pallet=an_poe \
   --extrinsic="*" \
   --execution=wasm \
   --wasm-execution=compiled \
   --heap-pages=4096 \
   --output=pallets/poe/src/weights.rs \
   --template=./templates/module-weight-template.hbs
```

