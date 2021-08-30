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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use anagolay::{CreatorId, GenericId};

// use frame_support::debug;

mod benchmarking;
mod mock;
mod tests;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
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
    /// Value was None
    NoneValue,
    /// Value reached maximum and cannot be incremented further
    StorageOverflow,
    /// Copyright already exists
    CopyrightAlreadyCreated,
    /// Ownership already exists
    OwnershipAlreadyCreated,
    /// Copyright doesn't exits, create one.
    NoSuchCopyright,
    /// Copyright doesn't exist
    CopyrightDoesntExist,
    /// Wrong claim type
    WrongClaimType,
    /// Proof already has this statement
    ProofHasStatement,
    /// Statement already exist
    StatementExist,
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
    StatementInfo<T::AccountId, T::BlockNumber>,
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
  /// }
  /// ```
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

  /// Anagolay Signature
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct AnagolaySignature {
    /// signing key in urn/did format 'urn:pgp:9cdf8dd38531511968c8d8cb524036585b62f15b'
    pub sig_key: Vec<u8>,
    /// Signature sign(prepared_statement, pvtKey(sigKey)) and encoded using multibase
    /// https://gitlab.com/sensio_group/sensio-faas/-/blob/master/sp-api/src/plugins/copyright/helpers.ts#L76
    pub sig: Vec<u8>,
    /// Content identifier of the sig field -- CID(sig)
    pub cid: GenericId,
  }

  /// Anagolay Signatures
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct AnagolaySignatures {
    pub holder: AnagolaySignature,
    pub issuer: AnagolaySignature,
  }
  /// Anagolay Claim Proportion
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct Proportion {
    /// Proportion sign, can be %
    pub sign: Vec<u8>,
    pub name: Vec<u8>,
    pub value: Vec<u8>,
  }
  /// Anagolay Validity
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct Validity {
    /// When the validity starts, this should be DATE_TIME
    pub from: Vec<u8>,
    /// When validity ends, this is calculate Validity.from + Expiration.value
    pub until: Vec<u8>,
  }

  /// Possible Expiration types
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub enum ExpirationType {
    Forever,
    Years,
    Months,
    Days,
    Minutes,
    Seconds,
  }

  impl Default for ExpirationType {
    fn default() -> Self {
      ExpirationType::Forever
    }
  }

  /// Anagolay Claim Expiration
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct Expiration {
    ///Possible Expiration types
    pub expiration_type: ExpirationType,
    ///How long is the expiration, if  ExpirationType::FOREVER then this is empty
    pub value: Vec<u8>,
  }

  /// Anagolay Claim types
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub enum AnagolayClaimType {
    Copyright,
    Ownership,
  }

  impl Default for AnagolayClaimType {
    fn default() -> Self {
      AnagolayClaimType::Copyright
    }
  }

  /// Anagolay Generic Claim
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct AnagolayClaim {
    /// Prev Anagolay Statement id in case this statement is revoked or changed
    pub prev_id: GenericId,
    /// PoE id of the record in question.
    pub poe_id: GenericId,
    /// Implemented rule
    pub rule_id: GenericId,
    /// In which proportion the statement is held
    pub proportion: Proportion,
    /// ATM this is the same as poe_id @TODO this should be unique representation of the subject that is NOT poe
    pub subject_id: GenericId,
    /// ATM this is the did representation of the substrate based account in format 'did:substrate:5EJA1oSrTx7xYMBerrUHLNktA3P89YHJBeTrevotTQab6gEY/anagolay-network'
    pub holder: CreatorId,
    /// ATM this is the did representation of the substrate based account in format 'did:substrate:Hcd78R7frJfUZHsqgpPEBLeiCZxV29uyyyURaPxB71ojNjy/anagolay-network'
    pub issuer: Vec<u8>,
    /// Generic type, ATM is Copyright or Ownership
    pub claim_type: AnagolayClaimType,
    /// How long this statement is valid
    pub valid: Validity,
    /// Setting when the statement should end
    pub expiration: Expiration,
    /// What happens after the expiration? this is default rule or smart contract that automatically does stuff,
    /// like move it to the public domain, transfer to relatives etc... need better definition
    pub on_expiration: Vec<u8>,
  }

  /// Copyright data
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct StatementData {
    pub signatures: AnagolaySignatures,
    pub claim: AnagolayClaim,
  }

  /// Anagolay copyright statement
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct AnagolayStatement {
    pub id: GenericId,
    pub data: StatementData,
  }

  impl Default for AnagolayStatement {
    fn default() -> Self {
      AnagolayStatement {
        id: b"".to_vec(),
        data: StatementData::default(),
      }
    }
  }

  /// Statement DB entry
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  pub struct StatementInfo<AccountId, BlockNumber> {
    /// Generated statement data
    pub statement: AnagolayStatement,
    pub account_id: AccountId,
    pub block_number: BlockNumber,
  }

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
        Error::<T>::CopyrightAlreadyCreated
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
        Error::<T>::OwnershipAlreadyCreated
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

  impl<T: Config> Pallet<T> {
    /// Decrease the statements count
    fn decrease_statements_count() {
      StatementsCount::<T>::mutate(|v| *v -= 1);
    }

    /// Increase the statements count
    fn increase_statements_count() {
      StatementsCount::<T>::mutate(|v| *v += 1);
    }

    /// Remove the statement from the Storage
    fn remove_statement(
      statement_id: GenericId,
      account_id: &T::AccountId,
    ) -> Result<bool, Error<T>> {
      let statement_info: StatementInfo<T::AccountId, T::BlockNumber> =
        Statements::<T>::get(&statement_id, &account_id);
      Self::remove_statement_proof_connection(
        statement_info.statement.data.claim.poe_id.clone(),
        statement_info.statement.id.clone(),
      )?;
      Statements::<T>::remove(&statement_id, &account_id);
      Self::decrease_statements_count();
      Ok(true)
    }

    /// Insert the statement to the Storage
    fn insert_statement(
      data: &StatementInfo<T::AccountId, T::BlockNumber>,
      account_id: &T::AccountId,
    ) {
      Statements::<T>::insert(&data.statement.id, &account_id, data.clone());
      Self::increase_statements_count();
    }

    ///Build the Statement info, storing to the DB
    fn build_statement_info(
      data: &AnagolayStatement,
      account_id: &T::AccountId,
      block_number: &T::BlockNumber,
    ) -> StatementInfo<T::AccountId, T::BlockNumber> {
      StatementInfo {
        statement: data.clone(),
        account_id: account_id.clone(),
        block_number: *block_number,
      }
    }

    /// Remove Statement <-> Proof connection
    fn remove_statement_proof_connection(
      poe_id: GenericId,
      statement_id: GenericId,
    ) -> Result<bool, Error<T>> {
      let mut proof_statement_list: Vec<GenericId> = ProofValidStatements::<T>::get(&poe_id);

      match proof_statement_list.binary_search(&statement_id) {
        // If the search succeeds, we found the Statement <-> Proof removal index,
        // so the statement_id can be removed from the proof_statement_list
        Ok(removal_index) => {
          proof_statement_list.remove(removal_index);
          ProofValidStatements::<T>::insert(&poe_id, proof_statement_list);
          Ok(true)
        }
        // If the search fails, the caller is not a member of the connection
        Err(_) => Err(Error::<T>::ProofHasStatement),
      }
    }
    /// Check does the Proof list is empty or not
    /// @TODO this might not be needed at all
    fn is_proof_statement_list_empty(statement: &AnagolayStatement) -> Result<bool, Error<T>> {
      let proof_statement_list: Vec<GenericId> =
        ProofValidStatements::<T>::get(&statement.data.claim.poe_id);

      if !proof_statement_list.is_empty() {
        // check here for existence of the statement given the condition where proportion is 100% or less
        // For now return error since we only can have one statement 100% per proof
        Err(Error::<T>::ProofHasStatement)
      } else {
        // ProofValidStatements::insert(&poe_id, vec![]);
        Ok(true)
      }
    }

    /// Add Statement to the Proof
    fn add_statement_to_proof(
      poe_id: GenericId,
      statement_id: GenericId,
    ) -> Result<bool, Error<T>> {
      let mut proof_statement_list: Vec<GenericId> = ProofValidStatements::<T>::get(&poe_id);

      match proof_statement_list.binary_search(&statement_id) {
        // If the search succeeds, the caller is already a member, so just return
        Ok(_) => Err(Error::<T>::ProofHasStatement),
        // If the search fails, the caller is not a member and we learned the index where
        // they should be inserted
        Err(index) => {
          // update the list
          proof_statement_list.insert(index, statement_id);
          ProofValidStatements::<T>::insert(poe_id, proof_statement_list);
          Ok(true)
        }
      }
    }
  }

  // match values.binary_search(value) {
  //     Ok(removal_index) =>,
  //     Err(_) => {} // value not contained.
  //   }
}
