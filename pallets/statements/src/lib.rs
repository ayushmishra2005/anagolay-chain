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

//! statements pallet is the interface for the creation and management of Statements.
//! Statement is a record that proves the truthfulness of the Claim using user's cryptographic
//! Signatures. On Anagolay every Statement is the product of a transparent process we call
//! Workflow. At this time we support two types of statements, Copyright and Ownership, more will be
//! added when we see the need for it and practical usecase. The types are part of the network and
//! cannot be deleted, updated or removed by users or validators

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

mod benchmarking;
mod functions;
mod mock;
mod tests;
pub mod types;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

mod constants {
  use anagolay_support::getter_for_constant;
  getter_for_constant!(MaxStatementsPerProof, u32);
}

#[frame_support::pallet]
pub mod pallet {
  use super::{constants::*, *};
  use crate::types::{ClaimType, Statement, StatementData, StatementId, StatementRecord};
  use anagolay_support::{AnagolayStructureData, Characters};
  use core::convert::TryInto;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use poe::types::ProofId;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Config of the statements pallet
  #[pallet::config]
  pub trait Config: frame_system::Config + poe::Config {
    /// The overarching event type.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Maximum number of Statements registered for a single Proof on Anagolay network at a given
    /// time.
    const MAX_STATEMENTS_PER_PROOF: u32;
  }

  #[pallet::extra_constants]
  impl<T: Config> Pallet<T> {
    #[pallet::constant_name(MaxStatementsPerProof)]
    fn max_statements_per_proof() -> u32 {
      T::MAX_STATEMENTS_PER_PROOF
    }
  }

  /// Retrieve a Statement with the Statement Id and the Account Id
  #[pallet::storage]
  #[pallet::getter(fn statement_by_statement_id_and_account_id)]
  pub type StatementByStatementIdAndAccountId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, StatementId, Twox64Concat, T::AccountId, StatementRecord<T>, OptionQuery>;

  /// Retrieve the parent Statement Id given a Statement Id
  /// If the StatementB has a parent StatementA in `prev_id` field this will be
  /// StatementA id
  /// Example:
  ///
  /// ```ts
  /// const aStatement = {
  ///   //   ... normal as the rest,
  ///   prev_id: None
  /// }
  ///
  /// const bStatement = {
  ///   //  ... normal as the rest,
  ///   prev_id: Some(aStatement.id)
  /// }```
  ///
  /// So this will be a map of StatementId to StatementId (parent)
  /// It's used to quickly check upon revoke: the revoke of `aStatement` it will fail,
  /// because it is the parent of the `bStatement`
  #[pallet::storage]
  #[pallet::getter(fn parent_statement_id_by_statement_id)]
  pub type ParentStatementIdByStatementId<T: Config> =
    StorageMap<_, Blake2_128Concat, StatementId, StatementId, ValueQuery>;

  /// Amount of saved Statements
  #[pallet::storage]
  #[pallet::getter(fn total)]
  pub type Total<T: Config> = StorageValue<_, u128, ValueQuery>;

  /// List of the statements connected to the Proof. If the statement claim is 100% then there will
  /// be only one entry, if it's not then as many entries is needed to get to 100%
  #[pallet::storage]
  #[pallet::getter(fn statement_ids_by_proof_id)]
  pub type StatementIdsByProofId<T: Config> =
    StorageMap<_, Blake2_128Concat, ProofId, BoundedVec<StatementId, MaxStatementsPerProofGet<T>>, ValueQuery>;

