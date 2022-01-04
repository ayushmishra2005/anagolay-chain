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

// this doesn't need to be written like this
// tracked here https://gitlab.com/anagolay/node/-/issues/51

pub use pallet::*;

#[frame_support::pallet]
mod pallet {
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
      Something::<T>::put(something);

      // Here we are raising the Something event
      Self::deposit_event(Event::SomethingStored(something, who));
      Ok(().into())
    }
  }

  #[pallet::event]
  #[pallet::generate_deposit(fn deposit_event)]
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
  #[pallet::getter(fn something)]
  // Just a dummy storage item.
  // Here we are declaring a StorageValue, `Something` as a Option<u32>
  // `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
  pub type Something<T: Config> = StorageValue<_, u32>;

  /// Generic ID, this is the content identifier of the payload, like rule or proof. for now it's CID string
  pub type GenericId = Vec<u8>;

  /// Placeholder for SSI and DID
  pub type CreatorId = Vec<u8>;

  /// List of equipment that needs rules generated
  #[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
  // #[cfg_attr(feature = "std", derive(Debug))]
  pub enum ForWhat {
    /// WE are creating it For what? This can be a part of the group
    GENERIC, // 0
    PHOTO,       // 1
    CAMERA,      // 2
    LENS,        // 3
    SMARTPHONE,  // 4
    USER,        // 5
    SYS,         // 6
    FLOWCONTROL, // 7
  }

  impl Default for ForWhat {
    fn default() -> Self {
      ForWhat::GENERIC
    }
  }

  /// Default values Hashing
  #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  #[cfg_attr(feature = "std", derive(Debug))]
  pub struct DefaultsHashing {
    algo: Vec<u8>,
    bits: u32,
  }

  impl Default for DefaultsHashing {
    fn default() -> Self {
      DefaultsHashing {
        algo: b"blake2b".to_vec(),
        bits: 256,
      }
    }
  }

  /// Default values Encoding
  #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  #[cfg_attr(feature = "std", derive(Debug))]
  pub struct DefaultsEncoding {
    algo: Vec<u8>,
    prefix: bool,
  }

  impl Default for DefaultsEncoding {
    fn default() -> Self {
      DefaultsEncoding {
        algo: b"hex".to_vec(),
        prefix: true,
      }
    }
  }

  /// Default values Content Identifier or CID
  #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  #[cfg_attr(feature = "std", derive(Debug))]
  pub struct DefaultsCid {
    version: u8,
    base: Vec<u8>,
    codec: Vec<u8>,
  }

  impl Default for DefaultsCid {
    fn default() -> Self {
      DefaultsCid {
        version: 1,
        base: b"base32".to_vec(),
        codec: b"dag-cbor".to_vec(),
      }
    }
  }

  /// Default values for this runtime
  #[derive(Encode, Decode, Clone, PartialEq, Eq)]
  #[cfg_attr(feature = "std", derive(Debug))]
  pub struct DefaultValues {
    hashing: DefaultsHashing,
    encoding: DefaultsEncoding,
    cid: DefaultsCid,
  }

  impl Default for DefaultValues {
    fn default() -> Self {
      DefaultValues {
        hashing: DefaultsHashing::default(),
        encoding: DefaultsEncoding::default(),
        cid: DefaultsCid::default(),
      }
    }
  }
}
