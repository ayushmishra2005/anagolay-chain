# Overview

Anagolay is split into multiple parts for maximum maintainability and ease of use.

- pallets
- node
- runtime

## Pallets

- _found in_: `/pallets`
- _crates prefix_: `pallet-`
- _constraints_:
  - all crates that go on chain must be `[no_std]`

_Pallets_ are individual modules within _Anagolay Network_ These are containers that host domain-specific logic. For example, `operations` contains logic for manipulating with operations.

Current list of pallets:

- operations
- poe (Proof-of-Existence)
- workflows
- anagolay-support ( all generics are here )
- statements

## Node

- _found in_: `/node`

The default (testing) application pulling together official substrate recommended setup of substrate-client with a wasm-contracts-supporting frame-runtime. The node pulls it all together and constructs the (upgradable) runtime. This is also what is being built and run if you do `cargo run`.

## Runtime

- _found in_: `/runtime`
- _constraints_:
  - must be `[no_std]`

This is the lowest level of abstraction and opinion that everything else builds upon. Here are defined the basics of the chain, from the token to the genesis and default accounts. This is the place where we connect out pallets to the and specify the connection between them.

## Integration Tests

- _found in_: `tests.rs`
- _constraints_: none at the moment

All tests must pass!!!

## Binaries and template

- _found in_: `/target/debug` and `/target/release`
- _binary name_ :
  - windows : `anagolay.exe`
  - linux : `anagolay`
