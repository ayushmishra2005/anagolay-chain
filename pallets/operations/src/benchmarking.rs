// This file is part of Anagolay Network.

// Copyright (C) 2019-2023 Anagolay Network.
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

//! Benchmarking setup for pallet-operations

#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::types::{
  Operation, OperationArtifactType, OperationData, OperationId, OperationVersion, OperationVersionData,
  OperationVersionExtra, OperationVersionId,
};
use anagolay_support::{AnagolayArtifactStructure, ArtifactId};
use core::convert::TryInto;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{sp_std::vec, traits::UnixTime};
use frame_system::RawOrigin;

#[allow(unused)]
use crate::Pallet as Operations;

benchmarks! {
  create {
    let caller: T::AccountId = whitelisted_caller();
    let op = Operation {
      id: OperationId::from("bafkr4ih2xmsije6aa6yfwjdfmztnnkbb6ip56g3ojfcyfgjx6jsh6bogoe"),
      data: OperationData {
        name: "op_aaaaa".into(),
        description: "op_aaaaa description".into(),
        repository: "https://github.com/anagolay/op_aaaaa".into(),
        license: "Apache 2.0".into(),
        features: vec!["std".into()].try_into().unwrap(),
        ..OperationData::default()
      },
      extra: None,
    };
    let op_ver = OperationVersion {
      id: OperationVersionId::from("bafybeihc2e5rshwlkcg47uojrhtw7dwhyq2cxwivf3sysfnx5jtuuafvia"),
      data: OperationVersionData {
        entity_id: Some(op.id.clone()),
        parent_id: None,
        artifacts: vec![AnagolayArtifactStructure {
          artifact_type: OperationArtifactType::Git,
          file_extension: "git".into(),
          ipfs_cid: ArtifactId::from("bafkreibft6r6ijt7lxmbu2x3oq2s2ehwm5kz2nflwnlktdhcq2yfhgd4ku"),
        }].try_into().unwrap(),
      },
      extra: Some(OperationVersionExtra {
        created_at: <T as Config>::TimeProvider::now().as_secs(),
      }),
    };
  }: _(RawOrigin::Signed(caller), op.data, op_ver.data)

  version_approve {
    let caller: T::AccountId = whitelisted_caller();
    let op_id = OperationId::from("a");
  }: _(RawOrigin::Signed(caller), op_id)
}

impl_benchmark_test_suite!(Operations, crate::mock::new_test_ext(None), crate::mock::Test);
