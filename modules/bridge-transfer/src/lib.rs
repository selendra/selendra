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

//! ## Overview
//! This pallet is used for bridging chains.

// Ensure we're `no_std` when compiling for WebAssembly.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub mod traits;
mod weights;

use crate::traits::WeightInfo;
use module_bridge::types::{ChainId, ResourceId};

pub use pallet::*;

use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{
		Currency, EnsureOrigin, ExistenceRequirement, ExistenceRequirement::AllowDeath, Get,
		OnUnbalanced, WithdrawReasons,
	},
	transactional, PalletId,
};

use frame_system::{ensure_root, pallet_prelude::OriginFor};
use sp_core::U256;
use sp_std::vec::Vec;

use sp_runtime::traits::{AccountIdConversion, CheckedAdd, CheckedSub, SaturatedConversion};

type BalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

// The name of the pallet is provided by `construct_runtime` and is used as
// the unique identifier for the pallet's storage. It is not defined in the
// pallet itself.
#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	// Bridge pallet type declaration.
	//
	// This structure is a placeholder for traits and functions implementation
	// for the pallet.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Bridge pallet's configuration trait.
	///
	/// Associated types and constants are declared in this trait. If the pallet
	/// depends on other super-traits, the latter must be added to this trait,
	/// Note that [`frame_system::Config`] must always be included.
	#[pallet::config]
	pub trait Config:
		frame_system::Config + module_bridge::Config + pallet_balances::Config
	{
		/// Pallet identifier.
		///
		/// The module identifier may be of the form ```PalletId(*b"c/bridge")``` (a string of eight
		/// characters) and set using the [`parameter_types`](https://substrate.dev/docs/en/knowledgebase/runtime/macros#parameter_types)
		/// macro in one of the runtimes (see runtime folder).
		#[pallet::constant]
		type BridgePalletId: Get<PalletId>;

		/// Specifies the origin check provided by the module_bridge for calls
		/// that can only be called by the module_bridge pallet.
		type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

		/// Admin user is able to modify transfer fees (see [NativeTokenTransferFee]).
		type AdminOrigin: EnsureOrigin<Self::Origin>;

		/// Currency as viewed from this pallet
		type Currency: Currency<Self::AccountId>;

		/// Associated type for Event enum
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		// Type for native token ID.
		#[pallet::constant]
		type NativeTokenId: Get<ResourceId>;

		/// Type for setting fee that are charged when transferring native tokens to target chains
		/// (in SELs).
		#[pallet::constant]
		type NativeTokenTransferFee: Get<u128>;

		/// The handler to absorb the fee.
		type OnTransactionFee: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Weight information for extrinsics in this pallet
		type WeightInfo: WeightInfo;
	}

	// The macro generates event metadata and derive Clone, Debug, Eq, PartialEq and Codec
	#[pallet::event]
	// The macro generates a function on Pallet to deposit an event
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Remark(T::Hash, ResourceId),
	}

	// Pallet storage items
	//
	// Additional fee charged when transferring native tokens to target chains (in SELs).
	#[pallet::storage]
	#[pallet::getter(fn get_native_token_transfer_fee)]
	pub type NativeTokenTransferFee<T> =
		StorageValue<_, u128, ValueQuery, <T as Config>::NativeTokenTransferFee>;

	// Pallet genesis configuration
	//
	// The genesis configuration type.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub chains: Vec<u8>,
		pub relayers: Vec<T::AccountId>,
		pub resources: Vec<(ResourceId, Vec<u8>)>,
		pub threshold: u32,
	}

	// The default value for the genesis config type.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self {
				chains: Default::default(),
				relayers: Default::default(),
				resources: Default::default(),
				threshold: Default::default(),
			}
		}
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			<Pallet<T>>::initialize(&self.chains, &self.relayers, &self.resources, &self.threshold);
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Invalid transfer
		InvalidTransfer,

		/// Not enough means for performing a transfer
		InsufficientBalance,

		/// Total amount to be transferred overflows balance type size
		TotalAmountOverflow,
	}

	// Declare Call struct and implement dispatchable (or callable) functions.
	//
	// Dispatchable functions are transactions modifying the state of the chain. They
	// are also called extrinsics are constitute the pallet's public interface.
	// Note that each parameter used in functions must implement `Clone`, `Debug`,
	// `Eq`, `PartialEq` and `Codec` traits.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Transfers some amount of the native token to some recipient on a (whitelisted)
		/// destination chain.
		#[pallet::weight(<T as Config>::WeightInfo::transfer_native())]
		#[transactional]
		pub fn transfer_native(
			origin: OriginFor<T>,
			amount: BalanceOf<T>,
			recipient: Vec<u8>,
			dest_id: ChainId,
		) -> DispatchResultWithPostInfo {
			let source = ensure_signed(origin)?;

			let token_transfer_fee: BalanceOf<T> =
				Self::get_native_token_transfer_fee().saturated_into();

			// Add fees to initial amount (so that to be sure account has sufficient funds)
			let total_transfer_amount =
				amount.checked_add(&token_transfer_fee).ok_or(Error::<T>::TotalAmountOverflow)?;

			// Ensure account has enough balance for both fee and transfer
			// Check to avoid balance errors down the line that leave balance storage in an
			// inconsistent state
			let remaining_balance = <T as pallet::Config>::Currency::free_balance(&source)
				.checked_sub(&total_transfer_amount)
				.ok_or(Error::<T>::InsufficientBalance)?;

			<T as pallet::Config>::Currency::ensure_can_withdraw(
				&source,
				total_transfer_amount,
				WithdrawReasons::all(),
				remaining_balance,
			)
			.map_err(|_| Error::<T>::InsufficientBalance)?;

			ensure!(
				<module_bridge::Pallet<T>>::chain_whitelisted(dest_id),
				Error::<T>::InvalidTransfer
			);

			// handle fee
			let imbalance = <T as Config>::Currency::withdraw(
				&source,
				NativeTokenTransferFee::<T>::get().saturated_into(),
				WithdrawReasons::FEE,
				ExistenceRequirement::AllowDeath,
			)?;

			T::OnTransactionFee::on_unbalanced(imbalance);

			let bridge_id = <module_bridge::Pallet<T>>::account_id();
			<T as pallet::Config>::Currency::transfer(
				&source,
				&bridge_id,
				amount.into(),
				AllowDeath,
			)?;

			let resource_id = T::NativeTokenId::get();
			<module_bridge::Pallet<T>>::transfer_fungible(
				dest_id,
				resource_id.into(),
				recipient,
				// Note: use u128 to restrict balance greater than 128bits
				U256::from(amount.saturated_into::<u128>()),
			)?;

			Ok(().into())
		}

		/// Executes a simple currency transfer using the module_bridge account as the source
		#[pallet::weight(<T as Config>::WeightInfo::transfer())]
		#[transactional]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
			_r_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			let source = T::BridgeOrigin::ensure_origin(origin)?;
			<T as pallet::Config>::Currency::transfer(&source, &to, amount.into(), AllowDeath)?;

			Ok(().into())
		}

		/// This can be called by the module_bridge to demonstrate an arbitrary call from a
		/// proposal.
		#[pallet::weight(<T as Config>::WeightInfo::remark())]
		pub fn remark(
			origin: OriginFor<T>,
			hash: T::Hash,
			r_id: ResourceId,
		) -> DispatchResultWithPostInfo {
			T::BridgeOrigin::ensure_origin(origin)?;
			Self::deposit_event(Event::Remark(hash, r_id));

			Ok(().into())
		}

		/// Modify native token transfer fee value
		#[pallet::weight(<T as Config>::WeightInfo::set_token_transfer_fee())]
		pub fn set_native_token_transfer_fee(
			origin: OriginFor<T>,
			new_fee: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin(origin)?;
			NativeTokenTransferFee::<T>::mutate(|fee_value| *fee_value = new_fee.saturated_into());

			Ok(().into())
		}
	}
}

