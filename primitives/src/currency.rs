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

#![allow(clippy::from_over_into)]

use crate::evm::EvmAddress;
use bstringify::bstringify;
use codec::{Decode, Encode, MaxEncodedLen};
pub use module_stable_asset::StableAssetPoolId;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

macro_rules! create_currency_id {
    ($(#[$meta:meta])*
	$vis:vis enum TokenSymbol {
        $($(#[$vmeta:meta])* $symbol:ident($name:expr, $deci:literal) = $val:literal,)*
    }) => {
		$(#[$meta])*
		$vis enum TokenSymbol {
			$($(#[$vmeta])* $symbol = $val,)*
		}

		impl TryFrom<u8> for TokenSymbol {
			type Error = ();

			fn try_from(v: u8) -> Result<Self, Self::Error> {
				match v {
					$($val => Ok(TokenSymbol::$symbol),)*
					_ => Err(()),
				}
			}
		}

		impl Into<u8> for TokenSymbol {
			fn into(self) -> u8 {
				match self {
					$(TokenSymbol::$symbol => ($val),)*
				}
			}
		}

		impl TryFrom<Vec<u8>> for CurrencyId {
			type Error = ();
			fn try_from(v: Vec<u8>) -> Result<CurrencyId, ()> {
				match v.as_slice() {
					$(bstringify!($symbol) => Ok(CurrencyId::Token(TokenSymbol::$symbol)),)*
					_ => Err(()),
				}
			}
		}

		impl TokenInfo for CurrencyId {
			fn currency_id(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($val),)*
					_ => None,
				}
			}
			fn name(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($name),)*
					_ => None,
				}
			}
			fn symbol(&self) -> Option<&str> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some(stringify!($symbol)),)*
					_ => None,
				}
			}
			fn decimals(&self) -> Option<u8> {
				match self {
					$(CurrencyId::Token(TokenSymbol::$symbol) => Some($deci),)*
					_ => None,
				}
			}
		}

		$(pub const $symbol: CurrencyId = CurrencyId::Token(TokenSymbol::$symbol);)*

		impl TokenSymbol {
			pub fn get_info() -> Vec<(&'static str, u32)> {
				vec![
					$((stringify!($symbol), $deci),)*
				]
			}
		}

		#[test]
		#[ignore]
		fn generate_token_resources() {
			use crate::TokenSymbol::*;

			#[allow(non_snake_case)]
			#[derive(Serialize, Deserialize)]
			struct Token {
				symbol: String,
				address: EvmAddress,
			}

			// Selendra tokens
			let mut selendra_tokens = vec![];
			$(
				if $val < 128 {
					selendra_tokens.push(Token {
						symbol: stringify!($symbol).to_string(),
						address: EvmAddress::try_from(CurrencyId::Token(TokenSymbol::$symbol)).unwrap(),
					});
				}
			)*

			selendra_tokens.push(Token {
				symbol: "SA_DOT".to_string(),
				address: EvmAddress::try_from(CurrencyId::StableAssetPoolToken(0)).unwrap(),
			});

			selendra_tokens.push(Token {
				symbol: "SA_3USD".to_string(),
				address: EvmAddress::try_from(CurrencyId::StableAssetPoolToken(1)).unwrap(),
			});

			let mut selendra_lp_tokens = vec![
				Token {
					symbol: "LP_SEL_KUSD".to_string(),
					address: EvmAddress::try_from(crate::TradingPair::from_currency_ids(CurrencyId::Token(SEL), CurrencyId::Token(KUSD)).unwrap().dex_share_currency_id()).unwrap(),
				},
				Token {
					symbol: "LP_DOT_KUSD".to_string(),
					address: EvmAddress::try_from(crate::TradingPair::from_currency_ids(CurrencyId::Token(DOT), CurrencyId::Token(KUSD)).unwrap().dex_share_currency_id()).unwrap(),
				},
				Token {
					symbol: "LP_RENBTC_KUSD".to_string(),
					address: EvmAddress::try_from(crate::TradingPair::from_currency_ids(CurrencyId::Token(RENBTC), CurrencyId::Token(KUSD)).unwrap().dex_share_currency_id()).unwrap(),
				},
			];
			selendra_tokens.append(&mut selendra_lp_tokens);

			frame_support::assert_ok!(std::fs::write("../predeploy-contracts/resources/selendra_tokens.json", serde_json::to_string_pretty(&selendra_tokens).unwrap()));
		}
    }
}

