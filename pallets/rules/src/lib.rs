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

// use frame_support::debug::native;
// use frame_support::debug;
use anagolay::{CreatorId, ForWhat, GenericId};

// Local files
mod mock;
mod tests;

pub use pallet::*;
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
  use super::*;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::prelude::*;

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

  /// Rule which must be applied
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  // #[cfg_attr(feature = "std", derive(Debug))]
  pub struct Rule {
    pub id: GenericId,
    pub data: RuleData,
  }

  impl Default for Rule {
    fn default() -> Self {
      Rule {
        id: b"".to_vec(),
        data: RuleData::default(),
      }
    }
  }

  ///OperationReference by id instead of full
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  // #[cfg_attr(feature = "std", derive(Debug))]
  pub struct OperationReference {
    id: GenericId,
    children: Vec<OperationReference>,
  }

  /// Rule Data, use this to generate rule_id
  #[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  // #[cfg_attr(feature = "std", derive(Debug))]
  pub struct RuleData {
    /// Version, maybe we remove this, we will see
    pub version: u32,
    /// max 128(0.12kb) characters, slugify to use _
    pub name: Vec<u8>,
    /// max 512(0.5kb) or 1024(1kb) chars, can be markdown but not html
    pub desc: Vec<u8>,
    pub creator: CreatorId,
    pub groups: Vec<ForWhat>,
    pub parent_id: GenericId,
    pub ops: Vec<OperationReference>,
  }

  impl Default for RuleData {
    fn default() -> Self {
      RuleData {
        version: 1,
        name: b"".to_vec(),
        desc: b"".to_vec(),
        creator: CreatorId::default(),
        groups: vec![ForWhat::default()],
        parent_id: b"".to_vec(),
        ops: vec![],
      }
    }
  }

  /// Rule Info, what gets stored
  #[derive(Default, Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
  // #[cfg_attr(feature = "std", derive(Debug))]
  pub struct RuleInfo<A, B> {
    pub rule: Rule,
    pub account_id: A,
    pub block_number: B,
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
      Self::deposit_event(Event::RuleCreated(sender, rule_info.rule.id));

      Ok(().into())
    }
  }

  #[pallet::event]
  #[pallet::generate_deposit(pub(super)fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  pub enum Event<T: Config> {
    ///Rule is created
    RuleCreated(T::AccountId, GenericId),
  }

  #[pallet::error]
  pub enum Error<T> {
    ///Value was None
    NoneValue,
    ///Value reached maximum and cannot be incremented further
    StorageOverflow,
    ///Rule already exists
    RuleAlreadyCreated,
    ///Rule doesn't exits, create one.
    NoSuchRule,
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
    RuleInfo<T::AccountId, T::BlockNumber>,
    ValueQuery,
  >;

  #[pallet::storage]
  #[pallet::getter(fn rule_count)]
  /// Amount of saved rules
  pub type RuleCount<T: Config> = StorageValue<_, u32, ValueQuery>;

  /// Pallets external access to the saving in the storage
  pub trait PutInStorage {}

  impl<T: Config> PutInStorage for Pallet<T> {}

  impl<T: Config> Pallet<T> {
    /// Increase the Rule count
    fn increase_rule_count() -> u32 {
      let rule_count = Self::rule_count();
      let new_rule_count = &rule_count + 1;
      <RuleCount<T>>::put(new_rule_count);

      new_rule_count
    }

    /// Save the Rule to the Storage
    pub fn create(
      account_id: &T::AccountId,
      block_number: &T::BlockNumber,
      rule: &Rule,
    ) -> RuleInfo<T::AccountId, T::BlockNumber> {
      let rule_info = RuleInfo {
        rule: rule.clone(),
        account_id: account_id.clone(),
        block_number: *block_number,
      };

      Rules::<T>::insert(&rule.id, &account_id, rule_info.clone());
      Self::increase_rule_count();

      rule_info
    }
  }
}
