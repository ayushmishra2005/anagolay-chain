// This file is part of Anagolay Foundation.

// Copyright (C) 2019-2021 Anagolay Foundation.
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

/// Operation structure. This contains all the needed parameters which define the operation.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationData {
  /// max 128(0.12kb) characters, slugify to use _
  name: Characters,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
  description: Characters,
  /// What operation accepts in the implementation. these are the params of the function with the types
  input: Vec<TypeName>,
  /// A map where keys are names of configuration parameters and values are collections of strings representing allowed values
  config: BTreeMap<Characters, Vec<Characters>>,
  /// A switch used to generate the Workflow segments  
  groups: Vec<ForWhat>,
  /// Data type name defining the operation output
  output: TypeName,
  /// The fully qualified URL for the repository, this can be any public repo URL
  repository: Characters,
  /// Short name of the license, like "Apache-2.0"
  license: Characters,
}

impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: vec![],
      description: vec![],
      input: vec![],
      config: BTreeMap::new(),
      groups: vec![],
      output: vec![],
      repository: vec![],
      license: vec![],
    }
  }
}

impl AnagolayStructureData for OperationData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationExtra {}
impl AnagolayStructureExtra for OperationExtra {}

pub type Operation = AnagolayStructure<OperationData, OperationExtra>;
pub type OperationVersionRecord<T> = AnagolayRecord<
  OperationVersion,
  <T as frame_system::Config>::AccountId,
  <T as frame_system::Config>::BlockNumber,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum PackageType {
  Crate,
  Wasm,
  Esm,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionPackage {
  pub package_type: PackageType,
  pub file_url: Characters,
  pub ipfs_cid: GenericId,
}

/// Operation Version structure. This contains all the needed parameters which define the operation version.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionData {
  pub operation_id: GenericId,
  pub parent_id: GenericId,
  pub rehosted_repo: Characters,
  pub packages: Vec<OperationVersionPackage>,
}

impl Default for OperationVersionData {
  fn default() -> Self {
    OperationVersionData {
      operation_id: vec![],
      parent_id: vec![],
      rehosted_repo: vec![],
      packages: vec![],
    }
  }
}

impl AnagolayStructureData for OperationVersionData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionExtra {
  pub created_at: u128,
}
impl AnagolayStructureExtra for OperationVersionExtra {}

pub type OperationVersion = AnagolayStructure<OperationVersionData, OperationVersionExtra>;
pub type OperationRecord<T> = AnagolayRecord<
  Operation,
  <T as frame_system::Config>::AccountId,
  <T as frame_system::Config>::BlockNumber,
>;
