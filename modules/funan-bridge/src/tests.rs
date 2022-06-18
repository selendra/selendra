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

//! Unit tests for Funan Bridge module.

#![cfg(test)]

use crate::mock::*;
use frame_support::assert_ok;

fn module_account() -> AccountId {
	FunanBridge::account_id()
}

#[test]
fn to_bridged_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(KMD, &ALICE), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(SUSD, &module_account()), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(KMD, &module_account()), dollar(1_000_000));

		assert_ok!(FunanBridge::to_bridged(Origin::signed(ALICE), dollar(5_000)));

		assert_eq!(Currencies::free_balance(SUSD, &ALICE), dollar(995_000));
		assert_eq!(Currencies::free_balance(KMD, &ALICE), dollar(1_005_000));
		assert_eq!(Currencies::free_balance(SUSD, &module_account()), dollar(1_005_000));
		assert_eq!(Currencies::free_balance(KMD, &module_account()), dollar(995_000));

		System::assert_last_event(Event::FunanBridge(crate::Event::ToBridged {
			who: ALICE,
			amount: dollar(5000),
		}));
	});
}

#[test]
fn from_bridged_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(Currencies::free_balance(SUSD, &ALICE), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(KMD, &ALICE), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(SUSD, &module_account()), dollar(1_000_000));
		assert_eq!(Currencies::free_balance(KMD, &module_account()), dollar(1_000_000));

		assert_ok!(FunanBridge::from_bridged(Origin::signed(ALICE), dollar(5_000)));

		assert_eq!(Currencies::free_balance(SUSD, &ALICE), dollar(1_005_000));
		assert_eq!(Currencies::free_balance(KMD, &ALICE), dollar(995_000));
		assert_eq!(Currencies::free_balance(SUSD, &module_account()), dollar(995_000));
		assert_eq!(Currencies::free_balance(KMD, &module_account()), dollar(1_005_000));

		System::assert_last_event(Event::FunanBridge(crate::Event::FromBridged {
			who: ALICE,
			amount: dollar(5000),
		}));
	});
}
