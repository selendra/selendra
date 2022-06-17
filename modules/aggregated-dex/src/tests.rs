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

//! Unit tests for the Aggregated DEX module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use nutsfinance_stable_asset::traits::StableAsset as StableAssetT;
use sp_runtime::traits::BadOrigin;

fn set_dex_swap_joint_list(joints: Vec<Vec<CurrencyId>>) {
	DexSwapJointList::set(joints);
}

fn inject_liquidity(
	currency_id_a: CurrencyId,
	currency_id_b: CurrencyId,
	max_amount_a: Balance,
	max_amount_b: Balance,
) -> Result<(), &'static str> {
	// set balance
	Tokens::deposit(currency_id_a, &BOB, max_amount_a)?;
	Tokens::deposit(currency_id_b, &BOB, max_amount_b)?;

	let _ = Dex::enable_trading_pair(Origin::signed(BOB.clone()), currency_id_a, currency_id_b);
	Dex::add_liquidity(
		Origin::signed(BOB),
		currency_id_a,
		currency_id_b,
		max_amount_a,
		max_amount_b,
		Default::default(),
		false,
	)?;

	Ok(())
}

fn inital_taiga_dot_kmd_pool() -> DispatchResult {
	<StableAsset as StableAssetT>::create_pool(
		STABLE_ASSET,
		vec![DOT, KMD],
		vec![1u128, 1u128],
		0,
		0,
		0,
		3000u128,
		BOB,
		BOB,
		10_000_000_000u128,
	)?;

	Tokens::deposit(DOT, &BOB, 100_000_000_000u128)?;
	Tokens::deposit(KMD, &BOB, 1_000_000_000_000u128)?;

	<StableAsset as StableAssetT>::mint(&BOB, 0, vec![100_000_000_000u128, 100_000_000_000u128], 0)?;

	Ok(())
}

#[test]
fn rebase_token_amount_convert_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(inital_taiga_dot_kmd_pool());

		assert_eq!(
			AggregatedDex::taiga_get_best_route(DOT, KMD, 100_000_000u128),
			Some((0, 0, 1, 999_983_600u128))
		);
		assert_eq!(
			AggregatedDex::taiga_get_best_route(KMD, DOT, 1_000_000_000u128),
			Some((0, 1, 0, 99_998_360u128))
		);

		assert_eq!(
			AggregatedDex::taiga_get_swap_input_amount(0, 0, 1, 999_983_600u128),
			Some((100_000_098u128, 999_983_600u128))
		);
		assert_eq!(
			AggregatedDex::taiga_get_swap_output_amount(0, 0, 1, 100_000_000u128),
			Some((100_000_000u128, 999_983_600u128))
		);

		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);
		assert_eq!(
			AggregatedDex::taiga_swap(&ALICE, 0, 0, 1, 100_000_000u128, 0),
			Ok((100_000_000u128, 999_983_600u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_900_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 999_983_600u128);
	});
}

#[test]
fn dex_swap_get_swap_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, SUSD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);

		assert_ok!(inject_liquidity(
			DOT,
			SUSD,
			100_000_000_000u128,
			200_000_000_000_000u128
		));
		assert_ok!(inject_liquidity(
			KMD,
			SUSD,
			1_000_000_000_000u128,
			200_000_000_000_000u128
		));

		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, SUSD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 1_980_198_019_801u128))
		);

		set_dex_swap_joint_list(vec![vec![SUSD]]);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_803_921_568u128))
		);

		assert_ok!(inject_liquidity(DOT, KMD, 100_000_000_000u128, 1_000_000_000_000u128));
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_900_990_099u128))
		);
	});
}

#[test]
fn dex_swap_swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(inject_liquidity(
			DOT,
			SUSD,
			100_000_000_000u128,
			200_000_000_000_000u128
		));
		assert_ok!(inject_liquidity(
			KMD,
			SUSD,
			1_000_000_000_000u128,
			200_000_000_000_000u128
		));

		assert_noop!(
			DexSwap::<Runtime>::swap(&ALICE, DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Error::<Runtime>::CannotSwap
		);

		set_dex_swap_joint_list(vec![vec![SUSD]]);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);

		assert_noop!(
			DexSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 10_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);
		assert_ok!(DexSwap::<Runtime>::swap(
			&ALICE,
			DOT,
			KMD,
			SwapLimit::ExactSupply(1_000_000_000u128, 5_000_000_000u128)
		));
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 9_803_921_568u128);

		assert_noop!(
			DexSwap::<Runtime>::swap(
				&ALICE,
				KMD,
				DOT,
				SwapLimit::ExactTarget(9_803_921_568u128, 1_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);
		assert_ok!(DexSwap::<Runtime>::swap(
			&ALICE,
			KMD,
			DOT,
			SwapLimit::ExactTarget(9_803_921_568u128, 500_000_000u128)
		));
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_500_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 4_950_495_048u128);
	});
}

