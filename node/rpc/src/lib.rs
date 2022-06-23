// This file is part of Selendra.

// Copyright (C) 2020-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! Selendra-specific RPCs implementation.

#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]

use std::sync::Arc;

use primitives::{AccountId, Balance, Block, BlockNumber, CurrencyId, DataProviderId, Hash, Nonce};
use sc_client_api::AuxStore;
use sc_consensus_babe::{Config, Epoch};
use sc_consensus_epochs::SharedEpochChanges;
use sc_finality_grandpa::{
	FinalityProofProvider, GrandpaJustificationStream, SharedAuthoritySet, SharedVoterState,
};
use sc_rpc::SubscriptionTaskExecutor;
pub use sc_rpc_api::DenyUnsafe;
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_consensus::SelectChain;
use sp_consensus_babe::BabeApi;
use sp_keystore::SyncCryptoStorePtr;

use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
use sc_consensus_babe_rpc::{Babe, BabeApiServer};
use sc_finality_grandpa_rpc::{Grandpa, GrandpaApiServer};
use sc_rpc::dev::{Dev, DevApiServer};
use sc_sync_state_rpc::{SyncState, SyncStateApiServer};
use substrate_frame_rpc_system::{System, SystemApiServer};
use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

/// orml rpc
use orml_oracle_rpc::{Oracle, OracleApiServer};
use orml_tokens_rpc::{Tokens, TokensApiServer};

/// module rpc
pub use evm_rpc::{EVMApiServer, EVMRuntimeRPCApi, EVM};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Extra dependencies for BABE.
pub struct BabeDeps {
	/// BABE protocol config.
	pub babe_config: Config,
	/// BABE pending epoch changes.
	pub shared_epoch_changes: SharedEpochChanges<Block, Epoch>,
	/// The keystore that manages the keys of the node.
	pub keystore: SyncCryptoStorePtr,
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
pub struct FullDeps<C, P, SC, B> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
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
}

/// Instantiate all Full RPC extensions.
pub fn create_full<C, P, SC, B>(
	deps: FullDeps<C, P, SC, B>,
	backend: Arc<B>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Sync
		+ Send
		+ 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BabeApi<Block>,
	C::Api: orml_oracle_rpc::OracleRuntimeApi<
		Block,
		DataProviderId,
		CurrencyId,
		runtime_common::TimeStampedPrice,
	>,
	C::Api: orml_tokens_rpc::TokensRuntimeApi<Block, CurrencyId, Balance>,
	C::Api: EVMRuntimeRPCApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + Sync + Send + 'static,
	SC: SelectChain<Block> + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	let mut module = RpcExtension::new(());
	let FullDeps {
		client,
		pool,
		select_chain,
		chain_spec,
		deny_unsafe,
		babe,
		grandpa,
	} = deps;

	let BabeDeps { keystore, babe_config, shared_epoch_changes } = babe;
	let GrandpaDeps {
		shared_voter_state,
		shared_authority_set,
		justification_stream,
		subscription_executor,
		finality_provider,
	} = grandpa;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

	module.merge(
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
	module.merge(
		Grandpa::new(
			subscription_executor,
			shared_authority_set.clone(),
			shared_voter_state,
			justification_stream,
			finality_provider,
		)
		.into_rpc(),
	)?;

	module.merge(
		SyncState::new(chain_spec, client.clone(), shared_authority_set, shared_epoch_changes)?
			.into_rpc(),
	)?;

	module.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;
	module.merge(Dev::new(client.clone(), deny_unsafe).into_rpc())?;

	// Making synchronous calls in light client freezes the browser currently,
	// more context: https://github.com/paritytech/substrate/pull/3480
	// These RPCs should use an asynchronous caller instead.
	module.merge(Oracle::new(client.clone()).into_rpc())?;
	module.merge(Tokens::new(client.clone()).into_rpc())?;
	module.merge(EVM::new(client.clone(), deny_unsafe).into_rpc())?;
	module.merge(Dev::new(client, deny_unsafe).into_rpc())?;

	Ok(module)
}
