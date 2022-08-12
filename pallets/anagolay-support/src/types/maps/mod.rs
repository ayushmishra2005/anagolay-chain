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

//! This module contains the shared datamodel of the chain,
//! like structs that deal with strings and serializable bounded maps

#[cfg(feature = "std")]
mod serializable;
#[cfg(feature = "std")]
pub use serializable::*;

#[cfg(not(feature = "std"))]
pub type MaybeSerializableBoundedBTreeMap<K, V, S> =
  frame_support::storage::bounded_btree_map::BoundedBTreeMap<K, V, S>;
