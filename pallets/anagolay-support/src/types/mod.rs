// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.
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

/// Package that contains the types dealing with ids
pub mod ids;
pub use ids::*;

/// Package that contains the types dealing with strings of characters
pub mod characters;
pub use characters::*;

/// Package that contains a serializable bounded map
pub mod maps;
pub use maps::*;

/// Package that contains the types dealing with rpc
pub mod rpc;

use crate::{anagolay_generic_id, getter_for_hardcoded_constant, types::ids::GenericId};
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;

getter_for_hardcoded_constant!(MaxArtifactsPerVersion, u32, 8);

/// Placeholder for SSI and DID
pub type CreatorId = Characters;

// The type of the values in the `ArtifactsByArtifactId` storage
anagolay_generic_id!(Artifact);

/// Textual representation of a type
pub type TypeName = Characters;

/// List of equipment that needs workflows generated
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Ord, PartialOrd, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub enum ForWhat {
  /// We are creating it For what? This can be a part of the group
  #[default]
  GENERIC, // 0
  PHOTO,       // 1
  CAMERA,      // 2
  LENS,        // 3
  SMARTPHONE,  // 4
  USER,        // 5
  SYS,         // 6
  FLOWCONTROL, // 7
}

/// This macro produces a record for an entity to be stored on chain. The struct defines the
/// following fields:
///  * record: the entity
///  * account_id: the owner of the entity
///  * block_number: the number of the block where the entity was stored on chain
///
/// According to the identifier `n` passed in as argument, the resulting struct will be called
/// `<n>Record`
///
/// # Arguments
///  * n - The identifier of the entity
#[macro_export]
macro_rules! anagolay_record {
  ( $n:ident ) => {
    $crate::paste::paste! {
      /// Record of an Anagolay entity that gets stored on chain along with owner `AccountId` and insertion `BlockNumber`
      #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
      #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
      #[scale_info(skip_type_params(T))]
      pub struct [<$n Record>] <T: frame_system::Config> {
        pub record: $n,
        pub account_id: T::AccountId,
        pub block_number: T::BlockNumber,
      }
    }
  };
}

/// This macro produces an Anagolay entity to be stored on chain. The struct defines the following
/// fields:
///  * id: the entity id
///  * data: the invariable, hashed AnagolayStructureData
///  * extra: an additional AnagolayStructureExtra struct that contains non-hashed, entity-specific
///    fields
///
/// According to the name `n` passed in as argument, the resulting struct will be called `<n>` and
/// will require three types for its fields as argument
///
/// # Arguments
///  * n - The name of the entity
///  * i - The type of the id of the entity
///
/// # Examples
///
/// ```
/// # use anagolay_support::MaxGenericIdLenGet;
/// use frame_support::pallet_prelude::*;
/// use codec::{Decode, Encode};
/// use anagolay_support::{anagolay_structure, anagolay_generic_id, AnagolayStructureData, AnagolayStructureExtra, Characters, generic_id::GenericId};
///
/// anagolay_generic_id!(Entity);
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
/// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
/// pub struct EntityData {
///   text: Characters
/// };
///
/// impl Default for EntityData {
///   fn default() -> Self {
///     EntityData {
///       text: Characters::default()
///     }
///   }
/// }
///
/// impl AnagolayStructureData for EntityData {
///   type Id = EntityId;
///
///   fn validate(&self) -> Result<(), Characters> {
///      Ok(())
///   }
/// }
///
/// #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
/// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
/// pub struct EntityExtra {
///   created_at: u64
/// };
///
/// impl AnagolayStructureExtra for EntityExtra {}
///
/// anagolay_structure!(Entity, EntityId, EntityData, EntityExtra);
///
/// let entity = Entity::new_with_extra(EntityData {
///   text: "hello".into()
/// }, EntityExtra {
///   created_at: 0
/// });
///
/// assert_eq!(Characters::from("hello"), entity.data.text);
/// assert!(entity.extra.is_some());
/// assert_eq!(0, entity.extra.unwrap().created_at);
#[macro_export]
macro_rules! anagolay_structure {
  ( $n:ident, $i:ty, $d:ty, $e:ty ) => {
    $crate::paste::paste! {
      /// Generic Anagolay structure representing composed of `id`, `data` and `extra` fields.
      /// Refer to [`anagolay_structure`] macro for the details
      #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
      #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
      pub struct $n {
        pub id: $i,
        pub data: $d,
        pub extra: Option<$e>,
      }

      impl Default for $n {
        fn default() -> Self {
          $n {
            id: $i::default(),
            data: $d::default(),
            extra: None,
          }
        }
      }

      impl $n {
        /// Produces an Anagolay structure with no extra information.
        pub fn new(data: $d) -> Self {
          $n {
            id: data.to_cid(),
            data,
            extra: None,
          }
        }

        /// Produces a an Anagolay structure with some extra information
        pub fn new_with_extra(data: $d, extra: $e) -> Self {
          $n {
            id: data.to_cid(),
            data,
            extra: Some(extra),
          }
        }
      }
    }
  }
}

