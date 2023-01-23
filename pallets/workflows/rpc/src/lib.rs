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

use anagolay_support::rpc::Error;
use frame_support::sp_std::vec::Vec;
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use workflows::types::{Workflow, WorkflowId, WorkflowVersion, WorkflowVersionId};
pub use workflows_rpc_runtime_api::WorkflowsApi as WorkflowsRuntimeApi;

use jsonrpsee::{
  core::{async_trait, RpcResult},
  proc_macros::rpc,
  types::error::{CallError, ErrorObject},
};

#[rpc(client, server)]
pub trait WorkflowsApi<BlockHash> {
  #[method(name = "workflows_getWorkflowsByIds")]
  fn get_workflows_by_ids(
    &self,
    workflow_ids: Vec<WorkflowId>,
    offset: u64,
    limit: u16,
    at: Option<BlockHash>,
  ) -> RpcResult<Vec<Workflow>>;

  #[method(name = "workflows_getWorkflowVersionsByIds")]
  fn get_workflow_versions_by_ids(
    &self,
    workflow_version_ids: Vec<WorkflowVersionId>,
    offset: u64,
    limit: u16,
    at: Option<BlockHash>,
  ) -> RpcResult<Vec<WorkflowVersion>>;
}

/// A struct that implements the `WorkflowsApi`.
pub struct Workflows<C, M> {
  client: Arc<C>,
  _marker: std::marker::PhantomData<M>,
}

impl<C, M> Workflows<C, M> {
  /// Create new `Workflows` instance with the given reference to the client.
  pub fn new(client: Arc<C>) -> Self {
    Self {
      client,
      _marker: Default::default(),
    }
  }
}

/// Mapper function to transform Runtime API error into RPC error
///
/// # Arguments
/// * e - API error
///
/// # Return
/// JSON RPC error
fn map_jsonrpc_err(e: ApiError) -> jsonrpsee::core::Error {
  CallError::Custom(ErrorObject::owned(
    Error::RuntimeError.into(),
    "Unable to query Workflows.",
    Some(e.to_string()),
  ))
  .into()
}

#[async_trait]
impl<C, Block> WorkflowsApiServer<<Block as BlockT>::Hash> for Workflows<C, Block>
where
  Block: BlockT,
  C: Send + Sync + 'static,
  C: ProvideRuntimeApi<Block>,
  C: HeaderBackend<Block>,
  C::Api: WorkflowsRuntimeApi<Block>,
{
  fn get_workflows_by_ids(
    &self,
    workflow_ids: Vec<WorkflowId>,
    offset: u64,
    limit: u16,
    at: Option<Block::Hash>,
  ) -> RpcResult<Vec<Workflow>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

    api
      .get_workflows_by_ids(&at, workflow_ids, offset, limit)
      .map_err(map_jsonrpc_err)
  }

  fn get_workflow_versions_by_ids(
    &self,
    workflow_version_ids: Vec<WorkflowVersionId>,
    offset: u64,
    limit: u16,
    at: Option<Block::Hash>,
  ) -> RpcResult<Vec<WorkflowVersion>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

    api
      .get_workflow_versions_by_ids(&at, workflow_version_ids, offset, limit)
      .map_err(map_jsonrpc_err)
  }
}
