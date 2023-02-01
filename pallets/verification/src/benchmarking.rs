// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.

//! Benchmarking for Verification Pallet

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use codec::Encode;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::app_crypto::RuntimePublic;
use types::{offchain::*, *};

#[allow(unused)]
use crate::Pallet as Verification;

benchmarks! {
  where_clause { where T::Public : From<sp_core::sr25519::Public>, T::Signature : From<sp_core::sr25519::Signature> }

  request_verification {
    let caller: T::AccountId = whitelisted_caller();
    T::Currency::make_free_balance_be(&caller, T::REGISTRATION_FEE);

    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let action = VerificationAction::DnsTxtRecord;
  }: _(RawOrigin::Signed(caller), context, action)

  submit_verification_status{
    let public_key = sp_core::sr25519::Public::generate_pair(sp_core::testing::SR25519, None);

    let caller: T::AccountId = whitelisted_caller();
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
    VerificationRequestByAccountIdAndVerificationContext::<T>::insert(caller.clone(), context.clone(), request.clone());

    let indexing_data = VerificationIndexingOutputData {
      verifier: caller,
      request,
      public: public_key.into()
    };

    let signature: T::Signature = public_key.sign(sp_core::testing::SR25519, &indexing_data.encode().as_slice()).unwrap().into();

  }: _(RawOrigin::None, indexing_data, signature)

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
