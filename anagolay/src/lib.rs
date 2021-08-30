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

use codec::{Decode, Encode};
use sp_std::vec::Vec;

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
