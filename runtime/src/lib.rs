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
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod constants;

#[cfg(feature = "try-runtime")]
use frame_try_runtime::UpgradeCheckSelect;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::{sr25519::AuthorityId as AuraId, SlotDuration};
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdLookup, BlakeTwo256, Block as BlockT, Bounded, ConvertInto, One, OpaqueKeys,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, FixedU128, Perquintill,
};
pub use sp_runtime::{FixedPointNumber, Perbill, Permill};
use sp_staking::EraIndex;
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime, parameter_types,
	traits::{
		ConstBool, ConstU32, EqualPrivilegeOnly, Nothing, SortedMembers,
		U128CurrencyToVote, WithdrawReasons,
	},
	weights::{constants::RocksDbWeight, IdentityFee, Weight},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSignedBy};

pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};

use selendra_primitives::{
	opaque, ApiError as IndraApiError, AuthorityId as IndraId, SessionAuthorityData,
	Version as FinalityVersion, DEFAULT_BAN_REASON_LENGTH, DEFAULT_MAX_WINNERS,
	DEFAULT_SESSIONS_PER_ERA, DEFAULT_SESSION_PERIOD, TOKEN,
};
pub use selendra_primitives::{
	AccountId, AccountIndex, Balance, BlockNumber, Hash, Index, Signature,
};
use selendra_runtime_common::{
	impls::DealWithFees,
	staking::{era_payout, MAX_NOMINATORS_REWARDED_PER_VALIDATOR},
	wrap_methods, BlockLength, BlockWeights,
};

use constants::{
	currency::*, time::*, CONTRACTS_DEBUG_OUTPUT, CONTRACT_DEPOSIT_PER_BYTE,
	LEGACY_DEPOSIT_PER_BYTE,
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
	pub const SS58Prefix: u8 = 204;
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

parameter_types! {
	pub const ExistentialDeposit: u128 = 500 * MILLI_CENT;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
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
	// This value increases the priority of `Operational` transactions by adding
	// a "virtual tip" that's equal to the `OperationalFeeMultiplier * final_fee`.
	// follows polkadot : https://github.com/paritytech/polkadot/blob/9ce5f7ef5abb1a4291454e8c9911b304d80679f9/runtime/polkadot/src/lib.rs#L369
	pub const OperationalFeeMultiplier: u8 = 5;
	// We expect that on average 25% of the normal capacity will be occupied with normal txs.
	pub const TargetSaturationLevel: Perquintill = Perquintill::from_percent(25);
	// During 20 blocks the fee may not change more than by 100%. This, together with the
	// `TargetSaturationLevel` value, results in variability ~0.067. For the corresponding
	// formulas please refer to Substrate code at `frame/transaction-payment/src/lib.rs`.
	pub FeeVariability: Multiplier = Multiplier::saturating_from_rational(67, 1000);
	// Fee should never be lower than the computational cost.
	pub MinimumMultiplier: Multiplier = Multiplier::one();
	pub MaximumMultiplier: Multiplier = Bounded::max_value();
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type LengthToFee = IdentityFee<Balance>;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = TargetedFeeAdjustment<
		Self,
		TargetSaturationLevel,
		FeeVariability,
		MinimumMultiplier,
		MaximumMultiplier,
	>;
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

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
}

impl pallet_indra::Config for Runtime {
	type AuthorityId = IndraId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInfoProvider = Session;
	type SessionManager = Elections;
	type NextSessionAuthorityProvider = Session;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
		pub indra: Indra,
	}
}

parameter_types! {
	pub const SessionPeriod: u32 = DEFAULT_SESSION_PERIOD;
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

impl pallet_randomness_collective_flip::Config for Runtime {}

parameter_types! {
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type SessionManager = Indra;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}

use sp_runtime::traits::Convert;

pub struct BalanceToU256;

impl Convert<Balance, sp_core::U256> for BalanceToU256 {
	fn convert(balance: Balance) -> sp_core::U256 {
		sp_core::U256::from(balance)
	}
}

pub struct U256ToBalance;

impl Convert<sp_core::U256, Balance> for U256ToBalance {
	fn convert(n: sp_core::U256) -> Balance {
		n.try_into().unwrap_or(Balance::max_value())
	}
}

impl pallet_nomination_pools::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RewardCounter = FixedU128;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type Staking = pallet_staking::Pallet<Self>;
	type PostUnbondingPoolsWindow = PostUnbondPoolsWindow;
	type MaxMetadataLen = ConstU32<256>;
	type MaxUnbonding = ConstU32<8>;
	type PalletId = NominationPoolsPalletId;
	type MaxPointsToBalance = MaxPointsToBalance;
}

parameter_types! {
	pub const BondingDuration: EraIndex = 14;
	pub const SlashDeferDuration: EraIndex = 13;
	// this is coupled with weights for payout_stakers() call
	// see custom implementation of WeightInfo below
	pub const MaxNominatorRewardedPerValidator: u32 = MAX_NOMINATORS_REWARDED_PER_VALIDATOR;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(33);
	pub const SessionsPerEra: EraIndex = DEFAULT_SESSIONS_PER_ERA;
	pub HistoryDepth: u32 = 84;
}

pub struct UniformEraPayout;

impl pallet_staking::EraPayout<Balance> for UniformEraPayout {
	fn era_payout(_: Balance, _: Balance, era_duration_millis: u64) -> (Balance, Balance) {
		era_payout(era_duration_millis)
	}
}

type SubstrateStakingWeights = pallet_staking::weights::SubstrateWeight<Runtime>;

pub struct PayoutStakersDecreasedWeightInfo;

