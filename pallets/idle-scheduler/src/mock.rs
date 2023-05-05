//! Mocks for idle-scheduler pallet.

#![cfg(test)]

use crate as pallet_idle_scheduler;
use selendra_primitives::{define_combined_task, task::TaskResult};
use frame_support::weights::Weight;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU32, ConstU64, Everything},
};
use pallets_support::scheduler::DispatchableTask;
pub use sp_runtime::offchain::storage::StorageValueRef;

use super::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub const BASE_WEIGHT: Weight = Weight::from_ref_time(1_000_000);
pub const RELAY_BLOCK_KEY: [u8; 32] = [0; 32];

pub type AccountId = u32;
impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type RuntimeOrigin = RuntimeOrigin;
	type Index = u64;
	type BlockNumber = u64;
	type RuntimeCall = RuntimeCall;
	type Hash = sp_runtime::testing::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = sp_runtime::traits::IdentityLookup<Self::AccountId>;
	type Header = sp_runtime::testing::Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

pub struct MockBlockNumberProvider;

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u32;

	fn current_block_number() -> Self::BlockNumber {
		// gets a local mock storage value
		u32::decode(&mut &sp_io::storage::get(&RELAY_BLOCK_KEY).unwrap()[..]).unwrap()
	}
}

// pub struct MockBlockNumberProvider;

// impl BlockNumberProvider for MockBlockNumberProvider {
// 	type BlockNumber = u32;

//     fn current_block_number() -> Self::BlockNumber {
//         System::block_number().try_into().unwrap()
//     }
// }

parameter_types! {
	pub MinimumWeightRemainInBlock: Weight = Weight::from_ref_time(100_000_000_000);
}

impl pallet_idle_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type BlockNumberProvider = MockBlockNumberProvider;
	type DisableBlockThreshold = ConstU32<6>;
}

// Mock dispatachable tasks
#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
pub enum BalancesTask {
	#[codec(index = 0)]
	OnIdle,
}
impl DispatchableTask for BalancesTask {
	fn dispatch(self, weight: Weight) -> TaskResult {
		TaskResult {
			result: Ok(()),
			used_weight: BASE_WEIGHT,
			finished: weight.ref_time() >= BASE_WEIGHT.ref_time(),
		}
	}
}


define_combined_task! {
	#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
	pub enum ScheduledTasks {
		BalancesTask(BalancesTask),
	}
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Pallet, Call, Event<T>},
		IdleScheduler: pallet_idle_scheduler::{Pallet, Call, Event<T>, Storage},
	}
);

#[derive(Default)]
pub struct ExtBuilder;
impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext.execute_with(|| sp_io::storage::set(&RELAY_BLOCK_KEY, &0_u32.encode()));
		ext
	}
}