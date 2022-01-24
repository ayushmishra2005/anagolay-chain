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

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};

pub use pallet::*;

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
    type Event: From<Event<Self>>
      + Into<<Self as frame_system::Config>::Event>
      + IsType<<Self as frame_system::Config>::Event>;
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn foo(origin: OriginFor<T>, something: u32) -> DispatchResultWithPostInfo {
      // Check it was signed and get the signer. See also: ensure_root and ensure_none
      let who = ensure_signed(origin)?;

      // Code to execute when something calls this.
      // For example: the following line stores the passed in u32 in the storage
      <Foo<T>>::put(something);

      // Here we are raising the Something event
      Self::deposit_event(Event::SomethingStored(something, who));
      Ok(().into())
    }
  }

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  #[pallet::metadata(T::AccountId = "AccountId")]
  pub enum Event<T: Config> {
    /// Just a dummy event.
    /// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
    /// To emit this event, we call the deposit function, from our runtime functions
    SomethingStored(u32, T::AccountId),
  }

  #[pallet::error]
  pub enum Error<T> {
    ///Value was None
    NoneValue,
    /// Value reached maximum and cannot be incremented further
    StorageOverflow,
  }

  #[pallet::storage]
  #[pallet::getter(fn foo)]
  // Just a dummy storage item.
  // Here we are declaring a StorageValue, `Something` as a Option<u32>
  // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
  pub type Foo<T: Config> = StorageValue<_, u32>;
}
