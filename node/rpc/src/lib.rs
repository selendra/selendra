//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

mod node_rpc;

use futures::channel::mpsc;
use jsonrpsee::RpcModule;
use std::sync::Arc;

use sp_api::{BlockT, HeaderT, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

use finality_selendra::JustificationNotification;
use selendra_primitives::{opaque::Block, AccountId, Balance, BlockNumber, Index};

pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;

/// Full client dependencies.
pub struct FullDeps<B, C, P>
where
	B: BlockT,
	B::Header: HeaderT<Number = BlockNumber>,
{
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// justification for transaction
	pub import_justification_tx: mpsc::UnboundedSender<JustificationNotification<B>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<B, C, P>(
	deps: FullDeps<B, C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
	B: BlockT,
	B::Header: HeaderT<Number = BlockNumber>,
{
	use crate::node_rpc::{SelendraNode, SelendraNodeApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool, deny_unsafe, import_justification_tx } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;

	module.merge(TransactionPayment::new(client).into_rpc())?;

	module.merge(SelendraNode::<B>::new(import_justification_tx).into_rpc())?;

	Ok(module)
}
