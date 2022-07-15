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

//! Unit tests for the dex module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	DOTBTCPair, DexModule, Event, ExtBuilder, KUSDBTCPair, KUSDDOTPair, KUSDJointSwap,
	ListingOrigin, Origin, Runtime, SELJointSwap, System, Tokens, ALICE, BOB, BTC, CAROL, DOT,
	KUSD, KUSD_DOT_POOL_RECORD, SEL,
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
				KUSD,
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
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		System::assert_last_event(Event::DexModule(crate::Event::ListProvisioning {
			trading_pair: KUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::list_provisioning(
				Origin::signed(ListingOrigin::get()),
				KUSD,
				KUSD,
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
				KUSD,
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
				KUSD,
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
				KUSD,
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
				KUSD,
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
				KUSD,
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
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(DexModule::update_provisioning_parameters(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT,
			2_000_000_000_000u128,
			0,
			3_000_000_000_000u128,
			2_000_000_000_000u128,
			50,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
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

		assert_noop!(DexModule::enable_trading_pair(Origin::signed(ALICE), KUSD, DOT), BadOrigin);

		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::EnableTradingPair {
			trading_pair: KUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), DOT, KUSD),
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
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));

		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::EnableTradingPair {
			trading_pair: KUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, BTC),
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
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			2_000_000_000_000u128
		));

		assert_noop!(
			DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), KUSD, BTC),
			Error::<Runtime>::UnqualifiedProvision
		);
		System::set_block_number(10);

		assert_eq!(
			DexModule::trading_pair_statuses(KUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 10,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDBTCPair::get()), Default::default());
		assert_eq!(DexModule::liquidity_pool(KUSDBTCPair::get()), (0, 0));
		assert_eq!(Tokens::total_issuance(KUSDBTCPair::get().dex_share_currency_id()), 0);
		assert_eq!(
			Tokens::free_balance(
				KUSDBTCPair::get().dex_share_currency_id(),
				&DexModule::account_id()
			),
			0
		);

		assert_ok!(DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), KUSD, BTC));
		System::assert_last_event(Event::DexModule(crate::Event::ProvisioningToEnabled {
			trading_pair: KUSDBTCPair::get(),
			pool_0: 1_000_000_000_000u128,
			pool_1: 2_000_000_000_000u128,
			share_amount: 2_000_000_000_000u128,
		}));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			DexModule::initial_share_exchange_rates(KUSDBTCPair::get()),
			(ExchangeRate::one(), ExchangeRate::checked_from_rational(1, 2).unwrap())
		);
		assert_eq!(
			DexModule::liquidity_pool(KUSDBTCPair::get()),
			(1_000_000_000_000u128, 2_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::total_issuance(KUSDBTCPair::get().dex_share_currency_id()),
			2_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(
				KUSDBTCPair::get().dex_share_currency_id(),
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
			DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, DOT),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			1000,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			KUSD,
			BTC,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
		));

		// not expired, nothing happened.
		System::set_block_number(2000);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, DOT));
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, BTC));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (1_000_000_000_000u128, 1_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDDOTPair::get()), Default::default());
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDBTCPair::get()), Default::default());

		// both expired, the provision for KUSD-DOT could be aborted, the provision for KUSD-BTC
		// couldn't be aborted because it's already met the target.
		System::set_block_number(3001);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::ProvisioningAborted {
			trading_pair: KUSDDOTPair::get(),
			accumulated_provision_0: 1_000_000_000_000u128,
			accumulated_provision_1: 1_000_000_000_000u128,
		}));

		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, BTC));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDDOTPair::get()), Default::default());
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				not_before: 1000,
			})
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDBTCPair::get()), Default::default());
	});
}

