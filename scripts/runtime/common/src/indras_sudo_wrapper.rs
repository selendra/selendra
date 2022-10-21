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

//! A simple wrapper allowing `Sudo` to call into `indras` routines.

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
pub use pallet::*;
use parity_scale_codec::Encode;
use primitives::v2::Id as IndraId;
use runtime_indracores::{
	configuration, dmp, hrmp, indras, indras::IndraGenesisArgs, ump, IndraLifecycle,
};
use sp_std::boxed::Box;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config:
		configuration::Config + indras::Config + dmp::Config + ump::Config + hrmp::Config
	{
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The specified indracore or indrabase is not registered.
		IndraDoesntExist,
		/// The specified indracore or indrabase is already registered.
		IndraAlreadyExists,
		/// A DMP message couldn't be sent because it exceeds the maximum size allowed for a downward
		/// message.
		ExceedsMaxMessageSize,
		/// Could not schedule indra cleanup.
		CouldntCleanup,
		/// Not a indrabase.
		NotIndrabase,
		/// Not a indracore.
		NotIndracore,
		/// Cannot upgrade indrabase.
		CannotUpgrade,
		/// Cannot downgrade indracore.
		CannotDowngrade,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Schedule a indra to be initialized at the start of the next session.
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_schedule_indra_initialize(
			origin: OriginFor<T>,
			id: IndraId,
			genesis: IndraGenesisArgs,
		) -> DispatchResult {
			ensure_root(origin)?;
			runtime_indracores::schedule_indra_initialize::<T>(id, genesis)
				.map_err(|_| Error::<T>::IndraAlreadyExists)?;
			Ok(())
		}

		/// Schedule a indra to be cleaned up at the start of the next session.
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_schedule_indra_cleanup(origin: OriginFor<T>, id: IndraId) -> DispatchResult {
			ensure_root(origin)?;
			runtime_indracores::schedule_indra_cleanup::<T>(id)
				.map_err(|_| Error::<T>::CouldntCleanup)?;
			Ok(())
		}

		/// Upgrade a indrabase to a indracore
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_schedule_indrabase_upgrade(
			origin: OriginFor<T>,
			id: IndraId,
		) -> DispatchResult {
			ensure_root(origin)?;
			// Indra backend should think this is a indrabase...
			ensure!(
				indras::Pallet::<T>::lifecycle(id) == Some(IndraLifecycle::Indrabase),
				Error::<T>::NotIndrabase,
			);
			runtime_indracores::schedule_indrabase_upgrade::<T>(id)
				.map_err(|_| Error::<T>::CannotUpgrade)?;
			Ok(())
		}

		/// Downgrade a indracore to a indrabase
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_schedule_indracore_downgrade(
			origin: OriginFor<T>,
			id: IndraId,
		) -> DispatchResult {
			ensure_root(origin)?;
			// Indra backend should think this is a indracore...
			ensure!(
				indras::Pallet::<T>::lifecycle(id) == Some(IndraLifecycle::Indracore),
				Error::<T>::NotIndracore,
			);
			runtime_indracores::schedule_indracore_downgrade::<T>(id)
				.map_err(|_| Error::<T>::CannotDowngrade)?;
			Ok(())
		}

		/// Send a downward XCM to the given indra.
		///
		/// The given indracore should exist and the payload should not exceed the preconfigured size
		/// `config.max_downward_message_size`.
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_queue_downward_xcm(
			origin: OriginFor<T>,
			id: IndraId,
			xcm: Box<xcm::opaque::VersionedXcm>,
		) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<indras::Pallet<T>>::is_valid_indra(id), Error::<T>::IndraDoesntExist);
			let config = <configuration::Pallet<T>>::config();
			<dmp::Pallet<T>>::queue_downward_message(&config, id, xcm.encode()).map_err(|e| match e
			{
				dmp::QueueDownwardMessageError::ExceedsMaxMessageSize =>
					Error::<T>::ExceedsMaxMessageSize.into(),
			})
		}

		/// Forcefully establish a channel from the sender to the recipient.
		///
		/// This is equivalent to sending an `Hrmp::hrmp_init_open_channel` extrinsic followed by
		/// `Hrmp::hrmp_accept_open_channel`.
		#[pallet::weight((1_000, DispatchClass::Operational))]
		pub fn sudo_establish_hrmp_channel(
			origin: OriginFor<T>,
			sender: IndraId,
			recipient: IndraId,
			max_capacity: u32,
			max_message_size: u32,
		) -> DispatchResult {
			ensure_root(origin)?;

			<hrmp::Pallet<T>>::init_open_channel(
				sender,
				recipient,
				max_capacity,
				max_message_size,
			)?;
			<hrmp::Pallet<T>>::accept_open_channel(recipient, sender)?;
			Ok(())
		}
	}
}
