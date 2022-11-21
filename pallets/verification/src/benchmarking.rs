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

    let request = VerificationRequest::<T> {
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
