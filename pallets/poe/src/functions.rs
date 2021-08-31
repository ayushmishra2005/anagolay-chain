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

use super::*;

impl<T: Config> Pallet<T> {
  /// Increase the count
  ///
  /// Does no checks!
  ///
  /// Returns the new Total proof count
  pub fn increase_proof_count() -> u128 {
    let count = Self::proofs_count();
    let new_count = &count + 1;
    <ProofsCount<T>>::put(new_count);
    new_count
  }
  /// Increase the count
  ///
  /// Does no checks!
  ///
  /// Returns the new Total phash count
  pub fn increase_phash_count() -> u128 {
    let count = Self::phash_count();
    let new_count = &count + 1;
    <PHashCount<T>>::put(new_count);
    new_count
  }
}
