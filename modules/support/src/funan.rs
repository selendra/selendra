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

use primitives::Position;
use sp_runtime::DispatchResult;

use crate::{ExchangeRate, Ratio};

pub trait RiskManager<AccountId, CurrencyId, Balance, DebitBalance> {
	fn get_debit_value(currency_id: CurrencyId, debit_balance: DebitBalance) -> Balance;

	fn check_position_valid(
		currency_id: CurrencyId,
		collateral_balance: Balance,
		debit_balance: DebitBalance,
		check_required_ratio: bool,
	) -> DispatchResult;

	fn check_debit_cap(
		currency_id: CurrencyId,
		total_debit_balance: DebitBalance,
	) -> DispatchResult;
}

#[cfg(feature = "std")]
impl<AccountId, CurrencyId, Balance: Default, DebitBalance>
	RiskManager<AccountId, CurrencyId, Balance, DebitBalance> for ()
{
	fn get_debit_value(_currency_id: CurrencyId, _debit_balance: DebitBalance) -> Balance {
		Default::default()
	}

	fn check_position_valid(
		_currency_id: CurrencyId,
		_collateral_balance: Balance,
		_debit_balance: DebitBalance,
		_check_required_ratio: bool,
	) -> DispatchResult {
		Ok(())
	}

	fn check_debit_cap(
		_currency_id: CurrencyId,
		_total_debit_balance: DebitBalance,
	) -> DispatchResult {
		Ok(())
	}
}

/// An abstraction of cdp treasury for Funan Protocol.
pub trait SelTreasury<AccountId> {
	type Balance;
	type CurrencyId;

	/// get debit amount of cdp treasury
	fn get_debit_pool() -> Self::Balance;

	/// calculate the proportion of specific debit amount for the whole system
	fn get_debit_proportion(amount: Self::Balance) -> Ratio;

	/// issue debit for cdp treasury
	fn on_system_debit(amount: Self::Balance) -> DispatchResult;

	/// issue debit to `who`
	/// if backed flag is true, means the debit to issue is backed on some
	/// assets, otherwise will increase same amount of debit to system debit.
	fn issue_debit(who: &AccountId, debit: Self::Balance, backed: bool) -> DispatchResult;

	/// burn debit(stable currency) of `who`
	fn burn_debit(who: &AccountId, debit: Self::Balance) -> DispatchResult;
}

pub trait EmergencyShutdown {
	fn is_shutdown() -> bool;
}

/// Functionality of Funan Protocol to be exposed to EVM+.
pub trait FunanManager<AccountId, CurrencyId, Amount, Balance> {
	/// Adjust CDP loan
	fn adjust_loan(
		who: &AccountId,
		currency_id: CurrencyId,
		collateral_adjustment: Amount,
		debit_adjustment: Amount,
	) -> DispatchResult;
	/// Close CDP loan using DEX
	fn close_loan_by_dex(
		who: AccountId,
		currency_id: CurrencyId,
		max_collateral_amount: Balance,
	) -> DispatchResult;
	/// Get open CDP corresponding to an account and collateral `CurrencyId`
	fn get_position(who: &AccountId, currency_id: CurrencyId) -> Position;
	/// Get liquidation ratio for collateral `CurrencyId`
	fn get_liquidation_ratio(currency_id: CurrencyId) -> Option<Ratio>;
	/// Get current ratio of collateral to debit of open CDP
	fn get_current_collateral_ratio(who: &AccountId, currency_id: CurrencyId) -> Option<Ratio>;
	/// Get exchange rate of debit units to debit value for a currency_id
	fn get_debit_exchange_rate(currency_id: CurrencyId) -> ExchangeRate;
}