impl pallet_staking::WeightInfo for PayoutStakersDecreasedWeightInfo {
	// To make possible to change nominators per validator we need to decrease weight for payout_stakers
	fn payout_stakers_alive_staked(n: u32) -> Weight {
		SubstrateStakingWeights::payout_stakers_alive_staked(n) / 2
	}
	wrap_methods!(
		(bond(), SubstrateStakingWeights, Weight),
		(bond_extra(), SubstrateStakingWeights, Weight),
		(unbond(), SubstrateStakingWeights, Weight),
		(withdraw_unbonded_update(s: u32), SubstrateStakingWeights, Weight),
		(withdraw_unbonded_kill(s: u32), SubstrateStakingWeights, Weight),
		(validate(), SubstrateStakingWeights, Weight),
		(kick(k: u32), SubstrateStakingWeights, Weight),
		(nominate(n: u32), SubstrateStakingWeights, Weight),
		(chill(), SubstrateStakingWeights, Weight),
		(set_payee(), SubstrateStakingWeights, Weight),
		(set_controller(), SubstrateStakingWeights, Weight),
		(set_validator_count(), SubstrateStakingWeights, Weight),
		(force_no_eras(), SubstrateStakingWeights, Weight),
		(force_new_era(), SubstrateStakingWeights, Weight),
		(force_new_era_always(), SubstrateStakingWeights, Weight),
		(set_invulnerables(v: u32), SubstrateStakingWeights, Weight),
		(force_unstake(s: u32), SubstrateStakingWeights, Weight),
		(cancel_deferred_slash(s: u32), SubstrateStakingWeights, Weight),
		(payout_stakers_dead_controller(n: u32), SubstrateStakingWeights, Weight),
		(rebond(l: u32), SubstrateStakingWeights, Weight),
		(reap_stash(s: u32), SubstrateStakingWeights, Weight),
		(new_era(v: u32, n: u32), SubstrateStakingWeights, Weight),
		(get_npos_voters(v: u32, n: u32), SubstrateStakingWeights, Weight),
		(get_npos_targets(v: u32), SubstrateStakingWeights, Weight),
		(chill_other(), SubstrateStakingWeights, Weight),
		(set_staking_configs_all_set(), SubstrateStakingWeights, Weight),
		(set_staking_configs_all_remove(), SubstrateStakingWeights, Weight),
		(force_apply_min_commission(), SubstrateStakingWeights, Weight),
		(set_min_commission(), SubstrateStakingWeights, Weight)
	);
}

pub struct StakingBenchmarkingConfig;

impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

impl pallet_staking::Config for Runtime {
	// Do not change this!!! It guarantees that we have DPoS instead of NPoS.
	type Currency = Balances;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = Elections;
	type GenesisElectionProvider = Elections;
	type MaxNominations = ConstU32<1>;
	type RewardRemainder = Treasury;
	type RuntimeEvent = RuntimeEvent;
	type Slash = Treasury;
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SessionInterface = Self;
	type EraPayout = UniformEraPayout;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
	type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Runtime>;
	type MaxUnlockingChunks = ConstU32<16>;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type WeightInfo = PayoutStakersDecreasedWeightInfo;
	type CurrencyBalance = Balance;
	type OnStakerSlash = NominationPools;
	type HistoryDepth = HistoryDepth;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type AdminOrigin = EnsureRoot<AccountId>;
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
	pub const MinVestedTransfer: Balance = MICRO_CENT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons = WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	// Maximum number of vesting schedules an account may have at a given moment
	// follows polkadot https://github.com/paritytech/polkadot/blob/9ce5f7ef5abb1a4291454e8c9911b304d80679f9/runtime/polkadot/src/lib.rs#L980
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
	// One storage item; key size is 32+32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = 120 * LEGACY_DEPOSIT_PER_BYTE;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 32 * LEGACY_DEPOSIT_PER_BYTE;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}

#[cfg(not(feature = "enable_treasury_proposals"))]
// This value effectively disables treasury.
pub const TREASURY_PROPOSAL_BOND: Balance = 100_000_000_000 * TOKEN;

#[cfg(feature = "enable_treasury_proposals")]
pub const TREASURY_PROPOSAL_BOND: Balance = 100 * TOKEN;

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

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
	type PalletsOrigin = OriginCaller;
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

parameter_types! {
	// bytes count taken from:
	// https://github.com/paritytech/polkadot/blob/016dc7297101710db0483ab6ef199e244dff711d/runtime/kusama/src/lib.rs#L995
	pub const BasicDeposit: Balance = 258 * LEGACY_DEPOSIT_PER_BYTE;
	pub const FieldDeposit: Balance = 66 * LEGACY_DEPOSIT_PER_BYTE;
	pub const SubAccountDeposit: Balance = 53 * LEGACY_DEPOSIT_PER_BYTE;
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRoot<AccountId>;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Self>;
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
		Indra: pallet_indra,
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

		fn authorities() -> Vec<IndraId> {
			Indra::authorities()
		}

		fn next_session_authorities() -> Result<Vec<IndraId>, IndraApiError> {
			let next_authorities = Indra::next_authorities();
			if next_authorities.is_empty() {
				return Err(IndraApiError::DecodeKey)
			}

			Ok(next_authorities)
		}

		fn authority_data() -> SessionAuthorityData {
			SessionAuthorityData::new(Indra::authorities(), Indra::emergency_finalizer())
		}

		fn next_session_authority_data() -> Result<SessionAuthorityData, IndraApiError> {
			Ok(SessionAuthorityData::new(
				Self::next_session_authorities()?,
				Indra::queued_emergency_finalizer(),
			))
		}

		fn finality_version() -> FinalityVersion {
			Indra::finality_version()
		}

		fn next_session_finality_version() -> FinalityVersion {
			Indra::next_session_finality_version()
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
