use crate::{
	Balances, OriginCaller, Runtime, RuntimeCall, RuntimeEvent, Treasury, MICRO_CENT, MILLI_CENT,
};

use sp_runtime::traits::ConvertInto;

use frame_support::{parameter_types, traits::WithdrawReasons};
use frame_system::EnsureRoot;

use selendra_primitives::{AccountId, Balance};

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
	type PalletsOrigin = OriginCaller;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = MICRO_CENT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons = WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
	// bytes count taken from:
	pub const BasicDeposit: Balance = 258 * MILLI_CENT;
	pub const FieldDeposit: Balance = 66 * MILLI_CENT;
	pub const SubAccountDeposit: Balance = 53 * MILLI_CENT;
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRoot<AccountId>;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Self>;
}

parameter_types! {
	// One storage item; key size is 32+32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = 120 * MILLI_CENT;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 32 * MILLI_CENT;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;
}
