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

//! Business logic is located here.
//!
//! Each pallet must have this file.

use super::*;
use sp_std::vec::Vec;

impl<T: Config> Pallet<T> {
  /// Verifies if the package passed as argument is already stored
  ///
  /// # Arguments
  ///  * package - The package to check
  ///
  /// # Return
  /// True if the package is already stored, false otherwise
  pub fn is_existing_package(package: &AnagolayPackageStructure<impl ArtifactType>) -> bool {
    match PackagesByPackageId::<T>::try_get().ok() {
      Some(packages) => packages.contains(&package.ipfs_cid),
      None => false,
    }
  }

  /// Store all packages passed as parameter.
  ///
  /// Does not do any check.
  ///
  /// # Arguments
  ///  * packages - The packages to store
  pub fn store_packages(packages: &Vec<AnagolayPackageStructure<impl ArtifactType>>) {
    PackagesByPackageId::<T>::mutate(|stored_packages| {
      packages
        .iter()
        .for_each(|package| stored_packages.push(package.ipfs_cid.clone()));
    });
  }

  /// Retrieve all stored packages
  ///
  /// # Arguments
  ///  * packages - The packages to store
  pub fn get_packages() -> Vec<PackageId> {
    PackagesByPackageId::<T>::get()
  }
}
