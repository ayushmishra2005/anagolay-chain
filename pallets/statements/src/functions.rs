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

use super::{constants::*, *};
use crate::{
  types::{Claim, Signature, Statement, StatementId, StatementRecord},
  Error::NoSuchStatement,
};
use anagolay_support::Characters;
use codec::{Decode, Encode};
use frame_support::{sp_std::vec::Vec, BoundedVec};
use poe::types::ProofId;
use sp_runtime::traits::Verify;

impl<T: Config> Pallet<T> {
  /// Decrease the statements count
  fn decrease_statements_count() {
    Total::<T>::mutate(|v| *v -= 1);
  }

  /// Increase the statements count
  fn increase_statements_count() {
    Total::<T>::mutate(|v| *v += 1);
  }

  /// Verify the Signature of a Claim
  pub fn verify_substrate_signature(claim: &Claim, signature: &Signature, public_key: &str) -> bool {
    let public_key = hex::decode(public_key.strip_prefix("0x").unwrap_or(public_key)).unwrap_or_default();
    let public_key = sp_core::sr25519::Public::decode(&mut public_key.as_slice());
    let signature = sp_core::sr25519::Signature::decode(&mut signature.sig.as_slice());
    let message = claim.encode();
    // Wrap message as polkadot-js extension does https://substrate.stackexchange.com/questions/4209/verify-a-signature-in-pallet
    let wrapped = ["<Bytes>".as_bytes(), message.as_slice(), "</Bytes>".as_bytes()].concat();

    let verified = match (public_key, signature) {
      // Try with the wrapping or without
      (Ok(public_key), Ok(signature)) => {
        signature.verify(wrapped.as_slice(), &public_key) || signature.verify(message.as_slice(), &public_key)
      }
      _ => false,
    };
    verified
  }

  /// Remove the Statement from the storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * statement_id - The id of the Statement to remove
  ///  * account_id - The issuer of the Statement
  /// # Return
  /// A unit-type `Result` if the Statement was removed, `Error` otherwise
  pub fn remove_statement(statement_id: StatementId, account_id: &T::AccountId) -> Result<(), Error<T>> {
    match StatementByStatementIdAndAccountId::<T>::get(&statement_id, &account_id) {
      Some(statement_info) => {
        Self::remove_statement_proof_connection(
          statement_info.record.data.claim.poe_id.clone(),
          statement_info.record.id,
        )?;
        StatementByStatementIdAndAccountId::<T>::remove(&statement_id, &account_id);
        Self::decrease_statements_count();
        Ok(())
      }
      _ => Err(NoSuchStatement),
    }
  }

  /// Insert the statement to the storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * record - The record of the Statement to store
  ///  * account_id - The issuer of the Statement
  pub fn insert_statement(record: &StatementRecord<T>, account_id: &T::AccountId) {
    StatementByStatementIdAndAccountId::<T>::insert(&record.record.id, &account_id, record.clone());
    Self::increase_statements_count();
  }

  /// Remove the connection between a Statement and a Proof
  ///
  /// # Arguments
  ///  * poe_id - The id of the Proof
  ///  * account_id - The issuer of the Statement
  /// # Return
  /// A unit-type `Result` if the connection was removed, `Error` otherwise
  pub fn remove_statement_proof_connection(poe_id: ProofId, statement_id: StatementId) -> Result<(), Error<T>> {
    let mut proof_statement_list: BoundedVec<StatementId, MaxStatementsPerProofGet<T>> =
      StatementIdsByProofId::<T>::get(&poe_id);

    match proof_statement_list.binary_search(&statement_id) {
      // If the search succeeds, we found the Statement <-> Proof removal index,
      // so the statement_id can be removed from the proof_statement_list
      Ok(removal_index) => {
        proof_statement_list.remove(removal_index);
        StatementIdsByProofId::<T>::insert(&poe_id, proof_statement_list);
        Ok(())
      }
      // If the search fails, the caller is not a member of the connection
      Err(_) => Err(Error::<T>::NoSuchStatement),
    }
  }
  /// Check does the Proof list is empty or not
  /// @TODO this might not be needed at all
  ///
  /// # Arguments
  ///  * statement_data - The data section of the Statement
  /// # Return
  /// A unit-type `Result` if no Proof is currently associated to the statement, `Error` otherwise
  pub fn is_proof_statement_list_empty(statement: Statement) -> Result<(), Error<T>> {
    let proof_statement_list: BoundedVec<StatementId, MaxStatementsPerProofGet<T>> =
      StatementIdsByProofId::<T>::get(&statement.data.claim.poe_id);

    if !proof_statement_list.is_empty() {
      // check here for existence of the statement given the condition where proportion is 100% or less
      // For now return error since we only can have one statement 100% per proof
      Err(Error::<T>::ProofHasStatements)
    } else {
      // ProofValidStatements::insert(&poe_id, vec![]);
      Ok(())
    }
  }

  /// Add a connection between a Statement and a Proof
  ///
  /// # Arguments
  ///  * poe_id - The id of the Proof
  ///  * statement_id - The id of the Statement
  /// # Return
  /// A unit-type `Result` if the connection was created, `Error` otherwise
  pub fn add_statement_to_proof(poe_id: ProofId, statement_id: StatementId) -> Result<(), Error<T>> {
    let mut proof_statement_list: BoundedVec<StatementId, MaxStatementsPerProofGet<T>> =
      StatementIdsByProofId::<T>::get(&poe_id);

    match proof_statement_list.binary_search(&statement_id) {
      // If the search succeeds, the caller is already a member, so just return
      Ok(_) => Err(Error::<T>::StatementAlreadyExists),
      // If the search fails, the caller is not a member and we learned the index where
      // they should be inserted
      Err(index) => {
        // update the list
        proof_statement_list
          .try_insert(index, statement_id)
          .map_err(|_err| Error::<T>::MaxStatementsPerProofLimitReached)?;
        StatementIdsByProofId::<T>::insert(poe_id, proof_statement_list);
        Ok(())
      }
    }
  }

  /// Validate the Statement signature and save it
  /// The signature must be in the form `urn:substrate:<hex encoded public key>` and must
  /// sign the associated Claim, encoded by parity scale encoder.
  ///
  /// # Arguments
  ///  * statement - The Statement to validate and save
  ///  * account_id - The issuer of the Statement
  ///  * block_number - Current block
  /// # Return
  /// A boolean determining whether the Statement validation passed or not. If not, the statement
  /// is not saved.
  pub fn validate_and_save_statement(
    statement: Statement,
    account_id: &T::AccountId,
    block_number: &T::BlockNumber,
  ) -> Result<(), Error<T>> {
    let holder_signature = &statement.data.signatures.holder;
    let split: Vec<Characters> = holder_signature.sig_key.split(":");
    if split.len() == 3 {
      let algorithm = split.get(1).unwrap().clone();
      let public_key = split.get(2).unwrap().clone();
      return match algorithm.as_str() {
        "substrate" => {
          if Self::verify_substrate_signature(&statement.data.claim, holder_signature, public_key.as_str()) {
            let statement_info = StatementRecord::<T> {
              record: statement.clone(),
              account_id: account_id.clone(),
              block_number: *block_number,
            };
            Self::add_statement_to_proof(statement.data.claim.poe_id.clone(), statement.id.clone())?;
            Self::insert_statement(&statement_info, account_id);
            Ok(())
          } else {
            Err(Error::<T>::InvalidSignature)
          }
        }
        _ => Err(Error::<T>::UnrecognizedSignature),
      };
    }
    Err(Error::<T>::UnrecognizedSignature)
  }
}
