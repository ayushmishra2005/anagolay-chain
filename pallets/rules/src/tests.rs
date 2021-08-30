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
fn rule_create_default() {
  new_test_ext().execute_with(|| {
    let rd = Rule::default();
    let res = TestRules::create_rule(Origin::signed(1), rd.clone());
    assert_ok!(res);
  });
}
#[test]
fn rule_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let rd = Rule::default();
    let res1 = TestRules::create_rule(Origin::signed(1), rd.clone());
    assert_ok!(res1);

    let res2 = TestRules::create_rule(Origin::signed(1), rd.clone());
    assert_noop!(res2, Error::<Test>::RuleAlreadyCreated);
  });
}
