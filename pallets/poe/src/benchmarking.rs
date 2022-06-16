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

//! Benchmarking for Proof of Existence Pallet.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use anagolay_support::AnagolayStructureData;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, prelude::*, vec::Vec};

use crate::types::ProofData;
#[allow(unused)]
use crate::Pallet as Poe;

const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

benchmarks! {
    create_proof {
        let caller: T::AccountId = whitelisted_caller();
        let proof_data = ProofData::default();
    }: _(RawOrigin::Signed(caller), proof_data)

    save_phash {
        let caller: T::AccountId = whitelisted_caller();
        let proof_data = ProofData::default();
        let phash = PERCEPTUAL_HASH.to_vec();
        let p_hash_payload = PhashInfo {
            p_hash: phash.clone(),
            proof_id: proof_data.to_cid(),
        };
        crate::Pallet::<T>::create_proof(RawOrigin::Signed(caller.clone()).into(), proof_data)?;
    }: _(RawOrigin::Signed(caller), p_hash_payload)
}

impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test);
