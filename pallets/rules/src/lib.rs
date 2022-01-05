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

mod benchmarking;
mod functions;
mod mock;
mod tests;
mod types;
pub mod weights;
use anagolay::{AnagolayRecord, GenericId};
pub use pallet::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use crate::types::Rule;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// The overarching event type.
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;

    /// Weight information for extrinsics for this pallet.
    type WeightInfo: WeightInfo;
  }

  #[pallet::storage]
  #[pallet::getter(fn rules)]
  /// Rules
  pub type Rules<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat,
    GenericId,
    Twox64Concat,
    T::AccountId,
    AnagolayRecord<Rule, T::AccountId, T::BlockNumber>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn rule_count)]
  /// Amount of saved rules
  pub type RuleCount<T: Config> = StorageValue<_, u32, ValueQuery>;

  /// Pallets external access to the saving in the storage
  pub trait PutInStorage {}

  impl<T: Config> PutInStorage for Pallet<T> {}

  #[pallet::event]
  #[pallet::generate_deposit(pub(super)fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  pub enum Event<T: Config> {
    ///Rule is created
    RuleCreated(T::AccountId, GenericId),
  }

  #[pallet::error]
  pub enum Error<T> {
    ///Rule already exists
    RuleAlreadyCreated,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// Create Rule
    #[pallet::weight(<T as Config>::WeightInfo::create_rule())]
    pub(super) fn create_rule(origin: OriginFor<T>, rule: Rule) -> DispatchResultWithPostInfo {
      let sender = ensure_signed(origin)?;
      let current_block = <frame_system::Pallet<T>>::block_number();

      ensure!(
        !Rules::<T>::contains_key(&rule.id, &sender),
        Error::<T>::RuleAlreadyCreated
      );

      let rule_info = Self::create(&sender, &current_block, &rule);

      // deposit the event
      Self::deposit_event(Event::RuleCreated(sender, rule_info.info.id));

      Ok(().into())
    }
  }
}
