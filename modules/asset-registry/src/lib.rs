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

//! # Asset Registry Module
//!
//! Local and foreign assets management. The foreign assets can be updated without runtime upgrade.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{
	assert_ok,
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::*,
	traits::{Currency, EnsureOrigin},
	transactional,
};
use frame_system::pallet_prelude::*;
use module_support::{AssetIdMapping, EVMBridge, Erc20InfoMapping, InvokeContext};
use primitives::{
	currency::{
		AssetIds, AssetMetadata, CurrencyIdType, DexShare, DexShareType, Erc20Id, ForeignAssetId,
		StableAssetPoolId, TokenInfo,
	},
	evm::{
		is_system_contract, EvmAddress, H160_POSITION_CURRENCY_ID_TYPE,
		H160_POSITION_DEXSHARE_LEFT_FIELD, H160_POSITION_DEXSHARE_LEFT_TYPE,
		H160_POSITION_DEXSHARE_RIGHT_FIELD, H160_POSITION_DEXSHARE_RIGHT_TYPE,
		H160_POSITION_FOREIGN_ASSET, H160_POSITION_STABLE_ASSET, H160_POSITION_TOKEN,
	},
	CurrencyId,
};
use sp_runtime::{traits::One, ArithmeticError};
use sp_std::{boxed::Box, vec::Vec};

mod mock;
mod tests;
mod weights;

pub use module::*;
pub use weights::WeightInfo;

