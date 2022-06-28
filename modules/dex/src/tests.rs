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

//! Unit tests for the dex module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	DOTBTCPair, DexModule, Event, ExtBuilder, ListingOrigin, Origin, Runtime, SELJointSwap,
	SUSDBTCPair, SUSDDOTPair, SUSDJointSwap, System, Tokens, ALICE, BOB, BTC, CAROL, DOT, SEL,
	SUSD, SUSD_DOT_POOL_RECORD,
};
use orml_traits::MultiReservableCurrency;
use sp_core::H160;
use sp_runtime::traits::BadOrigin;
use std::str::FromStr;
use support::{Swap, SwapError};

#[test]
fn list_provisioning_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			BadOrigin
		);

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		System::assert_last_event(Event::DexModule(crate::Event::ListProvisioning {
			trading_pair: SUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ListingOrigin::get()),
				SUSD,
				SUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::InvalidCurrencyId
		);

		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ListingOrigin::get()),
				SUSD,
				DOT,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::MustBeDisabled
		);

		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ListingOrigin::get()),
				CurrencyId::ForeignAsset(0),
				SUSD,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::AssetUnregistered
		);
		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ListingOrigin::get()),
				SUSD,
				CurrencyId::ForeignAsset(0),
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::AssetUnregistered
		);
	});
}

#[test]
fn update_provisioning_parameters_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::update_provisioning_parameters(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			BadOrigin
		);

		assert_noop!(
			DexModule::update_provisioning_parameters(
				Origin::signed(ListingOrigin::get()),
				SUSD,
				DOT,
				1_000_000_000_000u128,
				1_000_000_000_000u128,
				5_000_000_000_000u128,
				2_000_000_000_000u128,
				10,
			),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(DexModule::update_provisioning_parameters(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			2_000_000_000_000u128,
			0,
			3_000_000_000_000u128,
			2_000_000_000_000u128,
			50,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (2_000_000_000_000u128, 0),
				target_provision: (3_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 50,
			})
		);
	});
}

#[test]
fn enable_diabled_trading_pair_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(DexModule::enable_trading_pair(Origin::signed(ALICE), SUSD, DOT), BadOrigin);

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::EnableTradingPair {
			trading_pair: SUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), DOT, SUSD),
			Error::<Runtime>::AlreadyEnabled
		);
	});
}

#[test]
fn enable_provisioning_without_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::EnableTradingPair {
			trading_pair: SUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, BTC),
			Error::<Runtime>::StillProvisioning
		);
	});
}

#[test]
fn end_provisioning_trading_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			2_000_000_000_000u128
		));

		assert_noop!(
			DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), SUSD, BTC),
			Error::<Runtime>::UnqualifiedProvision
		);
		System::set_block_number(10);

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 10,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDBTCPair::get()), Default::default());
		assert_eq!(DexModule::liquidity_pool(SUSDBTCPair::get()), (0, 0));
		assert_eq!(Tokens::total_issuance(SUSDBTCPair::get().dex_share_currency_id()), 0);
		assert_eq!(
			Tokens::free_balance(
				SUSDBTCPair::get().dex_share_currency_id(),
				&DexModule::account_id()
			),
			0
		);

		assert_ok!(DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), SUSD, BTC));
		System::assert_last_event(Event::DexModule(crate::Event::ProvisioningToEnabled {
			trading_pair: SUSDBTCPair::get(),
			pool_0: 1_000_000_000_000u128,
			pool_1: 2_000_000_000_000u128,
			share_amount: 2_000_000_000_000u128,
		}));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			DexModule::initial_share_exchange_rates(SUSDBTCPair::get()),
			(ExchangeRate::one(), ExchangeRate::checked_from_rational(1, 2).unwrap())
		);
		assert_eq!(
			DexModule::liquidity_pool(SUSDBTCPair::get()),
			(1_000_000_000_000u128, 2_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::total_issuance(SUSDBTCPair::get().dex_share_currency_id()),
			2_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(
				SUSDBTCPair::get().dex_share_currency_id(),
				&DexModule::account_id()
			),
			2_000_000_000_000u128
		);
	});
}

