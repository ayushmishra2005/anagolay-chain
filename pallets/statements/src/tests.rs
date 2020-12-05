//! Tests for the module.

#![cfg(test)]

use super::mock::*;
use super::*;
use frame_support::{assert_noop, assert_ok};
use sn_cid::sn_cid;

#[test]
fn statements_create_ownership() {
    ExtBuilder::build().execute_with(|| {
        let mut r = SensioStatement::default();
        r.data.claim.claim_type = SensioClaimType::OWNERSHIP;
        let res = StatementsTest::create_ownership(Origin::signed(1), r);
        assert_ok!(res);
    });
}
#[test]
fn statements_create_ownership_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        let mut r = SensioStatement::default();
        r.data.claim.claim_type = SensioClaimType::OWNERSHIP;
        let res_first = StatementsTest::create_ownership(Origin::signed(1), r.clone());
        assert_ok!(res_first);

        let res_duplicate = StatementsTest::create_ownership(Origin::signed(1), r.clone());
        assert_noop!(res_duplicate, Error::<Test>::OwnershipAlreadyCreated);
    });
}
#[test]
fn statements_create_ownership_wrong_claim_type() {
    ExtBuilder::build().execute_with(|| {
        let r = SensioStatement::default();
        let res = StatementsTest::create_ownership(Origin::signed(1), r.clone());

        assert_noop!(res, Error::<Test>::WrongClaimType);
    });
}

#[test]
fn statements_create_copyright() {
    ExtBuilder::build().execute_with(|| {
        let r = SensioStatement::default();
        let res = StatementsTest::create_copyright(Origin::signed(1), r.clone());
        assert_ok!(res);
    });
}
#[test]
fn statements_create_copyright_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        let r = SensioStatement::default();
        let res_first = StatementsTest::create_copyright(Origin::signed(1), r.clone());
        assert_ok!(res_first);

        let res_duplicate = StatementsTest::create_copyright(Origin::signed(1), r.clone());
        assert_noop!(res_duplicate, Error::<Test>::CopyrightAlreadyCreated);
    });
}
#[test]
fn statements_create_copyright_wrong_claim_type() {
    ExtBuilder::build().execute_with(|| {
        let mut r = SensioStatement::default();
        r.data.claim.claim_type = SensioClaimType::OWNERSHIP;

        let res = StatementsTest::create_copyright(Origin::signed(1), r.clone());
        assert_noop!(res, Error::<Test>::WrongClaimType);
    });
}

#[test]
fn statements_cid() {
    ExtBuilder::build().execute_with(|| {
        let r = b"that is f... weird".to_vec();
        let cid = sn_cid(r.clone().encode());
        println!("CID {:?}", cid);
    });
}
#[test]
fn test_template() {
    ExtBuilder::build().execute_with(|| {});
}
