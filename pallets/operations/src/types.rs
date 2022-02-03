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

// use super::*;
use anagolay::{
  AnagolayRecord, AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, Characters,
  ForWhat, GenericId,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, collections::btree_map::BTreeMap, default::Default, vec, vec::Vec};

/// Textual representation of a type
pub type TypeName = Vec<u8>;

/// Operation data. This contains all the needed parameters which define the Operation and is hashed
/// to produce its id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationData {
  /// max 128(0.12kb) characters, slugify to use _
  pub name: Characters,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
  pub description: Characters,
  /// What operation accepts in the implementation. these are the params of the function with the types
  pub inputs: Vec<TypeName>,
  /// A map where keys are names of configuration parameters and values are collections of strings representing allowed values
  pub config: BTreeMap<Characters, Vec<Characters>>,
  /// A switch used to generate the Workflow segments  
  pub groups: Vec<ForWhat>,
  /// Data type name defining the operation output
  pub output: TypeName,
  /// The fully qualified URL for the repository, this can be any public repo URL
  pub repository: Characters,
  /// Short name of the license, like "Apache-2.0"
  pub license: Characters,
}

/// Implementation of Default trait for OperationData
impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: vec![],
      description: vec![],
      inputs: vec![],
      config: BTreeMap::new(),
      groups: vec![],
      output: vec![],
      repository: vec![],
      license: vec![],
    }
  }
}

/// Implementation of AnagolayStructureData trait for OperationData
impl AnagolayStructureData for OperationData {}

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
pub type OperationRecord<T> = AnagolayRecord<
  Operation,
  <T as frame_system::Config>::AccountId,
  <T as frame_system::Config>::BlockNumber,
>;

/// Operation Version package types. This enum corresponds to the different types of
/// packages created by the publisher service when an Operation Version is published
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum PackageType {
  /// Rust crate
  Crate,
  /// Web Assemby Module
  Wasm,
  /// ECMAScript Module
  Esm,
}

/// Operation Version package
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionPackage {
  /// Type of the package
  pub package_type: PackageType,
  /// Name of the file
  pub file_name: Characters,
  /// IPFS cid
  pub ipfs_cid: GenericId,
}

/// Operation Version data. This contains all the needed parameters which define the Operation
/// Version and is hashed to produce its id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionData {
  /// The id of the Operation to which this Operation Version is associated.
  /// __This field is read-only__
  pub operation_id: GenericId,
  /// The id of the previous Operation Version for the same operation, if any.
  pub parent_id: Option<GenericId>,
  /// The IPFS cid of the repository rehosting the original one specified in the Operation structure
  pub rehosted_repo_id: GenericId,
  /// Collection of packages that the publisher produced
  pub packages: Vec<OperationVersionPackage>,
}

/// Implementation of Default trait for OperationVersionData
impl Default for OperationVersionData {
  fn default() -> Self {
    OperationVersionData {
      operation_id: vec![],
      parent_id: None,
      rehosted_repo_id: vec![],
      packages: vec![],
    }
  }
}

/// Implementation of AnagolayStructureData trait for OperationVersionData
impl AnagolayStructureData for OperationVersionData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
/// Extra information (non hashed) for Operation Version entity
pub struct OperationVersionExtra {
  pub created_at: u64,
}
/// Implementation of AnagolayStructureExtra trait for OperationVersionExtra
impl AnagolayStructureExtra for OperationVersionExtra {}

/// OperationVersion entity, alias of `AnagolayStructure<OperationData, OperationExtra>`
pub type OperationVersion = AnagolayStructure<OperationVersionData, OperationVersionExtra>;

/// This is the Storage record of Operation Version.
pub type OperationVersionRecord<T> = AnagolayRecord<
  OperationVersion,
  <T as frame_system::Config>::AccountId,
  <T as frame_system::Config>::BlockNumber,
>;
