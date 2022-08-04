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

use anagolay_support::{constants::*, *};
use codec::{Decode, Encode};
use frame_support::{
  pallet_prelude::*,
  sp_runtime::RuntimeDebug,
  sp_std::{clone::Clone, default::Default},
};

getter_for_hardcoded_constant!(MaxOperationOutputLen, u32, 1024);
getter_for_hardcoded_constant!(MaxPHashLen, u32, 1024);
getter_for_hardcoded_constant!(MaxProofParams, u32, 16);

/// key-value where key is Operation.op and value is fn(Operation)
#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct ProofParams {
  /// Operation.name, hex encoded using Parity scale codec
  k: Characters,
  /// operation Output value serialized using cbor and represented as CID
  v: BoundedVec<u8, MaxOperationOutputLenGet>,
}

/// Perceptive hash information, what gets stored
#[derive(Encode, Decode, Clone, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct PhashInfo {
  /// The perceptive hash bytes
  pub p_hash: BoundedVec<u8, MaxPHashLenGet>,
  /// The id of the proof associated to this perceptive hash
  pub proof_id: ProofId,
}

/// Proof Incoming data
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct ProofData {
  /// The id of the Workflow that generated this Proof
  pub workflow_id: WorkflowId,
  /// which workflow is executed
  prev_id: WorkflowId,
  /// Identifier of the creator user or system as a reference to his account id on the blockchain,
  /// pgp key or email
  creator: CreatorId,
  /// Tells which groups the Proof belongs to
  pub groups: BoundedVec<ForWhat, MaxGroupsGet>,
  /// must be the same as for the Workflow
  params: BoundedVec<ProofParams, MaxProofParamsGet>,
}

impl AnagolayStructureData for ProofData {
  fn validate(&self) -> Result<(), Characters> {
    Ok(())
  }
}

impl Default for ProofData {
  fn default() -> Self {
    ProofData {
      workflow_id: WorkflowId::default(),
      prev_id: WorkflowId::default(),
      groups: BoundedVec::with_bounded_capacity(0),
      creator: CreatorId::default(),
      params: BoundedVec::with_bounded_capacity(0),
    }
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct ProofExtra {}
impl AnagolayStructureExtra for ProofExtra {}

/// PoE Proof
pub type Proof = AnagolayStructure<ProofData, ProofExtra>;

/// Storage record type
pub type ProofRecord<T> =
  AnagolayRecord<Proof, <T as frame_system::Config>::AccountId, <T as frame_system::Config>::BlockNumber>;
