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

//! `an_operations` pallet is the interface for the creation and management of Operations.
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

use anagolay_support::GenericId;
use sp_std::vec::Vec;

// use frame_support::debug;
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

// Type aliases for IDs used in the storage. Instead of writing large documentation we can show the
// user what the storage expects and what saves.

type OperationId = GenericId;
type VersionId = GenericId;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use crate::types::{
    Operation, OperationData, OperationRecord, OperationVersion, OperationVersionData, OperationVersionExtra,
    OperationVersionRecord,
  };
  use frame_support::{pallet_prelude::*, traits::UnixTime};
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  /// Config of the operations pallet
  pub trait Config: frame_system::Config + anagolay_support::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Timestamps provider
    type TimeProvider: UnixTime;
  }

  #[pallet::storage]
  #[pallet::getter(fn operations_by_account_id_and_operation_id)]
  /// Retrieve the Operation Manifest with the AccountId ( which is the owner ) and OperationId.
  pub type OperationsByAccountIdAndOperationId<T: Config> =
    StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Twox64Concat, OperationId, OperationRecord<T>, ValueQuery>;
  #[pallet::storage]
  #[pallet::getter(fn versions_by_operation_id)]
  /// Retrieve all Versions for a single Operation Manifest.
  pub type VersionsByOperationId<T: Config> = StorageMap<_, Blake2_128Concat, OperationId, Vec<VersionId>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn versions_by_version_id)]
  /// Retrieve the Version.
  pub type VersionsByVersionId<T: Config> =
    StorageMap<_, Blake2_128Concat, VersionId, OperationVersionRecord<T>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn total)]
  /// Total amount of Operations.
  pub type Total<T: Config> = StorageValue<_, u64, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  /// Events of the Operations pallet
  pub enum Event<T: Config> {
    /// Operation Manifest created together with Version and Packages.
    OperationCreated(T::AccountId, OperationId),
  }

  #[pallet::error]
  /// Errors of the Operations pallet
  pub enum Error<T> {
    /// Operation Manifest already exists.
    OperationAlreadyExists,
    /// Version pacakge already exists. If you think this is a bug in our system let us know [here](https://matrix.to/#/!FJvAuDoWRoMVuOFYwL:matrix.org?via=matrix.org).
    OperationVersionPackageAlreadyExists,
    /// The Operation already has an initial Version and cannot be published again.
    OperationAlreadyInitialized,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(<T as Config>::WeightInfo::create())]
    /// Create Operation manifest and a Version.
    ///
    /// Once you have created the Manifest this extrinsic will always fail with at least 3 different
    /// errors, each depend on the parts of the structure.
    /// There is a check that a user cannot cheat and create new pacakge if the package is
    /// connected to other Operation or any other Version.
    pub fn create(
      origin: OriginFor<T>,
      operation_data: OperationData,
      version_data: OperationVersionData,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let operation = Operation::new(operation_data);

      ensure!(
        !OperationsByAccountIdAndOperationId::<T>::contains_key(&sender, &operation.id),
        Error::<T>::OperationAlreadyExists
      );
      ensure!(
        !VersionsByOperationId::<T>::contains_key(&operation.id) ||
          VersionsByOperationId::<T>::get(&operation.id).is_empty(),
        Error::<T>::OperationAlreadyInitialized
      );
      ensure!(
        version_data
          .packages
          .iter()
          .find(|package| anagolay_support::Pallet::<T>::is_existing_package(package))
          .is_none(),
        Error::<T>::OperationVersionPackageAlreadyExists
      );

      let current_block = <frame_system::Pallet<T>>::block_number();

      Self::do_create_operation(&operation, &sender, current_block);

      let operation_version = OperationVersion::new_with_extra(
        OperationVersionData {
          operation_id: operation.id.clone(),
          ..version_data.clone()
        },
        OperationVersionExtra {
          created_at: T::TimeProvider::now().as_secs(),
        },
      );

      Self::do_create_operation_version(&operation_version, &sender, current_block);

      Self::deposit_event(Event::OperationCreated(sender, operation.id.clone()));

      Ok(().into())
    }

    #[pallet::weight(<T as Config>::WeightInfo::version_approve())]
    /// Approve Operation Version
    ///
    /// # Arguments
    ///  * origin - The call origin
    ///  * operation_id - The id of the Operation to approve
    pub fn version_approve(_origin: OriginFor<T>, _operation_id: GenericId) -> DispatchResultWithPostInfo {
      Ok(().into())
    }
  }
}
