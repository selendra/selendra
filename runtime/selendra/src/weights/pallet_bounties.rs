
//! Autogenerated weights for `pallet_bounties`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-25, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `SEL-KVM-02-01`, CPU: `AMD Ryzen 9 5900X 12-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./runtime/selendra/src/weights/pallet_bounties.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_bounties`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bounties::WeightInfo for WeightInfo<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	/// The range of component `d` is `[0, 16384]`.
	fn propose_bounty(d: u32, ) -> Weight {
		// Minimum execution time: 20_170 nanoseconds.
		Weight::from_ref_time(21_039_661 as u64)
			// Standard Error: 5
			.saturating_add(Weight::from_ref_time(428 as u64).saturating_mul(d as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		// Minimum execution time: 8_306 nanoseconds.
		Weight::from_ref_time(8_637_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		// Minimum execution time: 7_595 nanoseconds.
		Weight::from_ref_time(7_896_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn unassign_curator() -> Weight {
		// Minimum execution time: 27_505 nanoseconds.
		Weight::from_ref_time(28_327_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		// Minimum execution time: 19_800 nanoseconds.
		Weight::from_ref_time(20_311_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	fn award_bounty() -> Weight {
		// Minimum execution time: 17_014 nanoseconds.
		Weight::from_ref_time(17_716_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		// Minimum execution time: 48_537 nanoseconds.
		Weight::from_ref_time(49_949_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(6 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		// Minimum execution time: 31_513 nanoseconds.
		Weight::from_ref_time(32_295_000 as u64)
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		// Minimum execution time: 38_516 nanoseconds.
		Weight::from_ref_time(39_168_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		// Minimum execution time: 14_950 nanoseconds.
		Weight::from_ref_time(15_230_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:2 w:2)
	// Storage: System Account (r:4 w:4)
	/// The range of component `b` is `[0, 100]`.
	fn spend_funds(b: u32, ) -> Weight {
		// Minimum execution time: 5_561 nanoseconds.
		Weight::from_ref_time(128_722_031 as u64)
			// Standard Error: 218_980
			.saturating_add(Weight::from_ref_time(18_564_938 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(b as u64)))
	}
}
