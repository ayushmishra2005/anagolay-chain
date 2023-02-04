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

//! Pallet types.
//!
//! Each pallet must have this file.

use anagolay_support::{constants::*, generic_id::GenericId, *};
use codec::{Decode, Encode};
use frame_support::{
  pallet_prelude::*,
  sp_runtime::{BoundedVec, RuntimeDebug},
  sp_std::{clone::Clone, default::Default},
};

getter_for_hardcoded_constant!(MaxOperationConfigValuesPerEntry, u32, 16);
getter_for_hardcoded_constant!(MaxOperationFeatures, u32, 16);

// Operation id
anagolay_generic_id!(Operation);

// OperationVersion id
anagolay_generic_id!(OperationVersion);

/// Operation data. This contains all the needed parameters which define the Operation and is hashed
/// to produce its id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct OperationData {
  /// Operation name. min 8, max 128(0.12kb) characters, slugify to use _
  pub name: Characters,
  /// Description can be markdown but not html. min 8, max 1024(1kb) chars
  pub description: Characters,
  /// What operation accepts in the implementation. these are the params of the function with the
  /// types
  pub inputs: BoundedVec<TypeName, MaxOperationInputsLenGet>,
  /// A map where keys are names of configuration parameters and values are collections of strings
  /// representing allowed values
  pub config: MaybeSerializableBoundedBTreeMap<
    Characters,
    BoundedVec<Characters, MaxOperationConfigValuesPerEntryGet>,
    MaxOperationConfigEntriesGet,
  >,
  /// A switch used to generate the Workflow segments  
  pub groups: BoundedVec<ForWhat, MaxGroupsGet>,
  /// Data type name defining the operation output
  pub output: TypeName,
  /// The fully qualified URL for the repository, this can be any public repo URL. min 8, max
  /// 128(0.12kb) characters
  pub repository: Characters,
  /// Short name of the license, like "Apache-2.0". min 8, max 128(0.12kb) characters,
  pub license: Characters,
  /// Indicator of the features of the binary. Typically the following
  /// - `config_<key>` with _key_ coming from the config map allows conditional compilation of the
  ///   feature `config_<key>_<value>` where _value_ is the configuration selected at the moment the
  ///   operation is instantiated
  /// - `std` declares support for nostd as default and possibility to work with std. If this
  ///   feature is missing, the operation is intended to be working **only** in std
  pub features: BoundedVec<Characters, MaxOperationFeaturesGet>,
}

/// Implementation of Default trait for OperationData
impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: "".into(),
      description: "".into(),
      inputs: BoundedVec::with_bounded_capacity(0),
      config: MaybeSerializableBoundedBTreeMap::new(),
      groups: BoundedVec::with_bounded_capacity(0),
      output: "".into(),
      repository: "".into(),
      license: "".into(),
      features: BoundedVec::with_bounded_capacity(0),
    }
  }
}

/// Implementation of AnagolayStructureData trait for OperationData
impl AnagolayStructureData for OperationData {
  type Id = OperationId;

  fn validate(&self) -> Result<(), Characters> {
    if self.name.len() < 4 || self.name.len() > 128 {
      Err("OperationData.name: length must be between 4 and 128 characters".into())
    } else if self.description.len() < 4 || self.description.len() > 1024 {
      Err("OperationData.description: length must be between 4 and 1024 characters".into())
    } else if self.repository.len() < 4 || self.repository.len() > MaxCharactersLenGet::get() as usize {
      Err("OperationData.repository: length must be between 4 and MaxCharactersLen characters".into())
    } else if self.license.len() < 4 || self.license.len() > 128 {
      Err("OperationData.license: length must be between 4 and 128 characters".into())
    } else {
      Ok(())
    }
  }
}

/// Extra information (non hashed) for Operation entity
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct OperationExtra {}
/// Implementation of AnagolayStructureExtra trait for OperationExtra
impl AnagolayStructureExtra for OperationExtra {}
/// Implementation of Default trait for OperationExtra
impl Default for OperationExtra {
  fn default() -> Self {
    OperationExtra {}
  }
}

// Operation entity
anagolay_structure!(Operation, OperationId, OperationData, OperationExtra);

// This produces `OperationRecord<T>`, the Storage record of the Operation
anagolay_record!(Operation);

/// Operation Version artifact types. This enum corresponds to the different types of
/// packages created by the publisher service when an Operation Version is published
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all(deserialize = "camelCase")))]
pub enum OperationArtifactType {
  /// This refers to the documentation generated by the `cargo docs`. The entry point is predictable
  /// and always will be in following format `${ipfs_cid}/${manifest.data.name}/index.html`
  Docs,
  /// Original repository that was rehosted
  Git,
  /// Wasm artifacts built by the wasm-pack. They are split in subtypes where every type contains
  /// the same wasm file, and also includes the various `.js` and `.d.ts` files to increase
  /// developers experience
  Wasm(WasmArtifactSubType),
}

// The data type of the OperationVersion
anagolay_version_data!(OperationVersion, OperationVersionId, OperationId, OperationArtifactType);

// The extra type of the OperationVersion
anagolay_version_extra!(OperationVersion);

// OperationVersion entity
anagolay_structure!(
  OperationVersion,
  OperationVersionId,
  OperationVersionData,
  OperationVersionExtra
);

// This produces `OperationVersionRecord<T>`, the Storage record of the Operation Version.
anagolay_record!(OperationVersion);
