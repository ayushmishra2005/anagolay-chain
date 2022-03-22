use cid::{multihash::MultihashGeneric, Cid};
use codec::{Decode, Encode};
use multibase::Base;
use multihash::{Blake3_256, Code, Hasher};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Generic ID, this is the content identifier of the payload, like rule or proof. for now it's CID
/// string
pub type GenericId = Vec<u8>;

/// Placeholder for SSI and DID
pub type CreatorId = GenericId;

/// Alias for string
pub type Characters = GenericId;

/// The type of the values in the `PackagesByPackageId` storage
pub type PackageId = GenericId;

/// List of equipment that needs rules generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
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

/// Info, this is what gets stored
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq)]
pub struct AnagolayRecord<T, A, B> {
  pub record: T,
  pub account_id: A,
  pub block_number: B,
}

/// The trait for the data field of an Anagolay entity.
pub trait AnagolayStructureData: Default + Encode + Clone + PartialEq + Eq {
  /// Computes cid of the data, after encoding it using parity SCALE codec
  ///
  /// # Examples
  ///
  /// ```
  /// use codec::{Decode, Encode};
  /// use anagolay_support::{AnagolayStructureData, AnagolayStructureExtra};
  ///
  /// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  /// struct EntityData {
  ///   text: Vec<u8>
  /// };
  ///
  /// impl Default for EntityData {
  ///   fn default() -> Self {
  ///     EntityData {
  ///       text: b"".to_vec()
  ///     }
  ///   }
  /// }
  ///
  /// impl AnagolayStructureData for EntityData {}
  ///
  /// let entity = EntityData {
  ///   text: b"hello".to_vec()
  /// };
  ///
  /// assert_eq!(b"bafkr4iac2luovbttsv5iftbg2zl4okalixafa2vjwtbmf6exgwiuvukhmi".to_vec(), entity.to_cid());
  /// ```
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
    let cid_str = multibase::encode(Base::Base32Lower, cid.to_bytes());

    // make the string slice into vec bytes, usually we use that
    cid_str.into_bytes()
  }
}

/// The trait for the extra field of an Anagolay entity
pub trait AnagolayStructureExtra: Clone + PartialEq + Eq {}

/// Generic structure representing an Anagolay entity
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

impl<T: AnagolayStructureData, U: AnagolayStructureExtra> AnagolayStructure<T, U> {
  /// Produces an Anagolay entity with no extra information
  pub fn new(data: T) -> Self {
    AnagolayStructure {
      id: data.to_cid(),
      data,
      extra: None,
    }
  }

  /// Produces an Anagolay entity with some extra information
  ///
  /// # Examples
  ///
  /// ```
  /// use codec::{Decode, Encode};
  /// use anagolay_support::{AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra};
  ///
  /// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  /// struct EntityData {
  ///   text: Vec<u8>
  /// };
  ///
  /// impl Default for EntityData {
  ///   fn default() -> Self {
  ///     EntityData {
  ///       text: b"".to_vec()
  ///     }
  ///   }
  /// }
  ///
  /// impl AnagolayStructureData for EntityData {}
  ///
  /// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  /// struct EntityExtra {
  ///   created_at: u64
  /// };
  ///
  /// impl AnagolayStructureExtra for EntityExtra {}
  ///
  /// type Entity = AnagolayStructure<EntityData, EntityExtra>;
  ///
  /// let entity = Entity::new_with_extra(EntityData {
  ///   text: b"hello".to_vec()
  /// }, EntityExtra {
  ///   created_at: 0
  /// });
  ///
  /// assert_eq!(b"hello".to_vec(), entity.data.text);
  /// assert!(entity.extra.is_some());
  /// assert_eq!(0, entity.extra.unwrap().created_at);
  pub fn new_with_extra(data: T, extra: U) -> Self {
    AnagolayStructure {
      id: data.to_cid(),
      data,
      extra: Some(extra),
    }
  }
}

/// Trait used as type parameter in [`AnagolayPackageStructure`], allowing different structures to
/// define the enumeration of possible artifact types depending on the specific case:
///
/// # Examples
///
/// ```
/// use anagolay_support::{AnagolayPackageStructure, ArtifactType};
///
/// enum OperationArtifactType {
///   CRATE, CJS, WASM, ESM, WEB, DOCS, GIT
/// }
///
/// impl ArtifactType for OperationArtifactType {}
///
/// type OperationPackageStructure = AnagolayPackageStructure<OperationArtifactType>;
///
/// enum ImageArtifactType {
///   RAW
/// }
///
/// impl ArtifactType for ImageArtifactType {}
///
/// type ImagePackageStructure = AnagolayPackageStructure<OperationArtifactType>;
/// ```
pub trait ArtifactType {}

/// Operation Version package
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolayPackageStructure<T: ArtifactType> {
  /// Type of the package
  pub package_type: T,
  /// Name of the file
  pub file_name: Option<Characters>,
  /// IPFS cid
  pub ipfs_cid: GenericId,
}
