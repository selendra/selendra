use frame_support::pallet_prelude::Weight;
use selendra_primitives::{evm::task::TaskResult, Index};
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

#[cfg(feature = "std")]
impl DispatchableTask for () {
	fn dispatch(self, _weight: Weight) -> TaskResult {
		unimplemented!()
	}
}

#[cfg(feature = "std")]
impl<Task> IdleScheduler<Task> for () {
	fn schedule(_task: Task) -> Result<Index, DispatchError> {
		unimplemented!()
	}
	fn dispatch(_id: Index, _weight: Weight) -> Weight {
		unimplemented!()
	}
}
