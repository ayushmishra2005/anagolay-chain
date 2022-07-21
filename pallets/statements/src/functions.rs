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

use super::{constants::*, *};
use crate::{
  types::{Statement, StatementData, StatementRecord},
  Error::NoSuchStatement,
};
use anagolay_support::{AnagolayRecord, ProofId, StatementId};
use frame_support::BoundedVec;

impl<T: Config> Pallet<T> {
  /// Decrease the statements count
  fn decrease_statements_count() {
    Total::<T>::mutate(|v| *v -= 1);
  }

  /// Increase the statements count
  fn increase_statements_count() {
    Total::<T>::mutate(|v| *v += 1);
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
          statement_info.record.id.clone(),
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
  pub fn insert_statement(record: &AnagolayRecord<Statement, T::AccountId, T::BlockNumber>, account_id: &T::AccountId) {
    StatementByStatementIdAndAccountId::<T>::insert(&record.record.id, &account_id, record.clone());
    Self::increase_statements_count();
  }

  /// Build the [`StatementRecord`] to store
  ///
  /// # Arguments
  ///  * statement - The Statement
  ///  * account_id - The issuer of the Statement
  ///  * block_number - Current block
  /// # Return
  /// A [`StatementRecord`] containing the information provided as argument
  pub fn build_statement_info(
    statement: &Statement,
    account_id: &T::AccountId,
    block_number: &T::BlockNumber,
  ) -> StatementRecord<T> {
    StatementRecord::<T> {
      record: statement.clone(),
      account_id: account_id.clone(),
      block_number: *block_number,
    }
  }

  /// Remove the connection between a Statement and a Proof
  ///
  /// # Arguments
  ///  * poe_id - The id of the Proof
  ///  * account_id - The issuer of the Statement
  /// # Return
  /// A unit-type `Result` if the connection was removed, `Error` otherwise
  pub fn remove_statement_proof_connection(poe_id: ProofId, statement_id: StatementId) -> Result<(), Error<T>> {
    let mut proof_statement_list: BoundedVec<ProofId, MaxStatementsPerProofGet<T>> =
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
      Err(_) => Err(Error::<T>::ProofHasStatement),
    }
  }
  /// Check does the Proof list is empty or not
  /// @TODO this might not be needed at all
  ///
  /// # Arguments
  ///  * statement_data - The data section of the Statement
  /// # Return
  /// A unit-type `Result` if no Proof is currently associated to the statement, `Error` otherwise
  pub fn is_proof_statement_list_empty(statement_data: &StatementData) -> Result<(), Error<T>> {
    let proof_statement_list: BoundedVec<ProofId, MaxStatementsPerProofGet<T>> =
      StatementIdsByProofId::<T>::get(&statement_data.claim.poe_id);

    if !proof_statement_list.is_empty() {
      // check here for existence of the statement given the condition where proportion is 100% or less
      // For now return error since we only can have one statement 100% per proof
      Err(Error::<T>::ProofHasStatement)
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
    let mut proof_statement_list: BoundedVec<ProofId, MaxStatementsPerProofGet<T>> =
      StatementIdsByProofId::<T>::get(&poe_id);

    match proof_statement_list.binary_search(&statement_id) {
      // If the search succeeds, the caller is already a member, so just return
      Ok(_) => Err(Error::<T>::ProofHasStatement),
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
}
