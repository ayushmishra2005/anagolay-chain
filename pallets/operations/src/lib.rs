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

//! an_operations pallet is the interface for the creation and management of Operations.
//! Operation is an abstraction that represents one task in a sequence of tasks, a Workflow.
//! Every operation has a minimum of one Version which is created when the Operation is created.
//! Each Version contains all the information needed to execute it, download it,
//! and chain it in the Workflow.
//! The pallet also deals with creation and approval of Operation Versions.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use anagolay::GenericId;
use sp_std::vec::Vec;

// use frame_support::debug;
mod benchmarking;
mod functions;
mod mock;
mod tests;
mod types;
pub mod weights;

pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use crate::types::{
    Operation, OperationData, OperationRecord, OperationVersion, OperationVersionData,
    OperationVersionExtra, OperationVersionRecord,
  };
  use frame_support::{pallet_prelude::*, traits::UnixTime};
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  /// Config of the operations pallet
  pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;

    /// Timestamps provider
    type TimeProvider: UnixTime;
  }

  #[pallet::storage]
  #[pallet::getter(fn operation)]
  /// Operations storage. Double map storage where the index is the tuple Account id and Operation id.
  pub type Operations<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    Twox64Concat,
    GenericId,
    OperationRecord<T>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn operation_version)]
  /// Operation Version storage. Map storage where index is Operation id and values are collections of Operation Version ids
  pub type OperationVersions<T: Config> =
    StorageMap<_, Blake2_128Concat, GenericId, Vec<GenericId>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn version)]
  /// Operation Version storage. Map storage where index is Operation Version id and values are Operation Version Records`
  pub type Versions<T: Config> =
    StorageMap<_, Blake2_128Concat, GenericId, OperationVersionRecord<T>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn operation_count)]
  /// Total amount of the stored Operations
  pub type OperationCount<T: Config> = StorageValue<_, u64, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  /// Events of the Operations pallet
  pub enum Event<T: Config> {
    /// Operation created. \[ Account id, Operation id \]
    OperationCreated(T::AccountId, GenericId),
    /// Operation Version created. \[ Account id, Operation id \]
    OperationVersionCreated(T::AccountId, GenericId),
  }

  #[pallet::error]
  /// Errors of the Operations pallet
  pub enum Error<T> {
    /// Operation already exists when creating an Operation
    OperationAlreadyExists,
    /// Operation does not exist when creating initial Operation Version
    OperationDoesNotExists,
    /// Operation Version already exists when creating an Operation Version
    OperationVersionAlreadyExists,
    /// Operation Version package already exists when creating an Operation Version
    OperationVersionPackageAlreadyExists,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  fn mock_new_operation_version<T: Config>(operation: &Operation) -> OperationVersionData {
    // TODO: real op ver creation goes here
    OperationVersionData {
      operation_id: operation.id.clone(),
      ..OperationVersionData::default()
    }
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(<T as Config>::WeightInfo::create_manifest())]
    /// Create Operation manifest
    ///
    /// # Arguments
    ///  * origin - The call origin
    ///  * operation_data - The data of the Operation to create
    pub fn create_manifest(
      origin: OriginFor<T>,
      operation_data: OperationData,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let operation = Operation::new(operation_data);

      ensure!(
        !Operations::<T>::contains_key(&sender, &operation.id),
        Error::<T>::OperationAlreadyExists
      );
      ensure!(
        OperationVersions::<T>::get(&operation.id).is_empty(),
        Error::<T>::OperationVersionAlreadyExists
      );

      let current_block = <frame_system::Pallet<T>>::block_number();

      Self::do_create_operation(&operation, &sender, current_block);

      let operation_version_data = mock_new_operation_version::<T>(&operation);

      Self::create_initial_version(origin, operation_version_data)?;

      Self::deposit_event(Event::OperationCreated(sender, operation.id.clone()));

      Ok(().into())
    }

    #[pallet::weight(<T as Config>::WeightInfo::create_initial_version())]
    /// Create initial Operation Version.
    ///
    /// # Arguments
    ///  * origin - The call origin
    ///  * operation_version_data - The data of the Operation Version to create
    pub fn create_initial_version(
      origin: OriginFor<T>,
      operation_version_data: OperationVersionData,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let operation_id = &operation_version_data.operation_id;
      ensure!(
        Operations::<T>::contains_key(&sender, operation_id),
        Error::<T>::OperationDoesNotExists
      );
      ensure!(
        OperationVersions::<T>::get(operation_id).is_empty(),
        Error::<T>::OperationVersionAlreadyExists
      );
      ensure!(
        Versions::<T>::get(operation_id)
          .record
          .data
          .packages
          .iter()
          .find(|package| operation_version_data
            .packages
            .iter()
            .find(|new_package| package.ipfs_cid == new_package.ipfs_cid)
            .is_none())
          .is_none(),
        Error::<T>::OperationVersionPackageAlreadyExists
      );

      let current_block = <frame_system::Pallet<T>>::block_number();

      let operation_version = OperationVersion::new_with_extra(
        operation_version_data.clone(),
        OperationVersionExtra {
          created_at: T::TimeProvider::now().as_secs(),
        },
      );

      Self::do_create_operation_version(&operation_version, &sender, current_block);

      Self::deposit_event(Event::OperationVersionCreated(sender, operation_id.clone()));

      Ok(().into())
    }

    #[pallet::weight(<T as Config>::WeightInfo::version_approve())]
    /// Approve Operation Version
    ///
    /// # Arguments
    ///  * origin - The call origin
    ///  * operation_id - The id of the Operation to approve
    pub fn version_approve(
      _origin: OriginFor<T>,
      _operation_id: GenericId,
    ) -> DispatchResultWithPostInfo {
      Ok(().into())
    }
  }
}
