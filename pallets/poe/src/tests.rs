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

//! Tests for the module.

#![cfg(test)]
use super::{mock::*, *};
use crate::types::{PoeVerificationKeyGenerator, ProofData, ProofId};
use anagolay_support::{AnagolayArtifactStructure, AnagolayStructureData, ArtifactId};
use core::convert::TryInto;
use frame_support::{assert_noop, assert_ok, traits::UnixTime};
use verification::types::{VerificationContext, VerificationKeyGenerator};
use workflows::types::{
  Workflow, WorkflowArtifactType, WorkflowData, WorkflowId, WorkflowVersion, WorkflowVersionData, WorkflowVersionExtra,
  WorkflowVersionId,
};

const PERCEPTUAL_HASH: &[u8] = b"0x30303030303030303031313030303030303030303030303030303031313130303031313131313030313131313131313031313131313131313131313131313130303031313130303030303030303030303131313131313130303030303030303031313131313131313130303030303030313131313131313131313131313030303131313131313131313131313031313131313131313131313130313030313130313131303030303030303130303030303030303030303031303030303030303031313131313131313131313131313131313131313131313130303030303030303131313130303030303030303030303031313131303030303030303030303030";

fn mock_request() -> (Workflow, WorkflowVersion) {
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
      created_at: <Test as workflows::Config>::TimeProvider::now().as_secs(),
    }),
  };
  (wf, wf_ver)
}

pub fn build_default_proof_data(workflow_id: WorkflowId) -> ProofData {
  let mut proof_data = ProofData::default();
  proof_data.workflow_id = workflow_id;
  proof_data
}

#[test]
fn proof_create_default() {
  new_test_ext().execute_with(|| {
    let origin = mock::Origin::signed(1);
    let (wf, wf_ver) = mock_request();
    let wf_id = wf.data.clone().to_cid();
    Workflows::create(origin, wf.data, wf_ver.data).unwrap();

    let pd = build_default_proof_data(wf_id);
    let res = TestPoe::create_proof(mock::Origin::signed(1), pd.clone());
    assert_ok!(res);
  });
}

#[test]
fn proof_create_for_verification_context() {
  new_test_ext().execute_with(|| {
    let holder = 1u64;
    let context = VerificationContext::UrlForDomain("https://anagolay.network".into(), "anagolay.network".into());
    let identifier = "//Aliceanagolay.network";

    let key = PoeVerificationKeyGenerator::<Test>::generate(&holder, &context, identifier.as_bytes().to_vec()).unwrap();

    let (proof_id, _account_id, proof_record) = ProofByProofIdAndAccountId::<Test>::iter().next().unwrap();
    assert_eq!(
      key,
      proof_record
        .record
        .data
        .params
        .iter()
        .next()
        .unwrap()
        .as_bytes()
        .to_vec(),
      "Proof key does not correspond to the generated one"
    );

    let proofs_by_context = ProofIdsByVerificationContext::<Test>::get(context).unwrap();
    assert_eq!(
      proofs_by_context.into_iter().next().unwrap(),
      proof_id,
      "Proof id was not associated to the VerificationContext"
    );
  });
}

#[test]
fn proof_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let origin = mock::Origin::signed(1);
    let (wf, wf_ver) = mock_request();
    let wf_id = wf.data.clone().to_cid();
    Workflows::create(origin, wf.data, wf_ver.data).unwrap();

    // create the proof
    let pd = build_default_proof_data(wf_id);
    let res1 = TestPoe::create_proof(mock::Origin::signed(1), pd.clone());

    assert_ok!(res1);

    // create the proof AGAIN
    let res2 = TestPoe::create_proof(mock::Origin::signed(1), pd.clone());

    assert_noop!(res2, Error::<Test>::ProofAlreadyClaimed);
  });
}

#[test]
fn phash_save_phash() {
  new_test_ext().execute_with(|| {
    let origin = mock::Origin::signed(1);
    let (wf, wf_ver) = mock_request();
    let wf_id = wf.data.clone().to_cid();
    Workflows::create(origin, wf.data, wf_ver.data).unwrap();

    let pd = build_default_proof_data(wf_id);
    let res = TestPoe::create_proof(mock::Origin::signed(1), pd.clone());
    assert_ok!(res);

    let phash = PERCEPTUAL_HASH.to_vec();
    let p_hash_payload = PhashInfo {
      p_hash: phash.clone().try_into().unwrap(),
      proof_id: pd.to_cid(),
    };

    let res = TestPoe::save_phash(mock::Origin::signed(1), p_hash_payload);
    assert_ok!(res);
  });
}
#[test]
fn phash_save_phash_error_on_duplicate() {
  new_test_ext().execute_with(|| {
    let origin = mock::Origin::signed(1);
    let (wf, wf_ver) = mock_request();
    let wf_id = wf.data.clone().to_cid();
    Workflows::create(origin, wf.data, wf_ver.data).unwrap();

    let pd = build_default_proof_data(wf_id);
    let res = TestPoe::create_proof(mock::Origin::signed(1), pd.clone());
    assert_ok!(res);

    let phash = PERCEPTUAL_HASH.to_vec();

    let p_hash_payload = PhashInfo {
      p_hash: phash.clone().try_into().unwrap(),
      proof_id: pd.to_cid(),
    };

    let res = TestPoe::save_phash(mock::Origin::signed(1), p_hash_payload.clone());
    assert_ok!(res);

    let res2 = TestPoe::save_phash(mock::Origin::signed(1), p_hash_payload.clone());
    assert_noop!(res2, Error::<Test>::PHashAndProofIdComboAlreadyExist);
  });
}
#[test]
fn phash_save_phash_error_no_proof() {
  new_test_ext().execute_with(|| {
        let phash = PERCEPTUAL_HASH.to_vec();

        let proof_id = ProofId::from("0x6261666b32627a616365616d6c6e766678726c717175743274686f6b6c6a76726b68726f7a787562696a78746f3476743566646f776c6162747733686177");

        let p_hash_payload = PhashInfo {
        p_hash: phash.clone().try_into().unwrap(),
        proof_id: proof_id.clone(),
        };

        let res = TestPoe::save_phash(mock::Origin::signed(1), p_hash_payload);
        assert_noop!(res, Error::<Test>::NoSuchProof);
    });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
