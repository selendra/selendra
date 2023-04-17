use crate::{
	Balances, Elections, NominationPools, Runtime, RuntimeEvent, Session, Timestamp, Treasury,
};

use frame_support::{
	pallet_prelude::Weight,
	parameter_types, PalletId,
	traits::{ConstU32, U128CurrencyToVote},
};
use sp_runtime::{Perbill, FixedU128};
use sp_staking::EraIndex;

use selendra_primitives::{AccountId, Balance, DEFAULT_SESSIONS_PER_ERA};
use selendra_runtime_common::{
	staking::{era_payout, MAX_NOMINATORS_REWARDED_PER_VALIDATOR},
	wrap_methods, BalanceToU256, U256ToBalance
};

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
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
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

parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
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