#[test]
fn taiga_swap_get_swap_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactTarget(u128::MAX, 10_000_000_000u128)),
			None
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, SUSD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 9_998_360_751u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				SUSD,
				SwapLimit::ExactTarget(10_000_000_000u128, 10_000_000_000u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 9_998_360_750u128)
			),
			Some((1_000_000_098u128, 9_998_361_730u128))
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(1_000_000_097u128, 9_998_360_750u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				KMD,
				DOT,
				SwapLimit::ExactTarget(100_000_000_000u128, 1_000_000_000u128)
			),
			Some((10_001_640_760u128, 1_000_000_098u128))
		);
	});
}

#[test]
fn taiga_swap_swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			TaigaSwap::<Runtime>::swap(&ALICE, DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Error::<Runtime>::CannotSwap
		);
		assert_noop!(
			TaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(10_000_000_000u128, 9_998_360_750u128)
			),
			Error::<Runtime>::CannotSwap
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);

		assert_eq!(
			TaigaSwap::<Runtime>::swap(&ALICE, DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Ok((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 9_998_360_750u128);

		assert_noop!(
			TaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 10_000_000_000u128)
			),
			nutsfinance_stable_asset::Error::<Runtime>::SwapUnderMin
		);

		assert_eq!(
			TaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Ok((1_000_492_274u128, 10_000_000_980u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 97_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 19_998_361_730u128);

		assert_noop!(
			TaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(1_000_000_000u128, 10_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);
	});
}

#[test]
fn either_dex_or_taiga_swap_get_swap_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			None
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			None
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((1_000_164_076u128, 10_000_000_980u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((1_000_164_076u128, 10_000_000_980u128))
		);

		assert_ok!(inject_liquidity(DOT, KMD, 1_000_000_000u128, 30_000_000_000u128));
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 15_000_000_000u128))
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 15_000_000_000u128))
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((500_000_001u128, 10_000_000_000u128))
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((1_000_164_076u128, 10_000_000_980u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((500_000_001u128, 10_000_000_000u128))
		);

		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(10_000_000_000u128, 0)),
			Some((10_000_000_000u128, 27_272_727_272u128))
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(10_000_000_000u128, 0)),
			Some((10_000_000_000u128, 99_834_740_530u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(10_000_000_000u128, 0)),
			Some((10_000_000_000u128, 99_834_740_530u128))
		);
		assert_eq!(
			DexSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(10_000_000_000u128, 30_000_000_000u128)
			),
			None
		);
		assert_eq!(
			TaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(10_000_000_000u128, 30_000_000_000u128)
			),
			Some((3_001_477_523u128, 30_000_000_980u128))
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::get_swap_amount(
				DOT,
				KMD,
				SwapLimit::ExactTarget(10_000_000_000u128, 30_000_000_000u128)
			),
			Some((3_001_477_523u128, 30_000_000_980u128))
		);
	});
}

#[test]
fn either_dex_or_taiga_swap_swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			EitherDexOrTaigaSwap::<Runtime>::swap(&ALICE, DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Error::<Runtime>::CannotSwap
		);
		assert_noop!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);

		assert_noop!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 10_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 9_000_000_000u128)
			),
			Ok((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 9_998_360_750u128);

		assert_noop!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(1_000_000_000u128, 9_998_360_750u128)
			),
			Error::<Runtime>::CannotSwap
		);
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Ok((1_000_492_274u128, 10_000_000_980u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 97_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 19_998_361_730u128);

		assert_ok!(inject_liquidity(DOT, KMD, 100_000_000_000u128, 2_000_000_000_000u128));
		assert_eq!(
			EitherDexOrTaigaSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 10_000_000_000u128)
			),
			Ok((1_000_000_000u128, 19_801_980_198u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 96_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 39_800_341_928u128);
	});
}

