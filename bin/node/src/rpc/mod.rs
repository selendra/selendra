//! A collection of node-specific RPC methods.
//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

pub mod selendra_node_rpc;
pub mod eth;
pub use self::eth::{create_eth, EthDeps};
pub use selendra_node_rpc::{SelendraNode, SelendraNodeApiServer};

use std::sync::Arc;

use finality_aleph::{Justification, JustificationTranslator, ValidatorAddressCache};
use futures::channel::mpsc;
use jsonrpsee::RpcModule;
use primitives::{AccountId, Balance, Block, Nonce};
use sp_core::H256;
use sc_client_api::{
	backend::{Backend, StorageProvider},
	client::BlockchainEvents,
	AuxStore, UsageProvider,
};
use sc_transaction_pool_api::TransactionPool;
use sp_api::{CallApiAt, ProvideRuntimeApi};

use sc_rpc::SubscriptionTaskExecutor;

use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SyncOracle;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::traits::Block as BlockT;

/// Full client dependencies.
pub struct FullDeps<C, P, CT, CIDP, SO> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// import justification transaction
	pub import_justification_tx: mpsc::UnboundedSender<Justification>,
	/// import jjustification translator
	pub justification_translator: JustificationTranslator,
	/// syn oracle
	pub sync_oracle: SO,
	/// validator address cache
	pub validator_address_cache: Option<ValidatorAddressCache>,
	/// Ethereum-compatibility specific dependencies.
	pub eth: EthDeps<Block, C, P, CT, CIDP>,
}

pub struct DefaultEthConfig<C, BE>(std::marker::PhantomData<(C, BE)>);

impl<C, BE> fc_rpc::EthConfig<Block, C> for DefaultEthConfig<C, BE>
where
	C: StorageProvider<Block, BE> + Sync + Send + 'static,
	BE: Backend<Block> + 'static,
{
	type EstimateGasAdapter = ();
	type RuntimeStorageOverride =
		fc_rpc::frontier_backend_client::SystemAccountId32StorageOverride<Block, C, BE>;
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, BE, CT, CIDP, SO>(
	deps: FullDeps<C, P, CT, CIDP, SO>,
	subscription_task_executor: SubscriptionTaskExecutor,
	pubsub_notification_sinks: Arc<
		fc_mapping_sync::EthereumBlockNotificationSinks<
			fc_mapping_sync::EthereumBlockNotification<Block>,
		>,
	>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: CallApiAt<Block>
		+ ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ UsageProvider<Block>
		+ StorageProvider<Block, BE>
		+ AuxStore
		+ Send
		+ Sync
		+ 'static,
	C::Api: sp_block_builder::BlockBuilder<Block>,
	C::Api: sp_consensus_aura::AuraApi<Block, AuraId>,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	BE: Backend<Block> + 'static,
	P: TransactionPool<Block = Block, Hash = H256> + 'static,
	CIDP: CreateInherentDataProviders<Block, ()> + Send + 'static,
	CT: fp_rpc::ConvertTransaction<<Block as BlockT>::Extrinsic> + Send + Sync + 'static,
	SO: SyncOracle + Send + Sync + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		import_justification_tx,
		justification_translator,
		sync_oracle,
		validator_address_cache,
		eth,
		_phantom,
	} = deps;

	module.merge(System::new(client.clone(), pool.clone()).into_rpc())?;

	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
    
	module.merge(
		SelendraNode::new(
			import_justification_tx,
			justification_translator,
			client,
			sync_oracle,
			validator_address_cache,
		)
		.into_rpc(),
	)?;

	// Ethereum compatibility RPCs
	let module = create_eth::<_, _, _, _, _, _, DefaultEthConfig<C, BE>>(
		module,
		eth,
		subscription_task_executor,
		pubsub_notification_sinks,
	)?;

	Ok(module)
}
