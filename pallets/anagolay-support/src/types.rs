// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2022 Anagolay Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
use cid::{multihash::MultihashGeneric, Cid};
use codec::{Decode, Encode};
use core::any::type_name_of_val;
use multibase::Base;
use multihash::{Blake3_256, Code, Hasher};
use sp_runtime::RuntimeDebug;
use sp_std::{vec, vec::Vec};

/// Private package used to hide the types that are not supposed to be referenced by dependent
/// crates
mod private {
  use super::Characters;
  use cid::Cid;
  use codec::{Decode, Encode};
  use core::convert::TryFrom;
  use sp_std::{convert::From, str, vec, vec::Vec};

  /// Generic ID, this is the content identifier of the payload, like worflow or proof. It's a
  /// multibase encoded CID string. It must be in a private module, aliased by the types of each
  /// respective entity id, since it's used in [`AnagolayVersionData`] to refer to any entity id.
  ///
  /// Id aliases are also an important indicator of what type of id is expected in which place:
  /// instead of writing documentation we can immediately show the user what the storage or the
  /// data model expects.
  ///
  /// It follows NewType pattern to provide conversion to and from Vec<u8> for (de)serialization but
  /// also to provide additional behaviour, like validation.
  #[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
  pub struct GenericId(pub Vec<u8>);

  impl From<GenericId> for Vec<u8> {
    fn from(from: GenericId) -> Vec<u8> {
      from.0
    }
  }

  impl From<Vec<u8>> for GenericId {
    fn from(from: Vec<u8>) -> GenericId {
      GenericId(from)
    }
  }

  impl Default for GenericId {
    fn default() -> Self {
      vec![].into()
    }
  }

  impl GenericId {
    pub fn from(str: &str) -> Self {
      GenericId(str.as_bytes().to_vec())
    }

    /// Validate the CID. It will require a string to be multibase-decoded and then parsed as CID
    ///
    /// # Return
    /// An unit result if the validation is successful, a `Character` error with a description in
    /// case it fails
    pub fn validate(&self) -> Result<(), Characters> {
      multibase::decode(str::from_utf8(&self.0).unwrap())
        .map(|decoded| Cid::try_from(decoded.1).map_err(|_| Characters::from("Invalid CID")))
        .map(|_| ())
        .map_err(|_| Characters::from("Cannot decode CID"))
    }
  }
}
/// NewType pattern to handle strings.
/// It conveniently allows concatenation and deals with (de)serialization as well.
///
/// # Example
///
/// ```
/// use anagolay_support::Characters;
///
/// let chars: Characters = "hello".into();
///
/// assert_eq!(5, chars.len());
/// assert_eq!("hello2world", chars.concat_u8(2u8).concat("world").as_str());
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub struct Characters(Vec<u8>);

impl From<&str> for Characters {
  fn from(from: &str) -> Characters {
    Characters(from.as_bytes().to_vec())
  }
}

impl From<Vec<u8>> for Characters {
  fn from(from: Vec<u8>) -> Characters {
    Characters(from)
  }
}

impl Default for Characters {
  fn default() -> Self {
    vec![].into()
  }
}

impl Characters {
  /// Convenience method to create `Characters` from a string slice when the result type is implicit
  ///
  /// # Example
  ///
  /// ```
  /// use anagolay_support::Characters;
  /// let chars = Characters::from("hello");
  /// ```
  ///
  /// Most of the time, it's convenient to use `str.into()`
  ///
  /// ```
  /// use anagolay_support::Characters;
  /// let chars: Characters = "hello".into();
  /// ```
  pub fn from(str: &str) -> Self {
    Characters(str.as_bytes().to_vec())
  }

  /// # Return
  /// The `Characters` representation as a string slice
  pub fn as_str(&self) -> &str {
    sp_std::str::from_utf8(self.0.as_slice()).unwrap()
  }

  /// Append an unsigned integer to this `Characters`
  ///
  /// # Arguments
  /// * uint - The unsigned integer to append
  ///
  /// # Return
  /// This `Characters` with the argument appended
  pub fn concat_u8(mut self, uint: u8) -> Self {
    let mut n = uint;
    if n == 0 {
      self.0.append(&mut b"0".to_vec());
    } else {
      let mut buffer = [0u8; 100];
      let mut i = 0;
      while n > 0 {
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
      }
      let slice = &mut buffer[..i];
      slice.reverse();
      self.0.append(&mut slice.to_vec());
    }
    self
  }

