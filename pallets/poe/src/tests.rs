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

const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

pub fn build_default_proof(rule_id: Vec<u8>) -> Proof {
  let mut proof = Proof::default();
  proof.data.rule_id = rule_id;
  proof
}

#[test]
fn proof_create_default() {
  new_test_ext().execute_with(|| {
    // todo create default rule, figure out better way
    // T::ExternalRulesStorage::put_rule_in_storage(4);
    // todo create default rule, figure out better way

    let r_id = b"bafk".to_vec();
    let pd = build_default_proof(r_id);
    let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
    assert_ok!(res);
  });
}
#[test]
fn proof_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    // // todo create default rule, figure out better way
    // let r = create_default_rule();
    // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
    // assert_ok!(res);
    // // todo create default rule, figure out better way

    // create the proof
    let r_id = b"bafk".to_vec();
    let pd = build_default_proof(r_id);
    let res1 = TestPoe::create_proof(Origin::signed(1), pd.clone());

    assert_ok!(res1);

    // create the proof AGAIN
    let res2 = TestPoe::create_proof(Origin::signed(1), pd.clone());

    assert_noop!(res2, Error::<Test>::ProofAlreadyClaimed);
  });
}

#[test]
fn phash_save_phash() {
  new_test_ext().execute_with(|| {
    // // todo create default rule, figure out better way
    // let r = create_default_rule();
    // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
    // assert_ok!(res);
    // // todo create default rule, figure out better way
    let r_id = b"bafk".to_vec();
    let pd = build_default_proof(r_id);
    let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
    assert_ok!(res);

    let phash = PERCEPTUAL_HASH.to_vec();
    let p_hash_payload = PhashInfo {
      p_hash: phash.clone(),
      proof_id: pd.id.clone(),
    };

    let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload);
    assert_ok!(res);
  });
}
#[test]
fn phash_save_phash_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    // // todo create default rule, figure out better way
    // let r = create_default_rule();
    // let res = TestPoe::create_rule(Origin::signed(1), r.clone());
    // assert_ok!(res);
    // // todo create default rule, figure out better way
    let r_id = b"bafk".to_vec();
    let pd = build_default_proof(r_id);
    let res = TestPoe::create_proof(Origin::signed(1), pd.clone());
    assert_ok!(res);

    let phash = PERCEPTUAL_HASH.to_vec();

    let p_hash_payload = PhashInfo {
      p_hash: phash.clone(),
      proof_id: pd.id.clone(),
    };

    let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload.clone());
    assert_ok!(res);

    let res2 = TestPoe::save_phash(Origin::signed(1), p_hash_payload.clone());
    assert_noop!(res2, Error::<Test>::PHashAndProofIdComboAlreadyExist);
  });
}
#[test]
fn phash_save_phash_error_no_proof() {
  new_test_ext().execute_with(|| {
        let phash = PERCEPTUAL_HASH.to_vec();

        let proof_id = b"0x6261666b32627a616365616d6c6e766678726c717175743274686f6b6c6a76726b68726f7a787562696a78746f3476743566646f776c6162747733686177".to_vec();

        let p_hash_payload = PhashInfo {
        p_hash: phash.clone(),
        proof_id: proof_id.clone(),
        };

        let res = TestPoe::save_phash(Origin::signed(1), p_hash_payload);
        assert_noop!(res, Error::<Test>::NoSuchProof);
    });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
