// This file is part of Anagolay Network.

// Copyright (C) 2019-2022 Anagolay Network.

//! Benchmarking for Verification Pallet

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use types::*;

#[allow(unused)]
use crate::Pallet as Verification;

benchmarks! {
  request_verification {
    let caller: T::AccountId = whitelisted_caller();
    T::Currency::make_free_balance_be(&caller, T::REGISTRATION_FEE);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
  }: _(RawOrigin::Signed(caller), context, action)

  perform_verification{
    let caller: T::AccountId = whitelisted_caller();
    T::Currency::make_free_balance_be(&caller, T::REGISTRATION_FEE);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;

    let request = VerificationRequest::<T::AccountId> {
        context: context.clone(),
        action,
        holder: caller.clone(),
        status: VerificationStatus::Pending,
        key: "anagolay-domain-verification=test".into(),
        id: None,
      };
    VerificationRequestByAccountIdAndVerificationContext::<T>::insert(caller.clone(), context.clone(), request.clone());

  }: _(RawOrigin::Signed(caller), request)

}

impl_benchmark_test_suite!(Verification, crate::mock::new_test_ext(vec![]), crate::mock::Test);