#[test]
fn abort_provisioning_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, DOT),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			SUSD,
			BTC,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
		));

		// not expired, nothing happened.
		System::set_block_number(2000);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, DOT));
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, BTC));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 1_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDDOTPair::get()), Default::default());
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDBTCPair::get()), Default::default());

		// both expired, the provision for SUSD-DOT could be aborted, the provision for SUSD-BTC
		// couldn't be aborted because it's already met the target.
		System::set_block_number(3001);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::ProvisioningAborted {
			trading_pair: SUSDDOTPair::get(),
			accumulated_provision_0: 1_000_000_000_000u128,
			accumulated_provision_1: 1_000_000_000_000u128,
		}));

		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, BTC));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDDOTPair::get()), Default::default());
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDBTCPair::get()), Default::default());
	});
}

#[test]
fn refund_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			5_000_000_000_000_000_000u128,
			4_000_000_000_000_000_000u128,
			1000,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			1000,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			DOT,
			1_000_000_000_000_000_000u128,
			1_000_000_000_000_000_000u128
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			SUSD,
			DOT,
			0,
			600_000_000_000_000_000u128,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			SUSD,
			BTC,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
		));

		assert_noop!(
			DexModule::refund_provision(Origin::signed(ALICE), ALICE, SUSD, DOT),
			Error::<Runtime>::MustBeDisabled
		);

		// abort provisioning of SUSD-DOT
		System::set_block_number(3001);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), SUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::initial_share_exchange_rates(SUSDDOTPair::get()), Default::default());

		assert_eq!(
			DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE),
			(1_000_000_000_000_000_000u128, 1_000_000_000_000_000_000u128)
		);
		assert_eq!(
			DexModule::provisioning_pool(SUSDDOTPair::get(), BOB),
			(0, 600_000_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::free_balance(SUSD, &DexModule::account_id()),
			1_100_000_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(DOT, &DexModule::account_id()),
			1_600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 0);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 0);
		assert_eq!(Tokens::free_balance(SUSD, &BOB), 900_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 400_000_000_000_000_000u128);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(DexModule::refund_provision(Origin::signed(ALICE), ALICE, SUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::RefundProvision {
			who: ALICE,
			currency_0: SUSD,
			contribution_0: 1_000_000_000_000_000_000u128,
			currency_1: DOT,
			contribution_1: 1_000_000_000_000_000_000u128,
		}));

		assert_eq!(DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(
			Tokens::free_balance(SUSD, &DexModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(DOT, &DexModule::account_id()),
			600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);

		assert_ok!(DexModule::refund_provision(Origin::signed(ALICE), BOB, SUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::RefundProvision {
			who: BOB,
			currency_0: SUSD,
			contribution_0: 0,
			currency_1: DOT,
			contribution_1: 600_000_000_000_000_000u128,
		}));

		assert_eq!(DexModule::provisioning_pool(SUSDDOTPair::get(), BOB), (0, 0));
		assert_eq!(
			Tokens::free_balance(SUSD, &DexModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(SUSD, &BOB), 900_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);

		// not allow refund if the provisioning has been ended before.
		assert_ok!(DexModule::end_provisioning(Origin::signed(ALICE), SUSD, BTC));
		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(
			DexModule::provisioning_pool(SUSDBTCPair::get(), BOB),
			(100_000_000_000_000_000u128, 100_000_000_000_000_000u128)
		);
		assert_noop!(
			DexModule::refund_provision(Origin::signed(BOB), BOB, SUSD, BTC),
			Error::<Runtime>::NotAllowedRefund
		);
	});
}

#[test]
fn disable_trading_pair_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);

		assert_noop!(DexModule::disable_trading_pair(Origin::signed(ALICE), SUSD, DOT), BadOrigin);

		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::DisableTradingPair {
			trading_pair: SUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::disable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, DOT),
			Error::<Runtime>::MustBeEnabled
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_noop!(
			DexModule::disable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, BTC),
			Error::<Runtime>::MustBeEnabled
		);
	});
}