#[test]
fn refund_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			5_000_000_000_000_000_000u128,
			4_000_000_000_000_000_000u128,
			1000,
		));
		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC,
			1_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
			1000,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			DOT,
			1_000_000_000_000_000_000u128,
			1_000_000_000_000_000_000u128
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			KUSD,
			DOT,
			0,
			600_000_000_000_000_000u128,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			KUSD,
			BTC,
			100_000_000_000_000_000u128,
			100_000_000_000_000_000u128,
		));

		assert_noop!(
			DexModule::refund_provision(Origin::signed(ALICE), ALICE, KUSD, DOT),
			Error::<Runtime>::MustBeDisabled
		);

		// abort provisioning of KUSD-DOT
		System::set_block_number(3001);
		assert_ok!(DexModule::abort_provisioning(Origin::signed(ALICE), KUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::initial_share_exchange_rates(KUSDDOTPair::get()), Default::default());

		assert_eq!(
			DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE),
			(1_000_000_000_000_000_000u128, 1_000_000_000_000_000_000u128)
		);
		assert_eq!(
			DexModule::provisioning_pool(KUSDDOTPair::get(), BOB),
			(0, 600_000_000_000_000_000u128)
		);
		assert_eq!(
			Tokens::free_balance(KUSD, &DexModule::account_id()),
			1_100_000_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(DOT, &DexModule::account_id()),
			1_600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(KUSD, &ALICE), 0);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 0);
		assert_eq!(Tokens::free_balance(KUSD, &BOB), 900_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 400_000_000_000_000_000u128);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(DexModule::refund_provision(Origin::signed(ALICE), ALICE, KUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::RefundProvision {
			who: ALICE,
			currency_0: KUSD,
			contribution_0: 1_000_000_000_000_000_000u128,
			currency_1: DOT,
			contribution_1: 1_000_000_000_000_000_000u128,
		}));

		assert_eq!(DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(
			Tokens::free_balance(KUSD, &DexModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(
			Tokens::free_balance(DOT, &DexModule::account_id()),
			600_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(KUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);

		assert_ok!(DexModule::refund_provision(Origin::signed(ALICE), BOB, KUSD, DOT));
		System::assert_last_event(Event::DexModule(crate::Event::RefundProvision {
			who: BOB,
			currency_0: KUSD,
			contribution_0: 0,
			currency_1: DOT,
			contribution_1: 600_000_000_000_000_000u128,
		}));

		assert_eq!(DexModule::provisioning_pool(KUSDDOTPair::get(), BOB), (0, 0));
		assert_eq!(
			Tokens::free_balance(KUSD, &DexModule::account_id()),
			100_000_000_000_000_000u128
		);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(KUSD, &BOB), 900_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);

		// not allow refund if the provisioning has been ended before.
		assert_ok!(DexModule::end_provisioning(Origin::signed(ALICE), KUSD, BTC));
		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDBTCPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(
			DexModule::provisioning_pool(KUSDBTCPair::get(), BOB),
			(100_000_000_000_000_000u128, 100_000_000_000_000_000u128)
		);
		assert_noop!(
			DexModule::refund_provision(Origin::signed(BOB), BOB, KUSD, BTC),
			Error::<Runtime>::NotAllowedRefund
		);
	});
}

#[test]
fn disable_trading_pair_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);

		assert_noop!(DexModule::disable_trading_pair(Origin::signed(ALICE), KUSD, DOT), BadOrigin);

		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		System::assert_last_event(Event::DexModule(crate::Event::DisableTradingPair {
			trading_pair: KUSDDOTPair::get(),
		}));

		assert_noop!(
			DexModule::disable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, DOT),
			Error::<Runtime>::MustBeEnabled
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			BTC,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_noop!(
			DexModule::disable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, BTC),
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
				KUSD,
				BTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			assert_eq!(KUSD_DOT_POOL_RECORD.with(|v| *v.borrow()), (0, 0));

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				KUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			assert_eq!(KUSD_DOT_POOL_RECORD.with(|v| *v.borrow()), (5000000000000, 1000000000000));
		});
}

