# Rust implementation of the an_cid operation

Generates the CID based ont he default hashing and encoding algos.

```rust

use an_cid::an_cid;


let r = b"that is f... weird".to_vec();
let cid = an_cid(r.clone().encode());
println!("CID {:?}", cid);
```
