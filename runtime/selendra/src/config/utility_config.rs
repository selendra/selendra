use crate::{
	cent, config::evm_config::EvmTask, deposit, dollar, parameter_types, weights, AccountIndex,
	Balance, Balances, BlakeTwo256, Call, DispatchableTask, Event, InstanceFilter, OriginCaller,
	ProxyType, Runtime, RuntimeBlockWeights, RuntimeDebug, Treasury, Weight, SEL,
};
use codec::{Decode, Encode};
use primitives::{define_combined_task, task::TaskResult};
use runtime_common::EnsureRootOrThreeFourthsCouncil;
use scale_info::TypeInfo;

impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub DepositFactor: Balance = deposit(0, 32);
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

parameter_types! {
	pub ConfigDepositBase: Balance = 5 * dollar(SEL);
	pub FriendDepositFactor: Balance = 500 * cent(SEL);
	pub RecoveryDeposit: Balance = 10 * dollar(SEL);
	pub const MaxFriends: u16 = 10;
}

impl pallet_recovery::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
	type WeightInfo = ();
}

define_combined_task! {
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum ScheduledTasks {
		EvmTask(EvmTask<Runtime>),
	}
}

parameter_types!(
	// At least 2% of max block weight should remain before idle tasks are dispatched.
	pub MinimumWeightRemainInBlock: Weight = RuntimeBlockWeights::get().max_block / 50;
);

impl module_idle_scheduler::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
}

parameter_types! {
	pub IndexDeposit: Balance = 10 * dollar(SEL);
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Event = Event;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type WeightInfo = weights::pallet_indices::WeightInfo<Runtime>;
}

parameter_types! {
	pub BasicDeposit: Balance = deposit(1, 258);
	pub FieldDeposit: Balance = deposit(0, 66);
	pub SubAccountDeposit: Balance = deposit(1, 53);
	pub MaxSubAccounts: u32 = 100;
	pub MaxAdditionalFields: u32 = 100;
	pub MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type FieldDeposit = FieldDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxAdditionalFields = MaxAdditionalFields;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = EnsureRootOrThreeFourthsCouncil;
	type RegistrarOrigin = EnsureRootOrThreeFourthsCouncil;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub ProxyDepositFactor: Balance = deposit(0, 33);
	pub AnnouncementDepositBase: Balance = deposit(1, 8);
	pub AnnouncementDepositFactor: Balance = deposit(0, 66);
	pub const MaxProxies: u16 = 32;
	pub const MaxPending: u16 = 32;
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			// Always allowed Call::Utility no matter type.
			// Only transactions allowed by Proxy.filter can be executed,
			// otherwise `BadOrigin` will be returned in Call::Utility.
			_ if matches!(c, Call::Utility(..)) => true,
			ProxyType::Any => true,
			ProxyType::CancelProxy =>
				matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. })),
			ProxyType::Governance => {
				matches!(
					c,
					Call::Authority(..) |
						Call::Democracy(..) | Call::PhragmenElection(..) |
						Call::Council(..) | Call::FinancialCouncil(..) |
						Call::TechnicalCommittee(..) |
						Call::Treasury(..) | Call::Bounties(..) |
						Call::Tips(..)
				)
			},
			ProxyType::Staking => {
				matches!(c, Call::Staking(..) | Call::Session(..))
			},
			ProxyType::IdentityJudgement => matches!(
				c,
				Call::Identity(pallet_identity::Call::provide_judgement { .. }) | Call::Utility(..)
			),
			ProxyType::Auction => {
				matches!(c, Call::Auction(orml_auction::Call::bid { .. }))
			},
			ProxyType::Swap => {
				matches!(
					c,
					Call::Dex(module_dex::Call::swap_with_exact_supply { .. }) |
						Call::Dex(module_dex::Call::swap_with_exact_target { .. }) |
						Call::AggregatedDex(
							module_aggregated_dex::Call::swap_with_exact_supply { .. }
						) | Call::AggregatedDex(
						module_aggregated_dex::Call::swap_with_exact_target { .. }
					)
				)
			},
			ProxyType::Loan => {
				matches!(
					c,
					Call::Funan(module_funan::Call::adjust_loan { .. }) |
						Call::Funan(module_funan::Call::close_loan_has_debit_by_dex { .. }) |
						Call::Funan(module_funan::Call::adjust_loan_by_debit_value { .. }) |
						Call::Funan(module_funan::Call::transfer_debit { .. })
				)
			},
			ProxyType::DexLiquidity => {
				matches!(
					c,
					Call::Dex(module_dex::Call::add_liquidity { .. }) |
						Call::Dex(module_dex::Call::remove_liquidity { .. })
				)
			},
			ProxyType::StableAssetSwap => {
				matches!(c, Call::StableAsset(module_stable_asset::Call::swap { .. }))
			},
			ProxyType::StableAssetLiquidity => {
				matches!(
					c,
					Call::StableAsset(module_stable_asset::Call::mint { .. }) |
						Call::StableAsset(module_stable_asset::Call::redeem_proportion { .. }) |
						Call::StableAsset(module_stable_asset::Call::redeem_single { .. }) |
						Call::StableAsset(module_stable_asset::Call::redeem_multi { .. })
				)
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxPending = MaxPending;
	type MaxProxies = MaxProxies;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
	type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
}
