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

//! Unit tests for asset registry module.

#![cfg(test)]

use super::*;
use frame_support::{assert_noop, assert_ok};
use mock::{
	alice, deploy_contracts, deploy_contracts_same_prefix, erc20_address, erc20_address_not_exists,
	erc20_address_same_prefix, AssetRegistry, CouncilAccount, Event, ExtBuilder, Origin, Runtime, System,
};
use primitives::TokenSymbol;
use sp_core::H160;
use std::str::FromStr;

#[test]
fn register_erc20_asset_work() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));

			System::assert_last_event(Event::AssetRegistry(crate::Event::AssetRegistered {
				asset_id: AssetIds::Erc20(erc20_address()),
				metadata: AssetMetadata {
					name: b"long string name, long string name, long string name, long string name, long string name"
						.to_vec(),
					symbol: b"TestToken".to_vec(),
					decimals: 17,
					minimal_balance: 1,
				},
			}));

			assert_eq!(Erc20IdToAddress::<Runtime>::get(0x5dddfce5), Some(erc20_address()));

			assert_eq!(
				AssetMetadatas::<Runtime>::get(AssetIds::Erc20(erc20_address())),
				Some(AssetMetadata {
					name: b"long string name, long string name, long string name, long string name, long string name"
						.to_vec(),
					symbol: b"TestToken".to_vec(),
					decimals: 17,
					minimal_balance: 1,
				})
			);
		});
}

#[test]
fn register_erc20_asset_should_not_work() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			deploy_contracts_same_prefix();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));

			assert_noop!(
				AssetRegistry::register_erc20_asset(
					Origin::signed(CouncilAccount::get()),
					erc20_address_same_prefix(),
					1
				),
				Error::<Runtime>::AssetIdExisted
			);

			assert_noop!(
				AssetRegistry::register_erc20_asset(
					Origin::signed(CouncilAccount::get()),
					erc20_address_not_exists(),
					1
				),
				module_evm_bridge::Error::<Runtime>::InvalidReturnValue,
			);
		});
}

#[test]
fn update_erc20_asset_work() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));

			assert_ok!(AssetRegistry::update_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				Box::new(AssetMetadata {
					name: b"New Token Name".to_vec(),
					symbol: b"NTN".to_vec(),
					decimals: 13,
					minimal_balance: 2,
				})
			));

			System::assert_last_event(Event::AssetRegistry(crate::Event::AssetUpdated {
				asset_id: AssetIds::Erc20(erc20_address()),
				metadata: AssetMetadata {
					name: b"New Token Name".to_vec(),
					symbol: b"NTN".to_vec(),
					decimals: 13,
					minimal_balance: 2,
				},
			}));

			assert_eq!(
				AssetMetadatas::<Runtime>::get(AssetIds::Erc20(erc20_address())),
				Some(AssetMetadata {
					name: b"New Token Name".to_vec(),
					symbol: b"NTN".to_vec(),
					decimals: 13,
					minimal_balance: 2,
				})
			);
		});
}

#[test]
fn register_native_asset_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_ok!(AssetRegistry::register_native_asset(
			Origin::signed(CouncilAccount::get()),
			CurrencyId::Token(TokenSymbol::DOT),
			Box::new(AssetMetadata {
				name: b"Token Name".to_vec(),
				symbol: b"TN".to_vec(),
				decimals: 12,
				minimal_balance: 1,
			})
		));
		System::assert_last_event(Event::AssetRegistry(crate::Event::AssetRegistered {
			asset_id: AssetIds::NativeAssetId(CurrencyId::Token(TokenSymbol::DOT)),
			metadata: AssetMetadata {
				name: b"Token Name".to_vec(),
				symbol: b"TN".to_vec(),
				decimals: 12,
				minimal_balance: 1,
			},
		}));

		assert_eq!(
			AssetMetadatas::<Runtime>::get(AssetIds::NativeAssetId(CurrencyId::Token(TokenSymbol::DOT))),
			Some(AssetMetadata {
				name: b"Token Name".to_vec(),
				symbol: b"TN".to_vec(),
				decimals: 12,
				minimal_balance: 1,
			})
		);
		// Can't duplicate
		assert_noop!(
			AssetRegistry::register_native_asset(
				Origin::signed(CouncilAccount::get()),
				CurrencyId::Token(TokenSymbol::DOT),
				Box::new(AssetMetadata {
					name: b"Token Name".to_vec(),
					symbol: b"TN".to_vec(),
					decimals: 12,
					minimal_balance: 1,
				})
			),
			Error::<Runtime>::AssetIdExisted
		);
	});
}