#[test]
fn on_liquidity_pool_updated_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				BTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));
			assert_eq!(SUSD_DOT_POOL_RECORD.with(|v| *v.borrow()), (0, 0));

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));
			assert_eq!(SUSD_DOT_POOL_RECORD.with(|v| *v.borrow()), (5000000000000, 1000000000000));
		});
}

#[test]
fn add_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::add_provision(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				5_000_000_000_000u128,
				1_000_000_000_000u128,
			),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			5_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			10,
		));

		assert_noop!(
			DexModule::add_provision(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				4_999_999_999_999u128,
				999_999_999_999u128,
			),
			Error::<Runtime>::InvalidContributionIncrement
		);

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		let alice_ref_count_0 = System::consumers(&ALICE);

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			DOT,
			5_000_000_000_000u128,
			0,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE),
			(5_000_000_000_000u128, 0)
		);
		assert_eq!(Tokens::free_balance(SUSD, &ALICE), 999_995_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 5_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		let alice_ref_count_1 = System::consumers(&ALICE);
		assert_eq!(alice_ref_count_1, alice_ref_count_0 + 1);
		System::assert_last_event(Event::DexModule(crate::Event::AddProvision {
			who: ALICE,
			currency_0: SUSD,
			contribution_0: 5_000_000_000_000u128,
			currency_1: DOT,
			contribution_1: 0,
		}));
	});
}

#[test]
fn claim_dex_share_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			5_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			0,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			SUSD,
			DOT,
			1_000_000_000_000_000u128,
			200_000_000_000_000u128,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			SUSD,
			DOT,
			4_000_000_000_000_000u128,
			800_000_000_000_000u128,
		));

		assert_noop!(
			DexModule::claim_dex_share(Origin::signed(ALICE), ALICE, SUSD, DOT),
			Error::<Runtime>::StillProvisioning
		);

		assert_ok!(DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), SUSD, DOT));

		let lp_currency_id = SUSDDOTPair::get().dex_share_currency_id();

		assert!(InitialShareExchangeRates::<Runtime>::contains_key(SUSDDOTPair::get()),);
		assert_eq!(
			DexModule::initial_share_exchange_rates(SUSDDOTPair::get()),
			(ExchangeRate::one(), ExchangeRate::saturating_from_rational(5, 1))
		);
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &DexModule::account_id()),
			10_000_000_000_000_000u128
		);
		assert_eq!(
			DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE),
			(1_000_000_000_000_000u128, 200_000_000_000_000u128)
		);
		assert_eq!(
			DexModule::provisioning_pool(SUSDDOTPair::get(), BOB),
			(4_000_000_000_000_000u128, 800_000_000_000_000u128)
		);
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 0);
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 0);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(DexModule::claim_dex_share(Origin::signed(ALICE), ALICE, SUSD, DOT));
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &DexModule::account_id()),
			8_000_000_000_000_000u128
		);
		assert_eq!(DexModule::provisioning_pool(SUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 2_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);
		assert!(InitialShareExchangeRates::<Runtime>::contains_key(SUSDDOTPair::get()),);

		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT
		));
		assert_ok!(DexModule::claim_dex_share(Origin::signed(BOB), BOB, SUSD, DOT));
		assert_eq!(Tokens::free_balance(lp_currency_id, &DexModule::account_id()), 0);
		assert_eq!(DexModule::provisioning_pool(SUSDDOTPair::get(), BOB), (0, 0));
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 8_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);
		assert!(!InitialShareExchangeRates::<Runtime>::contains_key(SUSDDOTPair::get()),);
	});
}

#[test]
fn get_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (1000, 20));
		assert_eq!(DexModule::liquidity_pool(SUSDDOTPair::get()), (1000, 20));
		assert_eq!(DexModule::get_liquidity(SUSD, DOT), (1000, 20));
		assert_eq!(DexModule::get_liquidity(DOT, SUSD), (20, 1000));
	});
}

