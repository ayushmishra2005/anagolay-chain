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
use crate::{OperationId, VersionId};
use anagolay_support::{
  AnagolayPackageStructure, AnagolayRecord, AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra,
  ArtifactType, Characters, ForWhat, GenericId,
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
  /// max 128(0.12kb) characters, slugify to use _
  pub name: Characters,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
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
  /// The fully qualified URL for the repository, this can be any public repo URL
  pub repository: Characters,
  /// Short name of the license, like "Apache-2.0"
  pub license: Characters,
  /// Indicator of the capability of the Operation to work in no-std environment
  pub nostd: bool,
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
      nostd: false,
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
pub type OperationRecord<T> =
  AnagolayRecord<Operation, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;

/// Operation Version artifact types. This enum corresponds to the different types of
/// packages created by the publisher service when an Operation Version is published
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum OperationArtifactType {
  /// Rust crate
  CRATE,
  /// CommonJS module for the direct use in the nodejs env which doesn't have the ESM support. When
  /// Nodejs has native ESM support this should be used only for the legacy versions. Check
  /// [here](https://nodejs.org/api/esm.html) the Nodejs ESM status.
  CJS,
  /// Just a compiled WASM file without any acompanied JS or `.d.ts` files. You have to do all
  /// things manual.
  WASM,
  /// Native ES module, usually used with bundler software like webpack. You can use this just by
  /// including it, the wasm will be instatiated on require time. Example can be found [here](https://rustwasm.github.io/docs/wasm-bindgen/examples/hello-world.html) and official docd [here](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#bundlers). For the official NODEJS support see [this doc](https://nodejs.org/api/esm.html) If you want to use this with nodejs, use the bundler.
  ESM,
  /// This is an ES module with manual instatiation of the wasm. It doesn't include polyfills
  /// More info is on the
  /// [wasm-pack doc website](https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html#without-a-bundler)
  /// and [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/reference/browser-support.html)
  /// # Example in Javascript
  ///
  /// ```javascript
  /// import init, { execute } from './op-file'
  /// async function main() {
  ///   await init() //initialize wasm
  ///   const e =   execute([new Uint8Array(7)], new Map())
  ///   console.log(e.decode());
  /// }
  /// main().catch(console.error)
  /// ```
  WEB,
}

impl ArtifactType for OperationArtifactType {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
/// Extra information (non hashed) for Operation Version entity
pub struct OperationVersionExtra {
  pub created_at: u64,
}
/// Implementation of AnagolayStructureExtra trait for OperationVersionExtra
impl AnagolayStructureExtra for OperationVersionExtra {}

/// Operation version data. This contains all the needed parameters which define the Operation
/// Version and is hashed to produce its id
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationVersionData {
  /// The id of the Operation to which this Operation Version is associated.
  /// __This field is read-only__
  pub operation_id: OperationId,
  /// The id of the previous Operation Version for the same operation, if any.
  pub parent_id: Option<VersionId>,
  /// The IPFS cid of the repository rehosting the original one specified in the Operation structure
  pub rehosted_repo_id: GenericId,
  /// The IPFS cid of the documentation
  pub documentation_id: GenericId,
  /// Collection of packages that the publisher produced
  pub packages: Vec<AnagolayPackageStructure<OperationArtifactType>>,
}

/// Implementation of Default trait for OperationVersionData
impl Default for OperationVersionData {
  fn default() -> Self {
    OperationVersionData {
      operation_id: vec![],
      parent_id: None,
      rehosted_repo_id: vec![],
      documentation_id: vec![],
      packages: vec![],
    }
  }
}

/// Implementation of AnagolayStructureData trait for OperationVersionData
impl AnagolayStructureData for OperationVersionData {}

/// OperationVersion entity, alias of `AnagolayStructure<OperationData, OperationExtra>`
pub type OperationVersion = AnagolayStructure<OperationVersionData, OperationVersionExtra>;

/// This is the Storage record of Operation Version.
pub type OperationVersionRecord<T> =
  AnagolayRecord<OperationVersion, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;
