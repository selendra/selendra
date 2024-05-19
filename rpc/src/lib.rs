// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A collection of node-specific RPC methods.
//!
//! Since `substrate` core functionality makes no assumptions
//! about the modules used inside the runtime, so do
//! RPC methods defined in `sc-rpc` crate.
//! It means that `client/rpc` can't have any methods that
//! need some strong assumptions about the particular runtime.
//!
//! The RPCs available in this crate however can make some assumptions
//! about how the runtime is constructed and what FRAME pallets
//! are part of it. Therefore all node-runtime-specific RPCs can
//! be placed here or imported from corresponding FRAME RPC definitions.

// #![warn(unused_crate_dependencies)]

mod starknet;

use std::sync::Arc;

use jsonrpsee::RpcModule;
use sc_client_api::{client::BlockchainEvents, AuxStore, StorageProvider, UsageProvider,};
use sc_consensus_babe::BabeWorkerHandle;
use sc_consensus_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use selendra_primitives::{AccountId, Balance, Block, BlockNumber, Hash, Nonce, StarknetHasher};
use sp_api::{CallApiAt, ProvideRuntimeApi};
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::KeystorePtr;
use sc_transaction_pool_api::TransactionPool;
use sc_transaction_pool::{ChainApi, Pool};

pub use starknet::StarknetDeps;
use mc_genesis_data_provider::GenesisProvider;
use mc_rpc::{
	starknetrpcwrapper::StarknetRpcWrapper,
	MadaraRpcApiServer, Starknet, StarknetReadRpcApiServer, StarknetTraceRpcApiServer, StarknetWriteRpcApiServer,
};

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// A handle to the BABE worker for issuing requests.
	pub babe_worker_handle: BabeWorkerHandle<Block>,
	/// The keystore that manages the keys of the node.
	pub keystore: KeystorePtr,
}

/// Extra dependencies for GRANDPA
pub struct GrandpaDeps<B> {
	/// Voting round info.
	pub shared_voter_state: SharedVoterState,
	/// Authority set info.
	pub shared_authority_set: SharedAuthoritySet<Hash, BlockNumber>,
	/// Receives notifications about justification events from Grandpa.
	pub justification_stream: GrandpaJustificationStream<Block>,
	/// Executor to drive the subscription manager in the Grandpa RPC handler.
	pub subscription_executor: SubscriptionTaskExecutor,
	/// Finality proof provider.
	pub finality_provider: Arc<FinalityProofProvider<B, Block>>,
}

/// Full client dependencies.
pub struct FullDeps<C, P, SC, B, G: GenesisProvider,A: ChainApi,> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Extrinsic pool graph instance.
    pub graph: Arc<Pool<A>>,
	/// The SelectChain Strategy
	pub select_chain: SC,
	/// A copy of the chain spec.
	pub chain_spec: Box<dyn sc_chain_spec::ChainSpec>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
	/// BABE specific dependencies.
	pub babe: BabeDeps,
	/// GRANDPA specific dependencies.
	pub grandpa: GrandpaDeps<B>,
	/// The backend used by the node.
	pub backend: Arc<B>,
	/// Starknet dependencies
	pub starknet: StarknetDeps<C, G, Block>,
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B, G, A>(
	FullDeps {
		client,
		pool,
		graph,
		select_chain,
		chain_spec,
		deny_unsafe,
		babe,
		grandpa,
		backend,
		starknet: starknet_params
	}: FullDeps<C, P, SC, B, G, A>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	A: ChainApi<Block = Block> + 'static,
	C: ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ CallApiAt<Block>
		+ HeaderBackend<Block>
		+ StorageProvider<Block, B>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ BlockchainEvents<Block>
		+ UsageProvider<Block>
		+ 'static
		+ Sync
		+ Send
		+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: pallet_starknet_runtime_api::StarknetRuntimeApi<Block>
        + pallet_starknet_runtime_api::ConvertTransactionRuntimeApi<Block>,
	C::Api: BabeApi<Block>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool<Block = Block> + 'static,
	SC: SelectChain<Block> + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashingFor<Block>>,
	G: GenesisProvider + Send + Sync + 'static,
{
	use substrate_frame_rpc_system::{System, SystemApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use sc_consensus_babe_rpc::{Babe, BabeApiServer};
	use sc_consensus_grandpa_rpc::{Grandpa, GrandpaApiServer};
	use sc_rpc::dev::{Dev, DevApiServer};
	use sc_rpc_spec_v2::chain_spec::{ChainSpec, ChainSpecApiServer};
	use sc_sync_state_rpc::{SyncState, SyncStateApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	let mut io = RpcModule::new(());

	let BabeDeps { keystore, babe_worker_handle } = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	let chain_name = chain_spec.name().to_string();
	let genesis_hash = client.block_hash(0).ok().flatten().expect("Genesis block exists; qed");
	let properties = chain_spec.properties();

	let rpc_instance: StarknetRpcWrapper<_, _, _, _, _, _, StarknetHasher> =
	StarknetRpcWrapper(Arc::new(Starknet::<_, _, _, _, _, _, StarknetHasher>::new(
		client.clone(),
		starknet_params.madara_backend,
		starknet_params.overrides,
		pool.clone(),
		graph,
		starknet_params.sync_service,
		starknet_params.starting_block,
		starknet_params.genesis_provider,
	)));

	io.merge(MadaraRpcApiServer::into_rpc(rpc_instance.clone()))?;
	io.merge(StarknetReadRpcApiServer::into_rpc(rpc_instance.clone()))?;
	io.merge(StarknetWriteRpcApiServer::into_rpc(rpc_instance.clone()))?;
	io.merge(StarknetTraceRpcApiServer::into_rpc(rpc_instance.clone()))?;

	io.merge(ChainSpec::new(chain_name, genesis_hash, properties).into_rpc())?;
	io.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	io.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	io.merge(
		Babe::new(client.clone(), babe_worker_handle.clone(), keystore, select_chain, deny_unsafe)
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
		SyncState::new(chain_spec, client.clone(), shared_authority_set, babe_worker_handle)?
			.into_rpc(),
	)?;

	io.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;
	io.merge(Dev::new(client, deny_unsafe).into_rpc())?;

	Ok(io)
}
