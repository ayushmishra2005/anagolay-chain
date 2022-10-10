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
use crate::types::{ClaimType, StatementData, StatementId};
use anagolay_support::{AnagolayStructureData, Characters};
use codec::Encode;
use core::convert::TryInto;
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::{sr25519, Pair};

fn sign_statement(statement_data: &mut StatementData) {
  let (pair, _) = sr25519::Pair::from_string_with_seed("//Alice", None).unwrap();

  let message = [
    "<Bytes>".as_bytes(),
    &statement_data.claim.encode(),
    "</Bytes>".as_bytes(),
  ]
  .concat();
  let signature = pair.sign(message.as_slice()).encode();
  statement_data.signatures.holder.sig = signature.try_into().unwrap();
  statement_data.signatures.holder.sig_key =
    Characters::from(format!("urn:substrate:0x{}", hex::encode(pair.public())).as_str());
}

#[test]
fn statements_create_ownership() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    r.claim.claim_type = ClaimType::Ownership;
    sign_statement(&mut r);

    let res = TestStatements::create_ownership(mock::Origin::signed(1), r);
    assert_ok!(res);
  });
}
#[test]
fn statements_create_ownership_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    r.claim.claim_type = ClaimType::Ownership;
    sign_statement(&mut r);

    let res_first = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_ok!(res_first);

    let res_duplicate = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_noop!(res_duplicate, Error::<Test>::ProofHasStatement);
  });
}
#[test]
fn statements_create_ownership_wrong_claim_type() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    sign_statement(&mut r);

    let res = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());

    assert_noop!(res, Error::<Test>::WrongClaimType);
  });
}

#[test]
fn statements_create_copyright() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    sign_statement(&mut r);

    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_ok!(res);
  });
}
#[test]
fn copyright_create_child() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    r.claim.prev_id = Some(StatementId::from("my-fake-vec-id"));
    sign_statement(&mut r);

    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::CreatingChildStatementNotSupported);
  });
}
#[test]
fn ownership_create_child() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    r.claim.prev_id = Some(StatementId::from("my-fake-vec-id"));
    r.claim.claim_type = ClaimType::Ownership;
    sign_statement(&mut r);

    let res = TestStatements::create_ownership(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::CreatingChildStatementNotSupported);
  });
}
#[test]
fn statements_create_copyright_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    sign_statement(&mut r);

    let res_first = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_ok!(res_first);

    let res_duplicate = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res_duplicate, Error::<Test>::ProofHasStatement);
  });
}
#[test]
fn statements_create_copyright_wrong_claim_type() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    r.claim.claim_type = ClaimType::Ownership;
    sign_statement(&mut r);

    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::WrongClaimType);
  });
}

#[test]
fn statements_create_copyright_invalid_signature() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    sign_statement(&mut r);
    r.signatures.holder.sig_key = "urn:substrate:0xCAFEBABE".into();

    let res = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());
    assert_noop!(res, Error::<Test>::InvalidSignature);
  });
}

#[test]
fn statements_revoke() {
  new_test_ext().execute_with(|| {
    let mut s = StatementData::default();
    sign_statement(&mut s);
    let s_id = s.to_cid();

    let res1 = TestStatements::create_copyright(mock::Origin::signed(1), s.clone());
    assert_ok!(res1);
    let res2 = TestStatements::revoke(mock::Origin::signed(1), s_id);
    assert_ok!(res2);
  });
}

#[test]
fn statements_revoke_no_such_statements() {
  new_test_ext().execute_with(|| {
    let s_id = StatementId::from("my-fake-vec-id");

    let res = TestStatements::revoke(mock::Origin::signed(1), s_id);
    assert_noop!(res, Error::<Test>::NoSuchStatement);
  });
}

#[test]
fn statements_revoke_statement_has_child_statements() {
  new_test_ext().execute_with(|| {
    let mut r = StatementData::default();
    sign_statement(&mut r);
    let s_id = r.to_cid();

    let _res1 = TestStatements::create_copyright(mock::Origin::signed(1), r.clone());

    // do this after the create, since it will fail because we don't accept this ATM
    r.claim.prev_id = Some(StatementId::from("child-statement-id"));
    sign_statement(&mut r);

    ParentStatementIdByStatementId::<Test>::insert(&s_id, &r.claim.prev_id.unwrap());

    let res = TestStatements::revoke(mock::Origin::signed(1), s_id);

    assert_noop!(res, Error::<Test>::StatementHasChildStatement);
  });
}

#[test]
fn statements_signature_verification_substrate() {
  use crate::types::*;
  use anagolay_support::CreatorId;
  use frame_support::pallet_prelude::Get;
  use poe::types::ProofId;
  use workflows::types::WorkflowId;

  new_test_ext().execute_with(|| {
    let claim: Claim = Claim {
      prev_id: None,
      poe_id: ProofId::from("bafkr4ifwrblquyv4hskayffmo7llmpcha4vkgcfwcgeenzt63u5m74ukz4"),
      workflow_id: WorkflowId::from("bafkr4icflbi5pbomtcyejivr4l7dcdvcmvcsviwmnn7qp52flfnkvy2ebe"),
      proportion: Proportion {
        name: "percentage".into(),
        sign: "%".into(),
        value: "100".into(),
      },
      subject_id: ProofId::from("bafkr4ifwrblquyv4hskayffmo7llmpcha4vkgcfwcgeenzt63u5m74ukz4"),
      holder: CreatorId::from("5EHkcDMhHgwW7V4GR4Us4dhkPkP9f2k71kdSVbzzzpNsHsYo"),
      issuer: CreatorId::from("5EHkcDMhHgwW7V4GR4Us4dhkPkP9f2k71kdSVbzzzpNsHsYo"),
      claim_type: ClaimType::Ownership,
      valid: Validity {
        from: "1664967449137".into(),
        until: "".into(),
      },
      expiration: Expiration {
        expiration_type: ExpirationType::Forever,
        value: "".into(),
      },
      on_expiration: "".into(),
    };

    let encoded = claim.encode();
    let message = ["<Bytes>".as_bytes(), &encoded, "</Bytes>".as_bytes()].concat();
    let (pair, _) = sr25519::Pair::from_string_with_seed("//Alice", None).unwrap();

    let signature_wrapped = pair.sign(message.as_slice()).encode();
    let signature = pair.sign(&encoded).encode();
    let public_key = format!("0x{}", hex::encode(pair.public()));

    let mut bounded_vec: BoundedVec<u8, MaxSignatureLenGet> =
      BoundedVec::with_bounded_capacity(MaxSignatureLenGet::get() as usize);
    bounded_vec
      .try_append(&mut signature_wrapped.try_into().unwrap())
      .unwrap_or_default();

    let holder_signature = Signature {
      sig_key: format!("urn:substrate:{}", public_key).as_str().into(),
      sig: bounded_vec,
      cid: "signature_cid".into(),
    };

    assert!(TestStatements::verify_substrate_signature(
      &claim,
      &holder_signature,
      &public_key
    ));

    let mut bounded_vec: BoundedVec<u8, MaxSignatureLenGet> =
      BoundedVec::with_bounded_capacity(MaxSignatureLenGet::get() as usize);
    bounded_vec
      .try_append(&mut signature.try_into().unwrap())
      .unwrap_or_default();

    let holder_signature = Signature {
      sig_key: format!("urn:substrate:{}", public_key).as_str().into(),
      sig: bounded_vec,
      cid: "signature_cid".into(),
    };

    assert!(TestStatements::verify_substrate_signature(
      &claim,
      &holder_signature,
      &public_key
    ));
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
