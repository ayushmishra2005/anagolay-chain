//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use anagolay_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Index as Nonce};

use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
  /// The client instance to use.
  pub client: Arc<C>,
  /// Transaction pool instance.
  pub pool: Arc<P>,
  /// Whether to deny unsafe calls
  pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P>(deps: FullDeps<C, P>) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
  C: ProvideRuntimeApi<Block>
    + HeaderBackend<Block>
    + AuxStore
    + HeaderMetadata<Block, Error = BlockChainError>
    + Send
    + Sync
    + 'static,
  C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
  C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
  C::Api: operations_rpc::OperationsRuntimeApi<Block>,
  C::Api: workflows_rpc::WorkflowsRuntimeApi<Block>,
  C::Api: verification_rpc::VerificationRuntimeApi<Block, AccountId>,
  C::Api: tipping_rpc::TippingRuntimeApi<Block, Balance, AccountId, BlockNumber>,
  C::Api: BlockBuilder<Block>,
  P: TransactionPool + Sync + Send + 'static,
{
  use operations_rpc::{Operations, OperationsApiServer};
  use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
  use substrate_frame_rpc_system::{System, SystemApiServer};
  use tipping_rpc::{Tipping, TippingApiServer};
  use verification_rpc::{Verification, VerificationApiServer};
  use workflows_rpc::{Workflows, WorkflowsApiServer};

  let mut module = RpcExtension::new(());
  let FullDeps {
    client,
    pool,
    deny_unsafe,
  } = deps;

  module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
  module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

  module.merge(Operations::new(client.clone()).into_rpc())?;
  module.merge(Workflows::new(client.clone()).into_rpc())?;
  module.merge(Verification::new(client.clone()).into_rpc())?;
  module.merge(Tipping::new(client).into_rpc())?;

  Ok(module)
}
