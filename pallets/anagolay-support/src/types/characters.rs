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
use crate::constants::*;
use codec::{Decode, Encode};
use core::str::pattern::Pattern;
use frame_support::{pallet_prelude::*, sp_std::vec::Vec, BoundedVec};

/// NewType pattern to handle strings.
/// It conveniently allows concatenation and deals with (de)serialization as well.
///
/// # Example
///
/// ```
/// use anagolay_support::Characters;
///
/// let chars: Characters = "hello".into();
///
/// assert_eq!(5, chars.len());
/// assert_eq!("hello2world", chars.concat_u8(2u8).concat("world").as_str());
/// ```
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, MaxEncodedLen, TypeInfo)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct Characters(BoundedVec<u8, MaxCharactersLenGet>);

impl From<Characters> for BoundedVec<u8, MaxCharactersLenGet> {
  fn from(from: Characters) -> BoundedVec<u8, MaxCharactersLenGet> {
    from.0
  }
}

impl From<&str> for Characters {
  /// Creates a [`Characters'] from a string.
  /// The method is infallible, but the result will be truncated to the limit allowed by
  /// ['MAX_CHARACTERS_LEN`]
  ///
  /// # Arguments
  /// * str - The string to parse as a ['Characters']
  ///
  /// # Return
  /// A ['Characters'] from the argument, truncated if it's larger than the allowed limit
  fn from(from: &str) -> Characters {
    let truncate_index = from.len().min(MAX_CHARACTERS_LEN as usize);
    let mut bytes = from.as_bytes()[0..truncate_index].to_vec();
    let mut bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MAX_CHARACTERS_LEN as usize);
    bounded_vec.try_append(&mut bytes).unwrap_or_default();
    Characters(bounded_vec)
  }
}

impl From<BoundedVec<u8, MaxCharactersLenGet>> for Characters {
  fn from(from: BoundedVec<u8, MaxCharactersLenGet>) -> Characters {
    let mut bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MAX_CHARACTERS_LEN as usize);
    bounded_vec.clone_from(&from);
    Characters(bounded_vec)
  }
}

impl From<&[u8]> for Characters {
  fn from(from: &[u8]) -> Characters {
    let mut bytes = from.to_vec();
    let mut bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MaxCharactersLenGet::get() as usize);
    bounded_vec.try_append(&mut bytes).unwrap_or_default();
    Characters(bounded_vec)
  }
}

impl Default for Characters {
  fn default() -> Self {
    let bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MAX_CHARACTERS_LEN as usize);
    Characters(bounded_vec)
  }
}

impl Characters {
  /// Convenience method to create `Characters` from a string slice when the result type is implicit
  ///
  /// # Example
  ///
  /// ```
  /// use anagolay_support::Characters;
  /// let chars = Characters::from("hello");
  /// ```
  ///
  /// Most of the time, it's convenient to use `str.into()`
  ///
  /// ```
  /// use anagolay_support::Characters;
  /// let chars: Characters = "hello".into();
  /// ```
  pub fn from(str: &str) -> Self {
    str.into()
  }

  /// # Return
  /// The `Characters` representation as a string slice
  pub fn as_str(&self) -> &str {
    frame_support::sp_std::str::from_utf8(self.0.as_slice()).unwrap()
  }

  /// # Return
  /// The `Characters` representation as a slice of bytes
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_slice()
  }

  /// Append an unsigned integer to this `Characters`
  /// This method is infallible, but the result will be truncated to the limit allowed by
  /// ['MAX_CHARACTERS_LEN`]
  ///
  /// # Arguments
  /// * uint - The unsigned integer to append
  ///
  /// # Return
  /// This `Characters` with the argument appended
  pub fn concat_u8(self, uint: u8) -> Self {
    let mut n = uint;
    let mut vec = self.0.to_vec();
    if n == 0 {
      vec.append(&mut b"0".to_vec());
    } else {
      let mut buffer = [0u8; 100];
      let mut i = 0;
      while n > 0 {
        buffer[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
      }
      let slice = &mut buffer[..i];
      slice.reverse();
      vec.append(&mut slice.to_vec());
    }
    let mut bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MAX_CHARACTERS_LEN as usize);
    let truncate_index = vec.len().min(MAX_CHARACTERS_LEN as usize);
    let mut truncated = vec.as_slice()[0..truncate_index].to_vec();
    bounded_vec.try_append(&mut truncated).unwrap_or_default();
    bounded_vec.into()
  }

  /// Concatenate a string slice to this `Characters`.
  /// This method is infallible, but the result will be truncated to the limit allowed by
  /// ['MAX_CHARACTERS_LEN`]
  ///
  /// # Arguments
  /// * other - The string slice to concatenate
  ///
  /// # Return
  /// This `Characters` with the argument concatenated, truncated if it's larger than the allowed
  /// limit
  pub fn concat(self, other: &str) -> Self {
    let mut vec = self.0.to_vec();
    vec.append(&mut other.as_bytes().to_vec());
    let mut bounded_vec: BoundedVec<u8, MaxCharactersLenGet> =
      BoundedVec::with_bounded_capacity(MAX_CHARACTERS_LEN as usize);
    let truncate_index = vec.len().min(MAX_CHARACTERS_LEN as usize);
    let mut truncated = vec.as_slice()[0..truncate_index].to_vec();
    bounded_vec.try_append(&mut truncated).unwrap_or_default();
    bounded_vec.into()
  }

  /// # Return
  /// This `Characters` length
  pub fn len(&self) -> usize {
    self.0.len()
  }

  /// Splits this `Characters` using pat as delimeter
  ///
  /// # Example
  ///
  /// ```
  /// use anagolay_support::Characters;
  ///
  /// let chars: Characters = "hello world".into();
  /// let parts: Vec<Characters> = chars.split(" ");
  ///
  /// assert_eq!(2, parts.len());
  /// assert_eq!(Characters::from("hello"), parts.get(0).unwrap().clone());
  /// assert_eq!(Characters::from("world"), parts.get(1).unwrap().clone());
  /// ```
  ///
  /// # Arguments
  /// * pat - Pattern to use as delimiter for the split
  ///
  /// # Return
  /// Collection of splits of this `Characters` using pat as delimeter
  pub fn split<'a, P: Pattern<'a>>(&'a self, pat: P) -> Vec<Characters> {
    self.as_str().split(pat).map(|split| split.into()).collect()
  }
}
