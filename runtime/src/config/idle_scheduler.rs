use crate::{RuntimeEvent, Runtime, System};

use scale_info::TypeInfo;
use codec::{Decode, Encode};

use sp_core::ConstU32;
use sp_runtime::traits::BlockNumberProvider;
use frame_support::{
	pallet_prelude::Weight,
	parameter_types,
};
use pallet_idle_scheduler::DispatchableTask;

use selendra_primitives::{define_combined_task, task::TaskResult,};
use selendra_runtime_common::{
	BlockWeights
};

pub const BASE_WEIGHT: Weight = Weight::from_ref_time(1_000_000);

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


pub struct MockBlockNumberProvider;

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u32;

    fn current_block_number() -> Self::BlockNumber {
        System::block_number().try_into().unwrap()
    }
}


parameter_types!(
	// At least 2% of max block weight should remain before idle tasks are dispatched.
	pub MinimumWeightRemainInBlock: Weight = BlockWeights::get().max_block / 50;
);

impl pallet_idle_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type BlockNumberProvider = MockBlockNumberProvider;
	// Number of relay chain blocks produced with no parachain blocks finalized,
	// once this number is reached idle scheduler is disabled as block production is slow
	type DisableBlockThreshold = ConstU32<6>;
}
