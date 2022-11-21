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

use super::*;
use crate::{constants::*, types::ProofRecord};
use core::convert::TryInto;
use frame_support::{sp_std::vec::Vec, BoundedVec};
use verification::types::{VerificationContext, VerificationStatus};

impl<T: Config> Pallet<T> {
  /// # Arguments
  ///  * proof_id - The Proof id to validate
  ///  * account_id - The owner of the proof
  ///
  /// # Return
  /// True if the proof is indeed owned by the given account and associated to the context, false
  /// otherwise
  pub fn is_proof_id_valid_for_verification_context(proof_id: &ProofId, account_id: &T::AccountId) -> bool {
    match ProofByProofIdAndAccountId::<T>::get(proof_id, account_id) {
      Some(proof_by_account) => match (
        verification::Pallet::<T>::verification_request_by_account_id_and_verification_context(
          account_id,
          proof_by_account.record.data.context.clone(),
        ),
        ProofIdsByVerificationContext::<T>::get(proof_by_account.record.data.context.clone()),
      ) {
        (Some(verification_request), Some(proof_ids_by_context)) => {
          // Check VerificationRequest is successful and the proof id is indeed associated to the
          // VerificationContext
          proof_ids_by_context.contains(&proof_by_account.record.id) &&
            verification_request.status == VerificationStatus::Success
        }
        _ => {
          // If only proof exist it is valid only if its context is Unbounded
          proof_by_account.record.data.context == VerificationContext::Unbounded
        }
      },
      _ => false,
    }
  }

  /// Inserts the Proof into the `ProofByProofIdAndAccountId` storage
  /// Increases the `ProofTotal` count
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * proof - The Proof to insert
  ///  * account_id - The owner of the Operation
  ///  * block_number - Current block
  pub fn do_create_proof(proof: &Proof, account_id: &T::AccountId, block_number: T::BlockNumber) {
    let record = ProofRecord::<T> {
      record: proof.clone(),
      account_id: account_id.clone(),
      block_number,
    };

    ProofByProofIdAndAccountId::<T>::insert(&proof.id, &account_id, record);

    ProofTotal::<T>::put(Self::proof_total().saturating_add(1));
  }

  /// Inserts the Phash into the `PhashByHashAndAccountId` storage
  /// Increases the `PhashTotal` count
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * phash - The perceptual hash to save
  ///  * hash - encoded perceptual hash to use as key
  ///  * account_id - The owner of the Proof
  pub fn do_save_phash(phash: &PhashInfo, hash: &<T as frame_system::Config>::Hash, account_id: &T::AccountId) {
    PhashByHashAndAccountId::<T>::insert(&hash, &account_id, phash.clone());

    PhashTotal::<T>::put(Self::phash_total().saturating_add(1));
  }

  /// Inserts the Proofs into ProofIdsByVerificationContext and by calling `do_create_proof()`] also
  /// inserts the Proof into the `ProofByProofIdAndAccountId` storage and increases the `PhashTotal`
  /// count.
  ///
  /// Overwrites any existing link between the [`VerificationContext`] and the existing Proofs
  /// associated to it, but does not remove the existing Proofs.
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * holder - The account of the verification holder (owner of the Proof)
  ///  * context - The [`VerificationContext`] to which the proofs will be associated
  ///  * proofs - The proofs to insert
  pub fn do_create_proofs_of_verification(holder: &T::AccountId, context: &VerificationContext, proofs: Vec<Proof>) {
    let current_block = <frame_system::Pallet<T>>::block_number();

    // Old proofs, if any, will no longer be associated to this verification context
    ProofIdsByVerificationContext::<T>::remove(context);
    let proof_ids: BoundedVec<ProofId, MaxProofsPerWorkflowGet<T>> = proofs
      .iter()
      .map(|proof| proof.id.clone())
      .collect::<Vec<ProofId>>()
      .try_into()
      .unwrap_or_default();
    ProofIdsByVerificationContext::<T>::insert(context, proof_ids);

    proofs
      .iter()
      .for_each(|proof| Self::do_create_proof(proof, holder, current_block));
  }
}
