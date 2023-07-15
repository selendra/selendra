mod msg_routing;
mod rollup;

use crate::{
	origin::EnsureRootOrHalfCouncil, pallet_base_pool, pallet_computation, pallet_indra,
	pallet_indra_tokenomic, pallet_mq, pallet_registry, pallet_stake_pool, pallet_stake_pool_v2,
	pallet_vault, pallet_wrapped_balances, Balances, IndranetStakePoolv2, RandomnessCollectiveFlip,
	Runtime, RuntimeCall, RuntimeEvent, SecsPerBlock, Timestamp, Treasury, Vec, MILLI_CENT, TOKEN,
};

use codec::{Decode, Encode};
use sp_core::{ConstU32, Get};
use sp_runtime::{traits::TrailingZeroInput, AccountId32};

use frame_support::{parameter_types, traits::SortedMembers};
use frame_system::EnsureSignedBy;

use selendra_primitives::{AccountId, Balance};

pub struct MqCallMatcher;
impl pallet_mq::CallMatcher<Runtime> for MqCallMatcher {
	fn match_call(call: &RuntimeCall) -> Option<&pallet_mq::Call<Runtime>> {
		match call {
			RuntimeCall::IndranetMq(mq_call) => Some(mq_call),
			_ => None,
		}
	}
}

impl pallet_mq::Config for Runtime {
	type QueueNotifyConfig = msg_routing::MessageRouteConfig;
	type CallMatcher = MqCallMatcher;
}

parameter_types! {
	pub const NoneAttestationEnabled: bool = true;
	pub const VerifyPRuntime: bool = false;
	pub const VerifyRelaychainGenesisBlockHash: bool = false;
}

impl pallet_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type LegacyAttestationValidator = pallet_registry::IasValidator;
	type NoneAttestationEnabled = NoneAttestationEnabled;
	type VerifyPRuntime = VerifyPRuntime;
	type VerifyRelaychainGenesisBlockHash = VerifyRelaychainGenesisBlockHash;
	type GovernanceOrigin = EnsureRootOrHalfCouncil;
}

pub struct SetBudgetMembers;

impl SortedMembers<AccountId> for SetBudgetMembers {
	fn sorted_members() -> Vec<AccountId> {
		[pallet_computation::pallet::ContractAccount::<Runtime>::get()].to_vec()
	}
}

parameter_types! {
	pub ExpectedBlockTimeSec: u32 = SecsPerBlock::get() as u32;
	pub const MinInitP: u32 = 50;
}

impl pallet_computation::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ExpectedBlockTimeSec = ExpectedBlockTimeSec;
	type MinInitP = MinInitP;
	type Randomness = RandomnessCollectiveFlip;
	type OnReward = IndranetStakePoolv2;
	type OnUnbound = IndranetStakePoolv2;
	type OnStopped = IndranetStakePoolv2;
	type OnTreasurySettled = Treasury;
	type UpdateTokenomicOrigin = EnsureRootOrHalfCouncil;
	type SetBudgetOrigins = EnsureSignedBy<SetBudgetMembers, AccountId>;
	type SetContractRootOrigins = EnsureRootOrHalfCouncil;
}

parameter_types! {
	pub const MinContribution: Balance = 1 * MILLI_CENT;
	pub const WorkingGracePeriod: u64 = 7 * 24 * 3600;
	pub const MaxPoolWorkers: u32 = 200;
}

impl pallet_stake_pool_v2::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MinContribution = MinContribution;
	type GracePeriod = WorkingGracePeriod;
	type MaxPoolWorkers = MaxPoolWorkers;
}

impl pallet_stake_pool::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

parameter_types! {
	pub const InitialPriceCheckPoint: Balance = 1 * TOKEN;
	pub const VaultQueuePeriod: u64 = 21 * 24 * 3600;
}

impl pallet_vault::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type InitialPriceCheckPoint = InitialPriceCheckPoint;
	type VaultQueuePeriod = VaultQueuePeriod;
}

pub struct WrappedBalancesPalletAccount;

impl Get<AccountId32> for WrappedBalancesPalletAccount {
	fn get() -> AccountId32 {
		(b"wsel/")
			.using_encoded(|b| AccountId32::decode(&mut TrailingZeroInput::new(b)))
			.expect("Decoding zero-padded account id should always succeed; qed")
	}
}

impl pallet_wrapped_balances::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WSelAssetId = ConstU32<10000>;
	type WrappedBalancesAccountId = WrappedBalancesPalletAccount;
	type OnSlashed = Treasury;
}

pub struct MigrationAccount;

impl Get<AccountId32> for MigrationAccount {
	fn get() -> AccountId32 {
		let account: [u8; 32] =
			hex_literal::hex!("9e6399cd577e8ac536bdc017675f747b2d1893ad9cc8c69fd17eef73d4e6e51e");
		account.into()
	}
}

parameter_types! {
	pub const WSelMinBalance: Balance = MILLI_CENT;
}

impl pallet_base_pool::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MigrationAccountId = MigrationAccount;
	type WSelMinBalance = WSelMinBalance;
}

impl pallet_indra_tokenomic::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

impl pallet_indra::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type InkCodeSizeLimit = ConstU32<{ 1024 * 1024 * 2 }>;
	type SidevmCodeSizeLimit = ConstU32<{ 1024 * 1024 * 8 }>;
	type Currency = Balances;
}

impl indranet_pallets::IndranetConfig for Runtime {
	type Currency = Balances;
}
