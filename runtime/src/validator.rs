use crate::{
	Aleph, Aura, Balances, CommitteeManagement, Elections, Runtime, RuntimeEvent, Session,
	SessionKeys, Staking, Timestamp,
};

use sp_core::ConstU32;
use sp_runtime::Perbill;
use sp_staking::{currency_to_vote::U128CurrencyToVote, EraIndex};

pub use frame_support::{parameter_types, weights::Weight};
use frame_system::EnsureRoot;

use selendra_primitives::{
	staking::MAX_NOMINATORS_REWARDED_PER_VALIDATOR, wrap_methods, AccountId, Balance, BlockNumber,
	DEFAULT_BAN_REASON_LENGTH, DEFAULT_MAX_WINNERS, DEFAULT_SESSIONS_PER_ERA,
	DEFAULT_SESSION_PERIOD,
};

parameter_types! {
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionPeriod, Offset>;
	type SessionManager = Aleph;
	type SessionHandler = (Aura, Aleph);
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 0;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CommitteeManagement,);
}

parameter_types! {
    pub const BondingDuration: EraIndex = 14;
    pub const SlashDeferDuration: EraIndex = 13;
    // this is coupled with weights for payout_stakers() call
    // see custom implementation of WeightInfo below
    pub const MaxExposurePageSize: u32 = MAX_NOMINATORS_REWARDED_PER_VALIDATOR;
    pub const OffendingValidatorsThreshold: Perbill = Perbill::from_percent(33);
    pub const SessionsPerEra: EraIndex = DEFAULT_SESSIONS_PER_ERA;
    pub HistoryDepth: u32 = 84;
}

pub struct ExponentialEraPayout;

impl ExponentialEraPayout {
    fn era_payout(total_issuance: Balance, era_duration_millis: u64) -> (Balance, Balance) {
        const VALIDATOR_REWARD: Perbill = Perbill::from_percent(90);

        let azero_cap = pallet_aleph::AzeroCap::<Runtime>::get();
        let horizon = pallet_aleph::ExponentialInflationHorizon::<Runtime>::get();

        let total_payout: Balance =
            exp_helper(Perbill::from_rational(era_duration_millis, horizon))
                * (azero_cap.saturating_sub(total_issuance));
        let validators_payout = VALIDATOR_REWARD * total_payout;
        let rest = total_payout - validators_payout;

        (validators_payout, rest)
    }
}

/// Calculates 1 - exp(-x) for small positive x
fn exp_helper(x: Perbill) -> Perbill {
    let x2 = x * x;
    let x3 = x2 * x;
    let x4 = x2 * x2;
    let x5 = x4 * x;
    (x - x2 / 2 + x3 / 6 - x4 / 24 + x5 / 120).min(x)
}

