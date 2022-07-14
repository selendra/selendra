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

//! A set of constant values used in dev runtime.

/// Time and blocks.
pub mod time {
	use crate::parameter_types;
	use primitives::{BlockNumber, Moment};
	use runtime_common::prod_or_fast;

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;
	pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = prod_or_fast!(4 * HOURS, 1 * MINUTES);

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;

	// 1 in 4 blocks (on average, not counting collisions) will be primary babe blocks.
	pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);

	parameter_types! {
		pub const OneDay: BlockNumber = DAYS;
		pub const ZeroDay: BlockNumber = 0;
		pub const SevenDays: BlockNumber = 7 * DAYS;
	}
}

/// constant currency-related
pub mod currency {
	use crate::{
		cent, parameter_type_with_key, parameter_types, AssetIdMapping, AssetIdMaps, AssetIds,
		Balance, CurrencyId, Runtime, KUSD, LSEL, SEL,
	};
	use primitives::TokenSymbol;

	parameter_types! {
		pub NativeTokenExistentialDeposit: Balance = 10 * cent(SEL);
		pub const GetNativeCurrencyId: CurrencyId = SEL;
		pub const GetStableCurrencyId: CurrencyId = KUSD;
		pub const GetLiquidCurrencyId: CurrencyId = LSEL;
	}

	parameter_type_with_key! {
		pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
			match currency_id {
				CurrencyId::Token(symbol) => match symbol {
					TokenSymbol::SEL => cent(*currency_id),
					TokenSymbol::KUSD => cent(*currency_id),
					TokenSymbol::LSEL |
					TokenSymbol::DOT |
					TokenSymbol::KSM |
					TokenSymbol::DAI |
					TokenSymbol::RENBTC => Balance::max_value() // unsupported
				},
				CurrencyId::DexShare(dex_share_0, _) => {
					let currency_id_0: CurrencyId = (*dex_share_0).into();

					// initial dex share amount is calculated based on currency_id_0,
					// use the ED of currency_id_0 as the ED of lp token.
					if currency_id_0 == GetNativeCurrencyId::get() {
						NativeTokenExistentialDeposit::get()
					} else if let CurrencyId::Erc20(address) = currency_id_0 {
						// LP token with erc20
						AssetIdMaps::<Runtime>::get_asset_metadata(AssetIds::Erc20(address)).
							map_or(Balance::max_value(), |metatata| metatata.minimal_balance)
					} else {
						Self::get(&currency_id_0)
					}
				},
				CurrencyId::Erc20(_) => Balance::max_value(), // not handled by orml-tokens
				CurrencyId::StableAssetPoolToken(stable_asset_id) => {
					AssetIdMaps::<Runtime>::get_asset_metadata(AssetIds::StableAssetId(*stable_asset_id)).
						map_or(Balance::max_value(), |metatata| metatata.minimal_balance)
				},
				CurrencyId::ForeignAsset(foreign_asset_id) => {
					AssetIdMaps::<Runtime>::get_asset_metadata(AssetIds::ForeignAssetId(*foreign_asset_id)).
						map_or(Balance::max_value(), |metatata| metatata.minimal_balance)
				},
			}
		};
	}
}

/// Fee-related
pub mod fee {
	use frame_support::weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	};
	use primitives::Balance;
	use runtime_common::{cent, dollar, millicent, SEL};
	use smallvec::smallvec;
	use sp_runtime::Perbill;

	pub fn base_tx_in_sel() -> Balance {
		cent(SEL) / 10
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
			// in Selendra, extrinsic base weight (smallest non-zero weight) is mapped to 1/10 CENT:
			let p = base_tx_in_sel(); // 1_000_000_000;
			let q = Balance::from(ExtrinsicBaseWeight::get()); // 125_000_000
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q), // zero
				coeff_integer: p / q,                         // 8
			}]
		}
	}

	pub fn deposit(items: u32, bytes: u32) -> Balance {
		items as Balance * 2 * dollar(SEL) + (bytes as Balance) * 10 * millicent(SEL)
	}
}

/// election-related
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

/// account-related
pub mod accounts {
	use crate::{parameter_types, AccountId, LockIdentifier, PalletId, Vec};
	use sp_runtime::traits::AccountIdConversion;
	use sp_std::vec;

	// Pallet accounts of runtime
	parameter_types! {
		pub const TreasuryPalletId: PalletId = PalletId(*b"sel/trsy");
		pub const LoansPalletId: PalletId = PalletId(*b"sel/loan");
		pub const DEXPalletId: PalletId = PalletId(*b"sel/dexm");
		pub const CDPTreasuryPalletId: PalletId = PalletId(*b"sel/cdpt");
		pub const FunanTreasuryPalletId: PalletId = PalletId(*b"sel/hztr");
		pub const IncentivesPalletId: PalletId = PalletId(*b"sel/inct");
		pub const CollatorPotId: PalletId = PalletId(*b"sel/cpot");
		// Treasury reserve
		pub const TreasuryReservePalletId: PalletId = PalletId(*b"sel/reve");
		pub const PhragmenElectionPalletId: LockIdentifier = *b"sel/phre";
		pub const NftPalletId: PalletId = PalletId(*b"sel/aNFT");
		// This Pallet is only used to payment fee pool, it's not added to whitelist by design.
		// because transaction payment pallet will ensure the accounts always have enough ED.
		pub const TransactionPaymentPalletId: PalletId = PalletId(*b"sel/fees");
		pub const StableAssetPalletId: PalletId = PalletId(*b"nuts/sta");

		pub UnreleasedNativeVaultAccountId: AccountId = PalletId(*b"sel/urls").into_account_truncating();
		pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
	}

	pub fn get_all_module_accounts() -> Vec<AccountId> {
		vec![
			TreasuryPalletId::get().into_account_truncating(),
			LoansPalletId::get().into_account_truncating(),
			DEXPalletId::get().into_account_truncating(),
			CDPTreasuryPalletId::get().into_account_truncating(),
			FunanTreasuryPalletId::get().into_account_truncating(),
			IncentivesPalletId::get().into_account_truncating(),
			TreasuryReservePalletId::get().into_account_truncating(),
			CollatorPotId::get().into_account_truncating(),
			UnreleasedNativeVaultAccountId::get(),
			StableAssetPalletId::get().into_account_truncating(),
		]
	}
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
	}
}
