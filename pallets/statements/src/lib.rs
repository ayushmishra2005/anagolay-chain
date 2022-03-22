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

//! an_statements pallet is the interface for the creation and management of Statements.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use anagolay_support::GenericId;

// use frame_support::debug;

mod benchmarking;
mod functions;
mod mock;
mod tests;
mod types;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use crate::types::{AnagolayClaimType, AnagolayStatement, AnagolayStatementRecord};
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;
  }

  #[pallet::error]
  pub enum Error<T> {
    /// Wrong claim type
    WrongClaimType,
    /// Proof already has this statement
    ProofHasStatement,
    /// Statement doesn't exits.
    NoSuchStatement,
    /// Statement has child statement and it cannot be revoked
    StatementHasChildStatement,
    /// Create child statement is not yet supported
    CreatingChildStatementNotSupported,
  }

  #[pallet::storage]
  #[pallet::getter(fn statements)]
  /// ALL statements
  pub type Statements<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    GenericId,
    Twox64Concat,
    T::AccountId,
    AnagolayStatementRecord<T>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn prev_statement)]
  /// Statement to previous statement index table for quick check.
  /// The StatementB has a parent StatementA in `prev_id` field this will be
  /// Example:

  /// ```ts
  /// const aStatement = {
  ///   //   ... normal as the rest,
  ///   prev_id: ''
  /// }

  /// const bStatement = {
  ///   //  ... normal as the rest,
  ///   prev_id: aStatement.id
  /// }```

  /// so this will be a map of bStatement.GenericId => aStatement.GenericId
  /// And now if we try to revoke the `aStatement` it will fail,
  /// because it is the part of the `bStatement`
  pub type StatementToPrevious<T: Config> =
    StorageMap<_, Blake2_128Concat, GenericId, GenericId, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn statements_count)]
  /// Amount of saved statements
  pub type StatementsCount<T: Config> = StorageValue<_, u128, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn proof_valid_statement)]
  /// List of the statements connected to the Proof. If the statement claim is 100% then there
  /// will be only one entry, if it's not then as many entries is needed to get to 100%
  pub type ProofValidStatements<T: Config> =
    StorageMap<_, Blake2_128Concat, GenericId, Vec<GenericId>, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  pub enum Event<T: Config> {
    /// Copyright is created. [who, CID]
    CopyrightCreated(T::AccountId, GenericId),
    /// Ownership is created. [who, CID]
    OwnershipCreated(T::AccountId, GenericId),
    /// Statement revoked. [who, CID]
    StatementRevoked(T::AccountId, GenericId),
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create Copyright
    #[pallet::weight(<T as Config>::WeightInfo::create_copyright())]
    pub(super) fn create_copyright(
      origin: OriginFor<T>,
      statement: AnagolayStatement,
    ) -> DispatchResultWithPostInfo {
      // DispatchResult
      let sender = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      // Statement must be of type copyright
      ensure!(
        statement.data.claim.claim_type == AnagolayClaimType::Copyright,
        Error::<T>::WrongClaimType
      );

      // Ensure that previous statement is empty. we do not allow updating the statements at this point
      ensure!(
        statement.data.claim.prev_id.is_empty(),
        Error::<T>::CreatingChildStatementNotSupported
      );

      // Ensure that ProofValidStatements has or not the statement
      Self::is_proof_statement_list_empty(&statement)?;

      // Do we have such a statement?
      ensure!(
        !Statements::<T>::contains_key(&statement.id, &sender),
        Error::<T>::ProofHasStatement
      );

      //@FUCK this needs fixing, it's a work-around for https://gitlab.com/anagolay/node/-/issues/31
      let statement_info = Self::build_statement_info(&statement, &sender, &current_block);

      Self::add_statement_to_proof(statement.data.claim.poe_id.clone(), statement.id.clone())?;

      Self::insert_statement(&statement_info, &sender);

      // Emit an event when copyright is created
      Self::deposit_event(Event::CopyrightCreated(sender, statement.id));

      Ok(().into())
    }

    /// Create Ownership
    #[pallet::weight(<T as Config>::WeightInfo::create_ownership())]
    pub(super) fn create_ownership(
      origin: OriginFor<T>,
      statement: AnagolayStatement,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      // Statement must be of type ownership
      ensure!(
        statement.data.claim.claim_type == AnagolayClaimType::Ownership,
        Error::<T>::WrongClaimType
      );

      // Ensure that previous statement is empty. we do not allow updating the statements at this point
      ensure!(
        statement.data.claim.prev_id.is_empty(),
        Error::<T>::CreatingChildStatementNotSupported
      );

      // Ensure that ProofValidStatements has or not the statement
      Self::is_proof_statement_list_empty(&statement)?;

      // Do we have such a statement
      ensure!(
        !Statements::<T>::contains_key(&statement.id, &sender),
        Error::<T>::ProofHasStatement
      );

      //@FUCK this needs fixing, it's a work-around for https://gitlab.com/anagolay/node/-/issues/31
      let statement_info = Self::build_statement_info(&statement, &sender, &current_block);

      Self::add_statement_to_proof(statement.data.claim.poe_id.clone(), statement.id.clone())?;

      Self::insert_statement(&statement_info, &sender);

      // Emit an event when ownership is created
      Self::deposit_event(Event::OwnershipCreated(sender, statement.id));

      Ok(().into())
    }

    /// Allow the owner to revoke their statement.
    #[pallet::weight(<T as Config>::WeightInfo::revoke())]
    pub(super) fn revoke(
      origin: OriginFor<T>,
      statement_id: GenericId,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;

      // Verify that the specified statement has been claimed.
      ensure!(
        Statements::<T>::contains_key(&statement_id, &sender),
        Error::<T>::NoSuchStatement
      );

      // Ensure Statement to previous statement index is not present | no child statements support atm
      ensure!(
        !StatementToPrevious::<T>::contains_key(&statement_id),
        Error::<T>::StatementHasChildStatement
      );

      Self::remove_statement(statement_id.clone(), &sender)?;

      // Emit an event that the claim was erased.
      Self::deposit_event(Event::StatementRevoked(sender, statement_id));

      Ok(().into())
    }
  }
}