#[test]
fn check_swap_paths_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![]),
			Error::<Runtime>::InvalidSwapPath
		);
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Dex(vec![])]),
			Error::<Runtime>::InvalidSwapPath
		);
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Dex(vec![KMD])]),
			Error::<Runtime>::InvalidSwapPath
		);
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Dex(vec![KMD, KMD])]),
			Error::<Runtime>::InvalidSwapPath
		);
		assert_ok!(AggregatedDex::check_swap_paths(&vec![SwapPath::Dex(vec![KMD, SUSD])]));

		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Taiga(0, 0, 1)]),
			Error::<Runtime>::InvalidPoolId
		);
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Taiga(0, 0, 0)]),
			Error::<Runtime>::InvalidSwapPath
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_ok!(AggregatedDex::check_swap_paths(&vec![SwapPath::Taiga(0, 0, 1)]));
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Taiga(0, 2, 0)]),
			Error::<Runtime>::InvalidTokenIndex
		);

		assert_ok!(AggregatedDex::check_swap_paths(&vec![
			SwapPath::Taiga(0, 0, 1),
			SwapPath::Dex(vec![KMD, SUSD])
		]),);
		assert_noop!(
			AggregatedDex::check_swap_paths(&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![SUSD, KMD])]),
			Error::<Runtime>::InvalidSwapPath
		);

		assert_ok!(AggregatedDex::check_swap_paths(&vec![
			SwapPath::Dex(vec![SUSD, KMD]),
			SwapPath::Taiga(0, 1, 0)
		]),);
	});
}

#[test]
fn get_aggregated_swap_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			None
		);

		assert_ok!(inject_liquidity(
			KMD,
			SUSD,
			100_000_000_000u128,
			20_000_000_000_000u128
		));
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Some((1_000_000_000u128, 4_999_750u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 4_999_751u128)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD])],
				SwapLimit::ExactTarget(1_000_000_000u128, 4_999_750u128)
			),
			Some((999_999_998u128, 4_999_750u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD])],
				SwapLimit::ExactTarget(999_999_997u128, 4_999_750u128)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			None
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Some((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactSupply(1_000_000_000u128, 10_000_000_000u128)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Some((1_000_164_076u128, 10_000_000_980u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactTarget(1_000_000_000u128, 10_000_000_000u128)
			),
			None
		);

		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Some((1_000_000_000u128, 1_817_910_863_730u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 1_817_910_863_731u128)
			),
			None
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactTarget(2_000_000_000u128, 1_817_910_863_730u128)
			),
			Some((1_000_000_098u128, 1_817_911_025_719u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactTarget(1_000_000_097u128, 1_817_910_863_730u128)
			),
			None
		);

		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)],
				SwapLimit::ExactSupply(1_817_910_863_730u128, 0)
			),
			Some((1_817_910_863_730u128, 833_105_687u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)],
				SwapLimit::ExactTarget(3_000_000_000_000u128, 1_000_000_000u128)
			),
			Some((2_222_627_355_534u128, 1_000_000_098u128))
		);
		assert_eq!(
			AggregatedDex::get_aggregated_swap_amount(
				&vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)],
				SwapLimit::ExactTarget(2_222_627_355_533u128, 1_000_000_000u128)
			),
			None
		);
	});
}

