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
use frame_support::sp_std::borrow::ToOwned;

impl<T: Config> Pallet<T> {
  /// Inserts the Operation into the `OperationsByOperationIdAndAccountId` storage
  /// Increases the `Total` Operation count
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

    OperationByOperationIdAndAccountId::<T>::insert(operation.id.clone(), account_id.clone(), record);

    Total::<T>::put(Self::total().saturating_add(1));
  }

  /// Inserts the Operation Version into the `VersionsByOperationId` and
  /// `Versions` storages Insert each package cid in the `PackageCid` storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * operation_version - The Operation Version to insert
  ///  * account_id - The owner of the Operation
  ///  * block_number - Current block
  pub fn do_create_operation_version(
    operation_version: &OperationVersion,
    account_id: &T::AccountId,
    block_number: T::BlockNumber,
  ) -> Result<(), Error<T>> {
    let record = OperationVersionRecord::<T> {
      record: operation_version.clone(),
      account_id: account_id.clone(),
      block_number,
    };

    let operation_id = &operation_version.data.entity_id.as_ref().unwrap();
    let operation_version_id = operation_version.id.to_owned();

    VersionByVersionId::<T>::insert(&operation_version_id, record);

    VersionIdsByOperationId::<T>::try_mutate(operation_id, |versions| {
      versions
        .try_push(operation_version_id.clone())
        .map_err(|_err| Error::<T>::MaxVersionsPerOperationLimitReached)
    })?;

    anagolay_support::Pallet::<T>::store_artifacts(&operation_version.data.artifacts)
      .map_err(|_err| Error::<T>::MaxArtifactsLimitReached)
  }
}
