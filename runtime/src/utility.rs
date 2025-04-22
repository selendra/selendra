// impl pallet_utility::Config for Runtime {
// 	type RuntimeEvent = RuntimeEvent;
// 	type RuntimeCall = RuntimeCall;
// 	type PalletsOrigin = OriginCaller;
// 	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
// }
use crate::{Balances, OriginCaller, Runtime, RuntimeCall, RuntimeEvent};

use pallet_identity::legacy::IdentityInfo;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};

use frame_support::{parameter_types, traits::InstanceFilter};
use frame_system::EnsureRoot;

use sp_core::{ConstU32, RuntimeDebug};
use sp_runtime::traits::{BlakeTwo256, Verify};

use selendra_primitives::{
	currency::{MILLI_SEL, TOKEN},
	time::DAYS,
	AccountId, AccountIndex, Balance, Signature,
};

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
	type PalletsOrigin = OriginCaller;
}

parameter_types! {
	pub const IndexDeposit: Balance = 10 * TOKEN;
}

impl pallet_indices::Config for Runtime {
	type AccountIndex = AccountIndex;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_indices::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	// One storage item; key size is 32+32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = 120 * MILLI_SEL;
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = 32 * MILLI_SEL;
	pub const MaxSignatories: u32 = 100;
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

parameter_types! {
	pub const ConfigDepositBase: Balance = 500 * MILLI_SEL;
	pub const FriendDepositFactor: Balance = 50 * MILLI_SEL;
	pub const MaxFriends: u16 = 9;
	pub const RecoveryDeposit: Balance = 500 * MILLI_SEL;
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}

parameter_types! {
	pub const BasicDeposit: Balance = 258 * MILLI_SEL;
	pub const ByteDeposit: Balance = 66 * MILLI_SEL;
	pub const SubAccountDeposit: Balance = 53 * MILLI_SEL;
	pub const MaxSubAccounts: u32 = 100;
	pub const MaxAdditionalFields: u32 = 100;
	pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type ByteDeposit = ByteDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = (); //Treasury;
	type ForceOrigin = EnsureRoot<AccountId>;
	type RegistrarOrigin = EnsureRoot<AccountId>;
	type OffchainSignature = Signature;
	type SigningPublicKey = <Signature as Verify>::Signer;
	type UsernameAuthorityOrigin = EnsureRoot<AccountId>;
	type PendingUsernameExpiration = ConstU32<{ 7 * DAYS }>;
	type MaxSuffixLength = ConstU32<7>;
	type MaxUsernameLength = ConstU32<32>;
	type WeightInfo = pallet_identity::weights::SubstrateWeight<Self>;
	type IdentityInformation = IdentityInfo<MaxAdditionalFields>;
}

parameter_types! {
	// Key size = 32, value size = 8
	pub const ProxyDepositBase: Balance = 40 * MILLI_SEL;
	// One storage item (32) plus `ProxyType` (1) encode len.
	pub const ProxyDepositFactor: Balance = 33 * MILLI_SEL;
	// Key size = 32, value size 8
	pub const AnnouncementDepositBase: Balance =  40 * MILLI_SEL;
	// AccountId, Hash and BlockNumber sum up to 68
	pub const AnnouncementDepositFactor: Balance =  68 * MILLI_SEL;
}
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any = 0,
	NonTransfer = 1,
	Staking = 2,
	Nomination = 3,
}
impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<RuntimeCall> for ProxyType {
	fn filter(&self, c: &RuntimeCall) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => matches!(
				c,
				RuntimeCall::Staking(..)
					| RuntimeCall::Session(..)
					| RuntimeCall::Utility(..)
					| RuntimeCall::Multisig(..)
					| RuntimeCall::Recovery(pallet_recovery::Call::as_recovered { .. })
					| RuntimeCall::Recovery(pallet_recovery::Call::vouch_recovery { .. })
					| RuntimeCall::Recovery(pallet_recovery::Call::claim_recovery { .. })
					| RuntimeCall::Recovery(pallet_recovery::Call::close_recovery { .. })
					| RuntimeCall::Recovery(pallet_recovery::Call::remove_recovery { .. })
					| RuntimeCall::Recovery(pallet_recovery::Call::cancel_recovered { .. })
			),
			ProxyType::Staking => {
				matches!(
					c,
					RuntimeCall::Staking(..) | RuntimeCall::Session(..) | RuntimeCall::Utility(..)
				)
			},
			ProxyType::Nomination => {
				matches!(c, RuntimeCall::Staking(pallet_staking::Call::nominate { .. }))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		// ProxyType::Nomination ⊆ ProxyType::Staking ⊆ ProxyType::NonTransfer ⊆ ProxyType::Any
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => match o {
				ProxyType::Any => false,
				ProxyType::NonTransfer | ProxyType::Staking | ProxyType::Nomination => true,
			},
			ProxyType::Staking => match o {
				ProxyType::Any | ProxyType::NonTransfer => false,
				ProxyType::Staking | ProxyType::Nomination => true,
			},
			ProxyType::Nomination => match o {
				ProxyType::Any | ProxyType::NonTransfer | ProxyType::Staking => false,
				ProxyType::Nomination => true,
			},
		}
	}
}

impl pallet_proxy::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = pallet_proxy::weights::SubstrateWeight<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}
