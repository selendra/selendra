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

//! Common runtime code for Selendra.

#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

pub mod bench;
pub mod check_nonce;
pub mod currency;
pub mod evm;
pub mod origin;
pub mod precompile;

#[cfg(test)]
mod mock;

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

use sp_runtime::Perbill;
use sp_std::prelude::*;
use static_assertions::const_assert;

use frame_support::{
	parameter_types,
	traits::ConstU32,
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, WEIGHT_PER_MILLIS},
		DispatchClass, Weight,
	},
	RuntimeDebug,
};
use frame_system::limits;

pub use module_support::{ExchangeRate, PrecompileCallerFilter, Price, Rate, Ratio};
pub use precompile::{
	AllPrecompiles, DEXPrecompile, EVMPrecompile, MultiCurrencyPrecompile, NFTPrecompile,
	OraclePrecompile, SchedulePrecompile,
};
pub use primitives::{
	currency::{TokenInfo, DOT, KMD, KSM, RENBTC, SEL, SUSD},
	AccountId,
};
use primitives::{Balance, CurrencyId};

pub use check_nonce::CheckNonce;
pub use currency::*;
pub use evm::*;
pub use origin::*;

pub type TimeStampedPrice = orml_oracle::TimestampedValue<Price, primitives::Moment>;

// TODO: somehow estimate this value. Start from a conservative value.
pub const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);
/// The ratio that `Normal` extrinsics should occupy. Start from a conservative value.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(70);
/// Parachain only have 0.5 second of computation time.
pub const MAXIMUM_BLOCK_WEIGHT: Weight = 500 * WEIGHT_PER_MILLIS;

const_assert!(NORMAL_DISPATCH_RATIO.deconstruct() >= AVERAGE_ON_INITIALIZE_RATIO.deconstruct());

// Priority of unsigned transactions
parameter_types! {
	// Operational = final_fee * OperationalFeeMultiplier / TipPerWeightStep * max_tx_per_block + (tip + 1) / TipPerWeightStep * max_tx_per_block
	// final_fee_min = base_fee + len_fee + adjusted_weight_fee + tip
	// priority_min = final_fee * OperationalFeeMultiplier / TipPerWeightStep * max_tx_per_block + (tip + 1) / TipPerWeightStep * max_tx_per_block
	//              = final_fee_min * OperationalFeeMultiplier / TipPerWeightStep
	// Ensure Inherent -> Operational tx -> Unsigned tx -> Signed normal tx
	// Ensure `max_normal_priority < MinOperationalPriority / 2`
	pub TipPerWeightStep: Balance = cent(SEL); // 0.01 SEL
	pub MaxTipsOfPriority: Balance = 10_000 * dollar(SEL); // 10_000 SEL
	pub const OperationalFeeMultiplier: u64 = 100_000_000_000_000u64;

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
}

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
	NonTransfer,
	CancelProxy,
	Governance,
	Staking,
	IdentityJudgement,
	Swap,
	DexLiquidity,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

// The type used for currency conversion.
///
/// This must only be used as long as the balance type is `u128`.
pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
static_assertions::assert_eq_size!(primitives::Balance, u128);

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
	type MaxNominators = ConstU32<1000>;
	type MaxValidators = ConstU32<1000>;
}
