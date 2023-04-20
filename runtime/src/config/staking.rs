// Copyright 2023 Selendra by SmallWorld Cambodia
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

use crate::{
	Balances, Elections, NominationPools, Runtime, RuntimeEvent, Session, Timestamp, Treasury,
};

use sp_runtime::{FixedU128, Perbill};
use sp_staking::EraIndex;

use frame_support::{
	pallet_prelude::Weight,
	parameter_types,
	traits::{ConstU32, U128CurrencyToVote},
	PalletId,
};

use selendra_primitives::{
	AccountId, Balance, DEFAULT_SESSIONS_PER_ERA, MAX_NOMINATORS_REWARDED_PER_VALIDATOR,
};
use selendra_runtime_common::{staking::era_payout, wrap_methods, BalanceToU256, U256ToBalance, prod_or_fast};

parameter_types! {
	pub const BondingDuration: EraIndex = 14;
	pub const SlashDeferDuration: EraIndex = 13;
	// this is coupled with weights for payout_stakers() call
	// see custom implementation of WeightInfo below
	pub const MaxNominatorRewardedPerValidator: u32 = MAX_NOMINATORS_REWARDED_PER_VALIDATOR;
	pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(33);
	pub SessionsPerEra: EraIndex = prod_or_fast!(DEFAULT_SESSIONS_PER_ERA, 3);
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
