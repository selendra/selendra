
//! Autogenerated weights for `pallet_timestamp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `SEL-KVM-02-02`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_timestamp
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_timestamp.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_timestamp`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_timestamp::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:1)
	// Storage: Babe CurrentSlot (r:1 w:0)
	fn set() -> Weight {
		// Minimum execution time: 6_854 nanoseconds.
		Weight::from_ref_time(7_114_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	fn on_finalize() -> Weight {
		// Minimum execution time: 3_187 nanoseconds.
		Weight::from_ref_time(3_396_000)
	}
}
