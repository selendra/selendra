use crate::{AlephId, AuraId, Elections, Runtime, RuntimeEvent, Session, Staking, Balance};

pub use frame_support::{parameter_types, traits::EstimateNextSessionRotation};
use sp_core::ConstBool;

use pallet_committee_management::SessionAndEraManager;

use selendra_primitives::{
	BlockNumber, SessionIndex, SessionInfoProvider, SCORE_SUBMISSION_PERIOD,
};

parameter_types! {
	pub const MaxAuthorities: u32 = 100;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type MaxAuthorities = MaxAuthorities;
	type DisabledValidators = ();
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
}

pub struct SessionInfoImpl;
impl SessionInfoProvider<BlockNumber> for SessionInfoImpl {
	fn current_session() -> SessionIndex {
		pallet_session::CurrentIndex::<Runtime>::get()
	}
	fn next_session_block_number(current_block: BlockNumber) -> Option<BlockNumber> {
		<Runtime as pallet_session::Config>::NextSessionRotation::estimate_next_session_rotation(
			current_block,
		)
		.0
	}
}

pub struct TotalIssuanceProvider;
impl TotalIssuanceProviderT for TotalIssuanceProvider {
	fn get() -> Balance {
		pallet_balances::Pallet::<Runtime>::total_issuance()
	}
}

parameter_types! {
	pub const ScoreSubmissionPeriod: u32 = SCORE_SUBMISSION_PERIOD;
}

impl pallet_aleph::Config for Runtime {
	type AuthorityId = AlephId;
	type RuntimeEvent = RuntimeEvent;
	type SessionInfoProvider = SessionInfoImpl;
	type SessionManager = SessionAndEraManager<
		Staking,
		Elections,
		pallet_session::historical::NoteHistoricalRoot<Runtime, Staking>,
		Runtime,
	>;
	type NextSessionAuthorityProvider = Session;
	type TotalIssuanceProvider = TotalIssuanceProvider;
	type ScoreSubmissionPeriod = ScoreSubmissionPeriod;
}
