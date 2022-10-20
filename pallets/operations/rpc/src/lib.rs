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

use anagolay_support::rpc::Error;
use frame_support::sp_std::vec::Vec;
use operations::types::{Operation, OperationId, OperationVersion, OperationVersionId};
pub use operations_rpc_runtime_api::OperationsApi as OperationsRuntimeApi;
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

use jsonrpsee::{
  core::{async_trait, RpcResult},
  proc_macros::rpc,
  types::error::{CallError, ErrorObject},
};

#[rpc(client, server)]
pub trait OperationsApi<BlockHash> {
  #[method(name = "operations_getOperationsByIds")]
  fn get_operations_by_ids(
    &self,
    operation_ids: Vec<OperationId>,
    offset: u64,
    limit: u16,
    at: Option<BlockHash>,
  ) -> RpcResult<Vec<Operation>>;

  #[method(name = "operations_getOperationVersionsByIds")]
  fn get_operation_versions_by_ids(
    &self,
    operation_versions_ids: Vec<OperationVersionId>,
    offset: u64,
    limit: u16,
    at: Option<BlockHash>,
  ) -> RpcResult<Vec<OperationVersion>>;
}

/// A struct that implements the `OperationsApi`.
pub struct Operations<C, M> {
  client: Arc<C>,
  _marker: std::marker::PhantomData<M>,
}

impl<C, M> Operations<C, M> {
  /// Create new `Operations` instance with the given reference to the client.
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
    "Unable to query operations.",
    Some(e.to_string()),
  ))
  .into()
}

#[async_trait]
impl<C, Block> OperationsApiServer<<Block as BlockT>::Hash> for Operations<C, Block>
where
  Block: BlockT,
  C: Send + Sync + 'static,
  C: ProvideRuntimeApi<Block>,
  C: HeaderBackend<Block>,
  C::Api: OperationsRuntimeApi<Block>,
{
  fn get_operations_by_ids(
    &self,
    operation_ids: Vec<OperationId>,
    offset: u64,
    limit: u16,
    at: Option<Block::Hash>,
  ) -> RpcResult<Vec<Operation>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

    api
      .get_operations_by_ids(&at, operation_ids, offset, limit)
      .map_err(map_jsonrpc_err)
  }

  fn get_operation_versions_by_ids(
    &self,
    operation_version_ids: Vec<OperationVersionId>,
    offset: u64,
    limit: u16,
    at: Option<Block::Hash>,
  ) -> RpcResult<Vec<OperationVersion>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

    api
      .get_operation_versions_by_ids(&at, operation_version_ids, offset, limit)
      .map_err(map_jsonrpc_err)
  }
}
