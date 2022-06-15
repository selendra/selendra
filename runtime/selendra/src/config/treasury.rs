use crate::{
	Balances, Bounties, ChildBounties, CouncilCollective, Event, PhragmenElection, Runtime,
	Treasury,
};
use frame_support::{parameter_types, traits::EnsureOneOf, PalletId};
use frame_system::EnsureRoot;
use sp_runtime::{Percent, Permill};

use selendra_primitives::{AccountId, Balance, BlockNumber};
use selendra_runtime_constants::{
	currency::{CENTS, DOLLARS},
	time::DAYS,
};

type MoreThanHalfCouncil = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

type ApproveOrigin = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
>;

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const Burn: Permill = Permill::from_percent(10);
	pub const ProposalBondMinimum: Balance = 100 * DOLLARS;
	pub const ProposalBondMaximum: Balance = 500 * DOLLARS;
	pub const DataDepositPerByte: Balance = 1 * CENTS;
	pub const SpendPeriod: BlockNumber = 21 * DAYS;
	pub const MaxApprovals: u32 = 100;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");

}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = ApproveOrigin;
	type RejectOrigin = MoreThanHalfCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	type MaxApprovals = MaxApprovals;
}

parameter_types! {
	pub const BountyDepositBase: Balance = 1 * DOLLARS;
	pub const BountyDepositPayoutDelay: BlockNumber = 8 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 90 * DAYS;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 10 * DOLLARS;
	pub const CuratorDepositMax: Balance = 200 * DOLLARS;
	pub const BountyValueMinimum: Balance = 10 * DOLLARS;
	pub const MaximumReasonLength: u32 = 16384;
}

impl pallet_bounties::Config for Runtime {
	type Event = Event;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
	type ChildBountyManager = ChildBounties;
}

parameter_types! {
	pub const ChildBountyValueMinimum: Balance = BountyValueMinimum::get() / 10;
	pub const MaxActiveChildBountyCount: u32 = 50;
}

impl pallet_child_bounties::Config for Runtime {
	type Event = Event;
	type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
	type ChildBountyValueMinimum = ChildBountyValueMinimum;
	type WeightInfo = pallet_child_bounties::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const TipCountdown: BlockNumber = 1 * DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(5);
	pub const TipReportDepositBase: Balance = 1 * DOLLARS;
}

impl pallet_tips::Config for Runtime {
	type Event = Event;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = PhragmenElection;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type WeightInfo = pallet_tips::weights::SubstrateWeight<Runtime>;
}