// Pallet implementation block.
//
// This main implementation block contains two categories of functions, namely:
// - Public functions: These are functions that are `pub` and generally fall into inspector
//   functions that do not write to storage and operation functions that do.
// - Private functions: These are private helpers or utilities that cannot be called from other
//   pallets.
impl<T: Config> Pallet<T> {
	// *** Utility methods ***

	/// Return the account identifier of the RAD claims pallet.
	///
	/// This actually does computation. If you need to keep using it, then make
	/// sure you cache the value and only call this once.
	pub fn account_id() -> T::AccountId {
		<T as pallet::Config>::BridgePalletId::get().into_account_truncating()
	}

	/// Initialize pallet's genesis configuration.
	///
	/// This private helper function is used for setting up pallet genesis
	/// configuration.
	fn initialize(
		chains: &[u8],
		relayers: &[T::AccountId],
		resources: &Vec<(ResourceId, Vec<u8>)>,
		threshold: &u32,
	) {
		chains.into_iter().for_each(|c| {
			<module_bridge::Pallet<T>>::whitelist(*c).unwrap_or_default();
		});
		relayers.into_iter().for_each(|rs| {
			<module_bridge::Pallet<T>>::register_relayer(rs.clone()).unwrap_or_default();
		});

		<module_bridge::Pallet<T>>::set_relayer_threshold(*threshold).unwrap_or_default();

		resources.iter().for_each(|i| {
			let (rid, m) = (i.0.clone(), i.1.clone());
			<module_bridge::Pallet<T>>::register_resource(rid.into(), m.clone())
				.unwrap_or_default();
		});
	}

	// Ensure that the caller has admin rights
	fn ensure_admin(origin: OriginFor<T>) -> DispatchResult {
		<T as Config>::AdminOrigin::try_origin(origin)
			.map(|_| ())
			.or_else(ensure_root)?;
		Ok(())
	}
}
