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
use anagolay_support::{AnagolayStructureData, Characters};
use core::convert::TryInto;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use types::*;

#[allow(unused)]
use crate::Pallet as Statements;

benchmarks! {
  create_copyright{
    //Initializing benchmark for Copyright Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut copyright_statement = StatementData::default();
    copyright_statement.signatures.holder.sig = frame_support::sp_std::vec![232, 148, 217, 202, 67, 152, 64, 219, 171, 158, 57, 45, 243, 4, 178, 197, 37, 165, 172, 245, 16, 45, 94, 64, 230, 110, 158, 82, 13, 242, 174, 15, 214, 47, 113, 167, 4, 195, 19, 208, 191, 156, 246, 202, 23, 180, 205, 224, 106, 201, 144, 132, 184, 227, 44, 24, 88, 54, 105, 7, 110, 82, 142, 139].try_into().unwrap();
    copyright_statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
  }: _(RawOrigin::Signed(caller), copyright_statement)

  create_ownership{
    //Initializing benchmark for Ownership Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut ownership_statement = StatementData::default();
    ownership_statement.claim.claim_type = ClaimType::Ownership;
    ownership_statement.signatures.holder.sig = frame_support::sp_std::vec![148, 217, 120, 247, 168, 90, 43, 154, 142, 154, 134, 7, 247, 252, 27, 121, 113, 150, 197, 57, 16, 9, 182, 140, 188, 203, 85, 92, 152, 190, 212, 91, 227, 30, 24, 129, 15, 207, 25, 122, 2, 185, 167, 72, 220, 137, 41, 215, 26, 57, 87, 195, 110, 13, 77, 49, 243, 167, 170, 187, 134, 200, 213, 137].try_into().unwrap();
    ownership_statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");

  }: _(RawOrigin::Signed(caller), ownership_statement)

  revoke{
    //Initializing benchmark for Revoke Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut statement = StatementData::default();
    statement.signatures.holder.sig = frame_support::sp_std::vec![232, 148, 217, 202, 67, 152, 64, 219, 171, 158, 57, 45, 243, 4, 178, 197, 37, 165, 172, 245, 16, 45, 94, 64, 230, 110, 158, 82, 13, 242, 174, 15, 214, 47, 113, 167, 4, 195, 19, 208, 191, 156, 246, 202, 23, 180, 205, 224, 106, 201, 144, 132, 184, 227, 44, 24, 88, 54, 105, 7, 110, 82, 142, 139].try_into().unwrap();
    statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    let statement_id = statement.to_cid();
    crate::Pallet::<T>::create_copyright(RawOrigin::Signed(caller.clone()).into(), statement)?;
  }: _(RawOrigin::Signed(caller), statement_id)
}

impl_benchmark_test_suite!(Statements, crate::mock::new_test_ext(), crate::mock::Test);
