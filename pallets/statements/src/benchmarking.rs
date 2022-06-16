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

//! Benchmarking for Statement Pallet

#![cfg(feature = "runtime-benchmarks")]
use super::*;
use anagolay_support::AnagolayStructureData;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};
use types::*;

#[allow(unused)]
use crate::Pallet as Statements;

benchmarks! {
  create_copyright{
    //Initializing benchmark for Copyright Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let copyright_statement = StatementData::default();
  }: _(RawOrigin::Signed(caller), copyright_statement)

  create_ownership{
    //Initializing benchmark for Ownership Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut ownership_statement = StatementData::default();
    ownership_statement.claim.claim_type = ClaimType::Ownership;

  }: _(RawOrigin::Signed(caller), ownership_statement)

  revoke{
    //Initializing benchmark for Revoke Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let statements = StatementData::default();
    let statement_id = statements.to_cid();
    crate::Pallet::<T>::create_copyright(RawOrigin::Signed(caller.clone()).into(), statements)?;
  }: _(RawOrigin::Signed(caller), statement_id)
}

impl_benchmark_test_suite!(Statements, crate::mock::new_test_ext(), crate::mock::Test);
