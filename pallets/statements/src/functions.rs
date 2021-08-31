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

use super::*;
use anagolay::GenericId;
use sp_std::vec::Vec;

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
  pub fn remove_statement(
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
  pub fn insert_statement(
    data: &StatementInfo<T::AccountId, T::BlockNumber>,
    account_id: &T::AccountId,
  ) {
    Statements::<T>::insert(&data.statement.id, &account_id, data.clone());
    Self::increase_statements_count();
  }

  ///Build the Statement info, storing to the DB
  pub fn build_statement_info(
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
  pub fn remove_statement_proof_connection(
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
  pub fn is_proof_statement_list_empty(statement: &AnagolayStatement) -> Result<bool, Error<T>> {
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
  pub fn add_statement_to_proof(
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
