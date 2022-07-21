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

use super::*;
use crate::types::{Workflow, WorkflowRecord, WorkflowVersion, WorkflowVersionRecord};
use frame_support::sp_std::borrow::ToOwned;

impl<T: Config> Pallet<T> {
  /// Inserts the Operation into the `WorkflowsByAccountIdAndWorkflowId` storage
  /// Increases the `Total` Workflow count
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * workflow - The Workflow to insert
  ///  * account_id - The owner of the Workflow
  ///  * block_number - Current block
  pub fn do_create_workflow(workflow: &Workflow, account_id: &T::AccountId, block_number: &T::BlockNumber) {
    let workflow_record = WorkflowRecord::<T> {
      record: workflow.clone(),
      account_id: account_id.clone(),
      block_number: *block_number,
    };

    WorkflowByWorkflowIdAndAccountId::<T>::insert(&workflow.id, &account_id, workflow_record.clone());

    Total::<T>::put(Self::total().saturating_add(1));
  }

  /// Inserts the Workflow Version into the `VersionsByOperationId` and
  /// `Versions` storages Insert each package cid in the `PackageCid` storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * workflow_version - The Operation to insert
  ///  * account_id - The owner of the Operation
  ///  * block_number - Current block
  pub fn do_create_workflow_version(
    workflow_version: &WorkflowVersion,
    account_id: &T::AccountId,
    block_number: T::BlockNumber,
  ) -> Result<(), Error<T>> {
    let record = WorkflowVersionRecord::<T> {
      record: workflow_version.clone(),
      account_id: account_id.clone(),
      block_number,
    };

    let workflow_id = &workflow_version.data.entity_id.as_ref().unwrap();
    let workflow_version_id = workflow_version.id.to_owned();

    VersionByVersionId::<T>::insert(&workflow_version_id, record);

    VersionIdsByWorkflowId::<T>::try_mutate(workflow_id, |versions| {
      versions
        .try_push(workflow_version_id.clone())
        .map_err(|_err| Error::<T>::MaxArtifactsLimitReached)
    })?;

    anagolay_support::Pallet::<T>::store_artifacts(&workflow_version.data.artifacts)
      .map_err(|_err| Error::<T>::MaxArtifactsLimitReached)
  }
}
