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

use crate::{getter_for_hardcoded_constant, types::characters::Characters};
use cid::Cid;
use codec::{Decode, Encode};
use core::convert::TryFrom;
use frame_support::{
  pallet_prelude::*,
  sp_std::{convert::From, str},
  BoundedVec,
};

getter_for_hardcoded_constant!(MaxGenericIdLen, u32, 60);

/// Generic ID, this is the content identifier of the payload, like Worflow or Proof. It's a
/// multibase encoded CID string. It must be in a private module, aliased by the types of each
/// respective entity id, since it's used in [`AnagolayVersionData`] to refer to any entity id.
///
/// Id aliases are also an important indicator of what type of id is expected in which place:
/// instead of writing documentation we can immediately show the user what the storage or the
/// data model expects.
///
/// It follows NewType pattern to provide conversion to and from [`BoundedVec<u8,
/// MaxGenericIdLenGet>`] for (de)serialization but also to provide additional behaviour, like
/// validation.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericId(pub BoundedVec<u8, MaxGenericIdLenGet>);

impl From<GenericId> for BoundedVec<u8, MaxGenericIdLenGet> {
  fn from(from: GenericId) -> BoundedVec<u8, MaxGenericIdLenGet> {
    from.0
  }
}

impl From<BoundedVec<u8, MaxGenericIdLenGet>> for GenericId {
  fn from(from: BoundedVec<u8, MaxGenericIdLenGet>) -> GenericId {
    GenericId(from)
  }
}

impl Default for GenericId {
  fn default() -> Self {
    let bounded_vec: BoundedVec<u8, MaxGenericIdLenGet> = BoundedVec::with_bounded_capacity(0);
    bounded_vec.into()
  }
}

impl GenericId {
  /// Creates a [`GenericId'] from a string.
  /// The method is infallible, but an empty ['GenericId'] will be created if the string is longer
  /// than the limit allowed by ['MAX_GENERIC_ID_LEN`]
  ///
  /// # Arguments
  /// * str - The string to parse as a ['GenericId']
  ///
  /// # Return
  /// A ['GenericId'] from the argument, or empty in case of error
  pub fn from(str: &str) -> Self {
    let mut bytes = str.as_bytes().to_vec();
    let mut bounded_vec: BoundedVec<u8, MaxGenericIdLenGet> = BoundedVec::with_bounded_capacity(bytes.len());
    bounded_vec.try_append(&mut bytes).unwrap_or_default();
    GenericId(bounded_vec)
  }

  /// Validate the CID. It will require a string to be multibase-decoded and then parsed as CID
  ///
  /// # Return
  /// An unit result if the validation is successful, a `Character` error with a description in
  /// case it fails
  pub fn validate(&self) -> Result<(), Characters> {
    multibase::decode(str::from_utf8(&self.0).unwrap())
      .map(|decoded| Cid::try_from(decoded.1).map_err(|_| Characters::from("Invalid CID")))
      .map(|_| ())
      .map_err(|_| Characters::from("Cannot decode CID"))
  }
}
