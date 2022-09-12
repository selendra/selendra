use crate::{parameter_types, Call, ChainBridgePalletId, Event, Runtime};
use runtime_common::EnsureRootOrThreeFourthsFinancialCouncil;

parameter_types! {
	pub const ChainId: module_bridge::ChainId = 1;
	pub const ProposalLifetime: u32 = 500;
	pub const RelayerVoteThreshold: u32 = module_bridge::constants::DEFAULT_RELAYER_VOTE_THRESHOLD;
}

impl module_bridge::Config for Runtime {
	type Event = Event;
	type AdminOrigin = EnsureRootOrThreeFourthsFinancialCouncil;
	type Proposal = Call;
	type ChainId = ChainId;
	type PalletId = ChainBridgePalletId;
	type ProposalLifetime = ProposalLifetime;
	type RelayerVoteThreshold = RelayerVoteThreshold;
	type WeightInfo = ();
}
