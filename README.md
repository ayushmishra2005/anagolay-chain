# Sensio Network Node

Sensio is a next-generation framework for ownerships, copyrights and digital licenses. 🚀

## Trying it out

## NOTES

https://github.com/scs/substrate-api-client/blob/master/test_no_std/Cargo.toml

# Testing

To test the full suite run `clear; cargo make test`

to test single pallet run `clear; cargo test operations -- --nocapture`

## Dependencies

to install all the deps run `./devops/scripts/deps.sh`

## NOTES

if you work in the same scope as `decl_storage` then no need to import the storage like

```rust

use frame_support::{
    storage::IterableStorageDoubleMap,
};
```

`find . -name "\*.sh" -exec chmod +x {} \;`
