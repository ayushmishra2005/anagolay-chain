//! Tests for the module.

#![cfg(test)]
use super::mock::*;
use super::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn operations_create_operation() {
    ExtBuilder::build().execute_with(|| {
        let op = Operation::default();
        let res = OperationTest::create(Origin::signed(1), op.clone());
        assert_ok!(res);
    });
}
#[test]
fn operations_create_operation_error_on_duplicate() {
    ExtBuilder::build().execute_with(|| {
        let op = Operation::default();
        let res_first = OperationTest::create(Origin::signed(1), op.clone());
        assert_ok!(res_first);

        let op = Operation::default();
        let res_duplicate = OperationTest::create(Origin::signed(1), op.clone());
        assert_noop!(res_duplicate, Error::<Test>::OperationAlreadyExists);
    });
}

#[test]
fn test_template() {
    ExtBuilder::build().execute_with(|| {});
}