#[test]
fn get_target_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(DexModule::get_target_amount(10000, 0, 1000), 0);
		assert_eq!(DexModule::get_target_amount(0, 20000, 1000), 0);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 0), 0);
		assert_eq!(DexModule::get_target_amount(10000, 1, 1000000), 0);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 10000), 9949);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_supply_amount_work() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(DexModule::get_supply_amount(10000, 0, 1000), 0);
		assert_eq!(DexModule::get_supply_amount(0, 20000, 1000), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 0), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 1, 1), 0);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 9949), 9999);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 9999), 9949);
		assert_eq!(DexModule::get_supply_amount(10000, 20000, 1801), 1000);
		assert_eq!(DexModule::get_target_amount(10000, 20000, 1000), 1801);
	});
}

#[test]
fn get_target_amounts_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(SUSDBTCPair::get(), (100000, 10));
			assert_noop!(
				DexModule::get_target_amounts(&[DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, SUSD, BTC, DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, SUSD, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, SUSD, SEL], 10000),
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(DexModule::get_target_amounts(&[DOT, SUSD], 10000), Ok(vec![10000, 24874]));
			assert_eq!(
				DexModule::get_target_amounts(&[DOT, SUSD, BTC], 10000),
				Ok(vec![10000, 24874, 1])
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, SUSD, BTC], 100),
				Error::<Runtime>::ZeroTargetAmount,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, BTC], 100),
				Error::<Runtime>::InsufficientLiquidity,
			);
		});
}

#[test]
fn calculate_amount_for_big_number_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::<Runtime>::insert(
			SUSDDOTPair::get(),
			(171_000_000_000_000_000_000_000, 56_000_000_000_000_000_000_000),
		);
		assert_eq!(
			DexModule::get_supply_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				1_000_000_000_000_000_000_000
			),
			3_140_495_867_768_595_041_323
		);
		assert_eq!(
			DexModule::get_target_amount(
				171_000_000_000_000_000_000_000,
				56_000_000_000_000_000_000_000,
				3_140_495_867_768_595_041_323
			),
			1_000_000_000_000_000_000_000
		);
	});
}

#[test]
fn get_supply_amounts_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(SUSDBTCPair::get(), (100000, 10));
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, SUSD, BTC, DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, SUSD, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, SUSD, SEL], 10000),
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(DexModule::get_supply_amounts(&[DOT, SUSD], 24874), Ok(vec![10000, 24874]));
			assert_eq!(DexModule::get_supply_amounts(&[DOT, SUSD], 25000), Ok(vec![10102, 25000]));
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, SUSD, BTC], 10000),
				Error::<Runtime>::ZeroSupplyAmount,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, BTC], 10000),
				Error::<Runtime>::InsufficientLiquidity,
			);
		});
}

#[test]
fn _swap_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (50000, 10000));

			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (50000, 10000));
			assert_noop!(
				DexModule::_swap(SUSD, DOT, 50000, 5001),
				Error::<Runtime>::InvariantCheckFailed
			);
			assert_ok!(DexModule::_swap(SUSD, DOT, 50000, 5000));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (100000, 5000));
			assert_ok!(DexModule::_swap(DOT, SUSD, 100, 800));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (99200, 5100));
		});
}

#[test]
fn _swap_by_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(SUSDBTCPair::get(), (100000, 10));

			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (50000, 10000));
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (100000, 10));
			assert_ok!(DexModule::_swap_by_path(&[DOT, SUSD], &[10000, 25000]));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (25000, 20000));
			assert_ok!(DexModule::_swap_by_path(&[DOT, SUSD, BTC], &[100000, 20000, 1]));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (5000, 120000));
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (120000, 9));
		});
}

