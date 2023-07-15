#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_api::impl_runtime_apis;
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, SlotDuration};
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
pub use sp_runtime::{FixedPointNumber, Perbill, Permill};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub use frame_support::{
	construct_runtime, log, parameter_types,
	traits::{
		ConstBool, ConstU16, ConstU32, Currency, EqualPrivilegeOnly, EstimateNextNewSession,
		Imbalance, KeyOwnerProofSystem, LockIdentifier, Nothing, OnUnbalanced, Randomness,
		SortedMembers, ValidatorSet,
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
		},
		ConstantMultiplier, IdentityFee, Weight,
	},
	PalletId, StorageValue,
};
use frame_system::EnsureSignedBy;
#[cfg(feature = "try-runtime")]
use frame_try_runtime::UpgradeCheckSelect;

pub use pallet_balances::Call as BalancesCall;
use pallet_committee_management::SessionAndEraManager;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;

use selendra_primitives::{
	opaque, ApiError as SelendraApiError, SessionAuthorityData, Version as FinalityVersion, TOKEN,
	TREASURY_PROPOSAL_BOND,
};
pub use selendra_primitives::{
	AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, Signature,
};
use selendra_runtime_common::{
	impls::DealWithFees, BlockLength, BlockWeights, SlowAdjustingFeeUpdate,
};

use indra_offchain_rollup::{anchor as pallet_anchor, oracle as pallet_oracle};
pub use indranet_pallets::{
	pallet_base_pool, pallet_computation, pallet_indra, pallet_indra_tokenomic, pallet_mq,
	pallet_registry, pallet_stake_pool, pallet_stake_pool_v2, pallet_vault,
	pallet_wrapped_balances,
};

mod config;
pub mod constants;
mod origin;

use config::{SelendraId, SessionPeriod};
use constants::{currency::*, fee::WeightToFee, time::*};

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("selendra"),
	impl_name: create_runtime_str!("selendra-node"),
	authoring_version: 1,
	spec_version: 3002,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub selendra: Selendra,
	}
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const BlockHashCount: BlockNumber = 2400;
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
	#[cfg(feature = "runtime-testnet")]
	type SS58Prefix = ConstU16<42>;
	#[cfg(not(feature = "runtime-testnet"))]
	type SS58Prefix = ConstU16<204>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
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
	pub const ExistentialDeposit: u128 = 500 * PICO_CENT;
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type MaxReserves = ();
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
	type ScheduleOrigin = origin::EnsureRootOrHalfCouncil;
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

parameter_types! {
	// We do not burn any money within treasury.
	pub const Burn: Permill = Permill::from_percent(0);
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

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// Basic stuff; balances is uncallable initially.
		System: frame_system = 0,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 1,
		Scheduler: pallet_scheduler = 2,
		Aura: pallet_aura = 3,
		Timestamp: pallet_timestamp = 4,
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,
		Treasury: pallet_treasury = 20,

		// consensus sfuff
		Authorship: pallet_authorship = 30,
		Staking: pallet_staking = 31,
		History: pallet_session::historical = 32,
		Session: pallet_session = 33,
		Selendra: pallet_selendra = 34,
		Elections: pallet_elections = 35,
		CommitteeManagement: pallet_committee_management = 36,
		NominationPools: pallet_nomination_pools = 37,

		// Governance
		Democracy: pallet_democracy = 40,
		Council: pallet_collective::<Instance1> = 41 ,
		TechnicalCommittee: pallet_collective::<Instance2> = 42,
		TechnicalMembership: pallet_membership::<Instance1> = 43,

		// Asset and Nft
		Assets: pallet_assets = 50,
		Uniques: pallet_uniques::{Pallet, Storage, Event<T>} = 51,
		RmrkCore: pallet_rmrk_core::{Pallet, Call, Event<T>} = 52,

		// Indranet
		IndranetMq: pallet_mq = 60,
		IndranetRegistry: pallet_registry = 61,
		IndranetComputation: pallet_computation = 62,
		IndranetStakePoolv2: pallet_stake_pool_v2 = 63,
		IndranetStakePool: pallet_stake_pool = 64,
		IndranetVault: pallet_vault = 65,
		IndranetWrappedBalances: pallet_wrapped_balances = 66,
		IndranetBasePool: pallet_base_pool = 67,
		IndranetIndraContracts: pallet_indra = 68,
		IndranetIndraTokenomic: pallet_indra_tokenomic = 69,
		// Rollup and Oracles
		IndraRollupAnchor: pallet_anchor = 80,
		IndraOracle: pallet_oracle = 81,

		// Utility Suff
		Vesting: pallet_vesting = 90,
		Utility: pallet_utility = 91,
		Multisig: pallet_multisig = 92,
		Identity: pallet_identity = 93,

		// Temporary
		Sudo: pallet_sudo = 100,
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
	frame_system::CheckNonZeroSender<Runtime>,
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

pub type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	frame_benchmarking::define_benchmarks!([]);
}

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
		fn pending_rewards(member: AccountId) -> Balance {
			NominationPools::api_pending_rewards(member).unwrap_or_default()
		}

		fn points_to_balance(pool_id: pallet_nomination_pools::PoolId, points: Balance) -> Balance {
			NominationPools::api_points_to_balance(pool_id, points)
		}

		fn balance_to_points(pool_id: pallet_nomination_pools::PoolId, new_funds: Balance) -> Balance {
			NominationPools::api_balance_to_points(pool_id, new_funds)
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

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};
			use frame_support::traits::WhitelistedStorageKeys;

			let whitelist: Vec<TrackedStorageKey> = AllPalletsWithSystem::whitelisted_storage_keys();

			let params = (&config, &whitelist);
			let mut batches = Vec::<BenchmarkBatch>::new();
			add_benchmarks!(params, batches);

			Ok(batches)
		}
	 }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn state_version_must_be_zero() {
		assert_eq!(0, VERSION.state_version);
	}
}
