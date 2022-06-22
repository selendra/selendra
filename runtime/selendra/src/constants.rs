// This file is part of Selendra.

// Copyright (C) 2020-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! A set of constant values used in Selendra runtime.

/// Time and blocks.
pub mod time {
	use primitives::{Balance, BlockNumber, Moment};
	use runtime_common::{dollar, millicent, prod_or_fast, SEL};

	pub const MILLISECS_PER_BLOCK: Moment =  6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = prod_or_fast!(4 * HOURS, 1 * MINUTES);


	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	pub fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 10 * dollar(SEL) + (bytes as Balance) * millicent(SEL)
	}

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

/// Fee-related
pub mod fee {
	use frame_support::weights::{
		constants::{ExtrinsicBaseWeight},
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use primitives::Balance;
	use runtime_common::{millicent, SEL};
	use smallvec::smallvec;
	use sp_runtime::Perbill;

	pub fn base_tx_in_sel() -> Balance {
		millicent(SEL)
	}

	/// Handles converting a weight scalar to a fee value, based on the scale
	/// and granularity of the node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, system::MaximumBlockWeight]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some
	/// examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Selendra, extrinsic base weight (smallest non-zero weight) is mapped to 1/100 millicent
			let p = base_tx_in_sel();
			let q = 100 * Balance::from(ExtrinsicBaseWeight::get());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}

/// Fee-related
pub mod election {
	use crate::{RuntimeBlockLength, RuntimeBlockWeights};
	use frame_support::{
		parameter_types,
		weights::{constants::BlockExecutionWeight, DispatchClass, Weight},
	};
	use sp_runtime::Perbill;

	parameter_types! {
		/// A limit for off-chain phragmen unsigned solution submission.
		///
		/// We want to keep it as high as possible, but can't risk having it reject,
		/// so we always subtract the base block execution weight.
		pub MinerMaxWeight: Weight = RuntimeBlockWeights::get()
			.get(DispatchClass::Normal)
			.max_extrinsic.expect("Normal extrinsics have weight limit configured by default; qed")
			.saturating_sub(BlockExecutionWeight::get());

		/// A limit for off-chain phragmen unsigned solution length.
		///
		/// We allow up to 90% of the block's size to be consumed by the solution.
		pub MinerMaxLength: u32 = Perbill::from_rational(90_u32, 100) *
			*RuntimeBlockLength::get()
			.max
			.get(DispatchClass::Normal);
	}

	/// The numbers configured here could always be more than the the maximum limits of staking
	/// pallet to ensure election snapshot will not run out of memory. For now, we set them to
	/// smaller values since the staking is bounded and the weight pipeline takes hours for this
	/// single pallet.
	pub struct ElectionBenchmarkConfig;
	impl pallet_election_provider_multi_phase::BenchmarkingConfig for ElectionBenchmarkConfig {
		const VOTERS: [u32; 2] = [1000, 2000];
		const TARGETS: [u32; 2] = [500, 1000];
		const ACTIVE_VOTERS: [u32; 2] = [500, 800];
		const DESIRED_TARGETS: [u32; 2] = [200, 400];
		const SNAPSHOT_MAXIMUM_VOTERS: u32 = 1000;
		const MINER_MAXIMUM_VOTERS: u32 = 1000;
		const MAXIMUM_TARGETS: u32 = 300;
	}

	/// The accuracy type used for genesis election provider;
	pub type OnChainAccuracy = sp_runtime::Perbill;
}

#[cfg(test)]
mod tests {
	use crate::{constants::fee::base_tx_in_sel, Balance};
	use frame_support::weights::constants::ExtrinsicBaseWeight;

	#[test]
	fn check_weight() {
		let p = base_tx_in_sel();
		let q = Balance::from(ExtrinsicBaseWeight::get());

		assert_eq!(p, 1_000_000_000);
		assert_eq!(q, 85_795_000);
		assert_eq!(p / q, 11)
	}
}