/// Type alias for currency balance.
pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Currency type for withdraw and balance storage.
		type Currency: Currency<Self::AccountId>;

		/// Evm Bridge for getting info of contracts from the EVM.
		type EVMBridge: EVMBridge<Self::AccountId, BalanceOf<Self>>;

		/// Required origin for registering asset.
		type RegisterOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The given location could not be used (e.g. because it cannot be expressed in the
		BadLocation,
		/// AssetId not exists
		AssetIdNotExists,
		/// AssetId exists
		AssetIdExisted,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// The asset registered.
		AssetRegistered { asset_id: AssetIds, metadata: AssetMetadata<BalanceOf<T>> },
		/// The asset updated.
		AssetUpdated { asset_id: AssetIds, metadata: AssetMetadata<BalanceOf<T>> },
	}

	/// Next available Stable AssetId ID.
	///
	/// NextStableAssetId: StableAssetPoolId
	#[pallet::storage]
	#[pallet::getter(fn next_stable_asset_id)]
	pub type NextStableAssetId<T: Config> = StorageValue<_, StableAssetPoolId, ValueQuery>;

	/// The storages for EvmAddress.
	///
	/// Erc20IdToAddress: map Erc20Id => Option<EvmAddress>
	#[pallet::storage]
	#[pallet::getter(fn erc20_id_to_address)]
	pub type Erc20IdToAddress<T: Config> =
		StorageMap<_, Twox64Concat, Erc20Id, EvmAddress, OptionQuery>;

	/// The storages for AssetMetadatas.
	///
	/// AssetMetadatas: map AssetIds => Option<AssetMetadata>
	#[pallet::storage]
	#[pallet::getter(fn asset_metadatas)]
	pub type AssetMetadatas<T: Config> =
		StorageMap<_, Twox64Concat, AssetIds, AssetMetadata<BalanceOf<T>>, OptionQuery>;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub assets: Vec<(CurrencyId, BalanceOf<T>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			GenesisConfig { assets: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			self.assets.iter().for_each(|(asset, ed)| {
				assert_ok!(Pallet::<T>::do_register_native_asset(
					*asset,
					&AssetMetadata {
						name: asset.name().unwrap().as_bytes().to_vec(),
						symbol: asset.symbol().unwrap().as_bytes().to_vec(),
						decimals: asset.decimals().unwrap(),
						minimal_balance: *ed,
					}
				));
			});
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::register_stable_asset())]
		#[transactional]
		pub fn register_stable_asset(
			origin: OriginFor<T>,
			metadata: Box<AssetMetadata<BalanceOf<T>>>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			let stable_asset_id = Self::do_register_stable_asset(&metadata)?;

			Self::deposit_event(Event::<T>::AssetRegistered {
				asset_id: AssetIds::StableAssetId(stable_asset_id),
				metadata: *metadata,
			});
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_stable_asset())]
		#[transactional]
		pub fn update_stable_asset(
			origin: OriginFor<T>,
			stable_asset_id: StableAssetPoolId,
			metadata: Box<AssetMetadata<BalanceOf<T>>>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			Self::do_update_stable_asset(&stable_asset_id, &metadata)?;

			Self::deposit_event(Event::<T>::AssetUpdated {
				asset_id: AssetIds::StableAssetId(stable_asset_id),
				metadata: *metadata,
			});
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::register_erc20_asset())]
		#[transactional]
		pub fn register_erc20_asset(
			origin: OriginFor<T>,
			contract: EvmAddress,
			minimal_balance: BalanceOf<T>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			let metadata = Self::do_register_erc20_asset(contract, minimal_balance)?;

			Self::deposit_event(Event::<T>::AssetRegistered {
				asset_id: AssetIds::Erc20(contract),
				metadata,
			});
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_erc20_asset())]
		#[transactional]
		pub fn update_erc20_asset(
			origin: OriginFor<T>,
			contract: EvmAddress,
			metadata: Box<AssetMetadata<BalanceOf<T>>>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			Self::do_update_erc20_asset(contract, &metadata)?;

			Self::deposit_event(Event::<T>::AssetUpdated {
				asset_id: AssetIds::Erc20(contract),
				metadata: *metadata,
			});
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::register_native_asset())]
		#[transactional]
		pub fn register_native_asset(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			metadata: Box<AssetMetadata<BalanceOf<T>>>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			Self::do_register_native_asset(currency_id, &metadata)?;

			Self::deposit_event(Event::<T>::AssetRegistered {
				asset_id: AssetIds::NativeAssetId(currency_id),
				metadata: *metadata,
			});
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_native_asset())]
		#[transactional]
		pub fn update_native_asset(
			origin: OriginFor<T>,
			currency_id: CurrencyId,
			metadata: Box<AssetMetadata<BalanceOf<T>>>,
		) -> DispatchResult {
			T::RegisterOrigin::ensure_origin(origin)?;

			Self::do_update_native_asset(currency_id, &metadata)?;

			Self::deposit_event(Event::<T>::AssetUpdated {
				asset_id: AssetIds::NativeAssetId(currency_id),
				metadata: *metadata,
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn get_next_stable_asset_id() -> Result<StableAssetPoolId, DispatchError> {
		NextStableAssetId::<T>::try_mutate(|current| -> Result<StableAssetPoolId, DispatchError> {
			let id = *current;
			*current = current.checked_add(One::one()).ok_or(ArithmeticError::Overflow)?;
			Ok(id)
		})
	}

	fn do_register_stable_asset(
		metadata: &AssetMetadata<BalanceOf<T>>,
	) -> Result<StableAssetPoolId, DispatchError> {
		let stable_asset_id = Self::get_next_stable_asset_id()?;
		AssetMetadatas::<T>::try_mutate(
			AssetIds::StableAssetId(stable_asset_id),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_none(), Error::<T>::AssetIdExisted);

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)?;

		Ok(stable_asset_id)
	}

	fn do_update_stable_asset(
		stable_asset_id: &StableAssetPoolId,
		metadata: &AssetMetadata<BalanceOf<T>>,
	) -> DispatchResult {
		AssetMetadatas::<T>::try_mutate(
			AssetIds::StableAssetId(*stable_asset_id),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_some(), Error::<T>::AssetIdNotExists);

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)
	}

	fn do_register_erc20_asset(
		contract: EvmAddress,
		minimal_balance: BalanceOf<T>,
	) -> Result<AssetMetadata<BalanceOf<T>>, DispatchError> {
		let invoke_context =
			InvokeContext { contract, sender: Default::default(), origin: Default::default() };

		let metadata = AssetMetadata {
			name: T::EVMBridge::name(invoke_context)?,
			symbol: T::EVMBridge::symbol(invoke_context)?,
			decimals: T::EVMBridge::decimals(invoke_context)?,
			minimal_balance,
		};

		let erc20_id = Into::<Erc20Id>::into(DexShare::Erc20(contract));

		AssetMetadatas::<T>::try_mutate(
			AssetIds::Erc20(contract),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_none(), Error::<T>::AssetIdExisted);

				Erc20IdToAddress::<T>::try_mutate(erc20_id, |maybe_address| -> DispatchResult {
					ensure!(maybe_address.is_none(), Error::<T>::AssetIdExisted);
					*maybe_address = Some(contract);

					Ok(())
				})?;

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)?;

		Ok(metadata)
	}

	fn do_update_erc20_asset(
		contract: EvmAddress,
		metadata: &AssetMetadata<BalanceOf<T>>,
	) -> DispatchResult {
		AssetMetadatas::<T>::try_mutate(
			AssetIds::Erc20(contract),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_some(), Error::<T>::AssetIdNotExists);

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)
	}

	fn do_register_native_asset(
		asset: CurrencyId,
		metadata: &AssetMetadata<BalanceOf<T>>,
	) -> DispatchResult {
		AssetMetadatas::<T>::try_mutate(
			AssetIds::NativeAssetId(asset),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_none(), Error::<T>::AssetIdExisted);

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)?;

		Ok(())
	}

	fn do_update_native_asset(
		currency_id: CurrencyId,
		metadata: &AssetMetadata<BalanceOf<T>>,
	) -> DispatchResult {
		AssetMetadatas::<T>::try_mutate(
			AssetIds::NativeAssetId(currency_id),
			|maybe_asset_metadatas| -> DispatchResult {
				ensure!(maybe_asset_metadatas.is_some(), Error::<T>::AssetIdNotExists);

				*maybe_asset_metadatas = Some(metadata.clone());
				Ok(())
			},
		)
	}
}