#[test]
fn update_native_asset_works() {
	ExtBuilder::default().build().execute_with(|| {
		assert_noop!(
			AssetRegistry::update_native_asset(
				Origin::signed(CouncilAccount::get()),
				CurrencyId::Token(TokenSymbol::DOT),
				Box::new(AssetMetadata {
					name: b"New Token Name".to_vec(),
					symbol: b"NTN".to_vec(),
					decimals: 13,
					minimal_balance: 2,
				})
			),
			Error::<Runtime>::AssetIdNotExists
		);

		assert_ok!(AssetRegistry::register_native_asset(
			Origin::signed(CouncilAccount::get()),
			CurrencyId::Token(TokenSymbol::DOT),
			Box::new(AssetMetadata {
				name: b"Token Name".to_vec(),
				symbol: b"TN".to_vec(),
				decimals: 12,
				minimal_balance: 1,
			})
		));

		assert_ok!(AssetRegistry::update_native_asset(
			Origin::signed(CouncilAccount::get()),
			CurrencyId::Token(TokenSymbol::DOT),
			Box::new(AssetMetadata {
				name: b"New Token Name".to_vec(),
				symbol: b"NTN".to_vec(),
				decimals: 13,
				minimal_balance: 2,
			})
		));

		System::assert_last_event(Event::AssetRegistry(crate::Event::AssetUpdated {
			asset_id: AssetIds::NativeAssetId(CurrencyId::Token(TokenSymbol::DOT)),
			metadata: AssetMetadata {
				name: b"New Token Name".to_vec(),
				symbol: b"NTN".to_vec(),
				decimals: 13,
				minimal_balance: 2,
			},
		}));

		assert_eq!(
			AssetMetadatas::<Runtime>::get(AssetIds::NativeAssetId(CurrencyId::Token(TokenSymbol::DOT))),
			Some(AssetMetadata {
				name: b"New Token Name".to_vec(),
				symbol: b"NTN".to_vec(),
				decimals: 13,
				minimal_balance: 2,
			})
		);
	});
}

#[test]
fn name_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::Token(TokenSymbol::SEL)),
				Some(b"Selendra".to_vec())
			);
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::Erc20(erc20_address())),
				Some(b"long string name, long string name, long string name, long string name, long string name"[..32].to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::Erc20(erc20_address_not_exists())),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::DexShare(DexShare::Token(TokenSymbol::SEL), DexShare::Token(TokenSymbol::SUSD))),
				Some(b"LP Selendra - Selendra Dollar".to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::DexShare(DexShare::Erc20(erc20_address()), DexShare::Token(TokenSymbol::SUSD))),
				Some(b"LP long string name, long string name, long string name, long string name, long string name - Selendra Dollar"[..32].to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::DexShare(DexShare::Erc20(erc20_address()), DexShare::Erc20(erc20_address()))),
				Some(b"LP long string name, long string name, long string name, long string name, long string name - long string name, long string name, long string name, long string name, long string name"[..32].to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::DexShare(DexShare::Token(TokenSymbol::SEL), DexShare::Erc20(erc20_address_not_exists()))),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::name(CurrencyId::DexShare(DexShare::Erc20(erc20_address()), DexShare::Erc20(erc20_address_not_exists()))),
				None
			);
		});
}

#[test]
fn symbol_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::Token(TokenSymbol::SEL)),
				Some(b"SEL".to_vec())
			);
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::Erc20(erc20_address())),
				Some(b"TestToken".to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::Erc20(erc20_address_not_exists())),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				Some(b"LP_SEL_SUSD".to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				Some(b"LP_TestToken_SUSD".to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address())
				)),
				Some(b"LP_TestToken_TestToken".to_vec())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Erc20(erc20_address_not_exists())
				)),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::symbol(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address_not_exists())
				)),
				None
			);
		});
}

