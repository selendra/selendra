// Copyright 2022 Smallworld Selendra
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
// along with Selendra.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

pub use self::currency::DOLLARS;

/// Money matters.
pub mod currency {
	use primitives::Balance;

	/// The existential deposit.
	pub const EXISTENTIAL_DEPOSIT: Balance = 10 * CENTS;

	pub const UNITS: Balance = 1_000_000_000_000_000_000;
	pub const DOLLARS: Balance = UNITS; // 1_000_000_000_000_000_000
	pub const CENTS: Balance = DOLLARS / 100; // 10_000_000_000_000_000
	pub const MILLICENTS: Balance = CENTS / 1_000; // 10_000_000_000_000
	pub const MICROCENT: Balance = MILLICENTS / 1_000; // 10_000_000_000

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
	}
}

/// Time and blocks.
pub mod time {
	use primitives::{BlockNumber, Moment};
	use runtime_common::prod_or_fast;
	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = prod_or_fast!(4 * HOURS, 1 * MINUTES);

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	// The choice of is done in accordance to the slot duration and expected target
	// block time, for safely resisting network delays of maximum two seconds.
	// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
}

/// Fee-related.
pub mod fee {
	use crate::weights::ExtrinsicBaseWeight;
	use frame_support::weights::{
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use primitives::Balance;
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, `MAXIMUM_BLOCK_WEIGHT`]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Selendra, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 MILLICENTS:
			let p = super::currency::MILLICENTS;
			let q = 10 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}
}

/// XCM protocol related constants.
pub mod xcm {
	/// Pluralistic bodies existing within the consensus.
	pub mod body {
		// The bodies corresponding to the Polkadot OpenGov Origins.
		pub const FELLOWSHIP_ADMIN_INDEX: u32 = 1;
	}
}

/// System Parachains.
pub mod system_parachain {
	/// Statemint parachain ID.
	pub const INDRANET_ID: u32 = 1000;
}

#[cfg(test)]
mod tests {
	use super::{
		currency::{CENTS, MILLICENTS},
		fee::WeightToFee,
	};
	use crate::weights::ExtrinsicBaseWeight;
	use frame_support::weights::WeightToFee as WeightToFeeT;
	use runtime_common::MAXIMUM_BLOCK_WEIGHT;

	#[test]
	// Test that the fee for `MAXIMUM_BLOCK_WEIGHT` of weight has sane bounds.
	fn full_block_fee_is_correct() {
		// A full block should cost between 1 and 10 CENTS.
		let full_block = WeightToFee::weight_to_fee(&MAXIMUM_BLOCK_WEIGHT);
		assert!(full_block >= 1 * CENTS);
		assert!(full_block <= 100 * CENTS);
	}

	#[test]
	// This function tests that the fee for `ExtrinsicBaseWeight` of weight is correct
	fn extrinsic_base_fee_is_correct() {
		// `ExtrinsicBaseWeight` should cost 1/10 of a CENT
		println!("Base: {}", ExtrinsicBaseWeight::get());
		let x = WeightToFee::weight_to_fee(&ExtrinsicBaseWeight::get());
		let y = MILLICENTS / 10;
		assert!(x.max(y) - x.min(y) < MILLICENTS);
	}
}
