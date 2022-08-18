
//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benchamarking`, CPU: `DO-Regular`
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
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		(14_767_000 as Weight)
			// Standard Error: 53_000
			.saturating_add((5_516_000 as Weight).saturating_mul(c as Weight))
	}
	fn as_derivative() -> Weight {
		(7_588_000 as Weight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		(165_335_000 as Weight)
			// Standard Error: 63_000
			.saturating_add((5_291_000 as Weight).saturating_mul(c as Weight))
	}
	fn dispatch_as() -> Weight {
		(19_516_000 as Weight)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		(27_315_000 as Weight)
			// Standard Error: 73_000
			.saturating_add((5_477_000 as Weight).saturating_mul(c as Weight))
	}
}
