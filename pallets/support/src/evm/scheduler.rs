use frame_support::pallet_prelude::Weight;
use selendra_primitives::{task::TaskResult, Index};
use sp_runtime::DispatchError;

/// Dispatchable tasks
pub trait DispatchableTask {
	fn dispatch(self, weight: Weight) -> TaskResult;
}

#[cfg(feature = "std")]
impl<Task> IdleScheduler<Task> for () {
	fn schedule(_task: Task) -> Result<Nonce, DispatchError> {
		unimplemented!()
	}
	fn dispatch(_id: Nonce, _weight: Weight) -> Weight {
		unimplemented!()
	}
}
