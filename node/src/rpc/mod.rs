//! A collection of node-specific RPC methods.

use std::sync::Arc;

use futures::channel::mpsc;
use jsonrpsee::RpcModule;

// Substrate
use sc_client_api::{
	backend::{Backend, StorageProvider},
	client::BlockchainEvents,
	AuxStore, UsageProvider,
};
use sc_consensus_manual_seal::rpc::EngineCommand;
use sc_rpc::SubscriptionTaskExecutor;
use sc_rpc_api::DenyUnsafe;
use sc_service::TransactionPool;
use sc_transaction_pool::ChainApi;
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_inherents::CreateInherentDataProviders;
use sp_runtime::traits::Block as BlockT;
use sp_consensus::SyncOracle;

// Aleph
use finality_aleph::{Justification, JustificationTranslator, ValidatorAddressCache};

// Runtime
use selendra_primitives::{AccountId, Balance, Block, Hash, Nonce};

mod eth;
mod aleph_node_rpc;
pub use aleph_node_rpc::{AlephNode, AlephNodeApiServer};
pub use self::eth::{create_eth, overrides_handle, EthDeps};

/// Full client dependencies.
pub struct FullDeps<C, P, A: ChainApi, CT, CIDP, SO> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// import justification transaction
	pub import_justification_tx: mpsc::UnboundedSender<Justification>,
	/// import jjustification translator
    pub justification_translator: JustificationTranslator,
	/// syn oracle
    pub sync_oracle: SO,
	/// validator address cache
	pub validator_address_cache: Option<ValidatorAddressCache>,
	/// Manual seal command sink
	pub command_sink: Option<mpsc::Sender<EngineCommand<Hash>>>,
	/// Ethereum-compatibility specific dependencies.
	pub eth: EthDeps<Block, C, P, A, CT, CIDP>,
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
pub fn create_full<C, P, BE, A, CT, CIDP, SO>(
	deps: FullDeps<C, P, A, CT, CIDP, SO>,
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
	P: TransactionPool<Block = Block> + 'static,
	A: ChainApi<Block = Block> + 'static,
	CIDP: CreateInherentDataProviders<Block, ()> + Send + 'static,
	CT: fp_rpc::ConvertTransaction<<Block as BlockT>::Extrinsic> + Send + Sync + 'static,
	SO: SyncOracle + Send + Sync + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use sc_consensus_manual_seal::rpc::{ManualSeal, ManualSealApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut io = RpcModule::new(());
	let FullDeps { 
		client,
		pool,
		deny_unsafe,
		import_justification_tx,
        justification_translator,
        sync_oracle,
        validator_address_cache,
		command_sink,
		eth,
	} = deps;

	io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	if let Some(command_sink) = command_sink {
		io.merge(
			// We provide the rpc handler with the sending end of the channel to allow the rpc
			// send EngineCommands to the background block authorship task.
			ManualSeal::new(command_sink).into_rpc(),
		)?;
	}

    io.merge(
        AlephNode::new(
            import_justification_tx,
            justification_translator,
            client,
            sync_oracle,
            validator_address_cache,
        )
        .into_rpc(),
    )?;

	// Ethereum compatibility RPCs
	let io = create_eth::<_, _, _, _, _, _, _, DefaultEthConfig<C, BE>>(
		io,
		eth,
		subscription_task_executor,
		pubsub_notification_sinks,
	)?;

	Ok(io)
}
