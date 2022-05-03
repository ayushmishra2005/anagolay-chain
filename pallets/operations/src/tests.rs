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
use crate::types::{Operation, OperationArtifactType, OperationData, OperationVersion, OperationVersionData};
use anagolay_support::{AnagolayArtifactStructure, AnagolayStructureData, ArtifactId, OperationId, VersionId};
use frame_support::{assert_noop, assert_ok};

fn mock_request() -> (Operation, OperationVersion) {
  let op = Operation {
    id: OperationId::from("bafkr4ih2xmsije6aa6yfwjdfmztnnkbb6ip56g3ojfcyfgjx6jsh6bogoe"),
    data: OperationData {
      name: "op_aaaaa".into(),
      description: "op_aaaaa description".into(),
      repository: "https://github.com/anagolay/op_aaaaa".into(),
      license: "Apache 2.0".into(),
      ..OperationData::default()
    },
    extra: None,
  };
  let op_ver = OperationVersion {
    id: VersionId::from("bafybeihc2e5rshwlkcg47uojrhtw7dwhyq2cxwivf3sysfnx5jtuuafvia"),
    data: OperationVersionData {
      entity_id: Some(op.id.clone()),
      parent_id: None,
      artifacts: vec![AnagolayArtifactStructure {
        artifact_type: OperationArtifactType::Git,
        file_extension: "git".into(),
        ipfs_cid: ArtifactId::from("bafkreibft6r6ijt7lxmbu2x3oq2s2ehwm5kz2nflwnlktdhcq2yfhgd4ku"),
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
    op_ver.data.entity_id = Some(op_id.clone());
    let op_ver_id = &op_ver.data.to_cid();

    let operation = OperationsByOperationIdAndAccountId::<Test>::get(op_id, 1);
    assert_eq!(operation.record.data, op.data);
    assert_eq!(operation.record.extra, op.extra);

    let operation_versions = VersionsByOperationId::<Test>::get(op_id);
    assert_eq!(1, operation_versions.len());
    assert_eq!(op_ver_id, operation_versions.get(0).unwrap());

    let artifacts = anagolay_support::Pallet::<Test>::get_artifacts();
    assert_eq!(1, artifacts.len());
    assert_eq!(op_ver.data.artifacts[0].ipfs_cid, *artifacts.get(0).unwrap());

    let version = VersionsByVersionId::<Test>::get(op_ver_id);
    assert_eq!(version.record.data, op_ver.data);
    assert!(version.record.extra.is_some());

    let operation_total = Total::<Test>::get();
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

    anagolay_support::Pallet::<Test>::store_artifacts(&op_ver.data.artifacts);

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
      id: OperationId::default(),
      data: OperationData {
        name: "op_bbbbb".into(),
        description: "op_bbbbb description".into(),
        repository: "https://github.com/anagolay/op_bbbbb".into(),
        license: "Apache 2.0".into(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_b_ver_mixed = OperationVersion {
      id: VersionId::default(),
      data: op_a_ver.data.clone(),
      extra: None,
    };
    let res = OperationTest::create(mock::Origin::signed(1), op_b.data.clone(), op_b_ver_mixed.data.clone());
    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_bad_request() {
  new_test_ext().execute_with(|| {
    let (mut op, mut op_ver) = mock_request();
    op.data.name = "this_is_a_very_very_very_very_very_very_very_very_very_very_looooong_operation_name_that_does_not_respect_maximum_length_constraint".into();
    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);

    op_ver.data.artifacts[0].ipfs_cid = ArtifactId::from("bafk_invalid_cid");
    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);
  });
}

#[test]
fn operations_create_operation_with_config() {
  new_test_ext().execute_with(|| {
    let (mut op, op_ver) = mock_request();
    op.data
      .config
      .insert("test_key".into(), vec!["test_val0".into(), "test_val1".into()]);
    let res = OperationTest::create(mock::Origin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_ok!(res);

    let op_id = &op.data.to_cid();
    let stored_op = OperationsByOperationIdAndAccountId::<Test>::get(op_id, 1);
    let mut stored_op_keys = stored_op.record.data.config.into_keys();
    let mut op_keys = op.data.config.into_keys();
    assert_eq!(stored_op_keys.next().unwrap(), op_keys.next().unwrap());
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
