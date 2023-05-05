
//! Unit tests for idle-scheduler module.

#![cfg(test)]

use super::*;
use crate::mock::{IdleScheduler, RuntimeEvent, *};
use frame_support::assert_ok;

// Can schedule tasks
#[test]
fn can_schedule_tasks() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Tasks::<Runtime>::get(0), None);

		assert_ok!(IdleScheduler::schedule_task(
			RuntimeOrigin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));
		assert_eq!(
			Tasks::<Runtime>::get(0),
			Some(ScheduledTasks::BalancesTask(BalancesTask::OnIdle))
		);
		System::assert_has_event(RuntimeEvent::IdleScheduler(crate::Event::TaskAdded {
			task_id: 0,
			task: ScheduledTasks::BalancesTask(BalancesTask::OnIdle),
		}));

		assert_eq!(Tasks::<Runtime>::get(2), None);
	});
}

// can increment next task ID
#[test]
fn can_increment_next_task_id() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(NextTaskId::<Runtime>::get(), 0);
		assert_ok!(IdleScheduler::schedule_task(
			RuntimeOrigin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));

		assert_eq!(NextTaskId::<Runtime>::get(), 1);
	});
}

#[test]
fn on_idle_works() {
	ExtBuilder::default().build().execute_with(|| {
		IdleScheduler::on_initialize(0);
		assert_ok!(IdleScheduler::schedule_task(
			RuntimeOrigin::root(),
			ScheduledTasks::BalancesTask(BalancesTask::OnIdle)
		));
		// simulate relay block number jumping 10 blocks
		sp_io::storage::set(&RELAY_BLOCK_KEY, &10_u32.encode());
		assert_eq!(IdleScheduler::on_idle(System::block_number(), Weight::MAX), Weight::MAX);

		System::set_block_number(1);
		IdleScheduler::on_initialize(1);
		// On_initialize is called it will execute, as now relay block number is the same
		assert_eq!(
			IdleScheduler::on_idle(System::block_number(), Weight::MAX),
			BASE_WEIGHT + <()>::on_idle_base() + <()>::clear_tasks()
		);
		assert!(!PreviousBlockNumber::<Runtime>::exists());
	});
}