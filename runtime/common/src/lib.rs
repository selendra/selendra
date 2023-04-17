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
pub use sp_runtime::{FixedPointNumber, Perbill, Permill};

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

parameter_types! {
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(MAX_BLOCK_WEIGHT.set_proof_size(u64::MAX), NORMAL_DISPATCH_RATIO);

	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(MAX_BLOCK_SIZE, NORMAL_DISPATCH_RATIO);
}
