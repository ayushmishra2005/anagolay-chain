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

//! Benchmarking for Proof of Existence Pallet.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::types::ProofData;
#[allow(unused)]
use crate::Pallet as Poe;
use anagolay_support::{AnagolayArtifactStructure, AnagolayStructureData, ArtifactId};
use core::convert::TryInto;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_support::{sp_std::prelude::*, traits::UnixTime};
use frame_system::RawOrigin;
use workflows::types::{
  Workflow, WorkflowArtifactType, WorkflowData, WorkflowId, WorkflowVersion, WorkflowVersionData, WorkflowVersionExtra,
  WorkflowVersionId,
};

const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

pub fn mock_request<T: workflows::Config>() -> (Workflow, WorkflowVersion) {
  let wf = Workflow {
    id: WorkflowId::from("bafkr4ih2xmsije6aa6yfwjdfmztnnkbb6ip56g3ojfcyfgjx6jsh6bogoe"),
    data: WorkflowData {
      name: "wf_aaaaa".into(),
      description: "wf_aaaaa operation description".into(),
      creators: vec!["tester".into()].try_into().unwrap(),
      ..WorkflowData::default()
    },
    extra: None,
  };
  let wf_ver = WorkflowVersion {
    id: WorkflowVersionId::from("bafybeihc2e5rshwlkcg47uojrhtw7dwhyq2cxwivf3sysfnx5jtuuafvia"),
    data: WorkflowVersionData {
      entity_id: Some(wf.id.clone()),
      parent_id: None,
      artifacts: vec![AnagolayArtifactStructure {
        artifact_type: WorkflowArtifactType::Git,
        file_extension: "git".into(),
        ipfs_cid: ArtifactId::from("bafkreibft6r6ijt7lxmbu2x3oq2s2ehwm5kz2nflwnlktdhcq2yfhgd4ku"),
      }]
      .try_into()
      .unwrap(),
    },
    extra: Some(WorkflowVersionExtra {
      created_at: T::TimeProvider::now().as_secs(),
    }),
  };
  (wf, wf_ver)
}

pub fn build_default_proof_data(workflow_id: WorkflowId) -> ProofData {
  let mut proof_data = ProofData::default();
  proof_data.workflow_id = workflow_id;
  proof_data
}

benchmarks! {
    create_proof {
        let caller: T::AccountId = whitelisted_caller();
        let (wf, wf_ver) = mock_request::<T>();
        let wf_id = wf.data.clone().to_cid();
        workflows::Pallet::<T>::create(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), wf.data, wf_ver.data).unwrap();
        let proof_data = build_default_proof_data(wf_id);
    }: _(RawOrigin::Signed(caller), proof_data)

    save_phash {
        let caller: T::AccountId = whitelisted_caller();
        let (wf, wf_ver) = mock_request::<T>();
        let wf_id = wf.data.clone().to_cid();
        workflows::Pallet::<T>::create(<T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone())), wf.data, wf_ver.data).unwrap();
        let proof_data = build_default_proof_data(wf_id);

        let phash = PERCEPTUAL_HASH.to_vec();
        let p_hash_payload = PhashInfo {
            p_hash: phash.clone().try_into().unwrap(),
            proof_id: proof_data.to_cid(),
        };
        crate::Pallet::<T>::create_proof(RawOrigin::Signed(caller.clone()).into(), proof_data)?;
    }: _(RawOrigin::Signed(caller), p_hash_payload)
}

impl_benchmark_test_suite!(Poe, crate::mock::new_test_ext(), crate::mock::Test);
