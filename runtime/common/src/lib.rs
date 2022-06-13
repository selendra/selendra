// Copyright 2021-2022 Selendra.
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

//! Common runtime code for Selendra and Cardamom.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod impls;

use static_assertions::const_assert;

use sp_runtime::{FixedPointNumber, Perbill, Perquintill};
use frame_support::{
	parameter_types,
	traits::Currency,
	weights::{constants::WEIGHT_PER_SECOND, Weight},
};
use frame_system::limits;
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};

use primitives::{BlockNumber};

/// Implementations of some helper traits passed into runtime modules as associated types.
pub use impls::ToAuthor;
pub mod elections;

/// We assume that ~10% of the block weight is consumed by `on_initialize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used
/// by  Operational  extrinsics.
pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

// Common constants used in all runtimes.
parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;

	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);

	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);

	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);

	/// Maximum length of block. Up to 5MB.
	pub BlockLength: limits::BlockLength =
	limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
}

pub type SlowAdjustingFeeUpdate<R> =
	TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;


pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Implements the weight types for a runtime.
/// It expects the passed runtime constants to contain a `weights` module.
/// The generated weight types were formerly part of the common
/// runtime but are now runtime dependant.
#[macro_export]
macro_rules! impl_runtime_weights {
	($runtime:ident) => {
		use frame_support::weights::{DispatchClass, Weight};
		use frame_system::limits;
		use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
		pub use runtime_common::{
			impl_elections_weights, AVERAGE_ON_INITIALIZE_RATIO, MAXIMUM_BLOCK_WEIGHT,
			NORMAL_DISPATCH_RATIO,
		};
		use sp_runtime::{FixedPointNumber, Perquintill};

		// Implement the weight types of the elections module.
		impl_elections_weights!($runtime);

		// Expose the weight from the runtime constants module.
		pub use $runtime::weights::{
			BlockExecutionWeight, ExtrinsicBaseWeight, ParityDbWeight, RocksDbWeight,
		};

		parameter_types! {
			/// Block weights base values and limits.
			pub BlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
				.base_block($runtime::weights::BlockExecutionWeight::get())
				.for_class(DispatchClass::all(), |weights| {
					weights.base_extrinsic = $runtime::weights::ExtrinsicBaseWeight::get();
				})
				.for_class(DispatchClass::Normal, |weights| {
					weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
				})
				.for_class(DispatchClass::Operational, |weights| {
					weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
					// Operational transactions have an extra reserved space, so that they
					// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
					weights.reserved = Some(
						MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT,
					);
				})
				.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
				.build_or_panic();
		}
	};
}

/// The type used for currency conversion.
///
/// This must only be used as long as the balance type is `u128`.
pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
static_assertions::assert_eq_size!(primitives::Balance, u128);
