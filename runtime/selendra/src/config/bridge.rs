use crate::{parameter_types, Balances, Call, Event, Runtime, Treasury};
use runtime_common::EnsureRootOrThreeFourthsFinancialCouncil;

parameter_types! {
	pub const ChainId: u8 = 1;
	pub const ProposalLifetime: u32 = 500;
}

impl module_bridge::Config for Runtime {
	type Event = Event;
	type BridgeCommitteeOrigin = EnsureRootOrThreeFourthsFinancialCouncil;
	type Proposal = Call;
	type ChainId = ChainId;
	type ProposalLifetime = ProposalLifetime;
	type WeightInfo = ();
}

parameter_types! {
	pub NativeTokenResourceId: module_bridge::ResourceId = module_bridge::derive_resource_id(1, &sp_io::hashing::blake2_128(b"bSEL"));
}

impl module_bridge_transfer::Config for Runtime {
	type Event = Event;
	type BridgeOrigin = module_bridge::EnsureBridge<Runtime>;
	type Currency = Balances;
	type NativeTokenResourceId = NativeTokenResourceId;
	type OnFeeHandler = Treasury;
	type WeightInfo = ();
}
