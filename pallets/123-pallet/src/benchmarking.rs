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
      
//! Benchmarking for 123-pallet.

#![cfg(feature = "runtime-benchmarks")]
use super::*;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::{boxed::Box, vec, vec::Vec};

#[allow(unused)]
use crate::Pallet as 123Pallet;

// Details on using the benchmarks macro can be seen at:
//   https://substrate.dev/rustdocs/latest/frame_benchmarking/macro.benchmarks.html
benchmarks! {
  foo {
    let n in 1 .. 100;
    let caller: T::AccountId = whitelisted_caller();

  }: _(RawOrigin::Signed(caller), n)
}

impl_benchmark_test_suite!(123Pallet, crate::mock::new_test_ext(), crate::mock::Test);
