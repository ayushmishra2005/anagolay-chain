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

//! an_poe pallet is the interface for the creation and management of Proofs of Existence.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use anagolay::GenericId;
use rules::PutInStorage;
mod benchmarking;
mod functions;
mod mock;
mod tests;
mod types;
pub mod weights;

pub use pallet::*;
use types::{PhashInfo, Proof};
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use crate::types::ProofRecord;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_runtime::traits::Hash;
  use sp_std::prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  ///The pallet's configuration trait.
  pub trait Config: frame_system::Config + rules::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;
    // type ExternalRulesStorage: PutInStorage<Self::AccountId, Self::BlockNumber>;
    type ExternalRulesStorage: PutInStorage;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;
  }

  #[pallet::storage]
  #[pallet::getter(fn proofs)]
  /// PoE Proofs
  pub type Proofs<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    GenericId,
    Twox64Concat,
    T::AccountId,
    ProofRecord<T>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn proofs_count)]
  /// Proofs count
  pub(super) type ProofsCount<T: Config> = StorageValue<_, u128, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn p_hashes)]
  /// Perceptual hash finder hash(phash) : (PerceptualHash, ProofId)
  pub(super) type PHashes<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::Hash,
    Twox64Concat,
    T::AccountId,
    PhashInfo,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn phash_count)]
  /// PHashes count
  pub(super) type PHashCount<T: Config> = StorageValue<_, u128, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId", T::Hash = "Hash")]
  pub enum Event<T: Config> {
    /// Proof is created and claimed . \{owner, cid}\
    ProofCreated(T::AccountId, GenericId),
    /// Phash is created. \{owner, pHash}\
    PhashCreated(T::AccountId, T::Hash),
  }

  #[pallet::error]
  pub enum Error<T> {
    ///This proof has already been claimed
    ProofAlreadyClaimed,
    ///The proof does not exist, so it cannot be revoked
    NoSuchProof,
    ///ForWhat mismatch
    ProofRuleTypeMismatch,
    ///PHash + ProofId already exist
    PHashAndProofIdComboAlreadyExist,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create proof and claim
    #[pallet::weight(<T as Config>::WeightInfo::create_proof())]
    pub(super) fn create_proof(origin: OriginFor<T>, proof: Proof) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let rule_id = &proof.data.rule_id;

      let proof_id = proof.id.clone();

      let rule_record = rules::Pallet::<T>::rules(rule_id, &sender);

      // @TODO somehow figure this out. we don't need it NOW but must be done before the Milestone 2 is submitted
      // ensure!(&rule_record, Error::<T>::NoSuchRule);

      // The types must match
      if proof.data.groups != rule_record.record.data.groups {
        ensure!(false, Error::<T>::ProofRuleTypeMismatch);
      }

      // Proof exists?
      ensure!(
        !Proofs::<T>::contains_key(&proof_id, &sender),
        Error::<T>::ProofAlreadyClaimed
      );

      let proof_info = ProofRecord::<T> {
        record: proof.clone(),
        account_id: sender.clone(),
        block_number: <frame_system::Pallet<T>>::block_number(), // Call the `system` pallet to get the current block number
      };

      Proofs::<T>::insert(&proof_id, &sender, proof_info);

      Self::increase_proof_count();

      // Emit an event that the proof was created
      Self::deposit_event(Event::ProofCreated(sender, proof_id));

      Ok(().into())
    }

    /// INDEX storage, save the connection phash <-> proofId for hamming/leven distance calc. Eventually refactor this, for now leave it
    #[pallet::weight(<T as Config>::WeightInfo::save_phash())]
    pub(super) fn save_phash(
      origin: OriginFor<T>,
      payload_data: PhashInfo,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;

      // Check is do we have the proof, can't add without
      ensure!(
        Proofs::<T>::contains_key(&payload_data.proof_id, &sender),
        Error::<T>::NoSuchProof
      );

      let payload_data_digest =
        payload_data.using_encoded(<T as frame_system::Config>::Hashing::hash);

      ensure!(
        !PHashes::<T>::contains_key(&payload_data_digest, &sender),
        Error::<T>::PHashAndProofIdComboAlreadyExist
      );

      PHashes::<T>::insert(&payload_data_digest, &sender, payload_data.clone());

      Self::increase_phash_count();

      // Emit an event that the proof was created
      Self::deposit_event(Event::PhashCreated(sender, payload_data_digest));

      Ok(().into())
    }
  }
}