#[test]
fn add_liquidity_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_noop!(
				DexModule::add_liquidity(
					Origin::signed(ALICE),
					SEL,
					SUSD,
					100_000_000,
					100_000_000,
					0,
				),
				Error::<Runtime>::MustBeEnabled
			);
			assert_noop!(
				DexModule::add_liquidity(Origin::signed(ALICE), SUSD, DOT, 0, 100_000_000, 0,),
				Error::<Runtime>::InvalidLiquidityIncrement
			);

			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (0, 0));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE), 0);
			assert_eq!(
				Tokens::reserved_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				0
			);
			assert_eq!(Tokens::free_balance(SUSD, &ALICE), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));
			System::assert_last_event(Event::DexModule(crate::Event::AddLiquidity {
				who: ALICE,
				currency_0: SUSD,
				pool_0: 5_000_000_000_000,
				currency_1: DOT,
				pool_1: 1_000_000_000_000,
				share_increment: 10_000_000_000_000,
			}));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (5_000_000_000_000, 1_000_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 5_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(
				Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				10_000_000_000_000
			);
			assert_eq!(
				Tokens::reserved_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				0
			);
			assert_eq!(Tokens::free_balance(SUSD, &ALICE), 999_995_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);
			assert_eq!(Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB), 0);
			assert_eq!(
				Tokens::reserved_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB),
				0
			);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::add_liquidity(Origin::signed(BOB), SUSD, DOT, 4, 1, 0),
				Error::<Runtime>::InvalidLiquidityIncrement,
			);
		});
}

#[test]
fn remove_liquidity_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));
			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					SUSDDOTPair::get().dex_share_currency_id(),
					DOT,
					100_000_000,
					0,
					0,
				),
				Error::<Runtime>::InvalidCurrencyId
			);

			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (5_000_000_000_000, 1_000_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 5_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(
				Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				10_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(SUSD, &ALICE), 999_995_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);

			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					SUSD,
					DOT,
					8_000_000_000_000,
					4_000_000_000_001,
					800_000_000_000,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					SUSD,
					DOT,
					8_000_000_000_000,
					4_000_000_000_000,
					800_000_000_001,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_ok!(DexModule::remove_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				8_000_000_000_000,
				4_000_000_000_000,
				800_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::RemoveLiquidity {
				who: ALICE,
				currency_0: SUSD,
				pool_0: 4_000_000_000_000,
				currency_1: DOT,
				pool_1: 800_000_000_000,
				share_decrement: 8_000_000_000_000,
			}));
			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (1_000_000_000_000, 200_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000);
			assert_eq!(
				Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				2_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(SUSD, &ALICE), 999_999_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_800_000_000_000);

			// assert_ok!(DexModule::remove_liquidity(
			// 	Origin::signed(ALICE),
			// 	SUSD,
			// 	DOT,
			// 	2_000_000_000_000,
			// 	0,
			// 	0,
			// ));
			// System::assert_last_event(Event::DexModule(crate::Event::RemoveLiquidity {
			// 	who: ALICE,
			// 	currency_0: SUSD,
			// 	pool_0: 1_000_000_000_000,
			// 	currency_1: DOT,
			// 	pool_1: 200_000_000_000,
			// 	share_decrement: 2_000_000_000_000,
			// }));
			// assert_eq!(DexModule::get_liquidity(SUSD, DOT), (0, 0));
			// assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 0);
			// assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
			// assert_eq!(Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
			// 0); assert_eq!(Tokens::free_balance(SUSD, &ALICE), 1_000_000_000_000_000_000);
			// assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);

			// assert_eq!(Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB),
			// 0); assert_eq!(
			// 	Tokens::reserved_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB),
			// 	10_000_000_000_000
			// );
			// assert_eq!(Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB),
			// 0); assert_eq!(
			// 	Tokens::reserved_balance(SUSDDOTPair::get().dex_share_currency_id(), &BOB),
			// 	8_000_000_000_000
			// );
		});
}

