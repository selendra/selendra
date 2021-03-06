
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_utility
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_utility.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	fn batch(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 67_000
			.saturating_add((3_899_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(1_880_000 as Weight)
	}
	fn batch_all(c: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 64_000
			.saturating_add((4_183_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(10_822_000 as Weight)
	}
	fn force_batch(c: u32, ) -> Weight {
		(17_158_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((3_600_000 as Weight).saturating_mul(c as Weight))
	}
}
