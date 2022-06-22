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

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! # Emergency Shutdown Module
//!
//! ## Overview
//!
//! When a black swan occurs such as price plunge or fatal bug, the highest
//! priority is to minimize user losses as much as possible. When the decision
//! to shutdown system is made, emergency shutdown module needs to trigger all
//! related module to halt, and start a series of operations including close
//! some user entry, freeze feed prices, run offchain worker to settle
//! CDPs has debit, when debits and gaps are settled, the stable currency
//! holder are allowed to refund a basket of remaining collateral assets.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::unused_unit)]

use frame_support::{pallet_prelude::*, transactional};
use frame_system::pallet_prelude::*;
use primitives::{Balance, CurrencyId};
use sp_std::prelude::*;
use support::{EmergencyShutdown, LockablePrice, SelTreasury};

mod mock;
mod tests;
pub mod weights;

pub use module::*;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod module {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Price source to freeze currencies' price
		type PriceSource: LockablePrice<CurrencyId>;

		/// CDP treasury to escrow collateral assets after settlement
		type SelTreasury: SelTreasury<Self::AccountId, Balance = Balance, CurrencyId = CurrencyId>;

		/// The origin which may trigger emergency shutdown. Root can always do
		/// this.
		type ShutdownOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for the extrinsics in this module.
		type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
		/// System has already been shutdown
		AlreadyShutdown,
		/// Must after system shutdown
		MustAfterShutdown,
		/// Final redemption is still not opened
		CanNotRefund,
		/// Exist potential surplus, means settlement has not been completed
		ExistPotentialSurplus,
		/// Exist unhandled debit, means settlement has not been completed
		ExistUnhandledDebit,
	}

	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emergency shutdown occurs.
		Shutdown { block_number: T::BlockNumber },
		/// The final redemption opened.
		OpenRefund { block_number: T::BlockNumber },
		/// Refund info.
		Refund {
			who: T::AccountId,
			stable_coin_amount: Balance,
			refund_list: Vec<(CurrencyId, Balance)>,
		},
	}

	/// Emergency shutdown flag
	///
	/// IsShutdown: bool
	#[pallet::storage]
	#[pallet::getter(fn is_shutdown)]
	pub type IsShutdown<T: Config> = StorageValue<_, bool, ValueQuery>;

	/// Open final redemption flag
	///
	/// CanRefund: bool
	#[pallet::storage]
	#[pallet::getter(fn can_refund)]
	pub type CanRefund<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Start emergency shutdown
		///
		/// The dispatch origin of this call must be `ShutdownOrigin`.
		#[pallet::weight((T::WeightInfo::emergency_shutdown(), DispatchClass::Operational))]
		#[transactional]
		pub fn emergency_shutdown(origin: OriginFor<T>) -> DispatchResult {
			T::ShutdownOrigin::ensure_origin(origin)?;
			ensure!(!Self::is_shutdown(), Error::<T>::AlreadyShutdown);

			IsShutdown::<T>::put(true);
			Self::deposit_event(Event::Shutdown {
				block_number: <frame_system::Pallet<T>>::block_number(),
			});
			Ok(())
		}

		/// Stop emergency shutdown
		///
		/// The dispatch origin of this call must be `ShutdownOrigin`.
		#[pallet::weight((T::WeightInfo::emergency_shutdown(), DispatchClass::Operational))]
		#[transactional]
		pub fn stop_emergency_shutdown(origin: OriginFor<T>) -> DispatchResult {
			T::ShutdownOrigin::ensure_origin(origin)?;
			ensure!(!Self::is_shutdown(), Error::<T>::AlreadyShutdown);

			IsShutdown::<T>::put(false);
			Self::deposit_event(Event::Shutdown {
				block_number: <frame_system::Pallet<T>>::block_number(),
			});
			Ok(())
		}
	}
}

impl<T: Config> EmergencyShutdown for Pallet<T> {
	fn is_shutdown() -> bool {
		Self::is_shutdown()
	}
}
