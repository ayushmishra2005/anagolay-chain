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
use crate::types::ProofRecord;

impl<T: Config> Pallet<T> {
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
  ///  * account_id - The owner of the Operation
  pub fn do_save_phash(phash: &PhashInfo, hash: &<T as frame_system::Config>::Hash, account_id: &T::AccountId) {
    PhashByHashAndAccountId::<T>::insert(&hash, &account_id, phash.clone());

    PhashTotal::<T>::put(Self::phash_total().saturating_add(1));
  }
}
