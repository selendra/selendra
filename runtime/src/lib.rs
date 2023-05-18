// Copyright 2023 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "512"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod config;
pub mod constants;

#[cfg(feature = "try-runtime")]
use frame_try_runtime::UpgradeCheckSelect;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use sp_api::impl_runtime_apis;
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, SlotDuration};
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_std::prelude::*;

use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, OpaqueKeys},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, Perbill, Permill,
};

use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstBool, ConstU32, EqualPrivilegeOnly, Nothing, SortedMembers},
	weights::{constants::RocksDbWeight, ConstantMultiplier, Weight},
	PalletId,
};
use frame_system::EnsureSignedBy;

pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;

use selendra_primitives::{
	opaque, ApiError as SelendraApiError, AuthorityId as SelendraId,
	SessionAuthorityData, Version as FinalityVersion, DEFAULT_BAN_REASON_LENGTH,
	DEFAULT_MAX_WINNERS, DEFAULT_SESSION_PERIOD, TOKEN,
};
pub use selendra_primitives::{
	AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, Signature,
};
use selendra_runtime_common::{
	impls::DealWithFees, prod_or_fast, BlockLength, BlockWeights, SlowAdjustingFeeUpdate,
};

use constants::{
	currency::*, fee::WeightToFee, time::*, CONTRACTS_DEBUG_OUTPUT, CONTRACT_DEPOSIT_PER_BYTE,
};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("selendra"),
	impl_name: create_runtime_str!("selendra-node"),
	authoring_version: 1,
	spec_version: 3000,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 0,
	state_version: 0,
};

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = RocksDbWeight;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = pallet_balances::AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const MaxAuthorities: u32 = 100_000;
}

impl pallet_aura::Config for Runtime {
	type MaxAuthorities = MaxAuthorities;
	type AuthorityId = AuraId;
	type DisabledValidators = ();
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (Elections,);
}

impl pallet_selendra::Config for Runtime {
	type AuthorityId = SelendraId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInfoProvider = Session;
	type SessionManager = Elections;
	type NextSessionAuthorityProvider = Session;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub selendra: Selendra,
	}
}

parameter_types! {
	pub SessionPeriod: u32 = prod_or_fast!(DEFAULT_SESSION_PERIOD, 96);
	pub const MaximumBanReasonLength: u32 = DEFAULT_BAN_REASON_LENGTH;
	pub const MaxWinners: u32 = DEFAULT_MAX_WINNERS;
}

impl pallet_elections::Config for Runtime {
	type EraInfoProvider = Staking;
	type RuntimeEvent = RuntimeEvent;
	type DataProvider = Staking;
	type SessionInfoProvider = Session;
	type SessionPeriod = SessionPeriod;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Runtime, Staking>;
	type ValidatorRewardsHandler = Staking;
	type ValidatorExtractor = Staking;
	type MaximumBanReasonLength = MaximumBanReasonLength;
	type MaxWinners = MaxWinners;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500 * MILLI_CENT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

parameter_types! {
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type SessionManager = Selendra;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TransactionByteFee: Balance = 100 * MICRO_CENT;
	// This value increases the priority of `Operational` transactions by adding
	/// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`
	pub const OperationalFeeMultiplier: u8 = 5;

}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type WeightToFee = WeightToFee;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
}

impl pallet_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type PalletsOrigin = OriginCaller;
	type RuntimeCall = RuntimeCall;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type Preimages = ();
}

parameter_types! {
	pub const MinimumPeriod: u64 = MILLISECS_PER_BLOCK / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = RuntimeCall;
}

pub const TREASURY_PROPOSAL_BOND: Balance = 100 * TOKEN;

parameter_types! {
	// We do burn 5% money within treasury.
	pub const Burn: Permill = Permill::from_percent(5);
	// The fraction of the proposal that the proposer should deposit.
	// We agreed on non-progressive deposit.
	pub const ProposalBond: Permill = Permill::from_percent(0);
	// The minimal deposit for proposal.
	pub const ProposalBondMinimum: Balance = TREASURY_PROPOSAL_BOND;
	// The upper bound of the deposit for the proposal.
	pub const ProposalBondMaximum: Balance = TREASURY_PROPOSAL_BOND;
	// Maximum number of approvals that can wait in the spending queue.
	pub const MaxApprovals: u32 = 20;
	// Every 4 hours we fund accepted proposals.
	pub const SpendPeriod: BlockNumber = 4 * BLOCKS_PER_HOUR;
	pub const TreasuryPalletId: PalletId = PalletId(*b"a0/trsry");
}

