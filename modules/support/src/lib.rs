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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::from_over_into)]
#![allow(clippy::type_complexity)]

use frame_support::pallet_prelude::{DispatchClass, Pays, Weight};
use primitives::{task::TaskResult, CurrencyId, Multiplier, ReserveIdentifier};
use sp_runtime::{
	traits::CheckedDiv, transaction_validity::TransactionValidityError, DispatchError,
	DispatchResult, FixedU128,
};

pub mod dex;
pub mod evm;
pub mod funan;
pub mod incentives;
pub mod mocks;
pub mod stable_asset;

pub use crate::{dex::*, evm::*, funan::*, incentives::*, stable_asset::*};

pub type Price = FixedU128;
pub type ExchangeRate = FixedU128;
pub type Ratio = FixedU128;
pub type Rate = FixedU128;

pub trait PriceProvider<CurrencyId> {
	fn get_price(currency_id: CurrencyId) -> Option<Price>;
	fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<Price> {
		if let (Some(base_price), Some(quote_price)) =
			(Self::get_price(base), Self::get_price(quote))
		{
			base_price.checked_div(&quote_price)
		} else {
			None
		}
	}
}

pub trait DEXPriceProvider<CurrencyId> {
	fn get_relative_price(base: CurrencyId, quote: CurrencyId) -> Option<ExchangeRate>;
}

pub trait LockablePrice<CurrencyId> {
	fn lock_price(currency_id: CurrencyId) -> DispatchResult;
	fn unlock_price(currency_id: CurrencyId) -> DispatchResult;
}

pub trait ExchangeRateProvider {
	fn get_exchange_rate() -> ExchangeRate;
}

pub trait TransactionPayment<AccountId, Balance, NegativeImbalance> {
	fn reserve_fee(
		who: &AccountId,
		fee: Balance,
		named: Option<ReserveIdentifier>,
	) -> Result<Balance, DispatchError>;
	fn unreserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Balance;
	fn unreserve_and_charge_fee(
		who: &AccountId,
		weight: Weight,
	) -> Result<(Balance, NegativeImbalance), TransactionValidityError>;
	fn refund_fee(
		who: &AccountId,
		weight: Weight,
		payed: NegativeImbalance,
	) -> Result<(), TransactionValidityError>;
	fn charge_fee(
		who: &AccountId,
		len: u32,
		weight: Weight,
		tip: Balance,
		pays_fee: Pays,
		class: DispatchClass,
	) -> Result<(), TransactionValidityError>;
	fn weight_to_fee(weight: Weight) -> Balance;
	fn apply_multiplier_to_fee(fee: Balance, multiplier: Option<Multiplier>) -> Balance;
}

/// Dispatchable tasks
pub trait DispatchableTask {
	fn dispatch(self, weight: Weight) -> TaskResult;
}

/// Idle scheduler trait
pub trait IdleScheduler<Task> {
	fn schedule(task: Task) -> DispatchResult;
}

#[cfg(feature = "std")]
impl DispatchableTask for () {
	fn dispatch(self, _weight: Weight) -> TaskResult {
		unimplemented!()
	}
}

#[cfg(feature = "std")]
impl<Task> IdleScheduler<Task> for () {
	fn schedule(_task: Task) -> DispatchResult {
		unimplemented!()
	}
}
