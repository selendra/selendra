use crate::{
	deposit, origin::EnsureRootOrFullCouncil, Balances, Runtime, RuntimeEvent, MILLI_CENT, TOKEN,
};

use frame_support::{parameter_types, traits::AsEnsureOriginWithArg};
use selendra_primitives::{AccountId, Balance};
use sp_core::{ConstU128, ConstU32};

parameter_types! {
	pub const CollectionDeposit: Balance = TOKEN; // 1 Token deposit to create collection
	pub const ItemDeposit: Balance = 10 * MILLI_CENT; // 1/100 Token deposit to create item
	pub const StringLimit: u32 = 52100;
	pub const KeyLimit: u32 = 32000; // Max 32 bytes per key
	pub const ValueLimit: u32 = 512000; // Max 64 bytes per value
	pub const UniqueMetadataDepositBase: Balance = 5 * TOKEN;
}

impl pallet_uniques::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRootOrFullCouncil;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Locker = pallet_rmrk_core::Pallet<Runtime>;
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = UniqueMetadataDepositBase;
	type AttributeDepositBase = UniqueMetadataDepositBase;
	type DepositPerByte = UniqueMetadataDepositBase;
	type StringLimit = StringLimit;
	type KeyLimit = KeyLimit;
	type ValueLimit = ValueLimit;
	type WeightInfo = ();
}

parameter_types! {
	pub const AssetDeposit: Balance = 10 * MILLI_CENT; // 1 MILLI_CENT deposit to create asset
	pub const ApprovalDeposit: Balance = 10 * MILLI_CENT;
	pub const AssetsStringLimit: u32 = 50;
	pub const AssetAccountDeposit: u128 = 1 * TOKEN;
	pub const MetadataDepositBase: Balance = deposit(1, 68);
	pub const MetadataDepositPerByte: Balance = deposit(0, 1);
}

impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = codec::Compact<u32>;
	type Currency = Balances;
	type ForceOrigin = EnsureRootOrFullCouncil;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
	type RemoveItemsLimit = ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub ClassBondAmount: Balance = 100;
	pub MaxMetadataLength: u32 = 256;
	pub const ResourceSymbolLimit: u32 = 10;
	pub const PartsLimit: u32 = 10;
	pub const MaxPriorities: u32 = 3;
	pub const PropertiesLimit: u32 = 15;
	pub const CollectionSymbolLimit: u32 = 100;
	pub const MaxResourcesOnMint: u32 = 100;
}

impl pallet_rmrk_core::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ProtocolOrigin = EnsureRootOrFullCouncil;
	type NestingBudget = ConstU32<200>;
	type ResourceSymbolLimit = ResourceSymbolLimit;
	type PartsLimit = PartsLimit;
	type MaxPriorities = MaxPriorities;
	type PropertiesLimit = PropertiesLimit;
	type CollectionSymbolLimit = CollectionSymbolLimit;
	type MaxResourcesOnMint = MaxResourcesOnMint;
	// type TransferHooks = IndranetWrappedBalances;
	type TransferHooks = ();
	type WeightInfo = pallet_rmrk_core::weights::SubstrateWeight<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = pallet_rmrk_core::RmrkBenchmark;
}