#[test]
fn add_provision_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_noop!(
			DexModule::add_provision(
				Origin::signed(ALICE),
				KUSD,
				DOT,
				5_000_000_000_000u128,
				1_000_000_000_000u128,
			),
			Error::<Runtime>::MustBeProvisioning
		);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
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
				KUSD,
				DOT,
				4_999_999_999_999u128,
				999_999_999_999u128,
			),
			Error::<Runtime>::InvalidContributionIncrement
		);

		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(Tokens::free_balance(KUSD, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 0);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		let alice_ref_count_0 = System::consumers(&ALICE);

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			DOT,
			5_000_000_000_000u128,
			0,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (5_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000_000u128, 1_000_000_000_000_000u128),
				accumulated_provision: (5_000_000_000_000u128, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE),
			(5_000_000_000_000u128, 0)
		);
		assert_eq!(Tokens::free_balance(KUSD, &ALICE), 999_995_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 5_000_000_000_000u128);
		assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
		let alice_ref_count_1 = System::consumers(&ALICE);
		assert_eq!(alice_ref_count_1, alice_ref_count_0 + 1);
		System::assert_last_event(Event::DexModule(crate::Event::AddProvision {
			who: ALICE,
			currency_0: KUSD,
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
			KUSD,
			DOT,
			5_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000_000u128,
			1_000_000_000_000_000u128,
			0,
		));

		assert_ok!(DexModule::add_provision(
			Origin::signed(ALICE),
			KUSD,
			DOT,
			1_000_000_000_000_000u128,
			200_000_000_000_000u128,
		));
		assert_ok!(DexModule::add_provision(
			Origin::signed(BOB),
			KUSD,
			DOT,
			4_000_000_000_000_000u128,
			800_000_000_000_000u128,
		));

		assert_noop!(
			DexModule::claim_dex_share(Origin::signed(ALICE), ALICE, KUSD, DOT),
			Error::<Runtime>::StillProvisioning
		);

		assert_ok!(DexModule::end_provisioning(Origin::signed(ListingOrigin::get()), KUSD, DOT));

		let lp_currency_id = KUSDDOTPair::get().dex_share_currency_id();

		assert!(InitialShareExchangeRates::<Runtime>::contains_key(KUSDDOTPair::get()),);
		assert_eq!(
			DexModule::initial_share_exchange_rates(KUSDDOTPair::get()),
			(ExchangeRate::one(), ExchangeRate::saturating_from_rational(5, 1))
		);
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &DexModule::account_id()),
			10_000_000_000_000_000u128
		);
		assert_eq!(
			DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE),
			(1_000_000_000_000_000u128, 200_000_000_000_000u128)
		);
		assert_eq!(
			DexModule::provisioning_pool(KUSDDOTPair::get(), BOB),
			(4_000_000_000_000_000u128, 800_000_000_000_000u128)
		);
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 0);
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 0);

		let alice_ref_count_0 = System::consumers(&ALICE);
		let bob_ref_count_0 = System::consumers(&BOB);

		assert_ok!(DexModule::claim_dex_share(Origin::signed(ALICE), ALICE, KUSD, DOT));
		assert_eq!(
			Tokens::free_balance(lp_currency_id, &DexModule::account_id()),
			8_000_000_000_000_000u128
		);
		assert_eq!(DexModule::provisioning_pool(KUSDDOTPair::get(), ALICE), (0, 0));
		assert_eq!(Tokens::free_balance(lp_currency_id, &ALICE), 2_000_000_000_000_000u128);
		assert_eq!(System::consumers(&ALICE), alice_ref_count_0 - 1);
		assert!(InitialShareExchangeRates::<Runtime>::contains_key(KUSDDOTPair::get()),);

		assert_ok!(DexModule::disable_trading_pair(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT
		));
		assert_ok!(DexModule::claim_dex_share(Origin::signed(BOB), BOB, KUSD, DOT));
		assert_eq!(Tokens::free_balance(lp_currency_id, &DexModule::account_id()), 0);
		assert_eq!(DexModule::provisioning_pool(KUSDDOTPair::get(), BOB), (0, 0));
		assert_eq!(Tokens::free_balance(lp_currency_id, &BOB), 8_000_000_000_000_000u128);
		assert_eq!(System::consumers(&BOB), bob_ref_count_0 - 1);
		assert!(!InitialShareExchangeRates::<Runtime>::contains_key(KUSDDOTPair::get()),);
	});
}

