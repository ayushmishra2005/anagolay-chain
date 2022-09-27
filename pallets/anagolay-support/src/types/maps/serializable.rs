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

use crate::Characters;
use codec::{Decode, Encode};
use frame_support::{pallet_prelude::*, storage::bounded_btree_map::BoundedBTreeMap};
use serde::{
  ser::{Error, SerializeMap},
  *,
};
use std::{
  any::{Any, TypeId},
  collections::btree_map::IntoKeys,
};

/// NewType pattern to handle (de)serializable BoundedBTreeMap.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Ord, PartialOrd, Debug, MaxEncodedLen, TypeInfo)]
pub struct MaybeSerializableBoundedBTreeMap<K: Ord + Serialize, V: Ord + Serialize, S: Get<u32>>(
  BoundedBTreeMap<K, V, S>,
);

/// Implementation of MaybeSerializableBoundedBTreeMap.
/// Delegates to the inner type
impl<K, V, S> MaybeSerializableBoundedBTreeMap<K, V, S>
where
  K: Ord + Serialize + Clone,
  V: Ord + Serialize + Clone,
  S: Get<u32>,
{
  /// Create a new `MaybeSerializableBoundedBTreeMap`.
  ///
  /// Does not allocate.
  pub fn new() -> Self {
    MaybeSerializableBoundedBTreeMap(BoundedBTreeMap::<K, V, S>::new())
  }

  /// Exactly the same semantics as [`BTreeMap::insert`], but returns an `Err` (and is a noop) if
  /// the new length of the map exceeds `S`.
  ///
  /// In the `Err` case, returns the inserted pair so it can be further used without cloning.
  pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, (K, V)> {
    self.0.try_insert(key, value)
  }

  /// Creates a consuming iterator visiting all the keys, in sorted order.
  /// The map cannot be used after calling this.
  /// The iterator element type is `K`.
  ///
  /// # Examples
  ///
  /// ```
  /// use anagolay_support::{getter_for_hardcoded_constant, MaybeSerializableBoundedBTreeMap};
  ///
  /// getter_for_hardcoded_constant!(MapSize, u32, 2);
  ///
  /// let mut a = MaybeSerializableBoundedBTreeMap::<i32, &str, MapSizeGet>::new();
  /// a.try_insert(2, "b").unwrap();
  /// a.try_insert(1, "a").unwrap();
  /// assert!(a.try_insert(3, "c").is_err());
  ///
  /// let keys: Vec<i32> = a.into_keys().collect();
  /// assert_eq!(keys, [1, 2]);
  /// ```
  pub fn into_keys(self) -> IntoKeys<K, V> {
    self.0.into_inner().into_keys()
  }
}

/// Serialization of MaybeSerializableBoundedBTreeMap
/// Delegates to the inner type
impl<K, V, S> Serialize for MaybeSerializableBoundedBTreeMap<K, V, S>
where
  K: Ord + Serialize + Clone + 'static,
  V: Ord + Serialize + Clone,
  S: Get<u32>,
{
  fn serialize<U>(&self, s: U) -> Result<U::Ok, U::Error>
  where
    U: Serializer,
  {
    let mut ser_map = s.serialize_map(None)?;
    let mut iter = self.0.iter();
    if TypeId::of::<K>() == TypeId::of::<String>() {
      // handle strings specially so they don't get escaped and wrapped inside another string
      while let Some((k, v)) = iter.next() {
        let s = (k as &dyn Any)
          .downcast_ref::<String>()
          .ok_or(U::Error::custom("Failed to serialize String as string"))?;
        ser_map.serialize_entry(s, &v)?;
      }
    } else if TypeId::of::<K>() == TypeId::of::<Characters>() {
      // handle Characters specially so they don't get serialized to Hex
      while let Some((k, v)) = iter.next() {
        let s = (k as &dyn Any)
          .downcast_ref::<Characters>()
          .ok_or(U::Error::custom("Failed to serialize Characters as string"))?
          .as_str()
          .to_string();
        ser_map.serialize_entry(&s, &v)?;
      }
    } else {
      while let Some((k, v)) = iter.next() {
        ser_map.serialize_entry(
          match &serde_json::to_string(&k) {
            Ok(key_string) => key_string,
            Err(e) => {
              return Err(e).map_err(U::Error::custom);
            }
          },
          &v,
        )?;
      }
    }
    ser_map.end()
  }
}

/// A Visitor is a type that holds methods that a Deserializer can drive
/// depending on what is contained in the input data.
///
/// In the case of a map we need generic type parameters K and V to be
/// able to set the output type correctly, but don't require any state.
/// This is an example of a "zero sized type" in Rust. The PhantomData
/// keeps the compiler from complaining about unused generic type
/// parameters.
struct MaybeSerializableBoundedBTreeMapVisitor<K, V, S>
where
  K: Ord + Serialize,
  V: Ord + Serialize,
  S: Get<u32>,
{
  marker: PhantomData<fn() -> MaybeSerializableBoundedBTreeMap<K, V, S>>,
}

/// This is the trait that Deserializers are going to be driving. There
/// is one method for each type of data that our type knows how to
/// deserialize from. There are many other methods that are not
/// implemented here, for example deserializing from integers or strings.
/// By default those methods will return an error, which makes sense
/// because we cannot deserialize a MaybeSerializableBoundedBTreeMapVisitor
/// from an integer or string.
impl<K, V, S> MaybeSerializableBoundedBTreeMapVisitor<K, V, S>
where
  K: Ord + Serialize,
  V: Ord + Serialize,
  S: Get<u32>,
{
  fn new() -> Self {
    MaybeSerializableBoundedBTreeMapVisitor { marker: PhantomData }
  }
}

impl<'de, K, V, S> de::Visitor<'de> for MaybeSerializableBoundedBTreeMapVisitor<K, V, S>
where
  K: Ord + Serialize + Deserialize<'de> + Clone,
  V: Ord + Serialize + Deserialize<'de> + Clone,
  S: Get<u32>,
{
  // The type that our Visitor is going to produce.
  type Value = MaybeSerializableBoundedBTreeMap<K, V, S>;

  // Format a message stating what data this Visitor expects to receive.
  fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
    formatter.write_str("MaybeSerializableBoundedBTreeMap")
  }

  // Deserialize MyMap from an abstract "map" provided by the
  // Deserializer. The MapAccess input is a callback provided by
  // the Deserializer to let us see each entry in the map.
  fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
  where
    M: de::MapAccess<'de>,
  {
    let mut map = MaybeSerializableBoundedBTreeMap::<K, V, S>::new();

    // While there are entries remaining in the input, add them
    // into our map.
    while let Some((key, value)) = access.next_entry()? {
      map
        .try_insert(key, value)
        .map_err(|_err| de::Error::custom("MaybeSerializableBoundedBTreeMap::try_insert failed: Out of bounds"))?;
    }

    Ok(map)
  }
}

/// This is the trait that informs Serde how to deserialize MaybeSerializableBoundedBTreeMap.
impl<'de, K: Ord + Serialize + Deserialize<'de> + Clone, V: Ord + Serialize + Deserialize<'de> + Clone, S: Get<u32>>
  Deserialize<'de> for MaybeSerializableBoundedBTreeMap<K, V, S>
{
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    // Instantiate our Visitor and ask the Deserializer to drive
    // it over the input data, resulting in an instance of MaybeSerializableBoundedBTreeMapVisitor.
    deserializer.deserialize_map(MaybeSerializableBoundedBTreeMapVisitor::<K, V, S>::new())
  }
}
