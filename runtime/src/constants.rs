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

use selendra_primitives::{Balance, TOKEN};

// Prints debug output of the `contracts` pallet to stdout if the node is started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: bool = true;

// The storage per one byte of contract storage: 4*10^{-5} Selendra per byte.
pub const CONTRACT_DEPOSIT_PER_BYTE: Balance = 4 * (TOKEN / 100_000);

pub mod currency {
	use selendra_primitives::{Balance, TOKEN};

	pub const MILLI_CENT: Balance = TOKEN / 1000;
	pub const MICRO_CENT: Balance = MILLI_CENT / 1000;
	pub const NANO_CENT: Balance = MICRO_CENT / 1000;
	pub const PICO_CENT: Balance = NANO_CENT / 1000;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 10 * MILLI_CENT + (bytes as Balance) * 5 * MILLI_CENT
	}
}

pub mod time {
	use selendra_primitives::{BlockNumber, Moment};

	pub const MILLISECS_PER_BLOCK: u64 = 1000;
	pub const SECS_PER_BLOCK: Moment = MILLISECS_PER_BLOCK / 1000;
	pub const BLOCKS_PER_HOUR: u32 = 60 * 60 * 1000 / (MILLISECS_PER_BLOCK as u32);

	pub const MINUTES: BlockNumber = 60 / (SECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// Storage
	frame_support::parameter_types! {
		pub storage MillisecsPerBlock: Moment = MILLISECS_PER_BLOCK;
		pub storage SecsPerBlock: Moment = MILLISECS_PER_BLOCK / 1000;
	}
}

/// Fee-related.
pub mod fee {
	use frame_support::weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	};
	use selendra_primitives::Balance;
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
			let p = 100 * super::currency::MILLI_CENT;
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