pub struct TreasuryGovernance;

impl SortedMembers<AccountId> for TreasuryGovernance {
	fn sorted_members() -> Vec<AccountId> {
		pallet_sudo::Pallet::<Runtime>::key().into_iter().collect()
	}
}

impl pallet_treasury::Config for Runtime {
	type ApproveOrigin = EnsureSignedBy<TreasuryGovernance, AccountId>;
	type Burn = Burn;
	type BurnDestination = ();
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type MaxApprovals = MaxApprovals;
	type OnSlash = ();
	type PalletId = TreasuryPalletId;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type RejectOrigin = EnsureSignedBy<TreasuryGovernance, AccountId>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<u128>;
	type SpendPeriod = SpendPeriod;
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// Refundable deposit per storage item
	pub const DepositPerItem: Balance = 32 * CONTRACT_DEPOSIT_PER_BYTE;
	// Refundable deposit per byte of storage
	pub const DepositPerByte: Balance = CONTRACT_DEPOSIT_PER_BYTE;
	// How much weight of each block can be spent on the lazy deletion queue of terminated contracts
	pub DeletionWeightLimit: Weight = Perbill::from_percent(10) * BlockWeights::get().max_block; // 40ms
	// Maximum size of the lazy deletion queue of terminated contracts.
	pub const DeletionQueueDepth: u32 = 128;
	pub Schedule: pallet_contracts::Schedule<Runtime> = Default::default();
}

