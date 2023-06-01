use frame_support::parameter_types;
pub use primitives::AuthorityId as SelendraId;
use primitives::{
	AccountId, Balance, DEFAULT_BAN_REASON_LENGTH, DEFAULT_MAX_WINNERS, DEFAULT_SESSION_PERIOD,
};
use selendra_runtime_common::prod_or_fast;
use sp_runtime::traits::OpaqueKeys;

use crate::{
	Aura, CommitteeManagement, Elections, Runtime, RuntimeEvent, Selendra, Session,
	SessionAndEraManager, SessionKeys, Staking,
};

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CommitteeManagement,);
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

parameter_types! {
	pub SessionPeriod: u32 = prod_or_fast!(DEFAULT_SESSION_PERIOD, 96);
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
	type FinalityCommitteeManager = Selendra;
	type SessionPeriod = SessionPeriod;
}

impl pallet_selendra::Config for Runtime {
	type AuthorityId = SelendraId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInfoProvider = Session;
	type SessionManager = SessionAndEraManager<
		Staking,
		Elections,
		pallet_session::historical::NoteHistoricalRoot<Runtime, Staking>,
		Runtime,
	>;
	type NextSessionAuthorityProvider = Session;
}
