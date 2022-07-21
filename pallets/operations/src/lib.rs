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

//! `operations` pallet is the interface for the creation and management of Operations.
//!
//! Operation is an abstraction that represents one task in a sequence of tasks, a Workflow.
//!
//! Every operation has a minimum of one Version which is created when the Operation is created.
//!
//! Each Version contains all the information needed to execute it, download it,
//! and chain it in the Workflow.
//! The pallet also deals with creation and approval of Operation Versions.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// use frame_support::debug;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod functions;
pub mod types;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub use pallet::*;
pub use weights::WeightInfo;

mod constants {
  use anagolay_support::getter_for_constant;
  getter_for_constant!(MaxVersionsPerOperation, u32);
}

#[frame_support::pallet]
pub mod pallet {
  use super::{constants::*, *};
  use crate::types::{
    Operation, OperationData, OperationRecord, OperationVersion, OperationVersionData, OperationVersionRecord,
  };
  use anagolay_support::{
    AnagolayStructureData, AnagolayVersionData, AnagolayVersionExtra, Characters, OperationId, VersionId,
  };
  use core::convert::TryInto;
  use frame_support::{log::error, pallet_prelude::*, traits::UnixTime};
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Config of the operations pallet
  #[pallet::config]
  pub trait Config: frame_system::Config + anagolay_support::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Timestamps provider
    type TimeProvider: UnixTime;

