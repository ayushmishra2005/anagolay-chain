use codec::{Decode, Encode};
use sp_std::vec::Vec;

/// Generic ID, this is the content identifier of the payload, like rule or proof. for now it's CID string
pub type GenericId = Vec<u8>;

/// Placeholder for SSI and DID
pub type CreatorId = Vec<u8>;

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
  pub info: T,
  pub account_id: A,
  pub block_number: B,
}

pub trait AnagolayStructureData: Default + Clone + PartialEq + Eq {}
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
