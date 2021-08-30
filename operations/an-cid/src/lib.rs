// #![cfg_attr(not(feature = "std"), no_std)]

use cid::{Cid, Codec, Version};
use multibase::Base;
use multihash::Blake2b256;
use sp_std::vec::Vec;

mod tests;

/// Generate CID with multihash for a given input, This is the implementation of an_cid operation
pub fn an_cid(data: Vec<u8>) -> Vec<u8> {
  // gen the multihash with our default algo and bits
  // NOTE don't forget to follow th default hashing algo
  let h = Blake2b256::digest(data.as_slice());

  // ALWAYS use V1, for now base32 is used 'coz it can't be changed
  // TODO double-check on the Codec::Raw
  let cid = Cid::new(Version::V1, Codec::DagCBOR, h).unwrap();

  // create the string slice like `bafk...`
  let cid_str = multibase::encode(Base::Base32Lower, cid.to_bytes().as_slice());

  // make the string slice into vec bytes, usually we use that
  cid_str.into_bytes()
}
