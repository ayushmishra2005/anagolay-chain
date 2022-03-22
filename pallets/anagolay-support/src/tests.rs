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

//! Tests for the module.

use super::{mock::*, *};

#[test]
fn check_is_existing_package() {
  new_test_ext().execute_with(|| {
    enum TestArtifactType {
      TEST,
    }
    impl ArtifactType for TestArtifactType {}

    let test_cid = b"bafktesttesttest".to_vec();
    let package = AnagolayPackageStructure {
      package_type: TestArtifactType::TEST,
      ipfs_cid: test_cid.clone(),
      file_name: None,
    };

    assert!(!AnagolayTest::is_existing_package(&package));

    AnagolayTest::store_packages(&vec![package]);

    let packages = AnagolayTest::get_packages();
    assert_eq!(1, packages.len());
    assert_eq!(test_cid.clone(), *packages.get(0).unwrap())
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
