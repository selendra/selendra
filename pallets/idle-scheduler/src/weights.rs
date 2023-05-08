//! Autogenerated weights for pallet_idle_scheduler
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_idle_scheduler.
pub trait WeightInfo {
	fn on_initialize() -> Weight;
	fn on_idle_base() -> Weight;
	fn clear_tasks() -> Weight;
	fn schedule_task() -> Weight;
}

/// Weights for pallet_idle_scheduler using the Selendra node and recommended hardware.
pub struct SelendraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SelendraWeight<T> {
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: IdleScheduler PreviousRelayBlockNumber (r:0 w:1)
	fn on_initialize() -> Weight {
		Weight::from_ref_time(2_545_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: IdleScheduler PreviousRelayBlockNumber (r:1 w:0)
	fn on_idle_base() -> Weight {
		Weight::from_ref_time(3_627_000)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
	}
	// Storage: IdleScheduler Tasks (r:0 w:1)
	fn clear_tasks() -> Weight {
		Weight::from_ref_time(9_181_000)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: IdleScheduler NextTaskId (r:1 w:1)
	// Storage: IdleScheduler Tasks (r:0 w:1)
	fn schedule_task() -> Weight {
		Weight::from_ref_time(4_103_000)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn on_initialize() -> Weight {
		Weight::from_ref_time(2_545_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn on_idle_base() -> Weight {
		Weight::from_ref_time(3_627_000)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
	}
	fn clear_tasks() -> Weight {
		Weight::from_ref_time(9_181_000)
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	fn schedule_task() -> Weight {
		Weight::from_ref_time(4_103_000)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
}
