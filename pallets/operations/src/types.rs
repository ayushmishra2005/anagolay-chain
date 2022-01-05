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
  AnagolayStructure, AnagolayStructureData, AnagolayStructureExtra, ForWhat, GenericId,
};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::{clone::Clone, default::Default, vec, vec::Vec};

///Operation output definition, more info here https://gitlab.com/anagolay/node/-/issues/27
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationOutput {
  desc: Vec<u8>,
  ///Any valid type from the chain, written as string and converted to the appropriate type in the implementation
  output: Vec<u8>,
  decoded: Vec<u8>,
}

/// Input params for a generated implementation
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct CustomInputParam {
  ///  'AnByteArray' | 'ProofParams[]' | 'AnBoolean'
  data: Vec<u8>,
  /// The real data type check the outputDecoded in anagolay SDK, for more info check the https://gitlab.com/anagolay/node/-/issues/27
  decoded: Vec<u8>,
}

/// Operation structure. This contains all the needed parameters which define the operation.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
// #[cfg_attr(feature = "std", derive(Debug))]
pub struct OperationData {
  /// max 128(0.12kb) characters, slugify to use _
  name: Vec<u8>,
  /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
  desc: Vec<u8>,
  /// what operation accepts in the implementation. these are the params of the function with the types
  input: Vec<CustomInputParam>,
  output: OperationOutput,
  hashing_op: GenericId,
  enc_op: GenericId,
  groups: Vec<ForWhat>,
  /// this is the sum of all ops and the ops of the ops. tells how many operations this operation has. Based on this number we will decide which op is going to be executed first. This also tells which op has the longest chain or the deepest child op
  priority: u32,
  /// you can use the ops to build more complex rule and more complex op
  ops: Vec<Operation>,
}

impl AnagolayStructureData for OperationData {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct OperationExtra {}
impl AnagolayStructureExtra for OperationExtra {}

impl Default for OperationData {
  fn default() -> Self {
    OperationData {
      name: b"".to_vec(),
      desc: b"".to_vec(),
      input: vec![],
      output: OperationOutput::default(),
      hashing_op: b"an_cid".to_vec(),
      enc_op: b"an_enc_hex".to_vec(),
      groups: vec![ForWhat::SYS],
      priority: 0,
      ops: vec![],
    }
  }
}

pub type Operation = AnagolayStructure<OperationData, OperationExtra>;
