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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use anagolay::{GenericId, StorageInfo};

// use frame_support::debug;
mod benchmarking;
mod functions;
mod mock;
mod tests;
mod types;
pub mod weights;

pub use pallet::*;
use types::OperationStructure;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;
  }

  #[pallet::storage]
  #[pallet::getter(fn operation)]
  /// Operations storage. Double map storage where the index is `[OperationId, OwnerAccountId]`.
  pub type Operations<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    GenericId,
    Twox64Concat,
    T::AccountId,
    StorageInfo<OperationStructure, T::AccountId, T::BlockNumber>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn operation_count)]
  /// Total amount of the stored operations
  pub type OperationCount<T: Config> = StorageValue<_, u64, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(crate) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  pub enum Event<T: Config> {
    /// Operation Created. \[ who, OperationId \]
    OperationCreated(T::AccountId, GenericId),
  }

  #[pallet::error]
  pub enum Error<T> {
    /// Operation Already exists
    OperationAlreadyExists,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(<T as Config>::WeightInfo::create())]
    /// Create Operation
    pub fn create(
      origin: OriginFor<T>,
      operation: OperationStructure,
    ) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin.clone())?;

      let current_block = <frame_system::Pallet<T>>::block_number();

      ensure!(
        !Operations::<T>::contains_key(&operation.id, &sender),
        Error::<T>::OperationAlreadyExists
      );

      Self::do_insert_operation(&operation, &sender, &current_block);

      // Emit an event when operation is created
      Self::deposit_event(Event::OperationCreated(sender, operation.id.clone()));

      Ok(().into())
    }
  }
}