#[test]
fn do_swap_with_exact_supply_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				BTC,
				100_000_000_000_000,
				10_000_000_000,
				0
			));

			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 600_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, SUSD],
					100_000_000_000_000,
					250_000_000_000_000,
				),
				Error::<Runtime>::InsufficientTargetAmount
			);
			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, SUSD, BTC, DOT],
					100_000_000_000_000,
					0
				),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, SUSD, DOT],
					100_000_000_000_000,
					0
				),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_supply(&BOB, &[DOT, SEL], 100_000_000_000_000, 0),
				Error::<Runtime>::MustBeEnabled,
			);

			assert_ok!(DexModule::do_swap_with_exact_supply(
				&BOB,
				&[DOT, SUSD],
				100_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, SUSD],
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));
			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(251_256_281_407_036, 200_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 351_256_281_407_036);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_248_743_718_592_964);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_900_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::do_swap_with_exact_supply(
				&BOB,
				&[DOT, SUSD, BTC],
				200_000_000_000_000,
				1,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, SUSD, BTC],
				liquidity_changes: vec![200_000_000_000_000, 124_996_843_514_053, 5_530_663_837],
			}));
			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(126_259_437_892_983, 400_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (224_996_843_514_053, 4_469_336_163));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 351_256_281_407_036);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 400_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 4_469_336_163);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_248_743_718_592_964);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_700_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_005_530_663_837);
		});
}

#[test]
fn do_swap_with_exact_target_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				BTC,
				100_000_000_000_000,
				10_000_000_000,
				0,
			));

			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 600_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, SUSD],
					250_000_000_000_000,
					100_000_000_000_000,
				),
				Error::<Runtime>::ExcessiveSupplyAmount
			);
			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, SUSD, BTC, DOT],
					250_000_000_000_000,
					200_000_000_000_000,
				),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, SUSD, DOT],
					250_000_000_000_000,
					200_000_000_000_000,
				),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, SEL],
					250_000_000_000_000,
					200_000_000_000_000
				),
				Error::<Runtime>::MustBeEnabled,
			);

			assert_ok!(DexModule::do_swap_with_exact_target(
				&BOB,
				&[DOT, SUSD],
				250_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, SUSD],
				liquidity_changes: vec![101_010_101_010_102, 250_000_000_000_000],
			}));
			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(250_000_000_000_000, 201_010_101_010_102)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 350_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 201_010_101_010_102);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_250_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_898_989_898_989_898);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::do_swap_with_exact_target(
				&BOB,
				&[DOT, SUSD, BTC],
				5_000_000_000,
				2_000_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, SUSD, BTC],
				liquidity_changes: vec![137_654_580_386_993, 101_010_101_010_102, 5_000_000_000],
			}));
			assert_eq!(
				DexModule::get_liquidity(SUSD, DOT),
				(148_989_898_989_898, 338_664_681_397_095)
			);
			assert_eq!(DexModule::get_liquidity(SUSD, BTC), (201_010_101_010_102, 5_000_000_000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 350_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 338_664_681_397_095);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 5_000_000_000);
			assert_eq!(Tokens::free_balance(SUSD, &BOB), 1_000_250_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_761_335_318_602_905);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_005_000_000_000);
		});
}

#[test]
fn initialize_added_liquidity_pools_genesis_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.initialize_added_liquidity_pools(ALICE)
		.build()
		.execute_with(|| {
			System::set_block_number(1);

			assert_eq!(DexModule::get_liquidity(SUSD, DOT), (1000000, 2000000));
			assert_eq!(Tokens::free_balance(SUSD, &DexModule::account_id()), 2000000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 4000000);
			assert_eq!(
				Tokens::free_balance(SUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				2000000
			);
		});
}

#[test]
fn get_swap_amount_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (50000, 10000));
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, SUSD], SwapLimit::ExactSupply(10000, 0)),
				Some((10000, 24874))
			);
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, SUSD], SwapLimit::ExactSupply(10000, 24875)),
				None
			);
			assert_eq!(
				DexModule::get_swap_amount(
					&[DOT, SUSD],
					SwapLimit::ExactTarget(Balance::max_value(), 24874)
				),
				Some((10000, 24874))
			);
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, SUSD], SwapLimit::ExactTarget(9999, 24874)),
				None
			);
		});
}

