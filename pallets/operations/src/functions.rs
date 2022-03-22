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

//! Business logic is located here.
//!
//! Each pallet must have this file.

use super::*;
use crate::types::{Operation, OperationRecord, OperationVersion, OperationVersionRecord};

impl<T: Config> Pallet<T> {
  /// Inserts the Operation into the `Operations` storage
  /// Increases the `Total Operation Count`
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * operation - The Operation to insert
  ///  * account_id - The owner of the Operation
  ///  * block_number - Current block
  pub fn do_create_operation(operation: &Operation, account_id: &T::AccountId, block_number: T::BlockNumber) {
    let record = OperationRecord::<T> {
      record: operation.clone(),
      account_id: account_id.clone(),
      block_number,
    };

    OperationsByAccountIdAndOperationId::<T>::insert(account_id.clone(), operation.id.clone(), record);

    Total::<T>::put(Self::total().saturating_add(1));
  }

  /// Inserts the Operation Version into the `VersionsByOperationId``VersionsByOperationId` and
  /// `Versions` storages Insert each package cid in the `PackageCid` storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * operation - The Operation to insert
  ///  * account_id - The owner of the Operation
  ///  * block_number - Current block
  pub fn do_create_operation_version(
    operation_version: &OperationVersion,
    account_id: &T::AccountId,
    block_number: T::BlockNumber,
  ) {
    let record = OperationVersionRecord::<T> {
      record: operation_version.clone(),
      account_id: account_id.clone(),
      block_number,
    };

    let operation_id = &operation_version.data.operation_id;
    let operation_version_id = &operation_version.id.clone();

    VersionsByVersionId::<T>::insert(operation_version_id, record);

    VersionsByOperationId::<T>::mutate(operation_id, |versions| {
      versions.push(operation_version_id.clone());
    });

    anagolay_support::Pallet::<T>::store_packages(&operation_version.data.packages);
  }
}
