use cid::{multihash::MultihashGeneric, Cid};
use codec::{Decode, Encode};
use multibase::Base;
use multihash::{Blake3_256, Code, Hasher};
use sp_std::vec::Vec;
use unsigned_varint::encode as varint_encode;

/// Generic ID, this is the content identifier of the payload, like rule or proof. for now it's CID string
pub type GenericId = Vec<u8>;

/// Placeholder for SSI and DID
pub type CreatorId = Vec<u8>;

/// Alias for string
pub type Text = Vec<u8>;

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub enum ForWhat {
  /// WE are creating it For what? This can be a part of the group
  GENERIC, // 0
  PHOTO,       // 1
  CAMERA,      // 2
  LENS,        // 3
  SMARTPHONE,  // 4
  USER,        // 5
  SYS,         // 6
  FLOWCONTROL, // 7
}

impl Default for ForWhat {
  fn default() -> Self {
    ForWhat::GENERIC
  }
}

/// Default values Hashing
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultsHashing {
  algo: Vec<u8>,
  bits: u32,
}

impl Default for DefaultsHashing {
  fn default() -> Self {
    DefaultsHashing {
      algo: b"blake2b".to_vec(),
      bits: 256,
    }
  }
}

/// Default values Encoding
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultsEncoding {
  algo: Vec<u8>,
  prefix: bool,
}

impl Default for DefaultsEncoding {
  fn default() -> Self {
    DefaultsEncoding {
      algo: b"hex".to_vec(),
      prefix: true,
    }
  }
}

/// Default values Content Identifier or CID
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultsCid {
  version: u8,
  base: Vec<u8>,
  codec: Vec<u8>,
}

impl Default for DefaultsCid {
  fn default() -> Self {
    DefaultsCid {
      version: 1,
      base: b"base32".to_vec(),
      codec: b"dag-cbor".to_vec(),
    }
  }
}

/// Default values for this runtime
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct DefaultValues {
  hashing: DefaultsHashing,
  encoding: DefaultsEncoding,
  cid: DefaultsCid,
}

impl Default for DefaultValues {
  fn default() -> Self {
    DefaultValues {
      hashing: DefaultsHashing::default(),
      encoding: DefaultsEncoding::default(),
      cid: DefaultsCid::default(),
    }
  }
}

/// Info, this is what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq)]
pub struct AnagolayRecord<T, A, B> {
  pub record: T,
  pub account_id: A,
  pub block_number: B,
}

pub trait AnagolayStructureData: Default + Encode + Clone + PartialEq + Eq {
  fn to_cid(&self) -> Vec<u8> {
    let hash = MultihashGeneric::wrap(
      Code::Blake3_256.into(),
      Blake3_256::digest(self.encode().as_slice()).as_ref(),
    )
    .unwrap();

    // RAW codec from the multiformats
    const RAW: u64 = 0x55;
    let cid = Cid::new_v1(RAW, hash);

    // create the string slice like `bafk...`
    let mut cid_bytes = bytecursor::bytecursor::ByteCursor::new(Vec::new());
    let mut version_buf = varint_encode::u64_buffer();
    let version = varint_encode::u64(cid.version().into(), &mut version_buf);

    let mut codec_buf = varint_encode::u64_buffer();
    let codec = varint_encode::u64(cid.codec(), &mut codec_buf);

    cid_bytes.write_all(version).unwrap();
    cid_bytes.write_all(codec).unwrap();

    let mut code_buf = varint_encode::u64_buffer();
    let code = varint_encode::u64(cid.hash().code(), &mut code_buf);

    let mut size_buf = varint_encode::u8_buffer();
    let size = varint_encode::u8(cid.hash().size(), &mut size_buf);

    cid_bytes.write_all(code).unwrap();
    cid_bytes.write_all(size).unwrap();
    cid_bytes.write_all(cid.hash().digest()).unwrap();

    let cid_str = multibase::encode(Base::Base32Lower, cid_bytes.into_inner());

    // make the string slice into vec bytes, usually we use that
    cid_str.into_bytes()
  }
}

pub trait AnagolayStructureExtra: Clone + PartialEq + Eq {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct AnagolayStructure<T: AnagolayStructureData, U: AnagolayStructureExtra> {
  pub id: GenericId,
  pub data: T,
  pub extra: Option<U>,
}

impl<T: AnagolayStructureData, U: AnagolayStructureExtra> Default for AnagolayStructure<T, U> {
  fn default() -> Self {
    AnagolayStructure {
      id: b"".to_vec(),
      data: T::default(),
      extra: None,
    }
  }
}
