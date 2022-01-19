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

//! Tests for the module.

#![cfg(test)]
use super::{mock::*, *};
use crate::types::{Operation, OperationVersion};
use anagolay::AnagolayStructureData;
use frame_support::{assert_noop, assert_ok};

#[test]
fn operations_create_manifest() {
  new_test_ext().execute_with(|| {
    let op = Operation::default();
    let res = OperationTest::create_manifest(mock::Origin::signed(1), op.clone());
    assert_ok!(res);
  });
}

#[test]
fn operations_create_manifest_error_on_duplicate_operation() {
  new_test_ext().execute_with(|| {
    let op = Operation::default();
    let res = OperationTest::create_manifest(mock::Origin::signed(1), op.clone());
    assert_ok!(res);

    let res = OperationTest::create_manifest(mock::Origin::signed(1), op.clone());
    assert_noop!(res, Error::<Test>::OperationAlreadyExists);
  });
}

#[test]
fn operations_create_initial_version_error_on_nonexistent_operation() {
  new_test_ext().execute_with(|| {
    let op_ver = OperationVersion::default();
    let res = OperationTest::create_initial_version(mock::Origin::signed(1), op_ver.clone());
    assert_noop!(res, Error::<Test>::OperationDoesNotExists);
  });
}

#[test]
fn operations_create_initial_version_error_on_duplicate_version() {
  new_test_ext().execute_with(|| {
    let op = Operation::default();
    let res = OperationTest::create_manifest(mock::Origin::signed(1), op.clone());
    assert_ok!(res);

    let mut op_ver = OperationVersion::default();
    op_ver.data.operation_id = op.data.to_cid();
    let res = OperationTest::create_initial_version(mock::Origin::signed(1), op_ver);
    assert_noop!(res, Error::<Test>::OperationVersionAlreadyExists);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
