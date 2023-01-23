// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Benchmarking for Tipping Pallet

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use core::convert::TryInto;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{sp_std::vec, BoundedVec};
use frame_system::RawOrigin;
use types::*;

use verification::{
  consts::MaxVerificationRequestsPerContextGet,
  types::{VerificationAction, VerificationContext, VerificationRequest, VerificationStatus},
};

#[allow(unused)]
use crate::Pallet as Tipping;

benchmarks! {
    update_settings {
        let caller: T::AccountId = whitelisted_caller();
        let tipping_settings1 = TippingSettings::default();
        let tipping_settings2 = TippingSettings::default();
    }: _(RawOrigin::Signed(caller), vec![tipping_settings1, tipping_settings2])

    tip {
        let caller: T::AccountId = whitelisted_caller();
        let amount = 1u32.into();

        let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
        let action = VerificationAction::DnsTxtRecord;

        let request = VerificationRequest::<T::AccountId> {
            context: context.clone(),
            action,
            holder: caller.clone(),
            status: VerificationStatus::Success,
            key: "anagolay-domain-verification=test".into(),
            id: None,
        };
        let accounts: BoundedVec<T::AccountId, MaxVerificationRequestsPerContextGet<T>> = vec![caller.clone()].try_into().unwrap();
        verification::pallet::AccountIdsByVerificationContext::<T>::insert(context.clone(), accounts);
        verification::pallet::VerificationRequestByAccountIdAndVerificationContext::<T>::insert(caller.clone(), context.clone(), request.clone());

        let settings = TippingSettings {
            context: context.clone(),
            enabled: true,
            account: Some(caller.clone())
        };
        TippingSettingsByAccountIdAndVerificationContext::<T>::insert(caller.clone(), context.clone(), settings);
    }: _(RawOrigin::Signed(caller), amount, context)

}

impl_benchmark_test_suite!(Tipping, crate::mock::new_test_ext(vec![]), crate::mock::Test);
