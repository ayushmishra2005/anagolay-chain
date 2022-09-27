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

//! `workflows` pallet is the interface for the creation and management of Workflows.
//!
//! A Workflow is composed of Segments, which can be thought of as parts of the Workflow
//! that require other specific parts of the Workflow to be executed beforehand, or otherwise, parts
//! that are in need of some external input and the Workflow execution is paused while waiting for
//! such input.
//!
//! A Segment definition contains a sequence of Operations, the eventual configuration of each one
//! of them, and a reference to the input required to bootstrap the process. In fact, the required
//! input may come from other Segments of the Workflow or from external input as well (eg: end-user
//! interaction)
//!
//! At Segment execution, each Operation of the sequence is executed in order. The previous
//! execution result is passed on to the next execution input, and so on until there are no more
//! Operations to execute in the Segment or a non-recoverable error occurs.
//!
//! The pallet also deals with creation and approval of Workflow Versions.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Enable experimental features
#![feature(type_name_of_val)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod functions;

pub mod types;
pub mod weights;
pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod constants {
  use anagolay_support::getter_for_constant;
  getter_for_constant!(MaxVersionsPerWorkflow, u32);
}

#[frame_support::pallet]
pub mod pallet {
  use super::{constants::*, *};
  use crate::types::{
    Workflow, WorkflowData, WorkflowId, WorkflowRecord, WorkflowVersion, WorkflowVersionData, WorkflowVersionExtra,
    WorkflowVersionId, WorkflowVersionRecord,
  };
  use anagolay_support::{AnagolayStructureData, Characters};
  use core::convert::TryInto;
  use frame_support::{log::error, pallet_prelude::*, traits::UnixTime};
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Config of the workflows pallet
  #[pallet::config]
  pub trait Config: frame_system::Config + anagolay_support::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Timestamps provider
    type TimeProvider: UnixTime;