  /// Concatenate a string slice to this `Characters`
  ///
  /// # Arguments
  /// * other - The string slice to concatenate
  ///
  /// # Return
  /// This `Characters` with the argument concatenated
  pub fn concat(mut self, other: &str) -> Self {
    self.0.append(&mut other.as_bytes().to_vec());
    self
  }

  /// # Return
  /// This `Characters` length
  pub fn len(&self) -> usize {
    self.0.len()
  }
}

/// Placeholder for SSI and DID
pub type CreatorId = private::GenericId;

/// The type of the values in the `ArtifactsByArtifactId` storage
pub type ArtifactId = private::GenericId;

/// The type used for an Operation ID
pub type OperationId = private::GenericId;

/// The type used for a Workflow ID
pub type WorkflowId = private::GenericId;

/// The type used for any entity Version ID
pub type VersionId = private::GenericId;

/// List of equipment that needs workflows generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum ForWhat {
  /// We are creating it For what? This can be a part of the group
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

/// Info, this is what gets stored. The Generic `A` is usally the `AccountId` and `B` is
/// `BlockNumber`
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
  /// use anagolay_support::{AnagolayStructureData, AnagolayStructureExtra, Characters};
  /// # use anagolay_support::ArtifactId;
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
  /// impl AnagolayStructureData for EntityData {
  ///   fn validate(&self) -> Result<(), Characters> {
  ///      Ok(())
  ///   }
  /// }
  ///
  /// let entity = EntityData {
  ///   text: b"hello".to_vec()
  /// };
  ///
  /// let cid = entity.to_cid();
  /// # assert_eq!(ArtifactId::from("bafkr4iac2luovbttsv5iftbg2zl4okalixafa2vjwtbmf6exgwiuvukhmi"), cid);
  /// ```
  fn to_cid(&self) -> private::GenericId {
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
    private::GenericId(cid_str.into_bytes())
  }

  fn validate(&self) -> Result<(), Characters>;
}

/// The trait for the extra field of an Anagolay entity
pub trait AnagolayStructureExtra: Clone + PartialEq + Eq {}

/// Generic structure representing an Anagolay entity
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct AnagolayStructure<T: AnagolayStructureData, U: AnagolayStructureExtra> {
  pub id: private::GenericId,
  pub data: T,
  pub extra: Option<U>,
}

