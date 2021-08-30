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
use frame_support::{assert_noop, assert_ok};

#[test]
fn operations_create_operation() {
  new_test_ext().execute_with(|| {
    let op = OperationStructure::default();
    let res = OperationTest::create(Origin::signed(1), op.clone());
    assert_ok!(res);
  });
}
#[test]
fn operations_create_operation_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let op = OperationStructure::default();
    let res_first = OperationTest::create(Origin::signed(1), op.clone());
    assert_ok!(res_first);

    let op = OperationStructure::default();
    let res_duplicate = OperationTest::create(Origin::signed(1), op.clone());
    assert_noop!(res_duplicate, Error::<Test>::OperationAlreadyExists);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