  /// Events of the Statements pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Copyright is created
    CopyrightCreated(T::AccountId, StatementId),
    /// Ownership is created
    OwnershipCreated(T::AccountId, StatementId),
    /// Statement revoked
    StatementRevoked(T::AccountId, StatementId),
    /// Bad request error occurs and this event propagates a detailed description
    BadRequestError(T::AccountId, Characters),
  }

  /// Errors of the Statements pallet
  #[pallet::error]
  pub enum Error<T> {
    /// Wrong claim type
    WrongClaimType,
    /// Statement already exists
    StatementAlreadyExists,
    /// Proof already has associated statements
    ProofHasStatements,
    /// Statement doesn't exist.
    NoSuchStatement,
    /// Verification context is not valid
    InvalidVerificationContext,
    /// Statement has child statement and it cannot be revoked
    StatementHasChildStatement,
    /// Create child statement is not yet supported
    CreatingChildStatementNotSupported,
    /// A parameter of the request is invalid or does not respect a given constraint
    BadRequest,
    /// Insertion of Statement failed since MaxStatementsPerProof limit is reached
    MaxStatementsPerProofLimitReached,
    /// Statement signature did not validate
    InvalidSignature,
    /// Statement signature could not be parsed correctly
    UnrecognizedSignature,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create Copyright.
    ///
    /// On Anagolay Copyright statement is a exclusive right that holder claims over a subject in
    /// question.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * statement_data - the data section of the Statement
    ///
    /// # Errors
    /// * `WrongClaimType` - if the Statement type is not[`ClaimType::Copyright`]
    /// * `CreatingChildStatementNotSupported` - creating child Statements is not supported at the
    ///   moment
    /// * `StatementAlreadyExists` - the Statement already exists
    /// * `ProofHasStatements` - the Proof is already associated to existing Statements
    /// * `BadRequest` - if the request is invalid or does not respect a given constraint
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::create_copyright())]
    pub fn create_copyright(origin: OriginFor<T>, statement_data: StatementData) -> DispatchResultWithPostInfo {
      // DispatchResult
      let sender = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      let statement_validation = statement_data.validate();
      if let Err(ref message) = statement_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(statement_validation.is_ok(), Error::<T>::BadRequest);

      // Statement must be of type copyright
      ensure!(
        statement_data.claim.claim_type == ClaimType::Copyright,
        Error::<T>::WrongClaimType
      );

      // Ensure that previous statement is empty. we do not allow updating the statements at this point
      ensure!(
        statement_data.claim.prev_id.is_none(),
        Error::<T>::CreatingChildStatementNotSupported
      );

      let statement = Statement::new(statement_data);

      // Do we have such a statement?
      ensure!(
        !StatementByStatementIdAndAccountId::<T>::contains_key(&statement.id, &sender),
        Error::<T>::StatementAlreadyExists
      );

      // Ensure that Proof has or no associated statements
      Self::is_proof_statement_list_empty(statement.clone())?;

      Self::validate_and_save_statement(statement.clone(), &sender, &current_block)?;

      // Emit an event when copyright is created
      Self::deposit_event(Event::CopyrightCreated(sender, statement.id));

      Ok(().into())
    }

    /// Create Ownership for Verification
    ///
    /// On Anagolay Ownership statement is a exclusive right that holder claims over a subject in
    /// question. This Statement signs some Proofs generated by the verification pallet
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * statement_data - the data section of the Statement
    ///
    /// # Errors
    /// * `WrongClaimType` - if the Statement type is not [`ClaimType::Ownership`]
    /// * `CreatingChildStatementNotSupported` - creating child Statements is not supported at the
    ///   moment
    /// * `StatementAlreadyExists` - the Statement already exists
    /// * `InvalidVerificationContext` - if the proof does not exist, the verification is not
    ///   associated to the caller or the verification is not successful
    /// * `ProofHasStatements` - the Proof is already associated to existing Statements
    /// * `BadRequest` - if the request is invalid or does not respect a given constraint
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::create_ownership())]
    pub fn create_ownership(origin: OriginFor<T>, statement_data: StatementData) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      let statement_validation = statement_data.validate();
      if let Err(ref message) = statement_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(statement_validation.is_ok(), Error::<T>::BadRequest);

      // Verify correct issuer and validity of the verification
      ensure!(
        poe::Pallet::<T>::is_proof_id_valid_for_verification_context(&statement_data.claim.poe_id, &sender),
        Error::<T>::InvalidVerificationContext
      );

      // Statement must be of type ownership
      ensure!(
        statement_data.claim.claim_type == ClaimType::Ownership,
        Error::<T>::WrongClaimType
      );

      // Ensure that previous statement is empty. we do not allow updating the statements at this point
      ensure!(
        statement_data.claim.prev_id.is_none(),
        Error::<T>::CreatingChildStatementNotSupported
      );

      let statement = Statement::new(statement_data);

      // Do we have such a statement?
      ensure!(
        !StatementByStatementIdAndAccountId::<T>::contains_key(&statement.id, &sender),
        Error::<T>::StatementAlreadyExists
      );

      // Ensure that Proof has or no associated statements
      Self::is_proof_statement_list_empty(statement.clone())?;

      Self::validate_and_save_statement(statement.clone(), &sender, &current_block)?;

      // Emit an event when ownership is created
      Self::deposit_event(Event::OwnershipCreated(sender, statement.id));

      Ok(().into())
    }

    /// Allow the owner to revoke their statement.
    ///
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * statement_id - the id of the Statement to revoke
    ///
    /// # Errors
    /// * `NoSuchStatement` - if the Statement cannot be revoked since it does not exist
    /// * `StatementHasChildStatement` - if the Statement cannot be revoked since it has child
    ///   statement
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::revoke())]
    pub fn revoke(origin: OriginFor<T>, statement_id: StatementId) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;

      // Verify that the specified statement has been claimed.
      ensure!(
        StatementByStatementIdAndAccountId::<T>::contains_key(&statement_id, &sender),
        Error::<T>::NoSuchStatement
      );

      // Ensure Statement to previous statement index is not present | no child statements support atm
      ensure!(
        !ParentStatementIdByStatementId::<T>::contains_key(&statement_id),
        Error::<T>::StatementHasChildStatement
      );

      Self::remove_statement(statement_id.clone(), &sender)?;

      // Emit an event that the claim was erased.
      Self::deposit_event(Event::StatementRevoked(sender, statement_id));

      Ok(().into())
    }
  }
}
