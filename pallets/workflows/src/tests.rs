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
use crate::types::{Workflow, WorkflowArtifactType, WorkflowData, WorkflowVersion, WorkflowVersionData};
use anagolay_support::{AnagolayArtifactStructure, AnagolayStructureData, ArtifactId, VersionId, WorkflowId};
use frame_support::{assert_noop, assert_ok};

fn mock_request() -> (Workflow, WorkflowVersion) {
  let wf = Workflow {
    id: WorkflowId::from("bafkr4ih2xmsije6aa6yfwjdfmztnnkbb6ip56g3ojfcyfgjx6jsh6bogoe"),
    data: WorkflowData {
      name: "wf_aaaaa".into(),
      description: "wf_aaaaa operation description".into(),
      version: "0.1.0".into(),
      ..WorkflowData::default()
    },
    extra: None,
  };
  let wf_ver = WorkflowVersion {
    id: VersionId::from("bafybeihc2e5rshwlkcg47uojrhtw7dwhyq2cxwivf3sysfnx5jtuuafvia"),
    data: WorkflowVersionData {
      entity_id: Some(wf.id.clone()),
      parent_id: None,
      artifacts: vec![AnagolayArtifactStructure {
        artifact_type: WorkflowArtifactType::Git,
        file_extension: "git".into(),
        ipfs_cid: ArtifactId::from("bafkreibft6r6ijt7lxmbu2x3oq2s2ehwm5kz2nflwnlktdhcq2yfhgd4ku"),
      }],
    },
    extra: None,
  };
  (wf, wf_ver)
}

#[test]
fn workflows_create_workflow() {
  new_test_ext().execute_with(|| {
    let (wf, mut wf_ver) = mock_request();
    let origin = mock::Origin::signed(1);
    let res = WorkflowTest::create(origin, wf.data.clone(), wf_ver.data.clone());

    assert_ok!(res);

    let wf_id = &wf.data.to_cid();
    wf_ver.data.entity_id = Some(wf_id.clone());
    let wf_ver_id = &wf_ver.data.to_cid();

    let workflow = WorkflowsByWorkflowIdAndAccountId::<Test>::get(wf_id, 1);
    assert_eq!(workflow.record.data, wf.data);
    assert_eq!(workflow.record.extra, wf.extra);

    let workflow_versions = VersionsByWorkflowId::<Test>::get(wf_id);
    assert_eq!(1, workflow_versions.len());
    assert_eq!(wf_ver_id, workflow_versions.get(0).unwrap());

    let packages = anagolay_support::Pallet::<Test>::get_artifacts();
    assert_eq!(1, packages.len());
    assert_eq!(wf_ver.data.artifacts[0].ipfs_cid, *packages.get(0).unwrap());

    let version = VersionsByVersionId::<Test>::get(wf_ver_id);
    assert_eq!(version.record.data, wf_ver.data);
    assert!(version.record.extra.is_some());

    let workflow_total = Total::<Test>::get();
    assert_eq!(1, workflow_total);
  });
}

#[test]
fn workflows_create_workflow_error_on_duplicate_workflow() {
  new_test_ext().execute_with(|| {
    let (wf, wf_ver) = mock_request();
    let res = WorkflowTest::create(mock::Origin::signed(1), wf.data.clone(), wf_ver.data.clone());
    assert_ok!(res);

    let res = WorkflowTest::create(mock::Origin::signed(1), wf.data.clone(), wf_ver.data.clone());
    assert_noop!(res, Error::<Test>::WorkflowAlreadyExists);
  });
}

#[test]
fn workflows_create_workflow_error_reusing_package() {
  new_test_ext().execute_with(|| {
    let (wf, wf_ver) = mock_request();

    anagolay_support::Pallet::<Test>::store_artifacts(&wf_ver.data.artifacts);

    let res = WorkflowTest::create(mock::Origin::signed(1), wf.data.clone(), wf_ver.data.clone());

    assert_noop!(res, Error::<Test>::WorkflowVersionPackageAlreadyExists);
  });
}

#[test]
fn workflows_create_workflow_error_mixing_workflows() {
  new_test_ext().execute_with(|| {
    let (wf_a, wf_a_ver) = mock_request();

    let res = WorkflowTest::create(mock::Origin::signed(1), wf_a.data.clone(), wf_a_ver.data.clone());
    assert_ok!(res);

    let wf_b = Workflow {
      id: WorkflowId::default(),
      data: WorkflowData {
        name: "wf_bbbbb".into(),
        description: "wf_bbbbb operation description".into(),
        version: "0.1.0".into(),
        ..WorkflowData::default()
      },
      extra: None,
    };
    let wf_b_ver_mixed = WorkflowVersion {
      id: VersionId::default(),
      data: wf_a_ver.data.clone(),
      extra: None,
    };
    let res = WorkflowTest::create(mock::Origin::signed(1), wf_b.data.clone(), wf_b_ver_mixed.data.clone());
    assert_noop!(res, Error::<Test>::WorkflowVersionPackageAlreadyExists);
  });
}

#[test]
fn workflows_create_workflow_error_bad_request() {
  new_test_ext().execute_with(|| {
    let (mut wf, mut wf_ver) = mock_request();
    wf.data.name = "this_is_a_very_very_very_very_very_very_very_very_very_very_loooooong_workflow_name_that_does_not_respect_maximum_length_constraint".into();
    let res = WorkflowTest::create(mock::Origin::signed(1), wf.data.clone(), wf_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);

    wf_ver.data.artifacts[0].ipfs_cid = ArtifactId::from("bafk_invalid_cid");
    let res = WorkflowTest::create(mock::Origin::signed(1), wf.data.clone(), wf_ver.data.clone());
    assert_noop!(res, Error::<Test>::BadRequest);
  });
}

#[test]
fn test_template() {
  new_test_ext().execute_with(|| {});
}
