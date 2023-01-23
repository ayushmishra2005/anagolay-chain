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

use anagolay_support::{constants::*, generic_id::GenericId, *};
use codec::{Decode, Encode};
use core::convert::TryInto;
use frame_support::{
  pallet_prelude::*,
  sp_runtime::RuntimeDebug,
  sp_std::{clone::Clone, default::Default, vec, vec::Vec},
};
use verification::types::{VerificationContext, VerificationKeyGenerator};
use workflows::types::WorkflowId;

getter_for_hardcoded_constant!(MaxPHashLen, u32, 1024);
getter_for_hardcoded_constant!(MaxProofParams, u32, 16);

// Proof id
anagolay_generic_id!(Proof);

/// Perceptive hash information, what gets stored
#[derive(Encode, Decode, Clone, Eq, PartialEq, Default, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct PhashInfo {
  /// The perceptive hash bytes
  pub p_hash: BoundedVec<u8, MaxPHashLenGet>,
  /// The id of the proof associated to this perceptive hash
  pub proof_id: ProofId,
}

/// Proof Incoming data
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct ProofData {
  /// The id of the Workflow that generated this Proof
  pub workflow_id: WorkflowId,
  /// which workflow is executed
  pub prev_id: WorkflowId,
  /// Identifier of the creator user or system as a reference to his account id on the blockchain,
  /// pgp key or email
  pub creator: CreatorId,
  /// Tells which groups the Proof belongs to
  pub groups: BoundedVec<ForWhat, MaxGroupsGet>,
  /// must be the same as for the Workflow
  pub params: BoundedVec<Characters, MaxProofParamsGet>,
  /// The verification context
  pub context: VerificationContext,
}

impl AnagolayStructureData for ProofData {
  type Id = ProofId;

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
      context: VerificationContext::default(),
    }
  }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct ProofExtra {}
impl AnagolayStructureExtra for ProofExtra {}

// PoE Proof entity
anagolay_structure!(Proof, ProofId, ProofData, ProofExtra);

// This produces `ProofRecord<T>`,  the Storage record of the Proof.
anagolay_record!(Proof);

#[derive(Clone)]
pub struct PoeVerificationKeyGenerator<T: crate::Config> {
  _marker: PhantomData<T>,
}

impl<T: crate::Config> VerificationKeyGenerator<T> for PoeVerificationKeyGenerator<T> {
  /// Produces a CID v1 out of some identifier using an Anagolay workflow
  ///
  /// # Arguments
  /// * holder - The verification holder
  /// * context - The verification context
  /// * identifier - The identifier to use to compute the CID
  ///
  /// # Return
  /// The CID string ("bafk...") in the form of a collection of bytes
  fn generate(
    holder: &T::AccountId,
    context: &VerificationContext,
    identifier: Vec<u8>,
  ) -> Result<Vec<u8>, verification::Error<T>> {
    let cid = anagolay_support::Pallet::<T>::produce_cid(identifier);
    let proof_data = ProofData {
      // @FIXME expose get_id() as Workflow trait method in next iteration
      workflow_id: "bafkr4icflbi5pbomtcyejivr4l7dcdvcmvcsviwmnn7qp52flfnkvy2ebe".into(),
      // @FIXME this is unused
      prev_id: "".into(),
      // @FIXME this is scale encoded, not ss58 encoded (no ss58 codec in nostd)
      creator: holder
        .encode()
        .as_slice()
        .try_into()
        .map_err(|_| verification::Error::<T>::VerificationKeyGenerationError)?,
      groups: vec![ForWhat::GENERIC]
        .try_into()
        .map_err(|_| verification::Error::<T>::VerificationKeyGenerationError)?,
      params: vec![cid
        .as_slice()
        .try_into()
        .map_err(|_| verification::Error::<T>::VerificationKeyGenerationError)?]
      .try_into()
      .map_err(|_| verification::Error::<T>::VerificationKeyGenerationError)?,
      context: context.clone(),
    };

    let proof = Proof::new(proof_data);

    <crate::Pallet<T>>::do_create_proofs_of_verification(holder, context, vec![proof]);
    Ok(cid)
  }
}