#[test]
fn get_liquidity_work() {
	ExtBuilder::default().build().execute_with(|| {
		LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (1000, 20));
		assert_eq!(DexModule::liquidity_pool(KUSDDOTPair::get()), (1000, 20));
		assert_eq!(DexModule::get_liquidity(KUSD, DOT), (1000, 20));
		assert_eq!(DexModule::get_liquidity(DOT, KUSD), (20, 1000));
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
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(KUSDBTCPair::get(), (100000, 10));
			assert_noop!(
				DexModule::get_target_amounts(&[DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, KUSD, BTC, DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, KUSD, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, KUSD, SEL], 10000),
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(DexModule::get_target_amounts(&[DOT, KUSD], 10000), Ok(vec![10000, 24874]));
			assert_eq!(
				DexModule::get_target_amounts(&[DOT, KUSD, BTC], 10000),
				Ok(vec![10000, 24874, 1])
			);
			assert_noop!(
				DexModule::get_target_amounts(&[DOT, KUSD, BTC], 100),
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
			KUSDDOTPair::get(),
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
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(KUSDBTCPair::get(), (100000, 10));
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, KUSD, BTC, DOT], 10000),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, KUSD, DOT], 10000),
				Error::<Runtime>::InvalidTradingPath,
			);
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, KUSD, SEL], 10000),
				Error::<Runtime>::MustBeEnabled,
			);
			assert_eq!(DexModule::get_supply_amounts(&[DOT, KUSD], 24874), Ok(vec![10000, 24874]));
			assert_eq!(DexModule::get_supply_amounts(&[DOT, KUSD], 25000), Ok(vec![10102, 25000]));
			assert_noop!(
				DexModule::get_supply_amounts(&[DOT, KUSD, BTC], 10000),
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
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (50000, 10000));

			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (50000, 10000));
			assert_noop!(
				DexModule::_swap(KUSD, DOT, 50000, 5001),
				Error::<Runtime>::InvariantCheckFailed
			);
			assert_ok!(DexModule::_swap(KUSD, DOT, 50000, 5000));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (100000, 5000));
			assert_ok!(DexModule::_swap(DOT, KUSD, 100, 800));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (99200, 5100));
		});
}

