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

#![cfg(test)]
use super::{mock::*, *};
use crate::types::{
  Operation, OperationData, OperationVersion, OperationVersionData, OperationVersionPackage,
  PackageType,
};
use anagolay::AnagolayStructureData;
use frame_support::{assert_noop, assert_ok};

#[test]
fn operations_create_operation() {
  new_test_ext().execute_with(|| {
    let op = Operation::default();
    let op_ver = OperationVersion::default();
    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op.data.clone(),
      op_ver.data.clone(),
    );
    assert_ok!(res);
  });
}

#[test]
fn operations_create_operation_error_on_duplicate_operation() {
  new_test_ext().execute_with(|| {
    let op = Operation::default();
    let op_ver = OperationVersion::default();
    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op.data.clone(),
      op_ver.data.clone(),
    );
    assert_ok!(res);

    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op.data.clone(),
      op_ver.data.clone(),
    );
    assert_noop!(res, Error::<Test>::OperationAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_reusing_package() {
  new_test_ext().execute_with(|| {
    let op = Operation {
      id: vec![],
      data: OperationData {
        name: b"op".to_vec(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_ver = OperationVersion {
      id: vec![],
      data: OperationVersionData {
        operation_id: op.data.to_cid(),
        parent_id: None,
        rehosted_repo_id: b"https://github.com/op".to_vec(),
        packages: vec![OperationVersionPackage {
          package_type: PackageType::Crate,
          file_name: b"op.tgz".to_vec(),
          ipfs_cid: b"bafkopopopopopopop".to_vec(),
        }],
      },
      extra: None,
    };

    PackagesCid::<Test>::set(
      op_ver
        .data
        .packages
        .iter()
        .map(|package| package.ipfs_cid.clone())
        .collect(),
    );

    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op.data.clone(),
      op_ver.data.clone(),
    );

    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_mixing_operations() {
  new_test_ext().execute_with(|| {
    let op_a = Operation {
      id: vec![],
      data: OperationData {
        name: b"op_a".to_vec(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_a_ver = OperationVersion {
      id: vec![],
      data: OperationVersionData {
        operation_id: vec![],
        parent_id: None,
        rehosted_repo_id: b"https://github.com/op_a".to_vec(),
        packages: vec![OperationVersionPackage {
          package_type: PackageType::Crate,
          file_name: b"op_a.tgz".to_vec(),
          ipfs_cid: b"bafkopaopaopaopaopaopaopa".to_vec(),
        }],
      },
      extra: None,
    };

    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op_a.data.clone(),
      op_a_ver.data.clone(),
    );
    assert_ok!(res);

    let op_b = Operation {
      id: vec![],
      data: OperationData {
        name: b"op_b".to_vec(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_b_ver_mixed = OperationVersion {
      id: vec![],
      data: op_a_ver.data.clone(),
      extra: None,
    };
    let res = OperationTest::create_operation(
      mock::Origin::signed(1),
      op_b.data.clone(),
      op_b_ver_mixed.data.clone(),
    );
    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
