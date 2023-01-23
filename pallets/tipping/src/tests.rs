// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Tests for the module.

use super::{mock::*, *};
use crate::{constants::MaxTipsPerVerificationContextGet, types::*};
use core::convert::TryInto;
use frame_support::*;
use sp_core::{sr25519, Pair};

use verification::{
  consts::MaxVerificationRequestsPerContextGet,
  types::{VerificationAction, VerificationContext, VerificationRequest, VerificationStatus},
};

fn mock_account(ss58: &str) -> sr25519::Public {
  let (pair, _) = sr25519::Pair::from_string_with_seed(ss58, None).unwrap();
  pair.public()
}

fn mock_verification_context_for_tipping<T: Config>(
  holder: T::AccountId,
  context: VerificationContext,
  request_status: VerificationStatus,
  enable_tipping: bool,
) {
  let action = VerificationAction::DnsTxtRecord;

  let request = VerificationRequest::<T::AccountId> {
    context: context.clone(),
    action,
    holder: holder.clone(),
    status: request_status,
    key: "anagolay-domain-verification=test".into(),
    id: None,
  };
  let accounts: BoundedVec<T::AccountId, MaxVerificationRequestsPerContextGet<T>> =
    vec![holder.clone()].try_into().unwrap();
  verification::pallet::AccountIdsByVerificationContext::<T>::insert(context.clone(), accounts);
  verification::pallet::VerificationRequestByAccountIdAndVerificationContext::<T>::insert(
    holder.clone(),
    context.clone(),
    request.clone(),
  );

  let settings = TippingSettings {
    context: context.clone(),
    enabled: enable_tipping,
    account: Some(holder.clone()),
  };
  TippingSettingsByAccountIdAndVerificationContext::<T>::insert(holder, context.clone(), settings);
}

#[test]
fn update_settings_from_holder() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context_1 = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder.clone(), context_1.clone(), VerificationStatus::Success, true);
    let context_2 = VerificationContext::UrlForDomain("https://kelp.digital".into(), "kelp.digital".into());
    mock_verification_context_for_tipping::<Test>(holder.clone(), context_2.clone(), VerificationStatus::Success, true);

    let settings_1 = TippingSettings {
      context: context_1.clone(),
      enabled: true,
      account: None,
    };
    let settings_2 = TippingSettings {
      context: context_2.clone(),
      enabled: false,
      account: Some(holder.clone()),
    };

    let res = TippingTest::update_settings(origin, vec![settings_1.clone(), settings_2.clone()]);
    assert_ok!(res);

    let settings_1_updated = TippingSettings {
      context: context_1.clone(),
      enabled: true,
      account: Some(holder.clone()),
    };

    let res_1 = TippingSettingsByAccountIdAndVerificationContext::<Test>::get(holder.clone(), context_1.clone());
    assert_eq!(settings_1_updated, res_1, "settings_1 was not stored correctly");
    let res_2 = TippingSettingsByAccountIdAndVerificationContext::<Test>::get(holder, context_2.clone());
    assert_eq!(settings_2, res_2, "settings_2 was not stored correctly");
  });
}

#[test]
fn update_settings_from_non_holder() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder.clone(), context.clone(), VerificationStatus::Success, true);

    let settings_1 = TippingSettings {
      context: context.clone(),
      enabled: true,
      account: Some(holder.clone()),
    };

    let res = TippingTest::update_settings(origin, vec![settings_1.clone()]);
    assert_ok!(res);

    let non_holder = mock_account("//Bob");
    let origin = mock::Origin::signed(non_holder);

    let settings_2 = TippingSettings {
      context: context.clone(),
      enabled: false,
      account: None,
    };

    let res = TippingTest::update_settings(origin, vec![settings_2]);
    assert_ok!(res);

    let res = TippingSettingsByAccountIdAndVerificationContext::<Test>::get(holder.clone(), context.clone());
    assert_eq!(settings_1, res, "settings must not be updated from non holder");
    let res = TippingSettingsByAccountIdAndVerificationContext::<Test>::get(non_holder.clone(), context.clone());
    assert_eq!(
      TippingSettings::default(),
      res,
      "Non holder must receive default tipping settings"
    );
  });
}

#[test]
fn update_settings_for_failed_request() {
  new_test_ext(Vec::new()).execute_with(|| {
    let holder = mock_account("//Alice");
    let origin = mock::Origin::signed(holder);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(
      holder.clone(),
      context.clone(),
      VerificationStatus::Failure("any error".into()),
      true,
    );
    TippingSettingsByAccountIdAndVerificationContext::<Test>::insert(
      holder,
      context.clone(),
      TippingSettings::default(),
    );

    let settings = TippingSettings {
      context: context.clone(),
      enabled: true,
      account: Some(holder.clone()),
    };

    let res = TippingTest::update_settings(origin, vec![settings.clone()]);
    assert_ok!(res);

    let res = TippingSettingsByAccountIdAndVerificationContext::<Test>::get(holder.clone(), context.clone());
    assert_eq!(
      TippingSettings::default(),
      res,
      "settings must not be updated for the context of a Failed verification request"
    );
  });
}

