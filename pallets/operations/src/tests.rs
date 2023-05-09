// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.
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
use crate::types::{
  Operation, OperationArtifactType, OperationData, OperationId, OperationRecord, OperationVersion,
  OperationVersionData, OperationVersionExtra, OperationVersionId, OperationVersionRecord,
};
use anagolay_support::{AnagolayArtifactStructure, AnagolayStructureData, ArtifactId, Characters};
use core::convert::TryInto;
use frame_support::{assert_noop, assert_ok, sp_std::vec, traits::UnixTime};

fn mock_request() -> (Operation, OperationVersion) {
  let op = Operation {
    id: OperationId::from("bafkr4ih2xmsije6aa6yfwjdfmztnnkbb6ip56g3ojfcyfgjx6jsh6bogoe"),
    data: OperationData {
      name: "op_aaaaa".into(),
      description: "op_aaaaa description".into(),
      repository: "https://github.com/anagolay/op_aaaaa".into(),
      license: "Apache 2.0".into(),
      features: vec!["std".into()].try_into().unwrap(),
      ..OperationData::default()
    },
    extra: None,
  };
  let op_ver = OperationVersion {
    id: OperationVersionId::from("bafybeihc2e5rshwlkcg47uojrhtw7dwhyq2cxwivf3sysfnx5jtuuafvia"),
    data: OperationVersionData {
      entity_id: Some(op.id.clone()),
      parent_id: None,
      artifacts: vec![AnagolayArtifactStructure {
        artifact_type: OperationArtifactType::Git,
        file_extension: "git".into(),
        ipfs_cid: ArtifactId::from("bafkreibft6r6ijt7lxmbu2x3oq2s2ehwm5kz2nflwnlktdhcq2yfhgd4ku"),
      }]
      .try_into()
      .unwrap(),
    },
    extra: Some(OperationVersionExtra {
      created_at: <Test as crate::Config>::TimeProvider::now().as_secs(),
    }),
  };
  (op, op_ver)
}

#[test]
fn operations_test_genesis() {
  let (mut op, mut op_ver) = mock_request();
  op.id = op.data.to_cid();
  op_ver.data.entity_id = Some(op.id.clone());
  op_ver.id = op_ver.data.to_cid();
  let account_id: <Test as frame_system::Config>::AccountId = 1;
  new_test_ext(Some(crate::GenesisConfig::<Test> {
    operations: vec![OperationRecord::<Test> {
      record: op.clone(),
      account_id: account_id.clone(),
      block_number: 1,
    }]
    .try_into()
    .unwrap(),
    versions: vec![OperationVersionRecord::<Test> {
      record: op_ver.clone(),
      account_id: account_id.clone(),
      block_number: 1,
    }]
    .try_into()
    .unwrap(),
    total: 1,
  }))
  .execute_with(|| {
    let operation = OperationByOperationIdAndAccountId::<Test>::get(&op.id, account_id).unwrap();
    assert_eq!(operation.record.data, op.data);
    assert_eq!(operation.record.extra, op.extra);

    let operation_version_ids = VersionIdsByOperationId::<Test>::get(&op.id);
    assert_eq!(1, operation_version_ids.len());
    assert_eq!(&op_ver.id, operation_version_ids.get(0).unwrap());

    let artifacts = anagolay_support::Pallet::<Test>::get_artifacts();
    assert_eq!(1, artifacts.len());
    assert_eq!(op_ver.data.artifacts[0].ipfs_cid, *artifacts.get(0).unwrap());

    let version = VersionByVersionId::<Test>::get(&op_ver.id).unwrap();
    assert_eq!(version.record.data, op_ver.data);
    assert!(version.record.extra.is_some());

    let operation_total = Total::<Test>::get();
    assert_eq!(1, operation_total);
  });
}

#[test]
fn operations_create_operation() {
  new_test_ext(None).execute_with(|| {
    let (op, mut op_ver) = mock_request();

    // create an operation
    assert_ok!(OperationTest::create(
      mock::RuntimeOrigin::signed(1),
      op.data.clone(),
      op_ver.data.clone()
    ));

    let op_id = &op.data.to_cid();
    op_ver.data.entity_id = Some(op_id.clone());
    let op_ver_id = &op_ver.data.to_cid();

    // (operationId, accountId) -> opeartion
    let operation = OperationByOperationIdAndAccountId::<Test>::get(op_id, 1).unwrap();
    assert_eq!(operation.record.data, op.data);
    assert_eq!(operation.record.extra, op.extra);

    // operationId -> operationVersionId[]
    let operation_version_ids = VersionIdsByOperationId::<Test>::get(op_id);
    assert_eq!(1, operation_version_ids.len());
    assert_eq!(op_ver_id, operation_version_ids.get(0).unwrap());

    let artifacts = anagolay_support::Pallet::<Test>::get_artifacts();
    assert_eq!(1, artifacts.len());
    assert_eq!(op_ver.data.artifacts[0].ipfs_cid, *artifacts.get(0).unwrap());

    // operationVersionId -> version
    let version = VersionByVersionId::<Test>::get(op_ver_id).unwrap();
    assert_eq!(version.record.data, op_ver.data);
    assert!(version.record.extra.is_some());

    // operationId -> operation[]
    let operations = OperationTest::get_operations_by_ids([op.data.to_cid()].to_vec(), 0, 1);
    assert_eq!(operations.len(), 1);
    assert_eq!(operations[0].data, op.data);

    assert_eq!(OperationTest::get_operations_by_ids(vec![], 0, 0).len(), 0);

    // operationVersionId -> operationVersion[]
    let operation_versions = OperationTest::get_operation_versions_by_ids(operation_version_ids.to_vec(), 0, 1);
    assert_eq!(operation_versions.len(), 1);
    assert_eq!(operation_versions[0].data, op_ver.data);

    assert_eq!(OperationTest::get_operation_versions_by_ids(vec![], 0, 0).len(), 0);

    // version_approve
    assert_ok!(OperationTest::version_approve(
      mock::RuntimeOrigin::signed(1),
      op.data.to_cid()
    ));

    let operation_total = Total::<Test>::get();
    assert_eq!(1, operation_total);
  });
}

