//! A Substrate pallet implementation for Starknet, a decentralized, permissionless, and scalable
//! zk-rollup for general-purpose smart contracts.
//! See the [Starknet documentation](https://docs.starknet.io/) for more information.
//! The code consists of the following sections:
//! 1. Config: The trait Config is defined, which is used to configure the pallet by specifying the
//! parameters and types on which it depends. The trait also includes associated types for
//! StateRoot, SystemHash, and TimestampProvider.
//!
//! 2. Hooks: The Hooks trait is implemented for the pallet, which includes methods to be executed
//! during the block lifecycle: on_finalize, on_initialize, on_runtime_upgrade, and offchain_worker.
//!
//! 3. Storage: Several storage items are defined, including Pending, CurrentBlock, BlockHash,
//! ContractClassHashes, ContractClasses, Nonces, StorageView, LastKnownEthBlock, and
//! FeeTokenAddress. These storage items are used to store and manage data related to the Starknet
//! pallet.
//!
//! 4. Genesis Configuration: The GenesisConfig struct is defined, which is used to set up the
//! initial state of the pallet during genesis. The struct includes fields for contracts,
//! contract_classes, storage, fee_token_address, chain_id and _phantom. A GenesisBuild
//! implementation is provided to build the initial state during genesis.
//!
//! 5. Events: A set of events are defined in the Event enum, including KeepStarknetStrange,
//! StarknetEvent, and FeeTokenAddressChanged. These events are emitted during the execution of
//! various pallet functions.
//!
//! 6.Errors: A set of custom errors are defined in the Error enum, which is used to represent
//! various error conditions during the execution of the pallet.
//!
//! 7. Dispatchable Functions: The Pallet struct implements several dispatchable functions (ping,
//! invoke, ...), which allow users to interact with the pallet and invoke state changes. These
//! functions are annotated with weight and return a DispatchResult.
// Ensure we're `no_std` when compiling for Wasm.
#![allow(clippy::large_enum_variant)]

// use std::sync::Arc;

/// Starknet pallet.
/// Definition of the pallet's runtime storage items, events, errors, and dispatchable
/// functions.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
/// An adapter for the blockifier state related traits
pub mod blockifier_state_adapter;
#[cfg(feature = "genesis-loader")]
pub mod genesis_loader;
/// Simulation, estimations and execution trace logic.
pub mod simulations;
/// Transaction validation logic.
pub mod transaction_validation;
/// The Starknet pallet's runtime custom types.
pub mod types;

// // #[cfg(test)]
// // mod tests;

// use std::collections::BTreeSet;
// use std::str::from_utf8_unchecked;

// use sp_runtime::traits::UniqueSaturatedInto;
// use sp_runtime::DigestItem;
// use frame_support::pallet_prelude::*;
// use frame_support::traits::Time;
// use frame_system::pallet_prelude::*;

// use blockifier::blockifier::block::{BlockInfo, GasPrices};
// use blockifier::context::{BlockContext, ChainInfo, FeeTokenAddresses, TransactionContext};
// use blockifier::execution::call_info::CallInfo;
// use blockifier::execution::contract_class::ContractClass;
// use blockifier::execution::entry_point::{CallEntryPoint, CallType, EntryPointExecutionContext};
// use blockifier::state::cached_state::{CachedState, GlobalContractCache};
// use blockifier::transaction::account_transaction::AccountTransaction;
// use blockifier::transaction::objects::{DeprecatedTransactionInfo, TransactionInfo};
// use blockifier::transaction::transaction_execution::Transaction;
// use blockifier::transaction::transactions::{
//     DeclareTransaction, DeployAccountTransaction, InvokeTransaction, L1HandlerTransaction,
// };
// use blockifier::versioned_constants::VersionedConstants;

// use starknet_api::block::{BlockNumber, BlockTimestamp};
// use starknet_api::core::{ChainId, ClassHash, CompiledClassHash, ContractAddress, EntryPointSelector, Nonce};
// use starknet_api::deprecated_contract_class::EntryPointType;
// use starknet_api::hash::StarkFelt;
// use starknet_api::state::StorageKey;
// use starknet_api::transaction::{
//     Calldata, Event as StarknetEvent, Fee, MessageToL1, TransactionHash, TransactionVersion,
// };
// use starknet_crypto::FieldElement;

// use blockifier_state_adapter::BlockifierStateAdapter;

// use mp_block::{Block as StarknetBlock, Header as StarknetHeader};
// use mp_chain_id::MADARA_CHAIN_ID;
// use mp_digest_log::MADARA_ENGINE_ID;
// use mp_felt::Felt252Wrapper;
// use mp_sequencer_address::{InherentError, InherentType, DEFAULT_SEQUENCER_ADDRESS, INHERENT_IDENTIFIER};
// use mp_storage::{StarknetStorageSchemaVersion, PALLET_STARKNET_SCHEMA};
// use mp_transactions::execution::{
//     execute_l1_handler_transaction, run_non_revertible_transaction, run_revertible_transaction,
// };
// use mp_transactions::{get_transaction_nonce, get_transaction_sender_address};

// use crate::types::{CasmClassHash, ContractStorageKey, SierraClassHash, SierraOrCasmClassHash, StorageSlot};
// pub(crate) const LOG_TARGET: &str = "runtime::starknet";

// pub const ETHEREUM_EXECUTION_RPC: &[u8] = b"starknet::ETHEREUM_EXECUTION_RPC";
// pub const ETHEREUM_CONSENSUS_RPC: &[u8] = b"starknet::ETHEREUM_CONSENSUS_RPC";

// pub const SN_OS_CONFIG_HASH_VERSION: &str = "StarknetOsConfig1";


// syntactic sugar for logging.
#[macro_export]
macro_rules! log {
	($level:tt, $pattern:expr $(, $values:expr)* $(,)?) => {
		log::$level!(
			target: $crate::LOG_TARGET,
			concat!("[{:?}] 🐺 ", $pattern), <frame_system::Pallet<T>>::block_number() $(, $values)*
		)
	};
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    /// We're coupling the starknet pallet to the tx payment pallet to be able to override the fee
    /// mechanism and comply with starknet which uses an ER20 as fee token
    #[pallet::config]
    pub trait Config: frame_system::Config {

    }
}