/// This macro produces the hashed data structure of the Version of an entity stored on chain. The
/// struct defines the following fields:
///  * entity_id: the id of the entity
///  * parent_id: the id of the previous Version for the same entity
///  * artifacts: Collection of packages that the publisher produced
///
/// According to the name `n` passed in as argument, the resulting struct will be called `<n>Data`
///
/// # Arguments
///  * n - The name of the entity version
///  * i - The type of the id of the entity
///  * a - The type of the artifacts
#[macro_export]
macro_rules! anagolay_version_data {
  ( $n:ident, $i:ty, $j:ty, $a:ty ) => {
    $crate::paste::paste! {
      impl ArtifactType for $a {}

      /// Anagolay Version data. It contains all the needed parameters which define the Version and is
      /// hashed to produce the Version id. Refer to [`anagolay_version_data`] macro for more details.
      ///
      /// # Examples
      ///
      /// ```
      /// #![feature(type_name_of_val)]
      ///
      /// use frame_support::pallet_prelude::*;
      /// use codec::{Decode, Encode};
      /// use anagolay_support::{*, generic_id::GenericId};
      ///
      /// # struct Operation {}
      ///
      /// anagolay_generic_id!(Operation);
      ///
      /// anagolay_generic_id!(OperationVersion);
      ///
      /// #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
      /// #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
      /// #[cfg_attr(feature = "std", serde(rename_all(deserialize = "camelCase")))]
      /// pub enum OperationArtifactType {
      ///   Wasm, Docs, Git
      /// }
      ///
      /// anagolay_version_data!(OperationVersion, OperationVersionId, OperationId, OperationArtifactType);
      /// anagolay_version_extra!(OperationVersion);
      /// anagolay_structure!(OperationVersion, OperationVersionId, OperationVersionData, OperationVersionExtra);
      /// ```
      #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
      #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
      pub struct [<$n Data>] {
        /// The id of the Operation, Workflow or other entity to which this Version is
        /// associated. __This field is read-only__
        pub entity_id: Option<$j>,
        /// The id of the previous Operation Version for the same operation, if any.
        pub parent_id: Option<$i>,
        /// Collection of packages that the publisher produced
        pub artifacts: BoundedVec<AnagolayArtifactStructure<$a>, MaxArtifactsPerVersionGet>,
      }

      /// Implementation of Default trait for
      impl Default for [<$n Data>] {
        fn default() -> Self {
          [<$n Data>] {
            entity_id: None,
            parent_id: None,
            artifacts: BoundedVec::with_bounded_capacity(0),
          }
        }
      }

      /// Implementation of AnagolayStructureData trait
      impl AnagolayStructureData for [<$n Data>] {
        type Id = $i;

        fn validate(&self) -> Result<(), Characters> {
          if let Some(entity_id) = &self.entity_id {
            entity_id.validate().map_err(|err| {
              Characters::from(core::any::type_name_of_val(&self))
                .concat(".entity_id: ")
                .concat(err.as_str())
            })?;
          }
          if let Some(parent_id) = &self.parent_id {
            parent_id.validate().map_err(|err| {
              Characters::from(core::any::type_name_of_val(&self))
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
            let message = Characters::from(core::any::type_name_of_val(&self))
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
    }
  };
}

/// This macro produces the extra structure of the Version of an entity stored on chain. The struct
/// defines the following fields:
///  * created_at: the creation timestamp
///
/// According to the name `n` passed in as argument, the resulting struct will be called `<n>Extra`
///
/// # Arguments
///  * n - The name of the entity version
#[macro_export]
macro_rules! anagolay_version_extra {
  ( $n:ident ) => {
    $crate::paste::paste! {
      #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
      #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
      /// Extra information (non hashed) for the Anagolay Version
      pub struct [<$n Extra>] {
        pub created_at: u64,
      }

      /// Implementation of AnagolayStructureExtra trait
      impl AnagolayStructureExtra for [<$n Extra>] {}
    }
  };
}

/// The trait for the data field of an Anagolay entity.
pub trait AnagolayStructureData: Default + Encode + Clone + PartialEq + Eq {
  type Id: GenericId;
  /// Computes cid of the data, after encoding it using parity SCALE codec
  ///
  /// # Examples
  ///
  /// ```
  /// # use anagolay_support::MaxGenericIdLenGet;
  /// use frame_support::pallet_prelude::*;
  /// use codec::{Decode, Encode};
  /// use anagolay_support::{anagolay_generic_id, AnagolayStructureData, AnagolayStructureExtra, Characters, generic_id::GenericId};
  ///
  /// anagolay_generic_id!(Entity);
  ///
  /// #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  /// struct EntityData {
  ///   text: Characters
  /// };
  ///
  /// impl Default for EntityData {
  ///   fn default() -> Self {
  ///     EntityData {
  ///       text: Characters::default()
  ///     }
  ///   }
  /// }
  ///
  /// impl AnagolayStructureData for EntityData {
  ///   type Id = EntityId;
  ///
  ///   fn validate(&self) -> Result<(), Characters> {
  ///      Ok(())
  ///   }
  /// }
  ///
  /// let entity = EntityData {
  ///   text: "hello".into()
  /// };
  ///
  /// let cid = entity.to_cid();
  /// # assert_eq!(EntityId::from("bafkr4iac2luovbttsv5iftbg2zl4okalixafa2vjwtbmf6exgwiuvukhmi"), cid);
  /// ```
  fn to_cid(&self) -> Self::Id {
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
      Self::Id::from(cid_str.as_str())
    } else {
      Self::Id::default()
    }
  }

  /// Validate the following constraints:
  /// * entity_id: If present, must be a valid CID
  /// * parent_id: If present, must be a valid CID
  /// * artifacts: For each artifact, the file_extension must not be empty and the ipfs_cid must be
  ///   a valid CID
  ///
  /// # Return
  /// An unit result if the validation is successful, a `Character` error with a description in
  /// case it fails
  fn validate(&self) -> Result<(), Characters>;
}

/// The trait for the extra field of an Anagolay entity
pub trait AnagolayStructureExtra: Clone + PartialEq + Eq {}

/// Trait used as type parameter in [`AnagolayArtifactStructure`], allowing different structures to
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
#[cfg_attr(feature = "std", serde(rename_all(deserialize = "camelCase")))]
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
  Web,
}