#[test]
fn tip_error_invalid_verification_context() {
  new_test_ext(Vec::new()).execute_with(|| {
    let tipper = mock_account("//Alice");
    let origin = mock::Origin::signed(tipper);

    let holder = mock_account("//Bob");

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder, context, VerificationStatus::Success, true);
    let context = VerificationContext::UrlForDomain("https://kelp.digital".into(), "kelp.digital".into());

    let res = TippingTest::tip(origin.clone(), 1u32.into(), context);
    assert_noop!(res, Error::<Test>::InvalidVerificationContext);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(
      holder,
      context.clone(),
      VerificationStatus::Failure("any error".into()),
      true,
    );

    let res = TippingTest::tip(origin, 1u32.into(), context);
    assert_noop!(res, Error::<Test>::InvalidVerificationContext);
  });
}

#[test]
fn tip_error_invalid_configuration() {
  new_test_ext(Vec::new()).execute_with(|| {
    let tipper = mock_account("//Alice");
    let origin = mock::Origin::signed(tipper);

    let holder = mock_account("//Bob");

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder, context.clone(), VerificationStatus::Success, false);

    let res = TippingTest::tip(origin, 1u32.into(), context);
    assert_noop!(res, Error::<Test>::InvalidConfiguration);
  });
}

#[test]
fn tip_error_insufficient_balance() {
  new_test_ext(Vec::new()).execute_with(|| {
    let tipper = mock_account("//Alice");
    let origin = mock::Origin::signed(tipper);

    let holder = mock_account("//Bob");

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder, context.clone(), VerificationStatus::Success, true);

    let res = TippingTest::tip(origin, 1u32.into(), context);
    assert_noop!(res, pallet_balances::Error::<Test>::InsufficientBalance);
  });
}

#[test]
fn tip_test() {
  let tipper = mock_account("//Alice");
  new_test_ext(vec![(tipper, 100)]).execute_with(|| {
    let origin = mock::Origin::signed(tipper);

    let holder = mock_account("//Bob");

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder, context.clone(), VerificationStatus::Success, true);

    let res = TippingTest::tip(origin.clone(), 4u32.into(), context.clone());
    assert_ok!(res);
    let res = TippingTest::tip(origin.clone(), 3u32.into(), context.clone());
    assert_ok!(res);
    let res = TippingTest::tip(origin.clone(), 2u32.into(), context.clone());
    assert_ok!(res);
    let res = TippingTest::tip(origin.clone(), 1u32.into(), context.clone());
    assert_ok!(res);

    let tips: BoundedVec<
      Tip<BalanceOf<Test>, <Test as frame_system::Config>::AccountId, <Test as pallet_balances::Config>::Balance>,
      MaxTipsPerVerificationContextGet<Test>,
    > = TipsByAccountIdAndVerificationContext::get(holder.clone(), context);
    let tip_amounts: Vec<u64> = tips.iter().map(|tip| tip.amount).collect();
    assert_eq!(vec![3, 2, 1], tip_amounts);
    assert_eq!(Balances::free_balance(&tipper), 90);
    assert_eq!(Balances::free_balance(&holder), 10);
  });
}

#[test]
fn rpc_get_tips_sort() {
  let tipper = mock_account("//Alice");
  new_test_ext(vec![(tipper, 100)]).execute_with(|| {
    let holder = mock_account("//Bob");

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    mock_verification_context_for_tipping::<Test>(holder, context.clone(), VerificationStatus::Success, true);

    let tips: BoundedVec<
      Tip<BalanceOf<Test>, <Test as frame_system::Config>::AccountId, <Test as pallet_balances::Config>::Balance>,
      MaxTipsPerVerificationContextGet<Test>,
    > = vec![
      Tip {
        amount: 1u64,
        sender: tipper.clone(),
        receiver: holder.clone(),
        created_at: 10u64.into(),
        block_number: 10u64.into(),
      },
      Tip {
        amount: 5u64,
        sender: tipper.clone(),
        receiver: holder.clone(),
        created_at: 20u64.into(),
        block_number: 20u64.into(),
      },
      Tip {
        amount: 2u64,
        sender: tipper.clone(),
        receiver: holder.clone(),
        created_at: 30u64.into(),
        block_number: 30u64.into(),
      },
    ]
    .try_into()
    .unwrap();

    TipsByAccountIdAndVerificationContext::insert(holder.clone(), context.clone(), tips);

    let res = TippingTest::get_tips(holder.clone(), context.clone(), 0, 2);
    let tip_amounts: Vec<u64> = res.iter().map(|tip| tip.amount).collect();
    assert_eq!(vec![2, 5], tip_amounts);

    let res = TippingTest::total(holder.clone(), context.clone());
    assert_eq!(3, res);

    let res = TippingTest::total_received(holder, context);
    assert_eq!(8, res);
  });
}
