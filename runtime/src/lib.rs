#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::{DecodeLimit, Encode};
use sp_api::impl_runtime_apis;
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, SlotDuration};
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, StaticLookup},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, SaturatedConversion,
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
use pallet_evm::{runner::RunnerExtended, AccessListItem, CallInfo, CreateInfo};
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::CurrencyAdapter;

use selendra_primitives::{
	evm::BlockLimits, opaque, ApiError as SelendrapiError, SessionAuthorityData,
	Version as FinalityVersion, TOKEN, TREASURY_PROPOSAL_BOND,
};
pub use selendra_primitives::{
	evm::EstimateResourcesRequest, unchecked_extrinsic::SelendraUncheckedExtrinsic, AccountId,
	AccountIndex, Balance, BlockNumber, Hash, Index, ReserveIdentifier, Signature,
};
use selendra_runtime_common::{
	evm::EvmLimits, impls::DealWithFees, BlockLength, BlockWeights, SlowAdjustingFeeUpdate,
};

mod config;
pub mod constants;
mod impl_convert;
mod origin;

use config::{SelendraId, SessionPeriod, StorageDepositPerByte, TxFeePerGas};
use constants::{
	currency::*, fee::WeightToFee, time::*, CONTRACTS_DEBUG_OUTPUT, CONTRACT_DEPOSIT_PER_BYTE,
};
use impl_convert::ConvertEthereumTx;

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
	type Lookup = (AccountIdLookup<AccountId, AccountIndex>, EvmAccounts);
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
	type OnKilledAccount =
		(pallet_evm::CallKillAccount<Runtime>, pallet_evm_accounts::CallKillAccount<Runtime>);
	type AccountData = pallet_balances::AccountData<Balance>;
	type SystemWeightInfo = ();
	#[cfg(feature = "runtime-testnet")]
	type SS58Prefix = ConstU16<42>;
	#[cfg(not(feature = "runtime-testnet"))]
	type SS58Prefix = ConstU16<204>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(
		RuntimeCall,
		<UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload,
	)> {
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			selendra_runtime_common::check_nonce::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_evm::SetEvmOrigin::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = AccountIdLookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}
impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	RuntimeCall: From<C>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = UncheckedExtrinsic;
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
	type ReserveIdentifier = ReserveIdentifier;
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

		// Smart Contract
		Contracts: pallet_contracts = 50,

		// Evm
		IdleScheduler: pallet_idle_scheduler = 55,
		EVM: pallet_evm = 56,
		EvmAccounts: pallet_evm_accounts = 57,

		// Utility Suff
		Vesting: pallet_vesting = 90,
		Utility: pallet_utility = 91,
		Multisig: pallet_multisig = 92,
		Identity: pallet_identity = 93,

		// Temporary
		Sudo: pallet_sudo = 100,
	}
);

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<RuntimeCall, SignedExtra>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	selendra_runtime_common::check_nonce::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_evm::SetEvmOrigin<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = SelendraUncheckedExtrinsic<
	RuntimeCall,
	SignedExtra,
	ConvertEthereumTx,
	StorageDepositPerByte,
	TxFeePerGas,
>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

pub type Migrations = ();

/// Executive: handles dispatch to the various pallets.
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

		fn next_session_authorities() -> Result<Vec<SelendraId>, SelendrapiError> {
			let next_authorities = Selendra::next_authorities();
			if next_authorities.is_empty() {
				return Err(SelendrapiError::DecodeKey)
			}

			Ok(next_authorities)
		}

		fn authority_data() -> SessionAuthorityData {
			SessionAuthorityData::new(Selendra::authorities(), Selendra::emergency_finalizer())
		}

		fn next_session_authority_data() -> Result<SessionAuthorityData, SelendrapiError> {
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

	impl pallet_evm_rpc_runtime_api::EVMRuntimeRPCApi<Block, Balance> for Runtime {
		fn block_limits() -> BlockLimits {
			BlockLimits {
				max_gas_limit: EvmLimits::<Runtime>::max_gas_limit(),
				max_storage_limit: EvmLimits::<Runtime>::max_storage_limit(),
			}
		}

		fn call(
			from: H160,
			to: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			_estimate: bool,
		) -> Result<CallInfo, sp_runtime::DispatchError> {
			<Runtime as pallet_evm::Config>::Runner::rpc_call(
				from,
				from,
				to,
				data,
				value,
				gas_limit,
				storage_limit,
				access_list.unwrap_or_default().into_iter().map(|v| (v.address, v.storage_keys)).collect(),
				<Runtime as pallet_evm::Config>::config(),
			)
		}

		fn create(
			from: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			_estimate: bool,
		) -> Result<CreateInfo, sp_runtime::DispatchError> {
			<Runtime as pallet_evm::Config>::Runner::rpc_create(
				from,
				data,
				value,
				gas_limit,
				storage_limit,
				access_list.unwrap_or_default().into_iter().map(|v| (v.address, v.storage_keys)).collect(),
				<Runtime as pallet_evm::Config>::config(),
			)
		}

		fn get_estimate_resources_request(extrinsic: Vec<u8>) -> Result<EstimateResourcesRequest, sp_runtime::DispatchError> {
			let utx = UncheckedExtrinsic::decode_all_with_depth_limit(sp_api::MAX_EXTRINSIC_DEPTH, &mut &*extrinsic)
				.map_err(|_| sp_runtime::DispatchError::Other("Invalid parameter extrinsic, decode failed"))?;

			let request = match utx.0.function {
				RuntimeCall::EVM(pallet_evm::Call::call{target, input, value, gas_limit, storage_limit, access_list}) => {
					Some(EstimateResourcesRequest {
						from: None,
						to: Some(target),
						gas_limit: Some(gas_limit),
						storage_limit: Some(storage_limit),
						value: Some(value),
						data: Some(input),
						access_list: Some(access_list)
					})
				}
				RuntimeCall::EVM(pallet_evm::Call::create{input, value, gas_limit, storage_limit, access_list}) => {
					Some(EstimateResourcesRequest {
						from: None,
						to: None,
						gas_limit: Some(gas_limit),
						storage_limit: Some(storage_limit),
						value: Some(value),
						data: Some(input),
						access_list: Some(access_list)
					})
				}
				_ => None,
			};

			request.ok_or(sp_runtime::DispatchError::Other("Invalid parameter extrinsic, not evm Call"))
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