impl pallet_staking::EraPayout<Balance> for ExponentialEraPayout {
    fn era_payout(
        _: Balance,
        total_issuance: Balance,
        era_duration_millis: u64,
    ) -> (Balance, Balance) {
        ExponentialEraPayout::era_payout(total_issuance, era_duration_millis)
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
        (
            withdraw_unbonded_update(s: u32),
            SubstrateStakingWeights,
            Weight
        ),
        (
            withdraw_unbonded_kill(s: u32),
            SubstrateStakingWeights,
            Weight
        ),
        (validate(), SubstrateStakingWeights, Weight),
        (kick(k: u32), SubstrateStakingWeights, Weight),
        (nominate(n: u32), SubstrateStakingWeights, Weight),
        (chill(), SubstrateStakingWeights, Weight),
        (set_payee(), SubstrateStakingWeights, Weight),
        (update_payee(), SubstrateStakingWeights, Weight),
        (set_controller(), SubstrateStakingWeights, Weight),
        (set_validator_count(), SubstrateStakingWeights, Weight),
        (force_no_eras(), SubstrateStakingWeights, Weight),
        (force_new_era(), SubstrateStakingWeights, Weight),
        (force_new_era_always(), SubstrateStakingWeights, Weight),
        (set_invulnerables(v: u32), SubstrateStakingWeights, Weight),
        (deprecate_controller_batch(i: u32), SubstrateStakingWeights, Weight),
        (force_unstake(s: u32), SubstrateStakingWeights, Weight),
        (
            cancel_deferred_slash(s: u32),
            SubstrateStakingWeights,
            Weight
        ),
        (rebond(l: u32), SubstrateStakingWeights, Weight),
        (reap_stash(s: u32), SubstrateStakingWeights, Weight),
        (new_era(v: u32, n: u32), SubstrateStakingWeights, Weight),
        (
            get_npos_voters(v: u32, n: u32),
            SubstrateStakingWeights,
            Weight
        ),
        (get_npos_targets(v: u32), SubstrateStakingWeights, Weight),
        (chill_other(), SubstrateStakingWeights, Weight),
        (
            set_staking_configs_all_set(),
            SubstrateStakingWeights,
            Weight
        ),
        (
            set_staking_configs_all_remove(),
            SubstrateStakingWeights,
            Weight
        ),
        (
            force_apply_min_commission(),
            SubstrateStakingWeights,
            Weight
        ),
        (set_min_commission(), SubstrateStakingWeights, Weight)
    );
}

pub struct StakingBenchmarkingConfig;

impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
    type MaxValidators = ConstU32<1000>;
    type MaxNominators = ConstU32<1000>;
}

const MAX_NOMINATORS: u32 = 1;

impl pallet_staking::Config for Runtime {
    // Do not change this!!! It guarantees that we have DPoS instead of NPoS.
    type Currency = Balances;
    type UnixTime = Timestamp;
    type CurrencyToVote = U128CurrencyToVote;
    type ElectionProvider = Elections;
    type GenesisElectionProvider = Elections;
    type NominationsQuota = pallet_staking::FixedNominationsQuota<MAX_NOMINATORS>;
    type RewardRemainder = (); //Treasury
    type RuntimeEvent = RuntimeEvent;
    type Slash = (); //Treasury
    type Reward = ();
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    type SessionInterface = Self;
    type EraPayout = ExponentialEraPayout;
    type NextNewSession = Session;
    type MaxExposurePageSize = MaxExposurePageSize;
    type OffendingValidatorsThreshold = OffendingValidatorsThreshold;
    type VoterList = pallet_staking::UseNominatorsAndValidatorsMap<Runtime>;
    type MaxUnlockingChunks = ConstU32<16>;
    type MaxControllersInDeprecationBatch = ConstU32<4084>;
    type BenchmarkingConfig = StakingBenchmarkingConfig;
    type WeightInfo = PayoutStakersDecreasedWeightInfo;
    type CurrencyBalance = Balance;
    type HistoryDepth = HistoryDepth;
    type TargetList = pallet_staking::UseValidatorsMap<Self>;
    type AdminOrigin = EnsureRoot<AccountId>;
    type EventListeners = ();
}

parameter_types! {
	pub const SessionPeriod: u32 = DEFAULT_SESSION_PERIOD;
	pub const MaximumBanReasonLength: u32 = DEFAULT_BAN_REASON_LENGTH;
	pub const MaxWinners: u32 = DEFAULT_MAX_WINNERS;
}

impl pallet_elections::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type DataProvider = Staking;
	type ValidatorProvider = Staking;
	type MaxWinners = MaxWinners;
	type BannedValidators = CommitteeManagement;
}

impl pallet_committee_management::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BanHandler = Elections;
	type EraInfoProvider = Staking;
	type ValidatorProvider = Elections;
	type ValidatorRewardsHandler = Staking;
	type ValidatorExtractor = Staking;
	type FinalityCommitteeManager = Aleph;
	type SessionPeriod = SessionPeriod;
}