#[test]
fn decimals_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::Token(TokenSymbol::SEL)),
				Some(12)
			);
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::Erc20(erc20_address())),
				Some(17)
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::Erc20(erc20_address_not_exists())),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				Some(12)
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				Some(17)
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address())
				)),
				Some(17)
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decimals(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address_not_exists())
				)),
				Some(17)
			);
		});
}

#[test]
fn encode_evm_address_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));

			// Token
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Token(TokenSymbol::SEL)),
				H160::from_str("0x0000000000000000000100000000000000000000").ok()
			);

			// Erc20
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Erc20(erc20_address())),
				Some(erc20_address())
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Erc20(erc20_address_not_exists())),
				Some(erc20_address_not_exists())
			);

			// DexShare
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				H160::from_str("0x0000000000000000000200000000000000000001").ok()
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Token(TokenSymbol::SUSD)
				)),
				H160::from_str("0x00000000000000000002015dddfce50000000001").ok()
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SUSD),
					DexShare::Erc20(erc20_address())
				)),
				H160::from_str("0x000000000000000000020000000001015dddfce5").ok()
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address())
				)),
				H160::from_str("0x00000000000000000002015dddfce5015dddfce5").ok()
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Erc20(erc20_address_not_exists())
				)),
				None
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address_not_exists())
				)),
				None
			);

			// ForeignAsset
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::ForeignAsset(1)),
				H160::from_str("0x0000000000000000000300000000000000000001").ok()
			);
		});
}

#[test]
fn decode_evm_address_works() {
	ExtBuilder::default()
		.balances(vec![(alice(), 1_000_000_000_000)])
		.build()
		.execute_with(|| {
			deploy_contracts();
			assert_ok!(AssetRegistry::register_erc20_asset(
				Origin::signed(CouncilAccount::get()),
				erc20_address(),
				1
			));

			// Token
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Token(TokenSymbol::SEL)).unwrap()
				),
				Some(CurrencyId::Token(TokenSymbol::SEL))
			);

			// Erc20
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Erc20(erc20_address())).unwrap()
				),
				Some(CurrencyId::Erc20(erc20_address()))
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::Erc20(erc20_address_not_exists()))
						.unwrap()
				),
				None,
			);

			// DexShare
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
						DexShare::Token(TokenSymbol::SEL),
						DexShare::Token(TokenSymbol::SUSD)
					))
					.unwrap(),
				),
				Some(CurrencyId::DexShare(
					DexShare::Token(TokenSymbol::SEL),
					DexShare::Token(TokenSymbol::SUSD)
				))
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
						DexShare::Erc20(erc20_address()),
						DexShare::Token(TokenSymbol::SUSD)
					))
					.unwrap()
				),
				Some(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Token(TokenSymbol::SUSD)
				))
			);

			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::DexShare(
						DexShare::Erc20(erc20_address()),
						DexShare::Erc20(erc20_address())
					))
					.unwrap()
				),
				Some(CurrencyId::DexShare(
					DexShare::Erc20(erc20_address()),
					DexShare::Erc20(erc20_address())
				))
			);

			// decode invalid evm address
			// CurrencyId::DexShare(DexShare::Token(TokenSymbol::SEL),
			// DexShare::Erc20(erc20_address_not_exists()))
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					H160::from_str("0x0000000000000000000000010000000002000001").unwrap()
				),
				None
			);

			// decode invalid evm address
			// CurrencyId::DexShare(DexShare::Erc20(erc20_address()),
			// DexShare::Erc20(erc20_address_not_exists()))
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					H160::from_str("0x0000000000000000000000010200000002000001").unwrap()
				),
				None
			);

			// Allow non-system contracts
			let non_system_contracts = H160::from_str("0x1000000000000000000000000000000000000000").unwrap();
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(non_system_contracts),
				Some(CurrencyId::Erc20(non_system_contracts))
			);

			// ForeignAsset
			assert_eq!(
				EvmErc20InfoMapping::<Runtime>::decode_evm_address(
					EvmErc20InfoMapping::<Runtime>::encode_evm_address(CurrencyId::ForeignAsset(1)).unwrap()
				),
				Some(CurrencyId::ForeignAsset(1))
			);
		});
}