#[test]
fn operations_create_operation_error_on_duplicate_operation() {
  new_test_ext(None).execute_with(|| {
    let (op, op_ver) = mock_request();
    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_ok!(res);

    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::OperationAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_reusing_artifact() {
  new_test_ext(None).execute_with(|| {
    let (op, op_ver) = mock_request();

    anagolay_support::Pallet::<Test>::store_artifacts(&op_ver.data.artifacts).unwrap();

    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());

    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_mixing_operations() {
  new_test_ext(None).execute_with(|| {
    let (op_a, op_a_ver) = mock_request();

    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op_a.data.clone(), op_a_ver.data.clone());
    assert_ok!(res);

    let op_b = Operation {
      id: OperationId::default(),
      data: OperationData {
        name: "op_bbbbb".into(),
        description: "op_bbbbb description".into(),
        repository: "https://github.com/anagolay/op_bbbbb".into(),
        license: "Apache 2.0".into(),
        features: vec!["std".into()].try_into().unwrap(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_b_ver_mixed = OperationVersion {
      id: OperationVersionId::default(),
      data: op_a_ver.data.clone(),
      extra: None,
    };
    let res = OperationTest::create(
      mock::RuntimeOrigin::signed(1),
      op_b.data.clone(),
      op_b_ver_mixed.data.clone(),
    );
    assert_noop!(res, Error::<Test>::OperationVersionPackageAlreadyExists);
  });
}

#[test]
fn operations_create_operation_error_bad_request() {
  new_test_ext(None).execute_with(|| {
    let (mut op, mut op_ver) = mock_request();
    op.data.name = "this_is_a_very_very_very_very_very_very_very_very_very_very_looooong_operation_name_that_does_not_respect_maximum_length_constraint".into();
    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);

    op_ver.data.artifacts[0].ipfs_cid = ArtifactId::from("bafk_invalid_cid");
    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);
  });
}

#[test]
fn operations_create_operation_with_config() {
  new_test_ext(None).execute_with(|| {
    let (mut op, op_ver) = mock_request();
    op.data
      .config
      .try_insert(
        "test_key".into(),
        vec!["test_val0".into(), "test_val1".into()].try_into().unwrap(),
      )
      .unwrap();
    let res = OperationTest::create(mock::RuntimeOrigin::signed(1), op.data.clone(), op_ver.data.clone());
    assert_ok!(res);

    let op_id = &op.data.to_cid();
    let stored_op = OperationByOperationIdAndAccountId::<Test>::get(op_id, 1).unwrap();
    let mut stored_op_keys = stored_op.record.data.config.into_keys();
    let mut op_keys = op.data.config.into_keys();
    assert_eq!(stored_op_keys.next().unwrap(), op_keys.next().unwrap());
  });
}

#[test]
fn operatation_data_validate() {
  let str_4: Characters = "aaaa".into();
  let str_128: Characters = str::repeat("a", 128).as_str().into();
  let str_129: Characters = str::repeat("a", 129).as_str().into();
  let str_1024: Characters = str::repeat("a", 1024).as_str().into();
  let str_1025: Characters = str::repeat("a", 1025).as_str().into();

  let invalid_name: Result<(), Characters> = Err(Characters::from(
    "OperationData.name: length must be between 4 and 128 characters",
  ));
  let invalid_description: Result<(), Characters> = Err(Characters::from(
    "OperationData.description: length must be between 4 and 1024 characters",
  ));
  let invalid_repository: Result<(), Characters> = Err(Characters::from(
    "OperationData.repository: length must be between 4 and MaxCharactersLen characters",
  ));
  let invalid_license: Result<(), Characters> = Err(Characters::from(
    "OperationData.license: length must be between 4 and 128 characters",
  ));

  let mut op_data = OperationData::default();

  // name is too short
  assert_eq!(op_data.validate(), invalid_name.clone());

  op_data.name = str_129.clone();
  // name is too long
  assert_eq!(op_data.validate(), invalid_name.clone());

  // name is valid
  op_data.name = str_128.clone();

  // description is too short
  assert_eq!(op_data.validate(), invalid_description.clone());

  op_data.description = str_1025.clone();
  // description is too long
  assert_eq!(op_data.validate(), invalid_description.clone());

  op_data.description = str_1024.clone();

  // repository is too short
  assert_eq!(op_data.validate(), invalid_repository.clone());

  op_data.repository = str_1024.clone();

  // license is too short
  assert_eq!(op_data.validate(), invalid_license.clone());

  op_data.license = str_129;
  // license is too long
  assert_eq!(op_data.validate(), invalid_license.clone());

  op_data.license = str_4;
  assert_ok!(op_data.validate());
}

#[test]
fn test_template() {
  new_test_ext(None).execute_with(|| {});
}
