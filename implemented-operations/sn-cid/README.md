# Rust implementation of the sn_cid operation

Generates the CID based ont he default hashing and encoding algos.

```rust

use sn_cid::sn_cid;


let r = b"that is f... weird".to_vec();
let cid = sn_cid(r.clone().encode());
println!("CID {:?}", cid);
```
