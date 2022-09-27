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

getter_for_hardcoded_constant!(MaxGenericIdLen, u32, 60);

/// This trait is implemented by all the generic CIDv1 ids
pub trait GenericId: for<'a> From<&'a str> + Default {
  /// # Return
  /// The bytes of the CID
  fn as_bytes(&self) -> &[u8];

  /// Validate the CID. It will require a string to be multibase-decoded and then parsed as CID
  ///
  /// # Return
  /// An unit result if the validation is successful, a `Character` error with a description in
  /// case it fails
  fn validate(&self) -> Result<(), Characters> {
    use core::convert::TryFrom;
    multibase::decode(frame_support::sp_std::str::from_utf8(self.as_bytes()).unwrap())
      .map(|decoded| cid::Cid::try_from(decoded.1).map_err(|_| Characters::from("Invalid CID")))
      .map(|_| ())
      .map_err(|_| Characters::from("Cannot decode CID"))
  }
}

/// This macro produces a generic CIDv1 id wrapper struct, implementing the GenericId trait.
///
/// According to the identifier `n` passed in as argument, the resulting struct will be called
/// `<n>Id`
///
/// # Arguments
///  * n - The name of the entity for which to generate an Id type
#[macro_export]
macro_rules! anagolay_generic_id {
  ($n: ident) => {
    $crate::paste::paste! {
      /// This is the content identifier of the payload, like Worflow or Proof. It's a
      /// multibase encoded CID string. It is in a private module, aliased by the types of each
      /// respective entity id.
      ///
      /// Id aliases are also an important indicator of what type of id is expected in which place:
      /// instead of writing documentation we can immediately show the user what the storage or the
      /// data model expects.
      ///
      /// It follows NewType pattern to provide conversion to and from [`BoundedVec<u8,
      /// MaxGenericIdLenGet>`] for (de)serialization but also to provide additional behaviour, like
      /// validation.
      ///
      /// Refer to [`anagolay_generic_id`] macro for more details.
      #[derive(codec::Encode, codec::Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, MaxEncodedLen, TypeInfo)]
      #[cfg_attr(feature = "std", derive(serde::Serialize))]
      pub struct [<$n Id>](BoundedVec<u8, MaxGenericIdLenGet>);

      impl GenericId for [<$n Id>] {
        fn as_bytes(&self) -> &[u8] {
          self.0.as_slice()
        }
      }

      impl [<$n Id>] {
      }

      impl From<&[u8]> for [<$n Id>] {
        fn from(from: &[u8]) -> [<$n Id>] {
          let mut bytes = from.to_vec();
          let mut bounded_vec: BoundedVec<u8, MaxGenericIdLenGet> =
            BoundedVec::with_bounded_capacity(MaxGenericIdLenGet::get() as usize);
          bounded_vec.try_append(&mut bytes).unwrap_or_default();
          [<$n Id>](bounded_vec)
        }
      }

      impl From<&str> for [<$n Id>] {
        fn from(from: &str) -> [<$n Id>] {
          from.as_bytes().into()
        }
      }

      impl From<[<$n Id>]> for BoundedVec<u8, MaxGenericIdLenGet> {
        fn from(from: [<$n Id>]) -> BoundedVec<u8, MaxGenericIdLenGet> {
          from.0
        }
      }

      impl From<BoundedVec<u8, MaxGenericIdLenGet>> for [<$n Id>] {
        fn from(from: BoundedVec<u8, MaxGenericIdLenGet>) -> [<$n Id>] {
          let mut bounded_vec: BoundedVec<u8, MaxGenericIdLenGet> =
            BoundedVec::with_bounded_capacity(MaxGenericIdLenGet::get() as usize);
          bounded_vec.clone_from(&from);
          [<$n Id>](bounded_vec)
        }
      }

      impl Default for [<$n Id>] {
        fn default() -> Self {
          let bounded_vec: BoundedVec<u8, MaxGenericIdLenGet> =
            BoundedVec::with_bounded_capacity(MaxGenericIdLenGet::get() as usize);
            [<$n Id>](bounded_vec)
        }
      }

      #[cfg(feature = "std")]
      impl<'de> serde::Deserialize<'de> for [<$n Id>] {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
          D: serde::Deserializer<'de>,
        {
          let deserialized = String::deserialize(deserializer)
            .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?;
          let start = if deserialized.starts_with("0x") { 2 } else { 0 };
          let bytes: Result<Vec<u8>, D::Error>  = (start..deserialized.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&deserialized[i..i + 2], 16)
            .map_err(|e| serde::de::Error::custom(format!("{:?}", e))))
            .collect();
          Ok(Self::from(bytes.unwrap_or_default().as_slice()))
        }
      }
    }
  };
}
