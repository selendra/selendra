//! Autogenerated weights for orml_authority
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-09-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// /Users/ermal/Acala/target/release/acala
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=orml_authority
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=../templates/orml-weight-template.hbs
// --output=./authority/src/weights.rs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for orml_authority.
pub trait WeightInfo {
	fn dispatch_as() -> Weight;
	fn schedule_dispatch_without_delay() -> Weight;
	fn schedule_dispatch_with_delay() -> Weight;
	fn fast_track_scheduled_dispatch() -> Weight;
	fn delay_scheduled_dispatch() -> Weight;
	fn cancel_scheduled_dispatch() -> Weight;
	fn authorize_call() -> Weight;
	fn remove_authorized_call() -> Weight;
	fn trigger_call() -> Weight;
}

/// Default weights.
impl WeightInfo for () {
	fn dispatch_as() -> Weight {
		(12_000_000 as Weight)
	}
	fn schedule_dispatch_without_delay() -> Weight {
		(30_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn schedule_dispatch_with_delay() -> Weight {
		(32_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn fast_track_scheduled_dispatch() -> Weight {
		(42_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn delay_scheduled_dispatch() -> Weight {
		(42_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn cancel_scheduled_dispatch() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn authorize_call() -> Weight {
		(14_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_authorized_call() -> Weight {
		(16_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn trigger_call() -> Weight {
		(29_000_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}