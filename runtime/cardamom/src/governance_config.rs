use super::{
	authority::AuthorityConfigImpl, deposit, dollar, prod_or_fast, weights, AuthoritysOriginId,
	Balances, Call, Event, Council, Origin, OriginCaller,
	PhragmenElectionPalletId, PreimageByteDeposit, Runtime, Scheduler, SelendraOracle,
	TechnicalCommittee, Treasury, DAYS, HOURS, MINUTES, SEL,
};
use frame_support::parameter_types;
use frame_system::EnsureRoot;

use primitives::{AccountId, Balance, BlockNumber};
use runtime_common::{
	CurrencyToVote, EnsureRootOrAllCouncil, EnsureRootOrAllTechnicalCommittee,
	EnsureRootOrHalfCouncil, EnsureRootOrThreeFourthsCouncil,
	EnsureRootOrTwoThirdsCouncil, EnsureRootOrTwoThirdsTechnicalCommittee,
	CouncilInstance,
	CouncilMembershipInstance, OperatorMembershipInstanceSelendra,
	TechnicalCommitteeInstance, TechnicalMembershipInstance,
};

impl orml_authority::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type Scheduler = Scheduler;
	type AsOriginId = AuthoritysOriginId;
	type AuthorityConfig = AuthorityConfigImpl;
	type WeightInfo = weights::orml_authority::WeightInfo<Runtime>;
}

parameter_types! {
	pub LaunchPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1, "SEL_LAUNCH_PERIOD");
	pub VotingPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1 * MINUTES, "SEL_VOTING_PERIOD");
	pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(3 * HOURS, 1 * MINUTES, "SEL_FAST_TRACK_VOTING_PERIOD");
	pub EnactmentPeriod: BlockNumber = prod_or_fast!(28 * DAYS, 1, "SEL_ENACTMENT_PERIOD");
	pub CooloffPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "SEL_COOLOFF_PERIOD");
	pub MinimumDeposit: Balance = 200 * dollar(SEL);
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

impl pallet_democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod;
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = EnsureRootOrHalfCouncil;
	/// A majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin = EnsureRootOrHalfCouncil;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin = EnsureRootOrAllCouncil;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type InstantOrigin = EnsureRootOrAllTechnicalCommittee;
	type InstantAllowed = InstantAllowed;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = EnsureRootOrTwoThirdsCouncil;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EnsureRootOrAllTechnicalCommittee;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cooloff period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCommitteeInstance>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = PreimageByteDeposit;
	type OperationalPreimageOrigin =
		pallet_collective::EnsureMember<AccountId, CouncilInstance>;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = MaxVotes;
	type MaxProposals = MaxProposals;
	type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub CouncilMotionDuration: BlockNumber = prod_or_fast!(7 * DAYS, 2 * MINUTES, "SEL_MOTION_DURATION");
	pub const CouncilDefaultMaxProposals: u32 = 20;
	pub const CouncilDefaultMaxMembers: u32 = 30;
}

impl pallet_collective::Config<CouncilInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilDefaultMaxProposals;
	type MaxMembers = CouncilDefaultMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

impl pallet_membership::Config<CouncilMembershipInstance> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrThreeFourthsCouncil;
	type RemoveOrigin = EnsureRootOrThreeFourthsCouncil;
	type SwapOrigin = EnsureRootOrThreeFourthsCouncil;
	type ResetOrigin = EnsureRootOrThreeFourthsCouncil;
	type PrimeOrigin = EnsureRootOrThreeFourthsCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilDefaultMaxMembers;
	type WeightInfo = ();
}

parameter_types! {
	pub TechnicalCommitteeMotionDuration: BlockNumber = prod_or_fast!(7 * DAYS, 2 * MINUTES, "SEL_MOTION_DURATION");
}

impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = TechnicalCommitteeMotionDuration;
	type MaxProposals = CouncilDefaultMaxProposals;
	type MaxMembers = CouncilDefaultMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

impl pallet_membership::Config<TechnicalMembershipInstance> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrTwoThirdsCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = CouncilDefaultMaxMembers;
	type WeightInfo = ();
}

parameter_types! {
	pub const OperatorDefaultMembers: u32 = 50;
}

impl pallet_membership::Config<OperatorMembershipInstanceSelendra> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrTwoThirdsCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
	type MembershipInitialized = ();
	type MembershipChanged = SelendraOracle;
	type MaxMembers = OperatorDefaultMembers;
	type WeightInfo = ();
}

parameter_types! {
	/// Weekly council elections; scaling up to monthly eventually.
	pub TermDuration: BlockNumber = prod_or_fast!(7 * DAYS, 2 * MINUTES, "SEL_TERM_DURATION");
	pub CandidacyBond: Balance = 100 * dollar(SEL);
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub VotingBondBase: Balance = deposit(1, 64);
	// additional data per vote is 32 bytes (account id).
	pub VotingBondFactor: Balance = deposit(0, 32);
	/// 13 members initially, to be increased to 23 eventually.
	pub const DesiredMembers: u32 = 10;
	pub const DesiredRunnersUp: u32 = 15;
}

impl pallet_elections_phragmen::Config for Runtime {
	type Event = Event;
	type PalletId = PhragmenElectionPalletId;
	type Currency = Balances;
	type ChangeMembers = Council;
	// NOTE: this implies that council's genesis members cannot be set directly and must come from
	// this module.
	type InitializeMembers = Council;
	type CurrencyToVote = CurrencyToVote;
	type CandidacyBond = CandidacyBond;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;
	type LoserCandidate = ();
	type KickedMember = ();
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration;
	type WeightInfo = pallet_elections_phragmen::weights::SubstrateWeight<Runtime>;
}

impl pallet_sudo::Config for Runtime {
	type Event = Event;
	type Call = Call;
}