create_currency_id! {
	// Represent a Token symbol with 8 bit
	//
	// 0 - 127: Selendra Ecosystem tokens
	// 0 - 19: Selendra & Bridge native tokens
	// 20 - 39: External tokens (e.g. bridged)
	//
	// 130 - 147: Kusama & Pokadot Bridge tokens
	#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[repr(u8)]
	pub enum TokenSymbol {
		// 0 - 19: Selendra & Polkadot native tokens
		SEL("Selendra", 12) = 0,
		KUSD("Khmer Dollar", 12) = 1,
		LSEL("Liquid Selendra", 12) = 2,

		// 20 - 39: External tokens (e.g. bridged)
		RENBTC("Ren Protocol BTC", 8) = 20,
		DAI("Dai Stable coin", 18) = 21,

		// 130 - 147: Polkadot & Kumasam bridged tokens
		KSM("Kusama", 12) = 130,
		DOT("Polkadot", 10) = 131,
	}
}

pub trait TokenInfo {
	fn currency_id(&self) -> Option<u8>;
	fn name(&self) -> Option<&str>;
	fn symbol(&self) -> Option<&str>;
	fn decimals(&self) -> Option<u8>;
}

pub type ForeignAssetId = u16;
pub type Erc20Id = u32;

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TypeInfo,
	MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum DexShare {
	Token(TokenSymbol),
	Erc20(EvmAddress),
	ForeignAsset(ForeignAssetId),
	StableAssetPoolToken(StableAssetPoolId),
}

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TypeInfo,
	MaxEncodedLen,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub enum CurrencyId {
	Token(TokenSymbol),
	DexShare(DexShare, DexShare),
	Erc20(EvmAddress),
	StableAssetPoolToken(StableAssetPoolId),
	ForeignAsset(ForeignAssetId),
}

impl CurrencyId {
	pub fn is_token_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Token(_))
	}

	pub fn is_dex_share_currency_id(&self) -> bool {
		matches!(self, CurrencyId::DexShare(_, _))
	}

	pub fn is_erc20_currency_id(&self) -> bool {
		matches!(self, CurrencyId::Erc20(_))
	}

	pub fn is_foreign_asset_currency_id(&self) -> bool {
		matches!(self, CurrencyId::ForeignAsset(_))
	}

	pub fn is_trading_pair_currency_id(&self) -> bool {
		matches!(
			self,
			CurrencyId::Token(_) |
				CurrencyId::Erc20(_) |
				CurrencyId::ForeignAsset(_) |
				CurrencyId::StableAssetPoolToken(_)
		)
	}

	pub fn split_dex_share_currency_id(&self) -> Option<(Self, Self)> {
		match self {
			CurrencyId::DexShare(dex_share_0, dex_share_1) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();
				let currency_id_1: CurrencyId = (*dex_share_1).into();
				Some((currency_id_0, currency_id_1))
			},
			_ => None,
		}
	}

	pub fn join_dex_share_currency_id(currency_id_0: Self, currency_id_1: Self) -> Option<Self> {
		let dex_share_0 = match currency_id_0 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			CurrencyId::Erc20(address) => DexShare::Erc20(address),
			CurrencyId::ForeignAsset(foreign_asset_id) => DexShare::ForeignAsset(foreign_asset_id),
			CurrencyId::StableAssetPoolToken(stable_asset_pool_id) =>
				DexShare::StableAssetPoolToken(stable_asset_pool_id),
			// Unsupported
			CurrencyId::DexShare(..) => return None,
		};
		let dex_share_1 = match currency_id_1 {
			CurrencyId::Token(symbol) => DexShare::Token(symbol),
			CurrencyId::Erc20(address) => DexShare::Erc20(address),
			CurrencyId::ForeignAsset(foreign_asset_id) => DexShare::ForeignAsset(foreign_asset_id),
			CurrencyId::StableAssetPoolToken(stable_asset_pool_id) =>
				DexShare::StableAssetPoolToken(stable_asset_pool_id),
			// Unsupported
			CurrencyId::DexShare(..) => return None,
		};
		Some(CurrencyId::DexShare(dex_share_0, dex_share_1))
	}
}

