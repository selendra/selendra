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

//! Unit tests for the cdp treasury module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use sp_runtime::traits::BadOrigin;
use support::SwapError;

#[test]
fn surplus_pool_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(Currencies::deposit(
			GetStableCurrencyId::get(),
			&CDPTreasuryModule::account_id(),
			500
		));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 500);
	});
}

#[test]
fn total_collaterals_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 0);
		assert_ok!(Currencies::deposit(BTC, &CDPTreasuryModule::account_id(), 10));
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 10);
	});
}

#[test]
fn on_system_debit_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
		assert_ok!(CDPTreasuryModule::on_system_debit(1000));
		assert_eq!(CDPTreasuryModule::debit_pool(), 1000);
		assert_noop!(
			CDPTreasuryModule::on_system_debit(Balance::max_value()),
			ArithmeticError::Overflow,
		);
	});
}

#[test]
fn on_system_surplus_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(CDPTreasuryModule::on_system_surplus(1000));
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 1000);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 1000);
	});
}

#[test]
fn offset_surplus_and_debit_on_finalize_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
		assert_ok!(CDPTreasuryModule::on_system_surplus(1000));
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 1000);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 1000);
		CDPTreasuryModule::on_finalize(1);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 1000);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 1000);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
		assert_ok!(CDPTreasuryModule::on_system_debit(300));
		assert_eq!(CDPTreasuryModule::debit_pool(), 300);
		CDPTreasuryModule::on_finalize(2);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 700);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 700);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
		assert_ok!(CDPTreasuryModule::on_system_debit(800));
		assert_eq!(CDPTreasuryModule::debit_pool(), 800);
		CDPTreasuryModule::on_finalize(3);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_eq!(CDPTreasuryModule::debit_pool(), 100);
	});
}

#[test]
fn issue_debit_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 1000);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);

		assert_ok!(CDPTreasuryModule::issue_debit(&ALICE, 1000, true));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 2000);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);

		assert_ok!(CDPTreasuryModule::issue_debit(&ALICE, 1000, false));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 3000);
		assert_eq!(CDPTreasuryModule::debit_pool(), 1000);
	});
}

#[test]
fn burn_debit_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 1000);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
		assert_ok!(CDPTreasuryModule::burn_debit(&ALICE, 300));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 700);
		assert_eq!(CDPTreasuryModule::debit_pool(), 0);
	});
}

#[test]
fn deposit_surplus_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 1000);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(CDPTreasuryModule::deposit_surplus(&ALICE, 300));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 700);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 300);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 300);
	});
}

#[test]
fn withdraw_surplus_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_surplus(&ALICE, 300));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 700);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 300);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 300);

		assert_ok!(CDPTreasuryModule::withdraw_surplus(&ALICE, 200));
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), 900);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 100);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 100);
	});
}

#[test]
fn deposit_collateral_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 0);
		assert_eq!(Currencies::free_balance(BTC, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 1000);
		assert!(!CDPTreasuryModule::deposit_collateral(&ALICE, BTC, 10000).is_ok());
		assert_ok!(CDPTreasuryModule::deposit_collateral(&ALICE, BTC, 500));
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 500);
		assert_eq!(Currencies::free_balance(BTC, &CDPTreasuryModule::account_id()), 500);
		assert_eq!(Currencies::free_balance(BTC, &ALICE), 500);
	});
}

#[test]
fn withdraw_collateral_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&ALICE, BTC, 500));
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 500);
		assert_eq!(Currencies::free_balance(BTC, &CDPTreasuryModule::account_id()), 500);
		assert_eq!(Currencies::free_balance(BTC, &BOB), 1000);
		assert!(!CDPTreasuryModule::withdraw_collateral(&BOB, BTC, 501).is_ok());
		assert_ok!(CDPTreasuryModule::withdraw_collateral(&BOB, BTC, 400));
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 100);
		assert_eq!(Currencies::free_balance(BTC, &CDPTreasuryModule::account_id()), 100);
		assert_eq!(Currencies::free_balance(BTC, &BOB), 1400);
	});
}

#[test]
fn get_total_collaterals_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&ALICE, BTC, 500));
		assert_eq!(CDPTreasuryModule::get_total_collaterals(BTC), 500);
	});
}

#[test]
fn get_debit_proportion_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(
			CDPTreasuryModule::get_debit_proportion(100),
			Ratio::saturating_from_rational(100, Currencies::total_issuance(SUSD))
		);
	});
}

#[test]
fn swap_collateral_to_stable_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&BOB, BTC, 200));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, DOT, 1000));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			DOT,
			SUSD,
			1000,
			1000,
			0,
			false
		));

		assert_noop!(
			CDPTreasuryModule::swap_collateral_to_stable(BTC, SwapLimit::ExactTarget(200, 399)),
			SwapError::CannotSwap
		);
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(ALICE),
			BTC,
			DOT,
			100,
			1000,
			0,
			false
		));

		assert_eq!(
			CDPTreasuryModule::swap_collateral_to_stable(BTC, SwapLimit::ExactTarget(200, 399)).unwrap(),
			(198, 399)
		);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 399);

		assert_noop!(
			CDPTreasuryModule::swap_collateral_to_stable(DOT, SwapLimit::ExactSupply(1000, 1000)),
			SwapError::CannotSwap
		);

		assert_eq!(
			CDPTreasuryModule::swap_collateral_to_stable(DOT, SwapLimit::ExactSupply(1000, 0)).unwrap(),
			(1000, 225)
		);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 624);
	});
}

