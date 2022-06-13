use crate::{
	RuntimeVersion, Call, Origin, Indices, VERSION, Runtime, BlockWeights, BlockHashCount, BlockLength,
	RocksDbWeight, Event, PalletInfo, Babe,Weight, OriginCaller, Preimage
};

use frame_support::{
	parameter_types, traits::{Everything, ConstU32, EqualPrivilegeOnly},
};
use frame_system::EnsureRoot;

use sp_runtime::{
	generic, Perbill,
	traits::{
		BlakeTwo256
	},
};

use selendra_primitives::{BlockNumber, Nonce, AccountId, Hash, Moment, Balance};
use selendra_runtime_constants::time::SLOT_DURATION;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type DbWeight = RocksDbWeight;
	type Origin = Origin;
	type Call = Call;
	type Index = Nonce;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = Indices;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = frame_system::weights::SubstrateWeight<Runtime>;
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = pallet_timestamp::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		BlockWeights::get().max_block;
	// Retry a scheduled item every 10 blocks (1 minute) until the preimage exists.
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
}

