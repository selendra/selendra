// This file is part of Selendra.

// Copyright (C) 2021-2022 Selendra.
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

//! Selendra-specific RPCs implementation.

use std::{collections::BTreeMap, sync::Arc};

use jsonrpsee::RpcModule;
// Substrate
use sc_client_api::{
	backend::{AuxStore, Backend, StateBackend, StorageProvider},
	client::BlockchainEvents,
};
use sc_consensus_babe::{BabeConfiguration, Epoch};
use sc_finality_grandpa::FinalityProofProvider;
use sc_network::NetworkService;

pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool::{ChainApi, Pool};
use sc_transaction_pool_api::TransactionPool;
use selendra_primitives::v2::{AccountId, Balance, Block, BlockNumber, Hash, Nonce};
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{
	Backend as BlockchainBackend, Error as BlockChainError, HeaderBackend, HeaderMetadata,
};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::SyncCryptoStorePtr;
use sp_runtime::traits::BlakeTwo256;

use fc_rpc::{
	EthBlockDataCacheTask, OverrideHandle, RuntimeApiStorageOverride, SchemaV1Override,
	SchemaV2Override, SchemaV3Override, StorageOverride,
};
use fc_rpc_core::types::{FeeHistoryCache, FilterPool};
use fp_storage::EthereumStorageSchema;

pub fn open_frontier_backend<C>(
	client: Arc<C>,
	config: &sc_service::Configuration,
) -> Result<Arc<fc_db::Backend<Block>>, String>
where
	C: sp_blockchain::HeaderBackend<Block>,
{
	let config_dir = config
		.base_path
		.as_ref()
		.map(|base_path| base_path.config_dir(config.chain_spec.id()))
		.unwrap_or_else(|| {
			sc_service::BasePath::from_project("", "", "selendra")
				.config_dir(config.chain_spec.id())
		});
	let path = config_dir.join("frontier").join("db");

	Ok(Arc::new(fc_db::Backend::<Block>::new(
		client,
		&fc_db::DatabaseSettings { source: fc_db::DatabaseSource::RocksDb { path, cache_size: 0 } },
	)?))
}

pub fn overrides_handle<C, BE>(client: Arc<C>) -> Arc<OverrideHandle<Block>>
where
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError>,
	C: Send + Sync + 'static,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	BE: sc_client_api::Backend<Block> + 'static,
	BE::State: sc_client_api::StateBackend<BlakeTwo256>,
{
	let mut overrides_map = BTreeMap::new();
	overrides_map.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V2,
		Box::new(SchemaV2Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);
	overrides_map.insert(
		EthereumStorageSchema::V3,
		Box::new(SchemaV3Override::new(client.clone()))
			as Box<dyn StorageOverride<_> + Send + Sync>,
	);

	Arc::new(OverrideHandle {
		schemas: overrides_map,
		fallback: Box::new(RuntimeApiStorageOverride::new(client)),
	})
}

/// A type representing all RPC extensions.
pub type RpcExtension = RpcModule<()>;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: BabeConfiguration,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: sc_consensus_epochs::SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
}

/// Dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: sc_finality_grandpa::SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: sc_finality_grandpa::SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: sc_finality_grandpa::GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

use beefy_gadget::communication::notification::{
	BeefyBestBlockStream, BeefyVersionedFinalityProofStream,
};

/// Dependencies for BEEFY
pub struct BeefyDeps {
	/// Receives notifications about finality proof events from BEEFY.
	pub beefy_finality_proof_stream: BeefyVersionedFinalityProofStream<Block>,
	/// Receives notifications about best block events from BEEFY.
	pub beefy_best_block_stream: BeefyBestBlockStream<Block>,
	/// Executor to drive the subscription manager in the BEEFY RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
}