#[test]
fn _swap_by_path_work() {
	ExtBuilder::default()
		.initialize_enabled_trading_pairs()
		.build()
		.execute_with(|| {
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(KUSDBTCPair::get(), (100000, 10));

			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (50000, 10000));
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (100000, 10));
			assert_ok!(DexModule::_swap_by_path(&[DOT, KUSD], &[10000, 25000]));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (25000, 20000));
			assert_ok!(DexModule::_swap_by_path(&[DOT, KUSD, BTC], &[100000, 20000, 1]));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (5000, 120000));
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (120000, 9));
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
					KUSD,
					100_000_000,
					100_000_000,
					0,
					false
				),
				Error::<Runtime>::MustBeEnabled
			);
			assert_noop!(
				DexModule::add_liquidity(
					Origin::signed(ALICE),
					KUSD,
					DOT,
					0,
					100_000_000,
					0,
					false
				),
				Error::<Runtime>::InvalidLiquidityIncrement
			);

			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (0, 0));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE), 0);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				0
			);
			assert_eq!(Tokens::free_balance(KUSD, &ALICE), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				KUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			System::assert_last_event(Event::DexModule(crate::Event::AddLiquidity {
				who: ALICE,
				currency_0: KUSD,
				pool_0: 5_000_000_000_000,
				currency_1: DOT,
				pool_1: 1_000_000_000_000,
				share_increment: 10_000_000_000_000,
			}));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (5_000_000_000_000, 1_000_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 5_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(
				Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				10_000_000_000_000
			);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				0
			);
			assert_eq!(Tokens::free_balance(KUSD, &ALICE), 999_995_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB), 0);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB),
				0
			);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::add_liquidity(Origin::signed(BOB), KUSD, DOT, 4, 1, 0, true,),
				Error::<Runtime>::InvalidLiquidityIncrement,
			);

			assert_noop!(
				DexModule::add_liquidity(
					Origin::signed(BOB),
					KUSD,
					DOT,
					50_000_000_000_000,
					8_000_000_000_000,
					80_000_000_000_001,
					true,
				),
				Error::<Runtime>::UnacceptableShareIncrement
			);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(BOB),
				KUSD,
				DOT,
				50_000_000_000_000,
				8_000_000_000_000,
				80_000_000_000_000,
				true,
			));
			System::assert_last_event(Event::DexModule(crate::Event::AddLiquidity {
				who: BOB,
				currency_0: KUSD,
				pool_0: 40_000_000_000_000,
				currency_1: DOT,
				pool_1: 8_000_000_000_000,
				share_increment: 80_000_000_000_000,
			}));
			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(45_000_000_000_000, 9_000_000_000_000)
			);
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 45_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 9_000_000_000_000);
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB), 0);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB),
				80_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 999_960_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_992_000_000_000_000);
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
				KUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false
			));
			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					KUSDDOTPair::get().dex_share_currency_id(),
					DOT,
					100_000_000,
					0,
					0,
					false,
				),
				Error::<Runtime>::InvalidCurrencyId
			);

			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (5_000_000_000_000, 1_000_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 5_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(
				Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				10_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(KUSD, &ALICE), 999_995_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_000_000_000_000);

			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					KUSD,
					DOT,
					8_000_000_000_000,
					4_000_000_000_001,
					800_000_000_000,
					false,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_noop!(
				DexModule::remove_liquidity(
					Origin::signed(ALICE),
					KUSD,
					DOT,
					8_000_000_000_000,
					4_000_000_000_000,
					800_000_000_001,
					false,
				),
				Error::<Runtime>::UnacceptableLiquidityWithdrawn
			);
			assert_ok!(DexModule::remove_liquidity(
				Origin::signed(ALICE),
				KUSD,
				DOT,
				8_000_000_000_000,
				4_000_000_000_000,
				800_000_000_000,
				false,
			));
			System::assert_last_event(Event::DexModule(crate::Event::RemoveLiquidity {
				who: ALICE,
				currency_0: KUSD,
				pool_0: 4_000_000_000_000,
				currency_1: DOT,
				pool_1: 800_000_000_000,
				share_decrement: 8_000_000_000_000,
			}));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (1_000_000_000_000, 200_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 1_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000);
			assert_eq!(
				Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
				2_000_000_000_000
			);
			assert_eq!(Tokens::free_balance(KUSD, &ALICE), 999_999_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 999_999_800_000_000_000);

			assert_ok!(DexModule::remove_liquidity(
				Origin::signed(ALICE),
				KUSD,
				DOT,
				2_000_000_000_000,
				0,
				0,
				false,
			));
			System::assert_last_event(Event::DexModule(crate::Event::RemoveLiquidity {
				who: ALICE,
				currency_0: KUSD,
				pool_0: 1_000_000_000_000,
				currency_1: DOT,
				pool_1: 200_000_000_000,
				share_decrement: 2_000_000_000_000,
			}));
			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (0, 0));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 0);
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE), 0);
			assert_eq!(Tokens::free_balance(KUSD, &ALICE), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &ALICE), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::add_liquidity(
				Origin::signed(BOB),
				KUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				true
			));
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB), 0);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB),
				10_000_000_000_000
			);
			assert_ok!(DexModule::remove_liquidity(
				Origin::signed(BOB),
				KUSD,
				DOT,
				2_000_000_000_000,
				0,
				0,
				true,
			));
			assert_eq!(Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB), 0);
			assert_eq!(
				Tokens::reserved_balance(KUSDDOTPair::get().dex_share_currency_id(), &BOB),
				8_000_000_000_000
			);
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
				KUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				KUSD,
				BTC,
				100_000_000_000_000,
				10_000_000_000,
				0,
				false,
			));

			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 600_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, KUSD],
					100_000_000_000_000,
					250_000_000_000_000,
				),
				Error::<Runtime>::InsufficientTargetAmount
			);
			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, KUSD, BTC, DOT],
					100_000_000_000_000,
					0
				),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_supply(
					&BOB,
					&[DOT, KUSD, DOT],
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
				&[DOT, KUSD],
				100_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, KUSD],
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));
			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(251_256_281_407_036, 200_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 351_256_281_407_036);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 200_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_248_743_718_592_964);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_900_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::do_swap_with_exact_supply(
				&BOB,
				&[DOT, KUSD, BTC],
				200_000_000_000_000,
				1,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, KUSD, BTC],
				liquidity_changes: vec![200_000_000_000_000, 124_996_843_514_053, 5_530_663_837],
			}));
			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(126_259_437_892_983, 400_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (224_996_843_514_053, 4_469_336_163));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 351_256_281_407_036);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 400_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 4_469_336_163);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_248_743_718_592_964);
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
				KUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				KUSD,
				BTC,
				100_000_000_000_000,
				10_000_000_000,
				0,
				false,
			));

			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(500_000_000_000_000, 100_000_000_000_000)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 600_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 100_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 1_000_000_000_000_000_000);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, KUSD],
					250_000_000_000_000,
					100_000_000_000_000,
				),
				Error::<Runtime>::ExcessiveSupplyAmount
			);
			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, KUSD, BTC, DOT],
					250_000_000_000_000,
					200_000_000_000_000,
				),
				Error::<Runtime>::InvalidTradingPathLength,
			);
			assert_noop!(
				DexModule::do_swap_with_exact_target(
					&BOB,
					&[DOT, KUSD, DOT],
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
				&[DOT, KUSD],
				250_000_000_000_000,
				200_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, KUSD],
				liquidity_changes: vec![101_010_101_010_102, 250_000_000_000_000],
			}));
			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(250_000_000_000_000, 201_010_101_010_102)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (100_000_000_000_000, 10_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 350_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 201_010_101_010_102);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 10_000_000_000);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_250_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &BOB), 999_898_989_898_989_898);
			assert_eq!(Tokens::free_balance(BTC, &BOB), 1_000_000_000_000_000_000);

			assert_ok!(DexModule::do_swap_with_exact_target(
				&BOB,
				&[DOT, KUSD, BTC],
				5_000_000_000,
				2_000_000_000_000_000,
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, KUSD, BTC],
				liquidity_changes: vec![137_654_580_386_993, 101_010_101_010_102, 5_000_000_000],
			}));
			assert_eq!(
				DexModule::get_liquidity(KUSD, DOT),
				(148_989_898_989_898, 338_664_681_397_095)
			);
			assert_eq!(DexModule::get_liquidity(KUSD, BTC), (201_010_101_010_102, 5_000_000_000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 350_000_000_000_000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 338_664_681_397_095);
			assert_eq!(Tokens::free_balance(BTC, &DexModule::account_id()), 5_000_000_000);
			assert_eq!(Tokens::free_balance(KUSD, &BOB), 1_000_250_000_000_000_000);
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

			assert_eq!(DexModule::get_liquidity(KUSD, DOT), (1000000, 2000000));
			assert_eq!(Tokens::free_balance(KUSD, &DexModule::account_id()), 2000000);
			assert_eq!(Tokens::free_balance(DOT, &DexModule::account_id()), 4000000);
			assert_eq!(
				Tokens::free_balance(KUSDDOTPair::get().dex_share_currency_id(), &ALICE),
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
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (50000, 10000));
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, KUSD], SwapLimit::ExactSupply(10000, 0)),
				Some((10000, 24874))
			);
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, KUSD], SwapLimit::ExactSupply(10000, 24875)),
				None
			);
			assert_eq!(
				DexModule::get_swap_amount(
					&[DOT, KUSD],
					SwapLimit::ExactTarget(Balance::max_value(), 24874)
				),
				Some((10000, 24874))
			);
			assert_eq!(
				DexModule::get_swap_amount(&[DOT, KUSD], SwapLimit::ExactTarget(9999, 24874)),
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
			LiquidityPool::<Runtime>::insert(KUSDDOTPair::get(), (300000, 100000));
			LiquidityPool::<Runtime>::insert(KUSDBTCPair::get(), (50000, 10000));
			LiquidityPool::<Runtime>::insert(DOTBTCPair::get(), (10000, 10000));

			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![]
				),
				Some((vec![DOT, KUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 30),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(0, 0),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![SEL]]
				),
				Some((vec![DOT, KUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![DOT]]
				),
				Some((vec![DOT, KUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![KUSD]]
				),
				Some((vec![DOT, KUSD], 10, 29))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10, 0),
					vec![vec![BTC]]
				),
				Some((vec![DOT, BTC, KUSD], 10, 44))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactSupply(10000, 0),
					vec![vec![BTC]]
				),
				Some((vec![DOT, KUSD], 10000, 27024))
			);

			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![]
				),
				Some((vec![DOT, KUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(10, 30),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(0, 0),
					vec![]
				),
				None
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![SEL]]
				),
				Some((vec![DOT, KUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![DOT]]
				),
				Some((vec![DOT, KUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![KUSD]]
				),
				Some((vec![DOT, KUSD], 11, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(20, 30),
					vec![vec![BTC]]
				),
				Some((vec![DOT, BTC, KUSD], 8, 30))
			);
			assert_eq!(
				DexModule::get_best_price_swap_path(
					DOT,
					KUSD,
					SwapLimit::ExactTarget(100000, 20000),
					vec![vec![BTC]]
				),
				Some((vec![DOT, KUSD], 7216, 20000))
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
				KUSD,
				DOT,
				500_000_000_000_000,
				100_000_000_000_000,
				0,
				false,
			));

			assert_noop!(
				DexModule::swap_with_specific_path(
					&BOB,
					&[DOT, KUSD],
					SwapLimit::ExactSupply(100_000_000_000_000, 248_743_718_592_965)
				),
				Error::<Runtime>::InsufficientTargetAmount
			);

			assert_ok!(DexModule::swap_with_specific_path(
				&BOB,
				&[DOT, KUSD],
				SwapLimit::ExactSupply(100_000_000_000_000, 200_000_000_000_000)
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![DOT, KUSD],
				liquidity_changes: vec![100_000_000_000_000, 248_743_718_592_964],
			}));

			assert_noop!(
				DexModule::swap_with_specific_path(
					&BOB,
					&[KUSD, DOT],
					SwapLimit::ExactTarget(253_794_223_643_470, 100_000_000_000_000)
				),
				Error::<Runtime>::ExcessiveSupplyAmount
			);

			assert_ok!(DexModule::swap_with_specific_path(
				&BOB,
				&[KUSD, DOT],
				SwapLimit::ExactTarget(300_000_000_000_000, 100_000_000_000_000)
			));
			System::assert_last_event(Event::DexModule(crate::Event::Swap {
				trader: BOB,
				path: vec![KUSD, DOT],
				liquidity_changes: vec![253_794_223_643_471, 100_000_000_000_000],
			}));
		});
}