#[test]
fn do_aggregated_swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Error::<Runtime>::InvalidPoolId
		);
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			module_dex::Error::<Runtime>::MustBeEnabled
		);
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Error::<Runtime>::InvalidPoolId
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			module_dex::Error::<Runtime>::MustBeEnabled
		);
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			module_dex::Error::<Runtime>::MustBeEnabled
		);

		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);
		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Ok((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 9_998_360_750u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 0);

		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1)],
				SwapLimit::ExactTarget(2_000_000_000u128, 10_000_000_000u128)
			),
			Ok((1_000_492_274u128, 10_000_000_980u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 97_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 19_998_361_730u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 0);

		assert_ok!(inject_liquidity(
			KMD,
			SUSD,
			100_000_000_000u128,
			20_000_000_000_000u128
		));
		assert_noop!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 200_000_000_000u128)
			),
			Error::<Runtime>::CannotSwap
		);

		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Ok((1_000_000_000u128, 198_019_801_980u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 97_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 18_998_361_730u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 198_019_801_980u128);

		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactTarget(1_000_000_000u128, 10_000_000_000u128)
			),
			Ok((51_030_771u128, 10_000_000_090u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 97_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 18_947_330_959u128);
		// actually swap by ExactSupply, actual target amount may be slightly more than exact target amount
		// of limit
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 208_019_802_070u128);

		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactSupply(1_000_000_000u128, 0)
			),
			Ok((1_000_000_000u128, 1_780_911_406_971u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 96_999_507_726u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 18_947_330_959u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 1_988_931_209_041u128);

		assert_eq!(
			AggregatedDex::do_aggregated_swap(
				&ALICE,
				&vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])],
				SwapLimit::ExactTarget(1_000_000_000_000u128, 1_000_000_000_000u128)
			),
			Ok((653_482_016u128, 1_000_000_140_971u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 96_346_025_710u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 18_947_330_959u128);
		// actually swap by ExactSupply, actual target amount may be slightly more than exact target amount
		// of limit
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 2_988_931_350_012u128);
	});
}

#[test]
fn update_aggregated_swap_paths_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AggregatedDex::update_aggregated_swap_paths(Origin::signed(ALICE), vec![]),
			BadOrigin
		);

		assert_noop!(
			AggregatedDex::update_aggregated_swap_paths(
				Origin::signed(BOB),
				vec![
					(
						(DOT, SUSD),
						Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
					),
					(
						(SUSD, DOT),
						Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
					)
				]
			),
			Error::<Runtime>::InvalidPoolId
		);

		assert_ok!(inital_taiga_dot_kmd_pool());

		assert_noop!(
			AggregatedDex::update_aggregated_swap_paths(
				Origin::signed(BOB),
				vec![
					(
						(DOT, SUSD),
						Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
					),
					(
						(SUSD, DOT),
						Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
					)
				]
			),
			Error::<Runtime>::InvalidSwapPath
		);

		assert_eq!(AggregatedDex::aggregated_swap_paths((DOT, SUSD)), None);
		assert_eq!(AggregatedDex::aggregated_swap_paths((SUSD, DOT)), None);
		assert_ok!(AggregatedDex::update_aggregated_swap_paths(
			Origin::signed(BOB),
			vec![
				(
					(DOT, SUSD),
					Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
				),
				(
					(SUSD, DOT),
					Some(vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)])
				)
			]
		));
		assert_eq!(
			AggregatedDex::aggregated_swap_paths((DOT, SUSD)).unwrap(),
			vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])]
		);
		assert_eq!(
			AggregatedDex::aggregated_swap_paths((SUSD, DOT)).unwrap(),
			vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)]
		);

		assert_noop!(
			AggregatedDex::update_aggregated_swap_paths(
				Origin::signed(BOB),
				vec![(
					(DOT, SUSD),
					Some(vec![
						SwapPath::Taiga(0, 0, 1),
						SwapPath::Taiga(0, 1, 0),
						SwapPath::Taiga(0, 0, 1),
						SwapPath::Dex(vec![KMD, SUSD])
					])
				),]
			),
			Error::<Runtime>::InvalidSwapPath
		);

		assert_ok!(AggregatedDex::update_aggregated_swap_paths(
			Origin::signed(BOB),
			vec![((DOT, SUSD), None), ((SUSD, DOT), None)]
		));
		assert_eq!(AggregatedDex::aggregated_swap_paths((DOT, SUSD)), None);
		assert_eq!(AggregatedDex::aggregated_swap_paths((SUSD, DOT)), None);
	});
}