impl<T: AnagolayStructureData, U: AnagolayStructureExtra> Default for AnagolayStructure<T, U> {
  fn default() -> Self {
    AnagolayStructure {
      id: private::GenericId::default(),
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
  /// use anagolay_support::{AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, Characters};
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
  /// impl AnagolayStructureData for EntityData {
  ///   fn validate(&self) -> Result<(), Characters> {
  ///      Ok(())
  ///   }
  /// }
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
/// use codec::{Decode, Encode};
/// use anagolay_support::{AnagolayArtifactStructure, ArtifactType};
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum OperationArtifactType {
///   CRATE, WASM, DOCS, GIT
/// }
///
/// impl ArtifactType for OperationArtifactType {}
///
/// type OperationPackageStructure = AnagolayArtifactStructure<OperationArtifactType>;
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum ImageArtifactType {
///   RAW
/// }
///
/// impl ArtifactType for ImageArtifactType {}
///
/// type ImagePackageStructure = AnagolayArtifactStructure<ImageArtifactType>;
/// ```
pub trait ArtifactType: Encode + Decode + Clone + PartialEq + Eq {}

/// Operation Version artifact
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolayArtifactStructure<T: ArtifactType> {
  /// Type of the artifact
  pub artifact_type: T,
  /// Extension of the stored file
  pub file_extension: Characters,
  /// IPFS cid
  pub ipfs_cid: ArtifactId,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
/// Extra information (non hashed) for an entity Version
pub struct AnagolayVersionExtra {
  pub created_at: u64,
}

/// Implementation of AnagolayStructureExtra trait for OperationVersionExtra
impl AnagolayStructureExtra for AnagolayVersionExtra {}

/// Version data. It contains all the needed parameters which define the entity Version and is
/// hashed to produce the Version id
///
/// # Examples
///
/// ```
/// use codec::{Decode, Encode};
/// use anagolay_support::{AnagolayStructure, AnagolayVersionData, AnagolayVersionExtra, ArtifactType};
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum OperationArtifactType {
///   CRATE, WASM, DOCS, GIT
/// }
/// impl ArtifactType for OperationArtifactType {}
///
/// type OperationVersionData = AnagolayVersionData<OperationArtifactType>;
/// type OperationVersion = AnagolayStructure<OperationVersionData, AnagolayVersionExtra>;
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct AnagolayVersionData<T: ArtifactType> {
  /// The id of the Operation, Workflow or other entity to which this Version is
  /// associated. __This field is read-only__
  pub entity_id: Option<private::GenericId>,
  /// The id of the previous Operation Version for the same operation, if any.
  pub parent_id: Option<VersionId>,
  /// Collection of packages that the publisher produced
  pub artifacts: Vec<AnagolayArtifactStructure<T>>,
}

/// Implementation of Default trait for AnagolayVersionData
impl<T: ArtifactType> Default for AnagolayVersionData<T> {
  fn default() -> Self {
    AnagolayVersionData {
      entity_id: None,
      parent_id: None,
      artifacts: vec![],
    }
  }
}

/// Implementation of AnagolayStructureData trait for AnagolayVersionData
impl<T: ArtifactType> AnagolayStructureData for AnagolayVersionData<T> {
  /// Validate the following constraints:
  /// * entity_id: If present, must be a valid CID
  /// * parent_id: If present, must be a valid CID
  /// * artifacts: For each artifact, the file_extension must not be empty and the ipfs_cid must be
  ///   a valid CID
  ///
  /// # Return
  /// An unit result if the validation is successful, a `Character` error with a description in
  /// case it fails
  fn validate(&self) -> Result<(), Characters> {
    if let Some(entity_id) = &self.entity_id {
      entity_id.validate().map_err(|err| {
        Characters::from(type_name_of_val(&self))
          .concat(".entity_id: ")
          .concat(err.as_str())
      })?;
    }
    if let Some(parent_id) = &self.parent_id {
      parent_id.validate().map_err(|err| {
        Characters::from(type_name_of_val(&self))
          .concat(".parent_id: ")
          .concat(err.as_str())
      })?;
    }
    if let Some((index, artifact)) = &self
      .artifacts
      .iter()
      .enumerate()
      .find(|(_, artifact)| artifact.ipfs_cid.validate().is_err() || artifact.file_extension.len() == 0)
    {
      let message = Characters::from(type_name_of_val(&self))
        .concat(".artifacts[")
        .concat_u8(*index as u8)
        .concat("]");
      if artifact.file_extension.len() == 0 {
        return Err(message.concat(".file_extension: cannot be empty".into()));
      } else {
        artifact
          .ipfs_cid
          .validate()
          .map_err(|err| message.concat(".ipfs_cid: ").concat(err.as_str()))?;
      }
    }
    Ok(())
  }
}

/// WASM artifacts commonly produced for a published entity. The subtype should be passed as
/// parameter of the entity-defined artifact type enumeration, like in the example:
///
/// # Examples
///
/// ```
/// use codec::{Decode, Encode};
/// use anagolay_support::{ArtifactType, WasmArtifactSubType};
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum OperationArtifactType {
///   CRATE, WASM(WasmArtifactSubType), DOCS, GIT
/// }
/// impl ArtifactType for OperationArtifactType {}
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum WorkflowArtifactType {
///   CRATE, WASM(WasmArtifactSubType), DOCS, GIT
/// }
/// impl ArtifactType for WorkflowArtifactType {}
///
/// let op_esm_artifact_type = OperationArtifactType::WASM(WasmArtifactSubType::ESM);
/// let wf_esm_artifact_type = WorkflowArtifactType::WASM(WasmArtifactSubType::ESM);
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum WasmArtifactSubType {
  /// CommonJS module for the direct use in the nodejs env which doesn't have the ESM support. When
  /// Nodejs has native ESM support this should be used only for the legacy versions. Check
  /// [here](https://nodejs.org/api/esm.html) the Nodejs ESM status.
  CJS,
  /// Native ES module, usually used with bundler software like webpack. You can use this just by
  /// including it, the wasm will be instantiated on require time. Example can be found
  /// [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html) and official
  /// docs [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#bundlers).
  /// For the official NODEJS support see [this doc](https://nodejs.org/api/esm.html)
  /// If you want to use this with nodejs, use the bundler.
  ESM,
  /// This is an ES module with manual instantiation of the wasm. It doesn't include polyfills
  /// More info is on the
  /// [wasm-pack doc website](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#without-a-bundler)
  /// and [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/reference/browser-support.html)
  /// # Example in Javascript
  ///
  /// ```javascript
  /// import init, { execute } from './op-file'
  /// async function main() {
  ///   await init() //initialize wasm
  ///   const e = execute([new Uint8Array(7)], new Map())
  ///   console.log(e.decode());
  /// }
  /// main().catch(console.error)
  /// ```
  WEB,
  /// Just a compiled WASM file without any acompanied JS or `.d.ts` files. You have to do all
  /// things manual.
  WASM,
}