#[test]
fn get_best_price_swap_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(SUSDDOTPair::get(), (300000, 100000));
			LiquidityPool::<Runtime>::insert(SUSDBTCPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(DOTBTCPair::get(), (10000, 10000));

			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![]
				),
				Some((vec![DOT, SUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 30),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(0, 0),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![SEL]]
				),
				Some((vec![DOT, SUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![DOT]]
				),
				Some((vec![DOT, SUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![SUSD]]
				),
				Some((vec![DOT, SUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![BTC]]
				),
				Some((vec![DOT, BTC, SUSD], 10, 44))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactSupply(10000, 0),
					vec![vec![BTC]]
				),
				Some((vec![DOT, SUSD], 10000, 27024))
			);

			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![]
				),
				Some((vec![DOT, SUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(10, 30),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(0, 0),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![SEL]]
				),
				Some((vec![DOT, SUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![DOT]]
				),
				Some((vec![DOT, SUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![SUSD]]
				),
				Some((vec![DOT, SUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![BTC]]
				),
				Some((vec![DOT, BTC, SUSD], 8, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					SUSD,
					SwapLimit::ExactTarget(100000, 20000),
					vec![vec![BTC]]
				),
				Some((vec![DOT, SUSD], 7216, 20000))
			);
		});
}

#[test]
fn swap_with_specific_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			System::set_block_number(1);
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
			));

			assert_noop!(
				DexModule::swap_with_specific_path(
					&BOB,
					&[DOT, SUSD],
					SwapLimit::ExactSupply(100_000_000_000_000, 248_743_718_592_965)
				),
				Error::<Runtime>::InsufficientTargetAmount
			);

			assert_ok!(DexModule::swap_with_specific_path(
				&BOB,
				&[DOT, SUSD],
				SwapLimit::ExactSupply(100_000_000_000_000, 200_000_000_000_000)
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, SUSD],
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));

			assert_noop!(
				DexModule::swap_with_specific_path(
					&BOB,
					&[SUSD, DOT],
					SwapLimit::ExactTarget(253_794_223_643_470, 100_000_000_000_000)
				),
				Error::<Runtime>::ExcessiveSupplyAmount
			);

			assert_ok!(DexModule::swap_with_specific_path(
				&BOB,
				&[SUSD, DOT],
				SwapLimit::ExactTarget(300_000_000_000_000, 100_000_000_000_000)
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![SUSD, DOT],
				liquidity_changes: vec![253_794_223_643_471, 100_000_000_000_000],
			}));
		});
}

#[test]
fn get_liquidity_token_address_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::get_liquidity_token_address(SUSD, DOT), None);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			SUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			DexModule::get_liquidity_token_address(SUSD, DOT),
			Some(H160::from_str("0x0000000000000000000200000000010000000080").unwrap())
		);

		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), SUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(SUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			DexModule::get_liquidity_token_address(SUSD, DOT),
			Some(H160::from_str("0x0000000000000000000200000000010000000080").unwrap())
		);
	});
}

#[test]
fn specific_joint_swap_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				SUSD,
				BTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
			));

			assert_eq!(
				SUSDJointSwap::get_swap_amount(BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				Some((10000, 9800))
			);
			assert_eq!(
				SELJointSwap::get_swap_amount(BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				None
			);

			assert_noop!(
				SUSDJointSwap::swap(&CAROL, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				orml_tokens::Error::<Runtime>::BalanceTooLow,
			);
			assert_noop!(
				SUSDJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 9801)),
				SwapError::CannotSwap,
			);
			assert_noop!(
				SELJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				SwapError::CannotSwap,
			);

			assert_eq!(
				SUSDJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				Ok((10000, 9800)),
			);

			assert_eq!(
				SUSDJointSwap::swap(&BOB, DOT, BTC, SwapLimit::ExactTarget(20000, 10000)),
				Ok((10204, 10000)),
			);
		});
}
