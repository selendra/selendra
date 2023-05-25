mod msg_routing;
mod offchain_rollup;

use crate::{
	origin::EnsureRootOrHalfCouncil, pallet_base_pool, pallet_computation, pallet_mq,
	pallet_registry, pallet_stake_pool, pallet_stake_pool_v2, pallet_tokenomic, pallet_vault,
	pallet_webc, pallet_wrapped_balances, Balances, PhalaStakePoolv2, RandomnessCollectiveFlip,
	Runtime, RuntimeCall, RuntimeEvent, SecsPerBlock, Timestamp, Treasury, MICRO_CENT, MILLI_CENT,
};

use codec::{Decode, Encode};
use frame_support::{
	pallet_prelude::Get,
	parameter_types,
	traits::{ConstU32, SortedMembers},
};
use frame_system::EnsureSignedBy;
use selendra_primitives::{AccountId, Balance};
use sp_runtime::{traits::TrailingZeroInput, AccountId32};
use sp_std::prelude::*;

pub struct WrappedBalancesPalletAccount;

impl Get<AccountId32> for WrappedBalancesPalletAccount {
	fn get() -> AccountId32 {
		(b"wcon/")
			.using_encoded(|b| AccountId32::decode(&mut TrailingZeroInput::new(b)))
			.expect("Decoding zero-padded account id should always succeed; qed")
	}
}

impl pallet_wrapped_balances::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WPhaAssetId = ConstU32<10000>;
	type WrappedBalancesAccountId = WrappedBalancesPalletAccount;
	type OnSlashed = Treasury;
}

parameter_types! {
	pub const InitialPriceCheckPoint: Balance = 100 * MILLI_CENT;
	pub const VaultQueuePeriod: u64 = 21 * 24 * 3600;
}

impl pallet_vault::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type InitialPriceCheckPoint = InitialPriceCheckPoint;
	type VaultQueuePeriod = VaultQueuePeriod;
}

parameter_types! {
	pub const MinContribution: Balance = 1 * MICRO_CENT;
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
	pub const MinInitP: u32 = 50;
	pub const CheckWorkerRegisterTime: bool = true;
	pub ExpectedBlockTimeSec: u32 = SecsPerBlock::get() as u32;
}

pub struct SetBudgetMembers;
impl SortedMembers<AccountId> for SetBudgetMembers {
	fn sorted_members() -> Vec<AccountId> {
		[pallet_computation::pallet::ContractAccount::<Runtime>::get()].to_vec()
	}
}

impl pallet_computation::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ExpectedBlockTimeSec = ExpectedBlockTimeSec;
	type MinInitP = MinInitP;
	type Randomness = RandomnessCollectiveFlip;
	type OnReward = PhalaStakePoolv2;
	type OnUnbound = PhalaStakePoolv2;
	type OnStopped = PhalaStakePoolv2;
	type OnTreasurySettled = Treasury;
	type UpdateTokenomicOrigin = EnsureRootOrHalfCouncil;
	type SetBudgetOrigins = EnsureSignedBy<SetBudgetMembers, AccountId>;
	type SetContractRootOrigins = EnsureRootOrHalfCouncil;
}

impl pallet_mq::Config for Runtime {
	type QueueNotifyConfig = msg_routing::MessageRouteConfig;
	type CallMatcher = MqCallMatcher;
}

pub struct MqCallMatcher;
impl pallet_mq::CallMatcher<Runtime> for MqCallMatcher {
	fn match_call(call: &RuntimeCall) -> Option<&pallet_mq::Call<Runtime>> {
		match call {
			RuntimeCall::PhalaMq(mq_call) => Some(mq_call),
			_ => None,
		}
	}
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
	pub const WPhaMinBalance: Balance = 5 * MILLI_CENT;
}

impl pallet_base_pool::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MigrationAccountId = MigrationAccount;
	type WPhaMinBalance = WPhaMinBalance;
}

impl pallet_webc::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type InkCodeSizeLimit = ConstU32<{ 1024 * 1024 * 2 }>;
	type SidevmCodeSizeLimit = ConstU32<{ 1024 * 1024 * 8 }>;
	type Currency = Balances;
}

impl pallet_tokenomic::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
}

parameter_types! {
	pub const NoneAttestationEnabled: bool = true;
	pub const VerifyPRuntime: bool = false;
}

impl pallet_registry::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type LegacyAttestationValidator = pallet_registry::IasValidator;
	type NoneAttestationEnabled = NoneAttestationEnabled;
	type VerifyPRuntime = VerifyPRuntime;
	type GovernanceOrigin = EnsureRootOrHalfCouncil;
}

impl pallets_web_contract::WebContractConfig for Runtime {
	type Currency = Balances;
}