#[test]
fn aggregated_swap_get_swap_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			None
		);

		assert_ok!(inject_liquidity(DOT, KMD, 1_000_000_000u128, 30_000_000_000u128));
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 15_000_000_000u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			Some((3_000_000_000u128, 22_500_000_000u128))
		);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Some((1_000_000_000u128, 15_000_000_000u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, KMD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			Some((3_000_000_000u128, 29_985_240_300u128))
		);

		assert_ok!(inject_liquidity(KMD, SUSD, 30_000_000_000u128, 60_000_000_000u128));

		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, SUSD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			None
		);

		assert_ok!(AggregatedDex::update_aggregated_swap_paths(
			Origin::signed(BOB),
			vec![(
				(DOT, SUSD),
				Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
			),]
		));
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(DOT, SUSD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			Some((3_000_000_000u128, 29_992_618_334u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(SUSD, DOT, SwapLimit::ExactSupply(30_000_000_000u128, 0)),
			None
		);

		assert_ok!(AggregatedDex::update_aggregated_swap_paths(
			Origin::signed(BOB),
			vec![(
				(SUSD, DOT),
				Some(vec![SwapPath::Dex(vec![SUSD, KMD]), SwapPath::Taiga(0, 1, 0)])
			),]
		));
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(SUSD, KMD, SwapLimit::ExactSupply(30_000_000_000u128, 0)),
			Some((30_000_000_000u128, 10_000_000_000u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(KMD, DOT, SwapLimit::ExactSupply(10_000_000_000u128, 0)),
			Some((10_000_000_000u128, 999_836_075u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(SUSD, DOT, SwapLimit::ExactSupply(30_000_000_000u128, 0)),
			Some((30_000_000_000u128, 999_836_075u128))
		);

		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(
				KMD,
				DOT,
				SwapLimit::ExactTarget(20_000_000_000u128, 1_000_000_000u128)
			),
			Some((10_001_640_760u128, 1_000_000_098u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(
				SUSD,
				KMD,
				SwapLimit::ExactTarget(u128::MAX, 10_000_000_000u128)
			),
			Some((30_000_000_001u128, 10_000_000_000u128))
		);
		assert_eq!(
			AggregatedSwap::<Runtime>::get_swap_amount(SUSD, DOT, SwapLimit::ExactTarget(u128::MAX, 1_000_000_000u128)),
			Some((30_007_384_026u128, 1_000_000_098u128))
		);
	});
}

#[test]
fn aggregated_swap_swap_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AggregatedSwap::<Runtime>::swap(&ALICE, DOT, KMD, SwapLimit::ExactSupply(1_000_000_000u128, 0)),
			Error::<Runtime>::CannotSwap
		);

		assert_ok!(inject_liquidity(DOT, KMD, 1_000_000_000u128, 30_000_000_000u128));
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 100_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 0);

		assert_noop!(
			AggregatedSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 15_000_000_001u128)
			),
			Error::<Runtime>::CannotSwap
		);
		assert_ok!(AggregatedSwap::<Runtime>::swap(
			&ALICE,
			DOT,
			KMD,
			SwapLimit::ExactSupply(1_000_000_000u128, 15_000_000_000u128)
		));
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 99_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 15_000_000_000u128);

		assert_ok!(inital_taiga_dot_kmd_pool());
		assert_eq!(
			AggregatedSwap::<Runtime>::swap(
				&ALICE,
				DOT,
				KMD,
				SwapLimit::ExactSupply(1_000_000_000u128, 9_000_000_000u128)
			),
			Ok((1_000_000_000u128, 9_998_360_750u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 98_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 24_998_360_750u128);

		assert_ok!(inject_liquidity(KMD, SUSD, 30_000_000_000u128, 60_000_000_000u128));

		assert_noop!(
			AggregatedSwap::<Runtime>::swap(&ALICE, DOT, SUSD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			Error::<Runtime>::CannotSwap
		);

		assert_ok!(AggregatedDex::update_aggregated_swap_paths(
			Origin::signed(BOB),
			vec![(
				(DOT, SUSD),
				Some(vec![SwapPath::Taiga(0, 0, 1), SwapPath::Dex(vec![KMD, SUSD])])
			),]
		));

		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 0);
		assert_eq!(
			AggregatedSwap::<Runtime>::swap(&ALICE, DOT, SUSD, SwapLimit::ExactSupply(3_000_000_000u128, 0)),
			Ok((3_000_000_000u128, 29_987_688_109u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 95_000_000_000u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 24_998_360_750u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 29_987_688_109u128);

		assert_eq!(
			AggregatedSwap::<Runtime>::swap(&ALICE, DOT, SUSD, SwapLimit::ExactTarget(u128::MAX, 10_000_000_000u128)),
			Ok((3_002_366_414u128, 10_000_000_216u128))
		);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 91_997_633_586u128);
		assert_eq!(Tokens::free_balance(KMD, &ALICE), 24_998_360_750u128);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 39_987_688_325u128);
	});
}
