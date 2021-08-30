# Benchmarking

Create benchmarking for Statement pallet.

# Statements

To generate pallet weights and enable benchmarking, run the following script:

```sh

./scripts/run-benchmarks.sh dev statements

cargo test -p an-statements --features runtime-benchmarks --features --all benchmarking
```