#[test]
fn swap_collateral_to_stable_stable_asset_exact_target() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&BOB, STABLE_ASSET_LP, 200));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, DOT, 1000));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, BTC, 1000));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			DOT,
			SUSD,
			1000,
			1000,
			0,
			false
		));
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(ALICE),
			BTC,
			SUSD,
			1000,
			1000,
			0,
			false
		));
		assert_eq!(
			CDPTreasuryModule::swap_collateral_to_stable(STABLE_ASSET_LP, SwapLimit::ExactTarget(200, 100))
				.unwrap(),
			(200, 180)
		);
	});
}

#[test]
fn swap_collateral_to_stable_stable_asset_exact_supply() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&BOB, STABLE_ASSET_LP, 200));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, DOT, 1000));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, BTC, 1000));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			DOT,
			SUSD,
			1000,
			1000,
			0,
			false
		));
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(ALICE),
			BTC,
			SUSD,
			1000,
			1000,
			0,
			false
		));
	});
}

#[test]
fn swap_collateral_to_stable_stable_asset_failures() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::deposit_collateral(&BOB, STABLE_ASSET_LP, 200));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, DOT, 1000));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&CHARLIE, BTC, 1000));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			DOT,
			SUSD,
			1000,
			1000,
			0,
			false
		));
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(ALICE),
			BTC,
			SUSD,
			1000,
			1000,
			0,
			false
		));
		assert_noop!(
			CDPTreasuryModule::swap_collateral_to_stable(STABLE_ASSET_LP, SwapLimit::ExactTarget(200, 399)),
			Error::<Runtime>::CannotSwap
		);
		assert_noop!(
			CDPTreasuryModule::swap_collateral_to_stable(STABLE_ASSET_LP, SwapLimit::ExactSupply(200, 3999)),
			Error::<Runtime>::CannotSwap
		);
	});
}

#[test]
fn remove_liquidity_for_lp_collateral_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			SUSD,
			DOT,
			1000,
			100,
			0,
			false
		));
		assert_ok!(CDPTreasuryModule::deposit_collateral(&BOB, LP_SUSD_DOT, 200));
		assert_eq!(Currencies::total_issuance(LP_SUSD_DOT), 2000);
		assert_eq!(DEXModule::get_liquidity_pool(SUSD, DOT), (1000, 100));
		assert_eq!(
			Currencies::free_balance(LP_SUSD_DOT, &CDPTreasuryModule::account_id()),
			200
		);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 0);
		assert_eq!(Currencies::free_balance(DOT, &CDPTreasuryModule::account_id()), 0);

		assert_noop!(
			CDPTreasuryModule::remove_liquidity_for_lp_collateral(DOT, 200),
			Error::<Runtime>::NotDexShare
		);

		assert_eq!(
			CDPTreasuryModule::remove_liquidity_for_lp_collateral(LP_SUSD_DOT, 120),
			Ok((60, 6))
		);
		assert_eq!(Currencies::total_issuance(LP_SUSD_DOT), 1880);
		assert_eq!(DEXModule::get_liquidity_pool(SUSD, DOT), (940, 94));
		assert_eq!(
			Currencies::free_balance(LP_SUSD_DOT, &CDPTreasuryModule::account_id()),
			80
		);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 60);
		assert_eq!(Currencies::free_balance(DOT, &CDPTreasuryModule::account_id()), 6);
	});
}

#[test]
fn extract_surplus_to_treasury_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(CDPTreasuryModule::on_system_surplus(1000));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 1000);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 1000);
		assert_eq!(Currencies::free_balance(SUSD, &TreasuryAccount::get()), 0);

		assert_noop!(
			CDPTreasuryModule::extract_surplus_to_treasury(Origin::signed(5), 200),
			BadOrigin
		);
		assert_ok!(CDPTreasuryModule::extract_surplus_to_treasury(Origin::signed(1), 200));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 800);
		assert_eq!(Currencies::free_balance(SUSD, &CDPTreasuryModule::account_id()), 800);
		assert_eq!(Currencies::free_balance(SUSD, &TreasuryAccount::get()), 200);
	});
}

#[test]
fn exchange_collateral_to_stable_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(DEXModule::add_liquidity(
			Origin::signed(BOB),
			BTC,
			SUSD,
			200,
			1000,
			0,
			false
		));

		assert_ok!(Currencies::deposit(BTC, &CDPTreasuryModule::account_id(), 1000));
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 1000);
		assert_eq!(CDPTreasuryModule::surplus_pool(), 0);

		assert_noop!(
			CDPTreasuryModule::exchange_collateral_to_stable(Origin::signed(5), BTC, SwapLimit::ExactTarget(200, 200)),
			BadOrigin,
		);
		assert_noop!(
			CDPTreasuryModule::exchange_collateral_to_stable(Origin::signed(1), BTC, SwapLimit::ExactTarget(200, 1000)),
			SwapError::CannotSwap
		);

		assert_ok!(CDPTreasuryModule::exchange_collateral_to_stable(
			Origin::signed(1),
			BTC,
			SwapLimit::ExactTarget(200, 399)
		));
		assert_eq!(CDPTreasuryModule::surplus_pool(), 399);
		assert_eq!(CDPTreasuryModule::total_collaterals(BTC), 867);
	});
}
