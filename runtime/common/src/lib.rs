// Copyright 2023 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	parameter_types,
	traits::Currency,
	weights::{constants::WEIGHT_REF_TIME_PER_MILLIS, Weight},
};
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
pub use sp_runtime::{traits::Bounded, FixedPointNumber, Perbill, Permill, Perquintill};

pub mod impls;
pub mod staking;

pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

// We agreed to 5MB as the block size limit.
pub const MAX_BLOCK_SIZE: u32 = 5 * 1024 * 1024;

// 75% block weight is dedicated to normal extrinsics
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
// The whole process for a single block should take 1s, of which 400ms is for creation,
// 200ms for propagation and 400ms for validation. Hence the block weight should be within 400ms.
pub const MAX_BLOCK_WEIGHT: Weight =
	Weight::from_ref_time(WEIGHT_REF_TIME_PER_MILLIS.saturating_mul(400));

// Common constants used in all runtimes.
parameter_types! {
	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(75, 1000_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 10u128);
	/// The maximum amount of the multiplier.
	pub MaximumMultiplier: Multiplier = Bounded::max_value();

	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(MAX_BLOCK_WEIGHT.set_proof_size(u64::MAX), NORMAL_DISPATCH_RATIO);

	/// Maximum length of block. Up to 5MB.
	pub BlockLength: frame_system::limits::BlockLength =
		frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

/// Parameterized slow adjusting fee updated based on
/// https://research.web3.foundation/en/latest/selendra/overview/2-token-economics.html#-2.-slow-adjusting-mechanism
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
	R,
	TargetBlockFullness,
	AdjustmentVariable,
	MinimumMultiplier,
	MaximumMultiplier,
>;