pub struct AssetIdMaps<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> AssetIdMapping<AssetMetadata<BalanceOf<T>>> for AssetIdMaps<T> {
	fn get_asset_metadata(asset_ids: AssetIds) -> Option<AssetMetadata<BalanceOf<T>>> {
		Pallet::<T>::asset_metadatas(asset_ids)
	}
}

pub struct EvmErc20InfoMapping<T>(sp_std::marker::PhantomData<T>);

impl<T: Config> EvmErc20InfoMapping<T> {
	fn name_for_dex_share(symbol: DexShare) -> Option<Vec<u8>> {
		match symbol {
			DexShare::Token(symbol) =>
				CurrencyId::Token(symbol).name().map(|v| v.as_bytes().to_vec()),
			DexShare::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.name),
			DexShare::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id)).map(|v| v.name),
			DexShare::StableAssetPoolToken(stable_asset_pool_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_pool_id))
					.map(|v| v.name),
		}
	}

	fn symbol_for_dex_share(symbol: DexShare) -> Option<Vec<u8>> {
		match symbol {
			DexShare::Token(symbol) =>
				CurrencyId::Token(symbol).symbol().map(|v| v.as_bytes().to_vec()),
			DexShare::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.symbol),
			DexShare::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id))
					.map(|v| v.symbol),
			DexShare::StableAssetPoolToken(stable_asset_pool_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_pool_id))
					.map(|v| v.symbol),
		}
	}

	fn decimal_for_dex_share(symbol: DexShare) -> Option<u8> {
		match symbol {
			DexShare::Token(symbol) => CurrencyId::Token(symbol).decimals(),
			DexShare::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.decimals),
			DexShare::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id))
					.map(|v| v.decimals),
			DexShare::StableAssetPoolToken(stable_asset_pool_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_pool_id))
					.map(|v| v.decimals),
		}
	}

	fn decode_evm_address_for_dex_share(address: &[u8], left: bool) -> Option<DexShare> {
		let (dex_share_type, dex_share_field) = if left {
			(H160_POSITION_DEXSHARE_LEFT_TYPE, H160_POSITION_DEXSHARE_LEFT_FIELD)
		} else {
			(H160_POSITION_DEXSHARE_RIGHT_TYPE, H160_POSITION_DEXSHARE_RIGHT_FIELD)
		};
		match DexShareType::try_from(address[dex_share_type]).ok()? {
			DexShareType::Token => address[dex_share_field][3].try_into().map(DexShare::Token).ok(),
			DexShareType::Erc20 => {
				let id = u32::from_be_bytes(address[dex_share_field].try_into().ok()?);
				Erc20IdToAddress::<T>::get(id).map(DexShare::Erc20)
			},
			DexShareType::ForeignAsset => {
				let id =
					ForeignAssetId::from_be_bytes(address[dex_share_field][2..].try_into().ok()?);
				Some(DexShare::ForeignAsset(id))
			},
			DexShareType::StableAssetPoolToken => {
				let id =
					StableAssetPoolId::from_be_bytes(address[dex_share_field][..].try_into().ok()?);
				Some(DexShare::StableAssetPoolToken(id))
			},
		}
	}
}

