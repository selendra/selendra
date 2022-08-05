// This file is part of Selendra.

// Copyright (C) 2021-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Common runtime code for Selendra.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
	parameter_types,
	traits::{EnsureOneOf, Get},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_PER_MILLIS},
		DispatchClass, Weight,
	},
	RuntimeDebug,
};
use frame_system::{limits, EnsureRoot};
use module_evm::GenesisAccount;
use orml_traits::GetByKey;
use primitives::{evm::is_system_contract, Balance, CurrencyId, Nonce};
use scale_info::TypeInfo;
use sp_core::{Bytes, H160};
use sp_runtime::{
	traits::Convert, transaction_validity::TransactionPriority, FixedPointNumber, Perbill,
	Perquintill,
};
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData, prelude::*};
use static_assertions::const_assert;

pub use check_nonce::CheckNonce;
pub use module_support::{ExchangeRate, PrecompileCallerFilter, Price, Rate, Ratio};
pub use precompile::{
	AllPrecompiles, DEXPrecompile, EVMPrecompile, MultiCurrencyPrecompile, NFTPrecompile,
	OraclePrecompile, SchedulePrecompile, StableAssetPrecompile,
};
pub use primitives::{
	currency::{TokenInfo, DAI, DOT, KSM, KUSD, LSEL, RENBTC, SEL},
	AccountId, BlockNumber, Multiplier,
};

use module_transaction_payment::TargetedFeeAdjustment;

pub mod bench;
pub mod check_nonce;
pub mod currency;
pub mod evm;
pub mod impls;
pub mod origin;
pub mod precompile;

pub use currency::*;
pub use evm::*;
pub use origin::*;

mod gas_to_weight_ratio;
#[cfg(test)]
mod mock;

pub type TimeStampedPrice = orml_oracle::TimestampedValue<Price, primitives::Moment>;

// TODO: somehow estimate this value. Start from a conservative value.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// The ratio that `Normal` extrinsics should occupy. Start from a conservative value.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(70);
/// Parachain only have 0.5 second of computation time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = 500 * WEIGHT_PER_MILLIS;

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	/// Maximum length of block. Up to 5MB.
	pub RuntimeBlockLength: limits::BlockLength =
		limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	/// Block weights base values and limits.
	pub RuntimeBlockWeights: limits::BlockWeights = limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
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

	// Priority of unsigned transactions
	// Operational = final_fee * OperationalFeeMultiplier / TipPerWeightStep * max_tx_per_block + (tip + 1) / TipPerWeightStep * max_tx_per_block
	// final_fee_min = base_fee + len_fee + adjusted_weight_fee + tip
	// priority_min = final_fee * OperationalFeeMultiplier / TipPerWeightStep * max_tx_per_block + (tip + 1) / TipPerWeightStep * max_tx_per_block
	//              = final_fee_min * OperationalFeeMultiplier / TipPerWeightStep
	// Ensure Inherent -> Operational tx -> Unsigned tx -> Signed normal tx
	// Ensure `max_normal_priority < MinOperationalPriority / 2`
	pub TipPerWeightStep: Balance = cent(SEL); // 0.01 SEL
	pub MaxTipsOfPriority: Balance = 10_000 * dollar(SEL); // 10_000 SEL
	pub const OperationalFeeMultiplier: u64 = 100_000_000_000_000u64;
	// MinOperationalPriority = final_fee_min * OperationalFeeMultiplier / TipPerWeightStep
	MinOperationalPriority: TransactionPriority = (1_500_000_000u128 * OperationalFeeMultiplier::get() as u128 / TipPerWeightStep::get())
		.try_into()
		.expect("Check that there is no overflow here");
	pub CdpEngineUnsignedPriority: TransactionPriority = MinOperationalPriority::get() - 1000;
	pub AuctionManagerUnsignedPriority: TransactionPriority = MinOperationalPriority::get() - 2000;
	pub RenvmBridgeUnsignedPriority: TransactionPriority = MinOperationalPriority::get() - 3000;

	/// A limit for off-chain phragmen unsigned solution submission.
	///
	/// We want to keep it as high as possible, but can't risk having it reject,
	/// so we always subtract the base block execution weight.
	pub OffchainSolutionWeightLimit: Weight = RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic
		.expect("Normal extrinsics have weight limit configured by default; qed")
		.saturating_sub(BlockExecutionWeight::get());
}

pub type SlowAdjustingFeeUpdate<R> =
	TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	TypeInfo,
)]
pub enum ProxyType {
	Any,
	CancelProxy,
	Governance,
	Staking,
	IdentityJudgement,
	Auction,
	Swap,
	Loan,
	DexLiquidity,
	StableAssetSwap,
	StableAssetLiquidity,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

/// Macro to set a value (e.g. when using the `parameter_types` macro) to either a production value
/// or to an environment variable or testing value (in case the `fast-runtime` feature is selected).
/// Note that the environment variable is evaluated _at compile time_.
///
/// Usage:
/// ```Rust
/// parameter_types! {
/// 	// Note that the env variable version parameter cannot be const.
/// 	pub LaunchPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1, "KSM_LAUNCH_PERIOD");
/// 	pub const VotingPeriod: BlockNumber = prod_or_fast!(7 * DAYS, 1 * MINUTES);
/// }
#[macro_export]
macro_rules! prod_or_fast {
	($prod:expr, $test:expr) => {
		if cfg!(feature = "fast-runtime") {
			$test
		} else {
			$prod
		}
	};
	($prod:expr, $test:expr, $env:expr) => {
		if cfg!(feature = "fast-runtime") {
			core::option_env!($env).map(|s| s.parse().ok()).flatten().unwrap_or($test)
		} else {
			$prod
		}
	};
}

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = frame_support::traits::ConstU32<1000>;
	type MaxValidators = frame_support::traits::ConstU32<1000>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn check_max_normal_priority() {
		let max_normal_priority: TransactionPriority = (MaxTipsOfPriority::get() /
			TipPerWeightStep::get() *
			RuntimeBlockWeights::get()
				.max_block
				.min(*RuntimeBlockLength::get().max.get(DispatchClass::Normal) as u64) as u128)
			.try_into()
			.expect("Check that there is no overflow here");
		assert!(max_normal_priority < MinOperationalPriority::get() / 2); // 50%
	}
}