    /// Maximum number of Versions for a single Operation registered on Anagolay network at a given
    /// time.
    const MAX_VERSIONS_PER_OPERATION: u32;
  }

  #[pallet::extra_constants]
  impl<T: Config> Pallet<T> {
    #[pallet::constant_name(MaxVersionsPerOperation)]
    fn max_versions_per_operation() -> u32 {
      T::MAX_VERSIONS_PER_OPERATION
    }
  }

  /// Retrieve the Operation Manifest with the AccountId ( which is the owner ) and OperationId.
  #[pallet::storage]
  #[pallet::getter(fn operation_by_operation_id_and_account_id)]
  pub type OperationByOperationIdAndAccountId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, OperationId, Twox64Concat, T::AccountId, OperationRecord<T>, OptionQuery>;

  /// Retrieve all Versions for a single Operation Manifest.
  #[pallet::storage]
  #[pallet::getter(fn version_ids_by_operation_id)]
  pub type VersionIdsByOperationId<T: Config> =
    StorageMap<_, Blake2_128Concat, OperationId, BoundedVec<VersionId, MaxVersionsPerOperationGet<T>>, ValueQuery>;

  /// Retrieve the Version.
  #[pallet::storage]
  #[pallet::getter(fn version_by_version_id)]
  pub type VersionByVersionId<T: Config> =
    StorageMap<_, Blake2_128Concat, VersionId, OperationVersionRecord<T>, OptionQuery>;

  /// Total amount of Operations.
  #[pallet::storage]
  #[pallet::getter(fn total)]
  pub type Total<T: Config> = StorageValue<_, u64, ValueQuery>;

  /// The genesis config type.
  #[pallet::genesis_config]
  pub struct GenesisConfig<T: Config> {
    pub operations: Vec<OperationRecord<T>>,
    pub versions: Vec<OperationVersionRecord<T>>,
    pub total: u64,
  }

  /// The default value for the genesis config type.
  #[cfg(feature = "std")]
  impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
      Self {
        operations: Default::default(),
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
      for op_record in &self.operations {
        let operation_id = op_record.record.id.clone();
        OperationByOperationIdAndAccountId::<T>::insert(&operation_id, &op_record.account_id, op_record);

        for ver_record in &self.versions {
          let version_id = ver_record.record.id.clone();
          if ver_record
            .record
            .data
            .entity_id
            .clone()
            .unwrap_or(OperationId::default()) ==
            operation_id
          {
            let error_fn = |err| {
              error!(
                "Pallet operations genesis build (operation_id={:?}): {:?}",
                operation_id, err
              )
            };

            VersionIdsByOperationId::<T>::try_mutate(&operation_id, |version_ids| {
              version_ids
                .try_push(version_id.clone())
                .map_err(|_err| Error::<T>::MaxVersionsPerOperationLimitReached)
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
  /// Events of the Operations pallet
  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Operation Manifest created together with Version and Packages.
    OperationCreated(T::AccountId, OperationId),
    /// Bad request error occurs and this event propagates a detailed description
    BadRequestError(T::AccountId, Characters),
  }

  /// Errors of the Operations pallet
  #[pallet::error]
  pub enum Error<T> {
    /// Operation Manifest already exists.
    OperationAlreadyExists,
    /// Version package already exists. If you think this is a bug in our system let us know [here](https://matrix.to/#/!FJvAuDoWRoMVuOFYwL:matrix.org?via=matrix.org).
    OperationVersionPackageAlreadyExists,
    /// The Operation already has an initial Version and cannot be published again.
    OperationAlreadyInitialized,
    /// A parameter of the request is invalid or does not respect a given constraint
    BadRequest,
    /// Insertion of Artifact failed since MaxArtifacts limit is reached
    MaxArtifactsLimitReached,
    /// Insertion of Version failed since MaxVersionsPerOperation limit is reached
    MaxVersionsPerOperationLimitReached,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create Operation manifest and the initial Version.
    ///
    /// Once you have created the Manifest this extrinsic will always fail with different
    /// errors, each depend on the parts of the structure.
    /// There is a check that a user cannot cheat and create new package if the package is
    /// connected to other Operation or any other Version.
    ///
    /// # Arguments
    /// * origin - the call origin
    /// * operation_data - the data section of the Operation manifest
    /// * version_data - the data section of the Version manifest
    ///
    /// # Errors
    /// * `OperationAlreadyExists` - if an Operation with the same manifest was already created by
    ///   the caller or by another user
    /// * `OperationAlreadyInitialized` - if the Operation already has an initial Version
    /// * `OperationVersionPackageAlreadyExists` - one of the packages of the Version is already
    ///   registered to another Operation
    /// * `BadRequest` - if the request is invalid or does not respect a given constraint
    ///
    /// # Return
    /// `DispatchResultWithPostInfo` containing Unit type
    #[pallet::weight(<T as Config>::WeightInfo::create())]
    pub fn create(
      origin: OriginFor<T>,
      operation_data: OperationData,
      version_data: OperationVersionData,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let operation_data_validation = operation_data.validate();
      if let Err(ref message) = operation_data_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(operation_data_validation.is_ok(), Error::<T>::BadRequest);
      let version_data_validation = version_data.validate();
      if let Err(ref message) = version_data_validation {
        Self::deposit_event(Event::BadRequestError(sender.clone(), message.clone()));
      }
      ensure!(version_data_validation.is_ok(), Error::<T>::BadRequest);

      let operation = Operation::new(operation_data);

      ensure!(
        OperationByOperationIdAndAccountId::<T>::iter_prefix_values(&operation.id).count() == 0,
        Error::<T>::OperationAlreadyExists
      );
      ensure!(
        !VersionIdsByOperationId::<T>::contains_key(&operation.id) ||
          VersionIdsByOperationId::<T>::get(&operation.id).is_empty(),
        Error::<T>::OperationAlreadyInitialized
      );
      ensure!(
        version_data
          .artifacts
          .iter()
          .find(|package| anagolay_support::Pallet::<T>::is_existing_artifact(package))
          .is_none(),
        Error::<T>::OperationVersionPackageAlreadyExists
      );

      let current_block = <frame_system::Pallet<T>>::block_number();

      Self::do_create_operation(&operation, &sender, current_block);

      let operation_version = OperationVersion::new_with_extra(
        AnagolayVersionData {
          entity_id: Some(operation.id.clone()),
          ..version_data.clone()
        },
        AnagolayVersionExtra {
          created_at: T::TimeProvider::now().as_secs(),
        },
      );

      Self::do_create_operation_version(&operation_version, &sender, current_block)?;

      Self::deposit_event(Event::OperationCreated(sender, operation.id.clone()));

      Ok(().into())
    }

    /// Approve Operation Version
    ///
    /// # Arguments
    ///  * origin - The call origin
    ///  * operation_id - The id of the Operation to approve
    #[pallet::weight(<T as Config>::WeightInfo::version_approve())]
    pub fn version_approve(_origin: OriginFor<T>, _operation_id: OperationId) -> DispatchResultWithPostInfo {
      Ok(().into())
    }
  }
}
