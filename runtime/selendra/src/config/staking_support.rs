use crate::{voter_bags, Balances, Event, Runtime, Staking};

use frame_election_provider_support::VoteWeight;
use frame_support::{parameter_types, PalletId};
use selendra_primitives::Balance;
use sp_runtime::traits::Convert;

parameter_types! {
	pub const BagThresholds: &'static [u64] = &voter_bags::THRESHOLDS;
}

impl pallet_bags_list::Config for Runtime {
	type Event = Event;
	type ScoreProvider = Staking;
	type WeightInfo = pallet_bags_list::weights::SubstrateWeight<Runtime>;
	type BagThresholds = BagThresholds;
	type Score = VoteWeight;
}

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

parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MinPointsToBalance: u32 = 10;
	pub const MaxMetadataLen: u32 = 256;
}

impl pallet_nomination_pools::Config for Runtime {
	type WeightInfo = ();
	type Event = Event;
	type Currency = Balances;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type StakingInterface = Staking;
	type PostUnbondingPoolsWindow = PostUnbondPoolsWindow;
	type MaxMetadataLen = MaxMetadataLen;
	type MaxUnbonding = <Self as pallet_staking::Config>::MaxUnlockingChunks;
	type PalletId = NominationPoolsPalletId;
	type MinPointsToBalance = MinPointsToBalance;
}
