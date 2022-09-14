use crate::{parameter_types, Call, ChainBridgePalletId, Event, Runtime, SEL, BridgePalletId, Treasury, Balances};
use runtime_common::{dollar, EnsureRootOrThreeFourthsFinancialCouncil, EnsureRootOrTwoThirdsCouncil};

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

parameter_types! {
	// pub NativeTokenId: module_bridge::ResourceId = module_bridge::derive_resource_id(1, &sp_io::hashing::blake2_128(b"xSEL"));
	pub const NativeTokenId: [u8; 32] = hex_literal::hex!("00000000000000000000000000000063a7e2be78898ba83824b0c0cc8dfb6001");
	pub NativeTokenTransferFee: u128 = 10 * dollar(SEL);
}

impl module_bridge_transfer::Config for Runtime {
	type BridgePalletId = BridgePalletId;
	type BridgeOrigin = module_bridge::EnsureBridge<Runtime>;
	type AdminOrigin = EnsureRootOrTwoThirdsCouncil;
	type Currency = Balances;
	type Event = Event;
	type NativeTokenId = NativeTokenId;
	type NativeTokenTransferFee = NativeTokenTransferFee;
	type OnTransactionFee = Treasury;
	type WeightInfo = ();
}

