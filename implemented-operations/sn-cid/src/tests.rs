//! Tests for the module.

#![cfg(test)]

use super::*;

#[test]
fn test_sn_cid() {
    let r = b"that is f... weird".to_vec();
    let cid = sn_cid(r);
    println!("CID {:?}", cid);
}