#[test]
fn get_liquidity_token_address_work() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Disabled
		);
		assert_eq!(DexModule::get_liquidity_token_address(KUSD, DOT), None);

		assert_ok!(DexModule::list_provisioning(
			Origin::signed(ListingOrigin::get()),
			KUSD,
			DOT,
			1_000_000_000_000u128,
			1_000_000_000_000u128,
			5_000_000_000_000u128,
			2_000_000_000_000u128,
			10,
		));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Provisioning(ProvisioningParameters {
				min_contribution: (1_000_000_000_000u128, 1_000_000_000_000u128),
				target_provision: (5_000_000_000_000u128, 2_000_000_000_000u128),
				accumulated_provision: (0, 0),
				not_before: 10,
			})
		);
		assert_eq!(
			DexModule::get_liquidity_token_address(KUSD, DOT),
			Some(H160::from_str("0x0000000000000000000200000000010000000083").unwrap())
		);

		assert_ok!(DexModule::enable_trading_pair(Origin::signed(ListingOrigin::get()), KUSD, DOT));
		assert_eq!(
			DexModule::trading_pair_statuses(KUSDDOTPair::get()),
			TradingPairStatus::<_, _>::Enabled
		);
		assert_eq!(
			DexModule::get_liquidity_token_address(KUSD, DOT),
			Some(H160::from_str("0x0000000000000000000200000000010000000083").unwrap())
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
				KUSD,
				DOT,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));
			assert_ok!(DexModule::add_liquidity(
				Origin::signed(ALICE),
				KUSD,
				BTC,
				5_000_000_000_000,
				1_000_000_000_000,
				0,
				false,
			));

			assert_eq!(
				KUSDJointSwap::get_swap_amount(BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				Some((10000, 9800))
			);
			assert_eq!(
				SELJointSwap::get_swap_amount(BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				None
			);

			assert_noop!(
				KUSDJointSwap::swap(&CAROL, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				orml_tokens::Error::<Runtime>::BalanceTooLow,
			);
			assert_noop!(
				KUSDJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 9801)),
				SwapError::CannotSwap,
			);
			assert_noop!(
				SELJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				SwapError::CannotSwap,
			);

			assert_eq!(
				KUSDJointSwap::swap(&BOB, BTC, DOT, SwapLimit::ExactSupply(10000, 0)),
				Ok((10000, 9800)),
			);

			assert_eq!(
				KUSDJointSwap::swap(&BOB, DOT, BTC, SwapLimit::ExactTarget(20000, 10000)),
				Ok((10204, 10000)),
			);
		});
}