impl<T: Config> Erc20InfoMapping for EvmErc20InfoMapping<T> {
	// Returns the name associated with a given CurrencyId.
	// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	// the EvmAddress must have been mapped.
	fn name(currency_id: CurrencyId) -> Option<Vec<u8>> {
		let name = match currency_id {
			CurrencyId::Token(_) =>
				AssetMetadatas::<T>::get(AssetIds::NativeAssetId(currency_id)).map(|v| v.name),
			CurrencyId::DexShare(symbol_0, symbol_1) => {
				let name_0 = EvmErc20InfoMapping::<T>::name_for_dex_share(symbol_0)?;
				let name_1 = EvmErc20InfoMapping::<T>::name_for_dex_share(symbol_1)?;

				let mut vec = Vec::new();
				vec.extend_from_slice(&b"LP "[..]);
				vec.extend_from_slice(&name_0);
				vec.extend_from_slice(&b" - "[..]);
				vec.extend_from_slice(&name_1);
				Some(vec)
			},
			CurrencyId::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.name),
			CurrencyId::StableAssetPoolToken(stable_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_id)).map(|v| v.name),
			CurrencyId::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id)).map(|v| v.name),
		}?;

		// More than 32 bytes will be truncated.
		if name.len() > 32 {
			Some(name[..32].to_vec())
		} else {
			Some(name)
		}
	}

	// Returns the symbol associated with a given CurrencyId.
	// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	// the EvmAddress must have been mapped.
	fn symbol(currency_id: CurrencyId) -> Option<Vec<u8>> {
		let symbol = match currency_id {
			CurrencyId::Token(_) =>
				AssetMetadatas::<T>::get(AssetIds::NativeAssetId(currency_id)).map(|v| v.symbol),
			CurrencyId::DexShare(symbol_0, symbol_1) => {
				let token_symbol_0 = EvmErc20InfoMapping::<T>::symbol_for_dex_share(symbol_0)?;
				let token_symbol_1 = EvmErc20InfoMapping::<T>::symbol_for_dex_share(symbol_1)?;

				let mut vec = Vec::new();
				vec.extend_from_slice(&b"LP_"[..]);
				vec.extend_from_slice(&token_symbol_0);
				vec.extend_from_slice(&b"_"[..]);
				vec.extend_from_slice(&token_symbol_1);
				Some(vec)
			},
			CurrencyId::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.symbol),
			CurrencyId::StableAssetPoolToken(stable_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_id)).map(|v| v.symbol),
			CurrencyId::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id))
					.map(|v| v.symbol),
		}?;

		// More than 32 bytes will be truncated.
		if symbol.len() > 32 {
			Some(symbol[..32].to_vec())
		} else {
			Some(symbol)
		}
	}

	// Returns the decimals associated with a given CurrencyId.
	// If CurrencyId is CurrencyId::DexShare and contain DexShare::Erc20,
	// the EvmAddress must have been mapped.
	fn decimals(currency_id: CurrencyId) -> Option<u8> {
		match currency_id {
			CurrencyId::Token(_) =>
				AssetMetadatas::<T>::get(AssetIds::NativeAssetId(currency_id)).map(|v| v.decimals),
			CurrencyId::DexShare(symbol_0, _) => {
				// initial dex share amount is calculated based on currency_id_0,
				// use the decimals of currency_id_0 as the decimals of lp token.
				EvmErc20InfoMapping::<T>::decimal_for_dex_share(symbol_0)
			},
			CurrencyId::Erc20(address) =>
				AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|v| v.decimals),
			CurrencyId::StableAssetPoolToken(stable_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::StableAssetId(stable_asset_id))
					.map(|v| v.decimals),
			CurrencyId::ForeignAsset(foreign_asset_id) =>
				AssetMetadatas::<T>::get(AssetIds::ForeignAssetId(foreign_asset_id))
					.map(|v| v.decimals),
		}
	}

	// Encode the CurrencyId to EvmAddress.
	// If is CurrencyId::DexShare and contain DexShare::Erc20,
	// will use the u32 to get the DexShare::Erc20 from the mapping.
	fn encode_evm_address(v: CurrencyId) -> Option<EvmAddress> {
		match v {
			CurrencyId::DexShare(left, right) => {
				match left {
					DexShare::Erc20(address) => {
						// ensure erc20 is mapped
						AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|_| ())?;
					},
					DexShare::Token(_) |
					DexShare::ForeignAsset(_) |
					DexShare::StableAssetPoolToken(_) => {},
				};
				match right {
					DexShare::Erc20(address) => {
						// ensure erc20 is mapped
						AssetMetadatas::<T>::get(AssetIds::Erc20(address)).map(|_| ())?;
					},
					DexShare::Token(_) |
					DexShare::ForeignAsset(_) |
					DexShare::StableAssetPoolToken(_) => {},
				};
			},
			CurrencyId::Token(_) |
			CurrencyId::Erc20(_) |
			CurrencyId::StableAssetPoolToken(_) |
			CurrencyId::ForeignAsset(_) => {},
		};

		EvmAddress::try_from(v).ok()
	}

	// Decode the CurrencyId from EvmAddress.
	// If is CurrencyId::DexShare and contain DexShare::Erc20,
	// will use the u32 to get the DexShare::Erc20 from the mapping.
	fn decode_evm_address(addr: EvmAddress) -> Option<CurrencyId> {
		if !is_system_contract(addr) {
			return Some(CurrencyId::Erc20(addr))
		}

		let address = addr.as_bytes();
		let currency_id =
			match CurrencyIdType::try_from(address[H160_POSITION_CURRENCY_ID_TYPE]).ok()? {
				CurrencyIdType::Token =>
					address[H160_POSITION_TOKEN].try_into().map(CurrencyId::Token).ok(),
				CurrencyIdType::DexShare => {
					let left =
						EvmErc20InfoMapping::<T>::decode_evm_address_for_dex_share(address, true)?;
					let right =
						EvmErc20InfoMapping::<T>::decode_evm_address_for_dex_share(address, false)?;
					Some(CurrencyId::DexShare(left, right))
				},
				CurrencyIdType::StableAsset => {
					let id = StableAssetPoolId::from_be_bytes(
						address[H160_POSITION_STABLE_ASSET].try_into().ok()?,
					);
					Some(CurrencyId::StableAssetPoolToken(id))
				},
				CurrencyIdType::ForeignAsset => {
					let id = ForeignAssetId::from_be_bytes(
						address[H160_POSITION_FOREIGN_ASSET].try_into().ok()?,
					);
					Some(CurrencyId::ForeignAsset(id))
				},
			};

		// Make sure that every bit of the address is the same
		Self::encode_evm_address(currency_id?).and_then(|encoded| {
			if encoded == addr {
				currency_id
			} else {
				None
			}
		})
	}
}
