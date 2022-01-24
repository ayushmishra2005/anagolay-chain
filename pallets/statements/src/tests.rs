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

//! Tests for the statements module.

#![cfg(test)]

use super::{mock::*, *};
use crate::types::{AnagolayClaimType, AnagolayStatement};
use frame_support::{assert_noop, assert_ok};

#[test]
fn statements_create_ownership() {
  new_test_ext().execute_with(|| {
    let mut r = AnagolayStatement::default();
    r.data.claim.claim_type = AnagolayClaimType::Ownership;
    let res = TestStatements::create_ownership(mock::Origin::signed(1), r);
    assert_ok!(res);
  });
}
#[test]
fn statements_create_ownership_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let mut r = AnagolayStatement::default();
    r.data.claim.claim_type = AnagolayClaimType::Ownership;
    let res_first = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_ok!(res_first);

    let res_duplicate = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_noop!(res_duplicate, Error::<Test>::ProofHasStatement);
  });
}
#[test]
fn statements_create_ownership_wrong_claim_type() {
  new_test_ext().execute_with(|| {
    let r = AnagolayStatement::default();
    let res = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());

    assert_noop!(res, Error::<Test>::WrongClaimType);
  });
}

#[test]
fn statements_create_copyright() {
  new_test_ext().execute_with(|| {
    let r = AnagolayStatement::default();
    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_ok!(res);
  });
}
#[test]
fn copyright_create_child() {
  new_test_ext().execute_with(|| {
    let mut r = AnagolayStatement::default();
    r.data.claim.prev_id = b"my-fake-vec-id".to_vec();
    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::CreatingChildStatementNotSupported);
  });
}
#[test]
fn ownership_create_child() {
  new_test_ext().execute_with(|| {
    let mut r = AnagolayStatement::default();
    r.data.claim.prev_id = b"my-fake-vec-id".to_vec();
    r.data.claim.claim_type = AnagolayClaimType::Ownership;

    let res = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::CreatingChildStatementNotSupported);
  });
}
#[test]
fn statements_create_copyright_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let r = AnagolayStatement::default();
    let res_first = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_ok!(res_first);

    let res_duplicate = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res_duplicate, Error::<Test>::ProofHasStatement);
  });
}
#[test]
fn statements_create_copyright_wrong_claim_type() {
  new_test_ext().execute_with(|| {
    let mut r = AnagolayStatement::default();
    r.data.claim.claim_type = AnagolayClaimType::Ownership;

    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::WrongClaimType);
  });
}

#[test]
fn statements_revoke() {
  new_test_ext().execute_with(|| {
    let s = AnagolayStatement::default();
    let s_id = s.id.clone();
    let res1 = TestStatements::create_copyright(mock::Origin::signed(1), s.clone());
    assert_ok!(res1);
    let res2 = TestStatements::revoke(mock::Origin::signed(1), s_id);
    assert_ok!(res2);
  });
}

#[test]
fn statements_revoke_no_such_statements() {
  new_test_ext().execute_with(|| {
    let s_id = b"my-fake-vec-id".to_vec();
    let res = TestStatements::revoke(mock::Origin::signed(1), s_id);
    assert_noop!(res, Error::<Test>::NoSuchStatement);
  });
}

#[test]
fn statements_revoke_statement_has_child_statements() {
  new_test_ext().execute_with(|| {
    let mut s = AnagolayStatement::default();
    let _res1 = TestStatements::create_copyright(mock::Origin::signed(1), s.clone());

    // do this after the create, since it will fail because we don't accept this ATM
    s.data.claim.prev_id = b"child-statement-id".to_vec();

    StatementToPrevious::<Test>::insert(&s.id, &s.data.claim.prev_id);

    let res = TestStatements::revoke(mock::Origin::signed(1), s.id);

    assert_noop!(res, Error::<Test>::StatementHasChildStatement);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
