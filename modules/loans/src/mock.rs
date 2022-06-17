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

//! Mocks for the loans module.

#![cfg(test)]

use super::*;
use frame_support::{
	construct_runtime, ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, Everything, Nothing},
	PalletId,
};
use frame_system::EnsureSignedBy;
use orml_traits::parameter_type_with_key;
use primitives::currency::TokenSymbol;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{AccountIdConversion, IdentityLookup},
};
use sp_std::cell::RefCell;
use std::collections::HashMap;
use support::mocks::MockStableAsset;
use support::{RiskManager, SpecificJointsSwap};

pub type AccountId = u128;
pub type BlockNumber = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const SEL: CurrencyId = CurrencyId::Token(TokenSymbol::SEL);
pub const SUSD: CurrencyId = CurrencyId::Token(TokenSymbol::SUSD);
pub const DOT: CurrencyId = CurrencyId::Token(TokenSymbol::DOT);
pub const BTC: CurrencyId = CurrencyId::Token(TokenSymbol::RENBTC);

mod loans {
	pub use super::super::*;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = BlockNumber;
	type Call = Call;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_type_with_key! {
	pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
		100
	};
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type DustRemovalWhitelist = Nothing;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = frame_system::Pallet<Runtime>;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SEL;
}

impl orml_currencies::Config for Runtime {
	type MultiCurrency = Tokens;
	type NativeCurrency = AdaptedBasicCurrency;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type WeightInfo = ();
}
pub type AdaptedBasicCurrency = orml_currencies::BasicCurrencyAdapter<Runtime, PalletBalances, Amount, BlockNumber>;

ord_parameter_types! {
	pub const One: AccountId = 1;
}

parameter_types! {
	pub const GetStableCurrencyId: CurrencyId = SUSD;
	pub const CDPTreasuryPalletId: PalletId = PalletId(*b"sel/cdpt");
	pub TreasuryAccount: AccountId = PalletId(*b"sel/hztr").into_account_truncating();
	pub AlternativeSwapPathJointList: Vec<Vec<CurrencyId>> = vec![];
}

impl cdp_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type UpdateOrigin = EnsureSignedBy<One, AccountId>;
	type DEX = ();
	type Swap = SpecificJointsSwap<(), AlternativeSwapPathJointList>;
	type PalletId = CDPTreasuryPalletId;
	type TreasuryAccount = TreasuryAccount;
	type WeightInfo = ();
	type StableAsset = MockStableAsset<CurrencyId, Balance, AccountId, BlockNumber>;
}

// mock risk manager
pub struct MockRiskManager;
impl RiskManager<AccountId, CurrencyId, Balance, Balance> for MockRiskManager {
	fn get_debit_value(_currency_id: CurrencyId, debit_balance: Balance) -> Balance {
		debit_balance / Balance::from(2u64)
	}

	fn check_position_valid(
		currency_id: CurrencyId,
		_collateral_balance: Balance,
		_debit_balance: Balance,
		check_required_ratio: bool,
	) -> DispatchResult {
		match currency_id {
			DOT => {
				if check_required_ratio {
					Err(sp_runtime::DispatchError::Other(
						"mock below required collateral ratio error",
					))
				} else {
					Err(sp_runtime::DispatchError::Other("mock below liquidation ratio error"))
				}
			}
			BTC => Ok(()),
			_ => Err(sp_runtime::DispatchError::Other("mock below liquidation ratio error")),
		}
	}

	fn check_debit_cap(currency_id: CurrencyId, total_debit_balance: Balance) -> DispatchResult {
		match (currency_id, total_debit_balance) {
			(DOT, 1000) => Err(sp_runtime::DispatchError::Other("mock exceed debit value cap error")),
			(BTC, 1000) => Err(sp_runtime::DispatchError::Other("mock exceed debit value cap error")),
			(_, _) => Ok(()),
		}
	}
}

thread_local! {
	pub static DOT_SHARES: RefCell<HashMap<AccountId, Balance>> = RefCell::new(HashMap::new());
}

pub struct MockOnUpdateLoan;
impl Happened<(AccountId, CurrencyId, Amount, Balance)> for MockOnUpdateLoan {
	fn happened(info: &(AccountId, CurrencyId, Amount, Balance)) {
		let (who, currency_id, adjustment, previous_amount) = info;
		let adjustment_abs = TryInto::<Balance>::try_into(adjustment.saturating_abs()).unwrap_or_default();
		let new_share_amount = if adjustment.is_positive() {
			previous_amount.saturating_add(adjustment_abs)
		} else {
			previous_amount.saturating_sub(adjustment_abs)
		};

		if *currency_id == DOT {
			DOT_SHARES.with(|v| {
				let mut old_map = v.borrow().clone();
				old_map.insert(*who, new_share_amount);
				*v.borrow_mut() = old_map;
			});
		}
	}
}

parameter_types! {
	pub const LoansPalletId: PalletId = PalletId(*b"sel/loan");
}

impl Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type RiskManager = MockRiskManager;
	type CDPTreasury = CDPTreasuryModule;
	type PalletId = LoansPalletId;
	type OnUpdateLoan = MockOnUpdateLoan;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		LoansModule: loans::{Pallet, Storage, Call, Event<T>},
		Tokens: orml_tokens::{Pallet, Storage, Event<T>, Config<T>},
		PalletBalances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Currencies: orml_currencies::{Pallet, Call},
		CDPTreasuryModule: cdp_treasury::{Pallet, Storage, Call, Event<T>},
	}
);

pub struct ExtBuilder {
	balances: Vec<(AccountId, CurrencyId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balances: vec![
				(ALICE, DOT, 1000),
				(ALICE, BTC, 1000),
				(BOB, DOT, 1000),
				(BOB, BTC, 1000),
			],
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap();
		orml_tokens::GenesisConfig::<Runtime> {
			balances: self.balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}
}