    /// Maximum number of Versions for a single Operation registered on Anagolay network at a given
    /// time.
    const MAX_VERSIONS_PER_WORKFLOW: u32;
  }

  #[pallet::extra_constants]
  impl<T: Config> Pallet<T> {
    #[pallet::constant_name(MaxVersionsPerWorkflow)]
    fn max_versions_per_workflow() -> u32 {
      T::MAX_VERSIONS_PER_WORKFLOW
    }
  }

  /// Retrieve the Workflow Manifest with the WorkflowId and AccountId ( which is the owner )
  #[pallet::storage]
  #[pallet::getter(fn workflow_by_workflow_id_and_account_id)]
  pub type WorkflowByWorkflowIdAndAccountId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, WorkflowId, Twox64Concat, T::AccountId, WorkflowRecord<T>, OptionQuery>;

  /// Retrieve all Versions for a single Workflow Manifest.
  #[pallet::storage]
  #[pallet::getter(fn version_ids_by_workflow_id)]
  pub type VersionIdsByWorkflowId<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    WorkflowId,
    BoundedVec<WorkflowVersionId, MaxVersionsPerWorkflowGet<T>>,
    ValueQuery,
  >;

  /// Retrieve the Version.
  #[pallet::storage]
  #[pallet::getter(fn version_by_version_id)]
  pub type VersionByVersionId<T: Config> =
    StorageMap<_, Blake2_128Concat, WorkflowVersionId, WorkflowVersionRecord<T>, OptionQuery>;

  /// Amount of saved workflows
  #[pallet::storage]
  #[pallet::getter(fn total)]
  pub type Total<T: Config> = StorageValue<_, u64, ValueQuery>;

  /// The genesis config type.
  #[pallet::genesis_config]
  pub struct GenesisConfig<T: Config> {
    pub workflows: Vec<WorkflowRecord<T>>,
    pub versions: Vec<WorkflowVersionRecord<T>>,
    pub total: u64,
  }

  /// The default value for the genesis config type.
  #[cfg(feature = "std")]
  impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
      Self {
        workflows: Default::default(),
        versions: Default::default(),
        total: 0,
      }
    }
  }

  /// The build of genesis for the pallet.
  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
      Total::<T>::set(self.total);
      for wf_record in &self.workflows {
        let workflow_id = wf_record.record.id.clone();
        WorkflowByWorkflowIdAndAccountId::<T>::insert(&workflow_id, &wf_record.account_id, wf_record);

        for ver_record in &self.versions {
          let version_id = ver_record.record.id.clone();
          if ver_record
            .record
            .data
            .entity_id
            .clone()
            .unwrap_or(WorkflowId::default()) ==
            workflow_id
          {
            let error_fn = |err| {
              error!(
                "Pallet workflows genesis build (workflow_id={:?}): {:?}",
                workflow_id, err
              )
            };

            VersionIdsByWorkflowId::<T>::try_mutate(&workflow_id, |version_ids| {
              version_ids
                .try_push(version_id.clone())
                .map_err(|_err| Error::<T>::MaxVersionsPerWorkflowLimitReached)
            })
            .unwrap_or_else(error_fn);

            VersionByVersionId::<T>::insert(version_id, ver_record);

            anagolay_support::Pallet::<T>::store_artifacts(&ver_record.record.data.artifacts)
              .map_err(|_err| Error::<T>::MaxArtifactsLimitReached)
              .unwrap_or_else(error_fn);
          }
        }
      }
    }
  }

  /// Events of the Workflows pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(super)fn deposit_event)]
  pub enum Event<T: Config> {
    /// Workflow Manifest created together with Version and Packages.
    WorkflowCreated(T::AccountId, WorkflowId),
    /// Bad request error occurs and this event propagates a detailed description
    BadRequestError(T::AccountId, Characters),
  }

  /// Errors of the Workflows pallet
  #[pallet::error]
  pub enum Error<T> {
    /// Workflow Manifest already exists.
    WorkflowAlreadyExists,
    /// Version package already exists. If you think this is a bug in our system let us know [here](https://matrix.to/#/!FJvAuDoWRoMVuOFYwL:matrix.org?via=matrix.org).
    WorkflowVersionPackageAlreadyExists,
    /// The Workflow already has an initial Version and cannot be published again.
    WorkflowAlreadyInitialized,
    /// A parameter of the request is invalid or does not respect a given constraint
    BadRequest,
    /// Insertion of Artifact failed since MaxArtifacts limit is reached
    MaxArtifactsLimitReached,
    /// Insertion of Version failed since MaxVersionsPerWorkflow limit is reached
    MaxVersionsPerWorkflowLimitReached,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create Workflow manifest and the initial Version.
    ///
    /// Once you have created the Manifest this extrinsic will always fail with 3 different
    /// errors, each depend on the parts of the structure.
    /// There is a check that a user cannot cheat and create new package if the package is
    /// connected to other Workflow or any other Version.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * operation_data - the data section of the Workflow manifest
    /// * version_data - the data section of the Version manifest
    ///
    /// # Errors
    /// * `WorkflowAlreadyExists` - if an Workflow with the same manifest was already created by the
    ///   caller or by another user
    /// * `WorkflowAlreadyInitialized` - if the Workflow already has an initial Version
    /// * `WorkflowVersionPackageAlreadyExists` - one of the packages of the Version is already
    ///   registered to another Workflow
    /// * `BadRequest` - if the request is invalid or does not respect a given constraint
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::create())]
    pub fn create(
      origin: OriginFor<T>,
      workflow_data: WorkflowData,
      version_data: WorkflowVersionData,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let workflow_data_validation = workflow_data.validate();
      if let Err(ref message) = workflow_data_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(workflow_data_validation.is_ok(), Error::<T>::BadRequest);
      let version_data_validation = version_data.validate();
      if let Err(ref message) = version_data_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(version_data_validation.is_ok(), Error::<T>::BadRequest);

      let workflow = Workflow::new(workflow_data);

      ensure!(
        WorkflowByWorkflowIdAndAccountId::<T>::iter_prefix_values(&workflow.id).count() == 0,
        Error::<T>::WorkflowAlreadyExists
      );
      ensure!(
        !VersionIdsByWorkflowId::<T>::contains_key(&workflow.id) ||
          VersionIdsByWorkflowId::<T>::get(&workflow.id).is_empty(),
        Error::<T>::WorkflowAlreadyInitialized
      );
      ensure!(
        version_data
          .artifacts
          .iter()
          .find(|package| anagolay_support::Pallet::<T>::is_existing_artifact(package))
          .is_none(),
        Error::<T>::WorkflowVersionPackageAlreadyExists
      );

      let current_block = <frame_system::Pallet<T>>::block_number();

      Self::do_create_workflow(&workflow, &sender, &current_block);

      let workflow_version = WorkflowVersion::new_with_extra(
        WorkflowVersionData {
          entity_id: Some(workflow.id.clone()),
          ..version_data.clone()
        },
        WorkflowVersionExtra {
          created_at: T::TimeProvider::now().as_secs(),
        },
      );

      Self::do_create_workflow_version(&workflow_version, &sender, current_block)?;

      Self::deposit_event(Event::WorkflowCreated(sender, workflow.id.clone()));

      Ok(().into())
    }
  }
}