impl From<DexShare> for u32 {
	fn from(val: DexShare) -> u32 {
		let mut bytes = [0u8; 4];
		match val {
			DexShare::Token(token) => {
				bytes[3] = token.into();
			},
			DexShare::Erc20(address) => {
				// Use first 4 non-zero bytes as u32 to the mapping between u32 and evm address.
				// Take the first 4 non-zero bytes, if it is less than 4, add 0 to the left.
				let is_zero = |&&d: &&u8| -> bool { d == 0 };
				let leading_zeros = address.as_bytes().iter().take_while(is_zero).count();
				let index = if leading_zeros > 16 { 16 } else { leading_zeros };
				bytes[..].copy_from_slice(&address[index..index + 4][..]);
			},
			DexShare::ForeignAsset(foreign_asset_id) => {
				bytes[2..].copy_from_slice(&foreign_asset_id.to_be_bytes());
			},
			DexShare::StableAssetPoolToken(stable_asset_pool_id) => {
				bytes[..].copy_from_slice(&stable_asset_pool_id.to_be_bytes());
			},
		}
		u32::from_be_bytes(bytes)
	}
}

impl Into<CurrencyId> for DexShare {
	fn into(self) -> CurrencyId {
		match self {
			DexShare::Token(token) => CurrencyId::Token(token),
			DexShare::Erc20(address) => CurrencyId::Erc20(address),
			DexShare::ForeignAsset(foreign_asset_id) => CurrencyId::ForeignAsset(foreign_asset_id),
			DexShare::StableAssetPoolToken(stable_asset_pool_id) =>
				CurrencyId::StableAssetPoolToken(stable_asset_pool_id),
		}
	}
}

/// H160 CurrencyId Type enum
#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TryFromPrimitive,
	IntoPrimitive,
	TypeInfo,
)]
#[repr(u8)]
pub enum CurrencyIdType {
	Token = 1, // 0 is prefix of precompile and predeploy
	DexShare,
	StableAsset,
	ForeignAsset,
}

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	TryFromPrimitive,
	IntoPrimitive,
	TypeInfo,
)]
#[repr(u8)]
pub enum DexShareType {
	Token,
	Erc20,
	ForeignAsset,
	StableAssetPoolToken,
}

impl Into<DexShareType> for DexShare {
	fn into(self) -> DexShareType {
		match self {
			DexShare::Token(_) => DexShareType::Token,
			DexShare::Erc20(_) => DexShareType::Erc20,
			DexShare::ForeignAsset(_) => DexShareType::ForeignAsset,
			DexShare::StableAssetPoolToken(_) => DexShareType::StableAssetPoolToken,
		}
	}
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum AssetIds {
	Erc20(EvmAddress),
	StableAssetId(StableAssetPoolId),
	ForeignAssetId(ForeignAssetId),
	NativeAssetId(CurrencyId),
}

#[derive(Clone, Eq, PartialEq, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct AssetMetadata<Balance> {
	pub name: Vec<u8>,
	pub symbol: Vec<u8>,
	pub decimals: u8,
	pub minimal_balance: Balance,
}
