use selendra_primitives::{Index, task::TaskResult};
use frame_support::pallet_prelude::Weight;
use sp_runtime::DispatchError;

/// Dispatchable tasks
pub trait DispatchableTask {
	fn dispatch(self, weight: Weight) -> TaskResult;
}

/// Idle scheduler trait
pub trait IdleScheduler<Task> {
	fn schedule(task: Task) -> Result<Index, DispatchError>;
	fn dispatch(id: Index, weight: Weight) -> Weight;
}