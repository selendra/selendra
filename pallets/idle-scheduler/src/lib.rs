//! # Idle scheduler Module
//!
//! Allow pallets and chain maintainer to schedule a task to be dispatched when chain is idle.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]
#![allow(unused_must_use)]

use codec::FullCodec;
use scale_info::TypeInfo;

use sp_runtime::{
	traits::{BlockNumberProvider, One},
	ArithmeticError,
};
use sp_std::{cmp::PartialEq, fmt::Debug, prelude::*};

use selendra_primitives::{task::TaskResult, BlockNumber, Index};

use frame_support::{log, pallet_prelude::*};
use frame_system::pallet_prelude::*;
pub use pallets_support::scheduler::{DispatchableTask, IdleScheduler};

mod mock;
mod tests;
mod weights;

pub use weights::WeightInfo;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;

		/// Dispatchable tasks.
		type Task: DispatchableTask + FullCodec + Debug + Clone + PartialEq + TypeInfo;

		/// The minimum weight that should remain before idle tasks are dispatched.
		#[pallet::constant]
		type MinimumWeightRemainInBlock: Get<Weight>;

		/// Gets Block Number
		type BlockNumberProvider: BlockNumberProvider<BlockNumber = BlockNumber>;

		/// Number of Chain blocks skipped to disable `on_idle` dispatching scheduled tasks,
		/// this shuts down idle-scheduler when block production is slower than this number of
		///  blocks.
		#[pallet::constant]
		type DisableBlockThreshold: Get<BlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config> {
		/// A task has been dispatched on_idle.
		TaskDispatched { task_id: Index, result: DispatchResult },
		/// A task is added.
		TaskAdded { task_id: Index, task: T::Task },
	}

	/// The schedule tasks waiting to dispatch. After task is dispatched, it's removed.
	///
	/// Tasks: map Index => Task
	#[pallet::storage]
	#[pallet::getter(fn tasks)]
	pub type Tasks<T: Config> = StorageMap<_, Twox64Concat, Index, T::Task, OptionQuery>;

	/// The task id used to index tasks.
	#[pallet::storage]
	#[pallet::getter(fn next_task_id)]
	pub type NextTaskId<T: Config> = StorageValue<_, Index, ValueQuery>;

	/// A temporary variable used to check if should skip dispatch schedule task or not.
	#[pallet::storage]
	#[pallet::getter(fn previous_block)]
	pub type PreviousBlockNumber<T: Config> = StorageValue<_, BlockNumber, ValueQuery>;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {
		fn on_initialize(_n: T::BlockNumber) -> Weight {
			// This is the previous block because `on_initialize` is executed
			// before the inherent that sets the new chain block number
			let previous_block: BlockNumber = T::BlockNumberProvider::current_block_number();

			PreviousBlockNumber::<T>::put(previous_block);
			T::WeightInfo::on_initialize()
		}

		fn on_idle(_n: T::BlockNumber, remaining_weight: Weight) -> Weight {
			// Checks if we have skipped enough blocks without block production to skip dispatching
			// scheduled tasks
			let current_block_number: BlockNumber = T::BlockNumberProvider::current_block_number();
			let previous_block_number = PreviousBlockNumber::<T>::take();
			if current_block_number.saturating_sub(previous_block_number) >=
				T::DisableBlockThreshold::get()
			{
				log::debug!(
					target: "idle-scheduler",
					"Produced blocks without finalizing blocks. Idle-scheduler will not execute.\ncurrent number: {:?}\nprevious block number: {:?}",
					current_block_number,
					previous_block_number
				);
				// something is not correct so exhaust all remaining weight (note: any on_idle hooks after
				// IdleScheduler won't execute)
				remaining_weight
			} else {
				Self::do_dispatch_tasks(remaining_weight)
			}
		}

		fn on_finalize(_n: T::BlockNumber) {
			// Don't commit to storage, needed for the case block is full and `on_idle` isn't called
			PreviousBlockNumber::<T>::kill();
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(< T as Config >::WeightInfo::schedule_task())]
		pub fn schedule_task(origin: OriginFor<T>, task: T::Task) -> DispatchResult {
			ensure_root(origin)?;
			Self::do_schedule_task(task).map(|_| ())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Add the task to the queue to be dispatched later.
	fn do_schedule_task(task: T::Task) -> Result<Index, DispatchError> {
		let id = Self::get_next_task_id()?;
		Tasks::<T>::insert(id, &task);
		Self::deposit_event(Event::<T>::TaskAdded { task_id: id, task });
		Ok(id)
	}

	/// Retrieves the next task ID from storage, and increment it by one.
	fn get_next_task_id() -> Result<Index, DispatchError> {
		NextTaskId::<T>::mutate(|current| -> Result<Index, DispatchError> {
			let id = *current;
			*current = current.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
			Ok(id)
		})
	}

	/// Keep dispatching tasks in Storage, until insufficient weight remains.
	pub fn do_dispatch_tasks(total_weight: Weight) -> Weight {
		let mut weight_remaining = total_weight.saturating_sub(T::WeightInfo::on_idle_base());
		if weight_remaining.ref_time() <= T::MinimumWeightRemainInBlock::get().ref_time() {
			// return total weight so no `on_idle` hook will execute after IdleScheduler
			return total_weight
		}

		let mut completed_tasks: Vec<(Index, TaskResult)> = vec![];

		for (id, task) in Tasks::<T>::iter() {
			let result = task.dispatch(weight_remaining);
			weight_remaining = weight_remaining.saturating_sub(result.used_weight);
			if result.finished {
				completed_tasks.push((id, result));
				weight_remaining = weight_remaining.saturating_sub(T::WeightInfo::clear_tasks());
			}

			// If remaining weight falls below the minimmum, break from the loop.
			if weight_remaining.ref_time() <= T::MinimumWeightRemainInBlock::get().ref_time() {
				break
			}
		}

		Self::remove_completed_tasks(completed_tasks);

		total_weight.saturating_sub(weight_remaining)
	}

	/// Removes completed tasks and deposits events.
	pub fn remove_completed_tasks(completed_tasks: Vec<(Index, TaskResult)>) {
		// Deposit event and remove completed tasks.
		for (id, result) in completed_tasks {
			Self::deposit_event(Event::<T>::TaskDispatched { task_id: id, result: result.result });
			Tasks::<T>::remove(id);
		}
	}
}

impl<T: Config> IdleScheduler<T::Task> for Pallet<T> {
	fn schedule(task: T::Task) -> Result<Index, DispatchError> {
		Self::do_schedule_task(task)
	}

	/// If the task can be executed under given weight limit, dispatch it.
	/// Otherwise the scheduler will keep the task and run it later.
	/// NOTE: Only used for synchronous execution case, because `T::WeightInfo::clear_tasks()` is
	/// not considered.
	fn dispatch(id: Index, weight_limit: Weight) -> Weight {
		if let Some(task) = Tasks::<T>::get(id) {
			let result = task.dispatch(weight_limit);
			let used_weight = result.used_weight;
			if result.finished {
				Self::remove_completed_tasks(vec![(id, result)]);
			}

			weight_limit.saturating_sub(used_weight)
		} else {
			weight_limit
		}
	}
}