/// Full client dependencies
pub struct FullDeps<C, P, SC, B, A: ChainApi> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// The [`SelectChain`] Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// BEEFY specific dependencies.
	pub beefy: BeefyDeps,
	/// Graph pool instance.
	pub graph: Arc<Pool<A>>,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// The Node authority flag
	pub is_authority: bool,
	/// Frontier Backend.
	pub frontier_backend: Arc<fc_db::Backend<Block>>,
	/// EthFilterApi pool.
	pub filter_pool: FilterPool,
	/// Maximum fee history cache size.                                                                                    
	pub fee_history_limit: u64,
	/// Fee history cache.
	pub fee_history_cache: FeeHistoryCache,
	/// Ethereum data access overrides.
	pub overrides: Arc<OverrideHandle<Block>>,
	/// Cache for Ethereum block data.
	pub block_data_cache: Arc<EthBlockDataCacheTask<Block>>,
	/// Enable EVM RPC servers
	pub enable_evm_rpc: bool,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, SC, B, A>(
	deps: FullDeps<C, P, SC, B, A>,
	backend: Arc<B>,
	subscription_task_executor: SubscriptionTaskExecutor,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ StorageProvider<Block, B>
		+ AuxStore
		+ HeaderBackend<Block>
		+ BlockchainEvents<Block>
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_mmr_rpc::MmrRuntimeApi<
		Block,
		<Block as sp_runtime::traits::Block>::Hash,
		BlockNumber,
	>,
	C::Api: fp_rpc::ConvertTransactionRuntimeApi<Block>,
	C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + Sync + Send + 'static,
	SC: SelectChain<Block> + 'static,
	B: Backend<Block> + Send + Sync + 'static,
	B::State: StateBackend<sp_runtime::traits::HashFor<Block>>,
	B::Blockchain: BlockchainBackend<Block>,
	A: ChainApi<Block = Block> + 'static,
{
	use beefy_gadget_rpc::{Beefy, BeefyApiServer};
	use fc_rpc::{
		Eth, EthApiServer, EthFilter, EthFilterApiServer, EthPubSub,
		EthPubSubApiServer, Net, NetApiServer, Web3, Web3ApiServer,
	};
	use frame_rpc_system::{System, SystemApiServer};
	use pallet_mmr_rpc::{Mmr, MmrApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use sc_consensus_babe_rpc::{Babe, BabeApiServer};
	use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};
	use sc_sync_state_rpc::{SyncState, SyncStateApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	let mut io = RpcModule::new(());
	let FullDeps {
		client,
		pool,
		select_chain,
		chain_spec,
		deny_unsafe,
		babe,
		grandpa,
		beefy,
		graph,
		network,
		is_authority,
		frontier_backend,
		filter_pool,
		fee_history_limit,
		fee_history_cache,
		overrides,
		block_data_cache,
		enable_evm_rpc,
	} = deps;
	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	io.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;
	io.merge(System::new(client.clone(), pool.clone(), deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	io.merge(Mmr::new(client.clone()).into_rpc())?;
	io.merge(
		Babe::new(
			client.clone(),
			shared_epoch_changes.clone(),
			keystore,
			babe_config,
			select_chain,
			deny_unsafe,
		)
		.into_rpc(),
	)?;
	io.merge(
		Grandpa::new(
			subscription_executor,
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			finality_provider,
		)
		.into_rpc(),
	)?;
	io.merge(
		SyncState::new(chain_spec, client.clone(), shared_authority_set, shared_epoch_changes)?
			.into_rpc(),
	)?;

	io.merge(
		Beefy::<Block>::new(
			beefy.beefy_finality_proof_stream,
			beefy.beefy_best_block_stream,
			beefy.subscription_executor,
		)?
		.into_rpc(),
	)?;

	if !enable_evm_rpc {
		return Ok(io)
	}

	let no_tx_converter: Option<fp_rpc::NoTransactionConverter> = None;

	io.merge(
		Eth::new(
			client.clone(),
			pool.clone(),
			graph,
			no_tx_converter,
			network.clone(),
			Default::default(),
			overrides.clone(),
			frontier_backend.clone(),
			is_authority,
			block_data_cache.clone(),
			fee_history_cache,
			fee_history_limit,
			// Allow 10x max allowed weight for non-transactional calls
			10,
		)
		.into_rpc(),
	)?;

	let max_past_logs: u32 = 10_000;
	let max_stored_filters: usize = 500;
	io.merge(
		EthFilter::new(
			client.clone(),
			frontier_backend,
			filter_pool,
			max_stored_filters,
			max_past_logs,
			block_data_cache,
		)
		.into_rpc(),
	)?;

	io.merge(Net::new(client.clone(), network.clone(), true).into_rpc())?;

	io.merge(Web3::new(client.clone()).into_rpc())?;

	io.merge(
		EthPubSub::new(pool, client, network, subscription_task_executor, overrides).into_rpc(),
	)?;

	Ok(io)
}
