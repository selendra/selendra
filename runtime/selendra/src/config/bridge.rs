use crate::{parameter_types, Balances, Call, Event, Runtime, Treasury, SEL};
use runtime_common::{dollar, EnsureRootOrThreeFourthsFinancialCouncil};

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
	// pub NativeTokenId: module_bridge::ResourceId = module_bridge::derive_resource_id(1, &sp_io::hashing::blake2_128(b"xSEL"));
	pub const NativeTokenResourceId: [u8; 32] = hex_literal::hex!("00000000000000000000001BA648d8F62fD2eEd57CECE710f4ba2d6351f38E04");
	pub NativeTokenTransferFee: u128 = 10 * dollar(SEL);
}

impl module_bridge_transfer::Config for Runtime {
	type Event = Event;
	type BridgeOrigin = module_bridge::EnsureBridge<Runtime>;
	type Currency = Balances;
	type NativeTokenResourceId = NativeTokenResourceId;
	type OnFeePay = Treasury;
}
