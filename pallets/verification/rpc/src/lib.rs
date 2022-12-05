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

use codec::Decode;
use core::fmt::Debug;
use frame_support::sp_std::vec::Vec;
use sp_api::{ApiError, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use verification::types::*;
pub use verification_rpc_runtime_api::VerificationApi as VerificationRuntimeApi;

use jsonrpsee::{
  core::{async_trait, RpcResult},
  proc_macros::rpc,
  types::error::{CallError, ErrorObject},
};

/// Error type of the RPC api.
pub enum Error {
  /// The transaction was not decodable.
  DecodeError,
  /// The call to runtime failed.
  RuntimeError,
}

impl From<Error> for i32 {
  fn from(e: Error) -> i32 {
    match e {
      Error::RuntimeError => 1,
      Error::DecodeError => 2,
    }
  }
}

#[rpc(client, server)]
pub trait VerificationApi<BlockHash, AccountId: Debug + Decode> {
  #[method(name = "verification_getRequests")]
  fn get_requests(
    &self,
    contexts: Vec<VerificationContext>,
    status: Option<VerificationStatus>,
    offset: u64,
    limit: u16,
    at: Option<BlockHash>,
  ) -> RpcResult<Vec<VerificationRequest<AccountId>>>;
}

/// A struct that implements the `VerificationApi`.
pub struct Verification<C, M> {
  client: Arc<C>,
  _marker: std::marker::PhantomData<M>,
}

impl<C, M> Verification<C, M> {
  /// Create new `Verification` instance with the given reference to the client.
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
    "Unable to query verification.",
    Some(e.to_string()),
  ))
  .into()
}

#[async_trait]
impl<C, Block, AccountId> VerificationApiServer<<Block as BlockT>::Hash, AccountId> for Verification<C, Block>
where
  Block: BlockT,
  AccountId: Debug + Decode,
  C: Send + Sync + 'static,
  C: ProvideRuntimeApi<Block>,
  C: HeaderBackend<Block>,
  C::Api: VerificationRuntimeApi<Block, AccountId>,
{
  fn get_requests(
    &self,
    contexts: Vec<VerificationContext>,
    status: Option<VerificationStatus>,
    offset: u64,
    limit: u16,
    at: Option<Block::Hash>,
  ) -> RpcResult<Vec<VerificationRequest<AccountId>>> {
    let api = self.client.runtime_api();
    let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));

    api
      .get_requests(&at, contexts, status, offset, limit)
      .map_err(map_jsonrpc_err)
  }
}
