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

/// Private package used to hide the types that are not supposed to be referenced by dependent
/// crates
mod private;

/// Package that contains the types dealing with strings of characters
pub mod characters;
pub use characters::*;

/// Package that contains a serializable bounded map
pub mod maps;
pub use maps::*;

use self::private::GenericId;
use crate::getter_for_hardcoded_constant;
use codec::{Decode, Encode};
use core::any::type_name_of_val;
use frame_support::pallet_prelude::*;

getter_for_hardcoded_constant!(MaxArtifactsPerVersion, u32, 8);

/// Placeholder for SSI and DID
pub type CreatorId = Characters;

/// The type of the values in the `ArtifactsByArtifactId` storage
pub type ArtifactId = GenericId;

/// The type used for an Operation ID
pub type OperationId = GenericId;

/// The type used for a Workflow ID
pub type WorkflowId = GenericId;

/// The type used for a Statement ID
pub type StatementId = GenericId;

/// The type used for a Proof ID
pub type ProofId = GenericId;

/// The type used for a Signature ID
pub type SignatureId = GenericId;

/// The type used for any entity Version ID
pub type VersionId = GenericId;

/// Textual representation of a type
pub type TypeName = Characters;

/// List of equipment that needs workflows generated
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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
  fn to_cid(&self) -> GenericId {
    extern crate alloc;
    use alloc::{rc::Rc, vec, vec::*};
    use core::any::Any;

    let inputs: Vec<Rc<dyn Any>> = vec![Rc::new(self.encode())];
    let workflow = wf_cidv1_from_array::Workflow::new();
    if let Ok(result) = workflow.next(inputs) {
      let result = result.as_ref();
      let cid_str = result
        .get_output()
        .unwrap()
        .downcast_ref::<alloc::string::String>()
        .unwrap()
        .clone();
      GenericId::from(&cid_str)
    } else {
      GenericId::default()
    }
  }

  fn validate(&self) -> Result<(), Characters>;
}

/// The trait for the extra field of an Anagolay entity
pub trait AnagolayStructureExtra: Clone + PartialEq + Eq {}

/// Generic structure representing an Anagolay entity
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AnagolayStructure<T: AnagolayStructureData, U: AnagolayStructureExtra> {
  pub id: GenericId,
  pub data: T,
  pub extra: Option<U>,
}

impl<T: AnagolayStructureData, U: AnagolayStructureExtra> Default for AnagolayStructure<T, U> {
  fn default() -> Self {
    AnagolayStructure {
      id: GenericId::default(),
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
///   Wasm, Docs, Git
/// }
///
/// impl ArtifactType for OperationArtifactType {}
///
/// type OperationPackageStructure = AnagolayArtifactStructure<OperationArtifactType>;
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum ImageArtifactType {
///   Raw
/// }
///
/// impl ArtifactType for ImageArtifactType {}
///
/// type ImagePackageStructure = AnagolayArtifactStructure<ImageArtifactType>;
/// ```
pub trait ArtifactType: Encode + Decode + Clone + PartialEq + Eq {}

/// Artifact Structure, used as a single item in the list of stored Artifacts
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AnagolayArtifactStructure<T: ArtifactType> {
  /// Type of the artifact
  pub artifact_type: T,
  /// Extension of the stored file
  pub file_extension: Characters,
  /// IPFS cid
  pub ipfs_cid: ArtifactId,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
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
///   Wasm, Docs, Git
/// }
/// impl ArtifactType for OperationArtifactType {}
///
/// type OperationVersionData = AnagolayVersionData<OperationArtifactType>;
/// type OperationVersion = AnagolayStructure<OperationVersionData, AnagolayVersionExtra>;
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct AnagolayVersionData<T: ArtifactType> {
  /// The id of the Operation, Workflow or other entity to which this Version is
  /// associated. __This field is read-only__
  pub entity_id: Option<GenericId>,
  /// The id of the previous Operation Version for the same operation, if any.
  pub parent_id: Option<VersionId>,
  /// Collection of packages that the publisher produced
  pub artifacts: BoundedVec<AnagolayArtifactStructure<T>, MaxArtifactsPerVersionGet>,
}

/// Implementation of Default trait for AnagolayVersionData
impl<T: ArtifactType> Default for AnagolayVersionData<T> {
  fn default() -> Self {
    AnagolayVersionData {
      entity_id: None,
      parent_id: None,
      artifacts: BoundedVec::with_bounded_capacity(0),
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
///   Wasm(WasmArtifactSubType), Docs, Git
/// }
/// impl ArtifactType for OperationArtifactType {}
/// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
/// enum WorkflowArtifactType {
///   Wasm(WasmArtifactSubType), Docs, Git
/// }
/// impl ArtifactType for WorkflowArtifactType {}
///
/// let op_esm_artifact_type = OperationArtifactType::Wasm(WasmArtifactSubType::Esm);
/// let wf_esm_artifact_type = WorkflowArtifactType::Wasm(WasmArtifactSubType::Esm);
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum WasmArtifactSubType {
  /// CommonJS module for the direct use in the nodejs env which doesn't have the ESM support. When
  /// Nodejs has native ESM support this should be used only for the legacy versions. Check
  /// [here](https://nodejs.org/api/esm.html) the Nodejs ESM status.
  Cjs,
  /// Native ES module, usually used with bundler software like webpack. You can use this just by
  /// including it, the wasm will be instantiated on require time. Example can be found
  /// [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html) and official
  /// docs [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#bundlers).
  /// For the official NODEJS support see [this doc](https://nodejs.org/api/esm.html)
  /// If you want to use this with nodejs, use the bundler.
  Esm,
  /// Just a compiled WASM file without any acompanied JS or `.d.ts` files. You have to do all
  /// things manual.
  Wasm,
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
  Web,
}
