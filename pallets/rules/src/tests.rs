//! Tests for the module.

#![cfg(test)]

use super::mock::*;
use super::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn rule_create_default() {
    ExtBuilder::build().execute_with(|| {
        let r = Rule::default();
        let res = RulesTest::create_rule(Origin::signed(1), r);
        assert_ok!(res);
    });
}
#[test]
fn rule_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        let r1 = Rule::default();
        let res1 = RulesTest::create_rule(Origin::signed(1), r1.clone());
        assert_ok!(res1);

        let res2 = RulesTest::create_rule(Origin::signed(1), r1);
        assert_noop!(res2, Error::<Test>::RuleAlreadyCreated);
    });
}
