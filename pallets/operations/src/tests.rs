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
  Operation, OperationData, OperationVersion, OperationVersionData, OperationVersionPackage, PackageType,
};
use anagolay::AnagolayStructureData;
use frame_support::{assert_noop, assert_ok};

fn mock_request() -> (Operation, OperationVersion) {
  let op = Operation {
    id: vec![],
    data: OperationData {
      name: b"op_a".to_vec(),
      ..OperationData::default()
    },
    extra: None,
  };
  let op_ver = OperationVersion {
    id: vec![],
    data: OperationVersionData {
      operation_id: vec![],
      parent_id: None,
      rehosted_repo_id: b"bafkreporeporeporepo".to_vec(),
      documentation_id: b"bafkdocadocadocadoca".to_vec(),
      packages: vec![OperationVersionPackage {
        package_type: PackageType::CRATE,
        file_name: b"op_a.tgz".to_vec(),
        ipfs_cid: b"bafkopaopaopaopaopaopaopa".to_vec(),
      }],
    },
    extra: None,
  };
  (op, op_ver)
}

#[test]
fn operations_create_operation() {
  new_test_ext().execute_with(|| {
    let (op, mut op_ver) = mock_request();
    let origin = mock::Origin::signed(1);
    let res = OperationTest::create(origin, op.data.clone(), op_ver.data.clone());

    assert_ok!(res);

    let op_id = &op.data.to_cid();
    op_ver.data.operation_id = op_id.clone();
    let op_ver_id = &op_ver.data.to_cid();

    let operation = Operations::<Test>::get(1, op_id);
    assert_eq!(operation.record.data, op.data);
    assert_eq!(operation.record.extra, op.extra);

    let operation_versions = VersionsViaOperationId::<Test>::get(op_id);
    assert_eq!(1, operation_versions.len());
    assert_eq!(op_ver_id, operation_versions.get(0).unwrap());

    let packages = Packages::<Test>::get();
    assert_eq!(1, packages.len());
    assert_eq!(op_ver.data.packages[0].ipfs_cid, *packages.get(0).unwrap());

    let version = Versions::<Test>::get(op_ver_id);
    assert_eq!(version.record.data, op_ver.data);
    assert!(version.record.extra.is_some());

    let operation_total = OperationTotal::<Test>::get();
    assert_eq!(1, operation_total);
  });
}

#[test]
fn operations_create_operation_error_on_duplicate_operation() {
  new_test_ext().execute_with(|| {
    let (op, op_ver) = mock_request();
    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_ok!(res);

    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::OperationAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_reusing_package() {
  new_test_ext().execute_with(|| {
    let (op, op_ver) = mock_request();

    Packages::<Test>::set(
      op_ver
        .data
        .packages
        .iter()
        .map(|package| package.ipfs_cid.clone())
        .collect(),
    );

    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());

    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_mixing_operations() {
  new_test_ext().execute_with(|| {
    let (op_a, op_a_ver) = mock_request();

    let res = OperationTest::create(mock::Origin::signed(1), op_a.data.clone(), op_a_ver.data.clone());
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
    let res = OperationTest::create(mock::Origin::signed(1), op_b.data.clone(), op_b_ver_mixed.data.clone());
    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
