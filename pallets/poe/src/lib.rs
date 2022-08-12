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

//! `poe` pallet is the interface for the creation and management of Proofs of existence.
//!
//! Proofs of existence is a structured final output of the Workflow.
//! The pallet also deals with storage of perceptual hashes.
// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

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
  use crate::types::{ProofData, ProofRecord};
  use anagolay_support::{AnagolayStructureData, Characters, ProofId};
  use core::convert::TryInto;
  use frame_support::{pallet_prelude::*, sp_runtime::traits::Hash, sp_std::prelude::*};
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  pub struct Pallet<T>(_);

  /// The pallet's configuration trait.
  #[pallet::config]
  pub trait Config: frame_system::Config + workflows::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;
  }

  /// Retrieve the Proof with the ProofId and the AccountId
  #[pallet::storage]
  #[pallet::getter(fn proof_by_proof_id_and_account_id)]
  pub type ProofByProofIdAndAccountId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, ProofId, Twox64Concat, T::AccountId, ProofRecord<T>, OptionQuery>;

  /// Amount of saved Proofs
  #[pallet::storage]
  #[pallet::getter(fn proof_total)]
  pub(super) type ProofTotal<T: Config> = StorageValue<_, u128, ValueQuery>;

  /// Retrieve the PhashInfo with its digest and the AccountId
  #[pallet::storage]
  #[pallet::getter(fn phash_by_hash_and_account_id)]
  pub(super) type PhashByHashAndAccountId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, T::Hash, Twox64Concat, T::AccountId, PhashInfo, ValueQuery>;

  /// PHashes count
  #[pallet::storage]
  #[pallet::getter(fn phash_total)]
  pub(super) type PhashTotal<T: Config> = StorageValue<_, u128, ValueQuery>;

  /// Events of the Poe pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Proof is created and claimed
    ProofCreated(T::AccountId, ProofId),
    /// Phash is created
    PhashCreated(T::AccountId, T::Hash),
    /// Bad request error occurs and this event propagates a detailed description
    BadRequestError(T::AccountId, Characters),
  }

  /// Errors of the Poe pallet
  #[pallet::error]
  pub enum Error<T> {
    /// This Proof has already been claimed
    ProofAlreadyClaimed,
    /// The Proof does not exist, so it cannot be revoked
    NoSuchProof,
    /// The Workflow associated to the proof does not exist
    NoSuchWorkflow,
    /// The Workflow groups don't match the Proof groups
    ProofWorkflowTypeMismatch,
    /// PHash and ProofId combination already exist
    PHashAndProofIdComboAlreadyExist,
    /// A parameter of the request is invalid or does not respect a given constraint
    BadRequest,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create proof and claim
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * proof_data - the data section of the Proof
    ///
    /// # Errors
    /// * `ProofWorkflowTypeMismatch` - if the Workflow groups don't match the Proof groups
    /// * `ProofAlreadyClaimed` - if the Proof is already registered as claimed
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::create_proof())]
    pub fn create_proof(origin: OriginFor<T>, proof_data: ProofData) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let proof_validation = proof_data.validate();
      if let Err(ref message) = proof_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(proof_validation.is_ok(), Error::<T>::BadRequest);

      let proof = Proof::new(proof_data);

      let workflow_id = &proof.data.workflow_id;

      let proof_id = proof.id.clone();

      let workflow = workflows::Pallet::<T>::workflow_by_workflow_id_and_account_id(workflow_id, &sender)
        .ok_or(Error::<T>::NoSuchWorkflow)?;

      let current_block = <frame_system::Pallet<T>>::block_number();

      // @TODO somehow figure this out. we don't need it NOW but must be done before the Milestone 2 is
      // submitted ensure!(&rule_record, Error::<T>::NoSuchRule);

      // The types must match
      if proof.data.groups != workflow.record.data.groups {
        ensure!(false, Error::<T>::ProofWorkflowTypeMismatch);
      }

      // Proof exists?
      ensure!(
        !ProofByProofIdAndAccountId::<T>::contains_key(&proof_id, &sender),
        Error::<T>::ProofAlreadyClaimed
      );

      Self::do_create_proof(&proof, &sender, current_block);

      // Emit an event that the proof was created
      Self::deposit_event(Event::ProofCreated(sender, proof_id));

      Ok(().into())
    }

    /// INDEX storage, save the connection phash <-> proofId for hamming/leven distance calc.
    /// Eventually refactor this, for now leave it
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * phash_info - the perceptive hash information
    ///
    /// # Errors
    /// * `NoSuchProof` - if there is no such Proof as indicated in the phash_info
    /// * `PHashAndProofIdComboAlreadyExist` - if the relation between the perceptive hash and the
    ///   proof is already existing
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::save_phash())]
    pub fn save_phash(origin: OriginFor<T>, phash_info: PhashInfo) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;

      // Check is do we have the proof, can't add without
      ensure!(
        ProofByProofIdAndAccountId::<T>::contains_key(&phash_info.proof_id, &sender),
        Error::<T>::NoSuchProof
      );

      let phash_info_digest = phash_info.using_encoded(<T as frame_system::Config>::Hashing::hash);

      ensure!(
        !PhashByHashAndAccountId::<T>::contains_key(&phash_info_digest, &sender),
        Error::<T>::PHashAndProofIdComboAlreadyExist
      );

      Self::do_save_phash(&phash_info, &phash_info_digest, &sender);

      // Emit an event that the proof was created
      Self::deposit_event(Event::PhashCreated(sender, phash_info_digest));

      Ok(().into())
    }
  }
}
