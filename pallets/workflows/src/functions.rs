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

use super::*;
use crate::types::{Workflow, WorkflowId, WorkflowRecord, WorkflowVersion, WorkflowVersionId, WorkflowVersionRecord};
use frame_support::sp_std::{borrow::ToOwned, vec::Vec};

impl<T: Config> Pallet<T> {
  /// Inserts the Workflow into the `WorkflowsByAccountIdAndWorkflowId` storage
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

    WorkflowByWorkflowIdAndAccountId::<T>::insert(&workflow.id, &account_id, workflow_record);

    Total::<T>::put(Self::total().saturating_add(1));
  }

  /// Inserts the Workflow Version into the `VersionsByWorkflowId` and
  /// `Versions` storages Insert each package cid in the `PackageCid` storage
  ///
  /// Does no checks.
  ///
  /// # Arguments
  ///  * workflow_version - The Workflow to insert
  ///  * account_id - The owner of the Workflow
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

  /// Get a subset of Workflows representing a page, given the full set of the ids to paginate
  /// and the pagination information
  ///
  /// # Arguments
  ///  * workflow_ids - The full set of WorkflowIds. If empty, all Workflows will be considered
  ///  * offset - The index, inside the ids set, of the first Workflow on the page
  ///  * limit - The count of Workflows on the page
  ///
  /// # Return
  /// Collection of Workflows
  pub fn get_workflows_by_ids(workflow_ids: Vec<WorkflowId>, offset: u64, limit: u16) -> Vec<Workflow> {
    let mut workflows = Vec::new();

    let workflow_ids = if workflow_ids.is_empty() {
      let mut ids = Vec::new();
      WorkflowByWorkflowIdAndAccountId::<T>::iter_keys().for_each(|(k1, _)| ids.push(k1));
      ids
    } else {
      workflow_ids
    };

    let (_, workflow_ids) = workflow_ids.split_at(offset as usize);

    for workflow_id in workflow_ids.iter() {
      if workflows.len() >= limit as usize {
        break;
      }

      let workflow_record: Option<WorkflowRecord<T>> =
        WorkflowByWorkflowIdAndAccountId::<T>::iter_prefix_values(workflow_id).next();
      if let Some(workflow_record) = workflow_record {
        workflows.push(workflow_record.record)
      }
    }

    workflows
  }

  /// Get a subset of WorkflowVersions representing a page, given the full set of the ids to
  /// paginate and the pagination information
  ///
  /// # Arguments
  ///  * version_ids - The full set of WorkflowVersionIds. If empty, all WorkflowVersions will be
  ///    considered
  ///  * offset - The index, inside the ids set, of the first Workflow on the page
  ///  * limit - The count of Workflows on the page
  ///
  /// # Return
  /// Collection of WorkflowVersions
  pub fn get_workflow_versions_by_ids(
    workflow_version_ids: Vec<WorkflowVersionId>,
    offset: u64,
    limit: u16,
  ) -> Vec<WorkflowVersion> {
    let mut workflow_versions = Vec::new();

    let workflow_version_ids = if workflow_version_ids.is_empty() {
      let mut ids = Vec::new();
      VersionByVersionId::<T>::iter_keys().for_each(|k| ids.push(k));
      ids
    } else {
      workflow_version_ids
    };

    let (_, workflow_version_ids) = workflow_version_ids.split_at(offset as usize);

    for workflow_version_id in workflow_version_ids.iter() {
      if workflow_versions.len() >= limit as usize {
        break;
      }

      let workflow_version_record: Option<WorkflowVersionRecord<T>> = VersionByVersionId::<T>::get(workflow_version_id);
      if let Some(workflow_version_record) = workflow_version_record {
        workflow_versions.push(workflow_version_record.record)
      }
    }

    workflow_versions
  }
}
