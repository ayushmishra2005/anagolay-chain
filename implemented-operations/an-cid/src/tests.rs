//! Tests for the module.

#![cfg(test)]

use super::*;

#[test]
fn test_an_cid() {
    let r = b"that is f... weird".to_vec();
    let cid = an_cid(r);
    println!("CID {:?}", cid);
}
