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

//! Pallet types.
//!
//! Each pallet must have this file.

// use super::*;
use anagolay_support::{
  AnagolayRecord, AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, AnagolayVersionData,
  AnagolayVersionExtra, ArtifactType, Characters, ForWhat, WasmArtifactSubType,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, collections::btree_map::BTreeMap, default::Default, vec, vec::Vec};

/// Textual representation of a type
/// @TODO we might have an issue with this because this can be anything and it transforms into the
/// `string[]` in typescript which is not the same as String
pub type TypeName = Vec<u8>;

/// Operation data. This contains all the needed parameters which define the Operation and is hashed
/// to produce its id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationData {
  /// Operation name. min 8, max 128(0.12kb) characters, slugify to use _
  pub name: Characters,
  /// Description can be markdown but not html. min 8, max 1024(1kb) chars
  pub description: Characters,
  /// What operation accepts in the implementation. these are the params of the function with the
  /// types
  pub inputs: Vec<TypeName>,
  /// A map where keys are names of configuration parameters and values are collections of strings
  /// representing allowed values
  pub config: BTreeMap<Characters, Vec<Characters>>,
  /// A switch used to generate the Workflow segments  
  pub groups: Vec<ForWhat>,
  /// Data type name defining the operation output
  pub output: TypeName,
  /// The fully qualified URL for the repository, this can be any public repo URL. min 8, max
  /// 128(0.12kb) characters
  pub repository: Characters,
  /// Short name of the license, like "Apache-2.0". min 8, max 128(0.12kb) characters,
  pub license: Characters,
  /// Indicator of the capability of the Operation to work in no-std environment
  pub nostd: bool,
}

/// Implementation of Default trait for OperationData
impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: "".into(),
      description: "".into(),
      inputs: vec![],
      config: BTreeMap::new(),
      groups: vec![],
      output: vec![],
      repository: "".into(),
      license: "".into(),
      nostd: false,
    }
  }
}

/// Implementation of AnagolayStructureData trait for OperationData
impl AnagolayStructureData for OperationData {
  fn validate(&self) -> Result<(), Characters> {
    if self.name.len() < 4 || self.name.len() > 128 {
      Err("OperationData.name: length must be between 4 and 128 characters".into())
    } else if self.description.len() < 4 || self.description.len() > 1024 {
      Err("OperationData.description: length must be between 4 and 1024 characters".into())
    } else if self.repository.len() < 4 || self.repository.len() > 128 {
      Err("OperationData.repository: length must be between 4 and 128 characters".into())
    } else if self.license.len() < 4 || self.license.len() > 128 {
      Err("OperationData.license: length must be between 4 and 128 characters".into())
    } else {
      Ok(())
    }
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
/// Extra information (non hashed) for Operation entity
pub struct OperationExtra {}
/// Implementation of AnagolayStructureExtra trait for OperationExtra
impl AnagolayStructureExtra for OperationExtra {}
/// Implementation of Default trait for OperationExtra
impl Default for OperationExtra {
  fn default() -> Self {
    OperationExtra {}
  }
}

/// Operation entity, alias of `AnagolayStructure<OperationData, OperationExtra>`
pub type Operation = AnagolayStructure<OperationData, OperationExtra>;

/// This is the Storage record of Operation
pub type OperationRecord<T> =
  AnagolayRecord<Operation, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;

/// Operation Version artifact types. This enum corresponds to the different types of
/// packages created by the publisher service when an Operation Version is published
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum OperationArtifactType {
  /// Rust crate. This is a `tar.gz` of the git repo not containing the `.git*` and `target`   
  CRATE,
  /// Wasm artifacts built by the wasm-pack. They are split in subtypes where every type contains
  /// the same wasm file, and also includes the various `.js` and `.d.ts` files to increase
  /// developers experience
  WASM(WasmArtifactSubType),
  /// This refers to the documentation generated by the `cargo docs`. The entry point is predictable
  /// and always will be in following format `${ipfs_cid}/${manifest.data.name}/index.html`
  DOCS,
  /// Original repository that was rehosted
  GIT,
}

impl ArtifactType for OperationArtifactType {}

/// Alias for the data type of the Workflow version
pub type OperationVersionData = AnagolayVersionData<OperationArtifactType>;

/// `OperationVersion` type, alias of
/// [`AnagolayStructure<WorkflowVersionData,AnagolayVersionExtra>`]
pub type OperationVersion = AnagolayStructure<OperationVersionData, AnagolayVersionExtra>;

/// This is the Storage record of Operation Version.
pub type OperationVersionRecord<T> =
  AnagolayRecord<OperationVersion, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;