impl pallet_contracts::Config for Runtime {
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	// The safest default is to allow no calls at all. This is unsafe experimental feature with no support in ink!
	type CallFilter = Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type WeightPrice = pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type DeletionQueueDepth = DeletionQueueDepth;
	type DeletionWeightLimit = DeletionWeightLimit;
	type Schedule = Schedule;
	type CallStack = [pallet_contracts::Frame<Self>; 5];
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 123 * 1024 }>;
	type MaxStorageKeyLen = ConstU32<128>;
	type UnsafeUnstableInterface = ConstBool<false>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Scheduler: pallet_scheduler,
		Aura: pallet_aura,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		TransactionPayment: pallet_transaction_payment,
		Authorship: pallet_authorship,
		Staking: pallet_staking,
		History: pallet_session::historical,
		Session: pallet_session,
		Selendra: pallet_selendra,
		Elections: pallet_elections,
		Treasury: pallet_treasury,
		Vesting: pallet_vesting,
		Utility: pallet_utility,
		Multisig: pallet_multisig,
		Sudo: pallet_sudo,
		Contracts: pallet_contracts,
		NominationPools: pallet_nomination_pools,
		Identity: pallet_identity,
	}
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	(),
>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> SlotDuration {
			SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().to_vec()
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
		fn query_weight_to_fee(weight: Weight) -> Balance {
			TransactionPayment::weight_to_fee(weight)
		}
		fn query_length_to_fee(length: u32) -> Balance {
			TransactionPayment::length_to_fee(length)
		}
	}

	impl selendra_primitives::SelendraSessionApi<Block> for Runtime {
		fn millisecs_per_block() -> u64 {
			MILLISECS_PER_BLOCK
		}

		fn session_period() -> u32 {
			SessionPeriod::get()
		}

		fn authorities() -> Vec<SelendraId> {
			Selendra::authorities()
		}

		fn next_session_authorities() -> Result<Vec<SelendraId>, SelendraApiError> {
			let next_authorities = Selendra::next_authorities();
			if next_authorities.is_empty() {
				return Err(SelendraApiError::DecodeKey)
			}

			Ok(next_authorities)
		}

		fn authority_data() -> SessionAuthorityData {
			SessionAuthorityData::new(Selendra::authorities(), Selendra::emergency_finalizer())
		}

		fn next_session_authority_data() -> Result<SessionAuthorityData, SelendraApiError> {
			Ok(SessionAuthorityData::new(
				Self::next_session_authorities()?,
				Selendra::queued_emergency_finalizer(),
			))
		}

		fn finality_version() -> FinalityVersion {
			Selendra::finality_version()
		}

		fn next_session_finality_version() -> FinalityVersion {
			Selendra::next_session_finality_version()
		}
	}

	impl pallet_nomination_pools_runtime_api::NominationPoolsApi<Block, AccountId, Balance> for Runtime {
		fn pending_rewards(member_account: AccountId) -> Balance {
			NominationPools::pending_rewards(member_account).unwrap_or_default()
		}
	}

	impl pallet_contracts::ContractsApi<Block, AccountId, Balance, BlockNumber, Hash>
		for Runtime
	{
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			input_data: Vec<u8>,
		) -> pallet_contracts_primitives::ContractExecResult<Balance> {
			let gas_limit = gas_limit.unwrap_or(BlockWeights::get().max_block);
			Contracts::bare_call(
				origin,
				dest,
				value,
				gas_limit,
				storage_deposit_limit,
				input_data,
				CONTRACTS_DEBUG_OUTPUT,
				pallet_contracts::Determinism::Deterministic,
			)
		}

		fn instantiate(
			origin: AccountId,
			value: Balance,
			gas_limit: Option<Weight>,
			storage_deposit_limit: Option<Balance>,
			code: pallet_contracts_primitives::Code<Hash>,
			data: Vec<u8>,
			salt: Vec<u8>,
		) -> pallet_contracts_primitives::ContractInstantiateResult<AccountId, Balance>
		{
			let gas_limit = gas_limit.unwrap_or(BlockWeights::get().max_block);
			Contracts::bare_instantiate(
				origin,
				value,
				gas_limit,
				storage_deposit_limit,
				code,
				data,
				salt,
				CONTRACTS_DEBUG_OUTPUT
			)
		}

		fn upload_code(
			origin: AccountId,
			code: Vec<u8>,
			storage_deposit_limit: Option<Balance>,
			determinism: pallet_contracts::Determinism,
		) -> pallet_contracts_primitives::CodeUploadResult<Hash, Balance>
		{
			Contracts::bare_upload_code(origin, code, storage_deposit_limit, determinism)
		}

		fn get_storage(
			address: AccountId,
			key: Vec<u8>,
		) -> pallet_contracts_primitives::GetStorageResult {
			Contracts::get_storage(address, key)
		}
	}

	#[cfg(feature = "try-runtime")]
	 impl frame_try_runtime::TryRuntime<Block> for Runtime {
		  fn on_runtime_upgrade(checks: UpgradeCheckSelect) -> (Weight, Weight) {
			   let weight = Executive::try_runtime_upgrade(checks).unwrap();
			   (weight, BlockWeights::get().max_block)
		  }

		  fn execute_block(
			   block: Block,
			   state_root_check: bool,
			   checks: bool,
			   select: frame_try_runtime::TryStateSelect,
		  ) -> Weight {
			Executive::try_execute_block(block, state_root_check, checks, select).unwrap()
		}
	 }
}

#[cfg(test)]
mod tests {
	use frame_support::traits::Get;
	use selendra_primitives::HEAP_PAGES;
	use smallvec::Array;

	use super::*;

	#[test]
	fn state_version_must_be_zero() {
		assert_eq!(0, VERSION.state_version);
	}

	#[test]
	fn check_contracts_memory_parameters() {
		// Memory limit of one instance of a runtime
		const MAX_RUNTIME_MEM: u32 = HEAP_PAGES as u32 * 64 * 1024;
		// Max stack size defined by wasmi - 1MB
		const MAX_STACK_SIZE: u32 = 1024 * 1024;
		// Max heap size is 16 mempages of 64KB each - 1MB
		let max_heap_size =
			<Runtime as pallet_contracts::Config>::Schedule::get().limits.max_memory_size();
		// Max call depth is CallStack::size() + 1
		let max_call_depth = <Runtime as pallet_contracts::Config>::CallStack::size() as u32 + 1;
		// Max code len
		let max_code_len: u32 = <Runtime as pallet_contracts::Config>::MaxCodeLen::get();

		// The factor comes from allocator, contracts representation, and wasmi
		let lhs = max_call_depth * (72 * max_code_len + max_heap_size + MAX_STACK_SIZE);
		// We allocate only 75% of all runtime memory to contracts execution. Important: it's not
		// enforeced in wasmtime
		let rhs = MAX_RUNTIME_MEM * 3 / 4;

		assert!(lhs < rhs);
	}
}
