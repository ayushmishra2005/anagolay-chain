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
    copyright_statement.signatures.holder.sig = frame_support::sp_std::vec![188, 174, 18, 5, 230, 169, 26, 59, 5, 221, 70, 121, 33, 198, 98, 19, 241, 149, 42, 114, 34, 137, 156, 255, 69, 104, 157, 166, 232, 238, 129, 100, 163, 13, 118, 59, 5, 136, 88, 103, 153, 36, 161, 203, 48, 71, 17, 80, 179, 11, 107, 167, 225, 81, 210, 31, 15, 112, 203, 122, 60, 78, 153, 138].try_into().unwrap();
    copyright_statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
  }: _(RawOrigin::Signed(caller), copyright_statement)

  create_ownership{
    //Initializing benchmark for Ownership Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut ownership_statement = StatementData::default();
    ownership_statement.claim.claim_type = ClaimType::Ownership;
    ownership_statement.signatures.holder.sig = frame_support::sp_std::vec![4, 162, 137, 242, 15, 129, 216, 106, 125, 59, 141, 17, 134, 176, 229, 224, 108, 11, 244, 151, 218, 201, 30, 104, 192, 84, 61, 109, 206, 151, 222, 63, 140, 244, 153, 184, 240, 163, 40, 0, 169, 52, 44, 42, 52, 254, 75, 210, 159, 237, 237, 98, 64, 129, 170, 176, 32, 36, 140, 231, 32, 128, 72, 140].try_into().unwrap();
    ownership_statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");

  }: _(RawOrigin::Signed(caller), ownership_statement)

  revoke{
    //Initializing benchmark for Revoke Extrinsic
    let caller: T::AccountId = whitelisted_caller();
    let mut statement = StatementData::default();
    statement.signatures.holder.sig = frame_support::sp_std::vec![188, 174, 18, 5, 230, 169, 26, 59, 5, 221, 70, 121, 33, 198, 98, 19, 241, 149, 42, 114, 34, 137, 156, 255, 69, 104, 157, 166, 232, 238, 129, 100, 163, 13, 118, 59, 5, 136, 88, 103, 153, 36, 161, 203, 48, 71, 17, 80, 179, 11, 107, 167, 225, 81, 210, 31, 15, 112, 203, 122, 60, 78, 153, 138].try_into().unwrap();
    statement.signatures.holder.sig_key = Characters::from("urn:substrate:0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d");
    let statement_id = statement.to_cid();
    crate::Pallet::<T>::create_copyright(RawOrigin::Signed(caller.clone()).into(), statement)?;
  }: _(RawOrigin::Signed(caller), statement_id)
}

impl_benchmark_test_suite!(Statements, crate::mock::new_test_ext(), crate::mock::Test);
