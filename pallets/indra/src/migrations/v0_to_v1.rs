// Copyright 2023 Smallworld Selendra
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

use frame_support::{
	log, storage_alias,
	traits::{Get, OnRuntimeUpgrade, PalletInfoAccess, StorageVersion},
	weights::Weight,
};
use selendra_primitives::SessionIndex;
use sp_std::vec::Vec;
#[cfg(feature = "try-runtime")]
use {
	codec::{Decode, Encode},
	frame_support::ensure,
	pallets_support::ensure_storage_version,
};

use crate::Config;

type Accounts<T> = Vec<<T as frame_system::Config>::AccountId>;

#[storage_alias]
type SessionForValidatorsChange = StorageValue<Indra, SessionIndex>;

#[storage_alias]
type Validators<T> = StorageValue<Indra, Accounts<T>>;

/// Flattening double `Option<>` storage.
pub struct Migration<T, P>(sp_std::marker::PhantomData<(T, P)>);

#[cfg(feature = "try-runtime")]
#[derive(Decode, Encode)]
struct MigrationChecksState<T: Config> {
	session: Option<Option<SessionIndex>>,
	validators: Option<Option<Accounts<T>>>,
}

impl<T: Config, P: PalletInfoAccess> OnRuntimeUpgrade for Migration<T, P> {
	fn on_runtime_upgrade() -> Weight {
		log::info!(target: "pallet_indra", "Running migration from STORAGE_VERSION 0 to 1");

		let mut writes = 0;

		match SessionForValidatorsChange::translate(
			|old: Option<Option<SessionIndex>>| -> Option<SessionIndex> {
				log::info!(target: "pallet_indra", "Current storage value for SessionForValidatorsChange {:?}", old);
				match old {
					Some(Some(x)) => Some(x),
					_ => None,
				}
			},
		) {
			Ok(_) => {
				writes += 1;
				log::info!(target: "pallet_indra", "Successfully migrated storage for SessionForValidatorsChange");
			},
			Err(why) => {
				log::error!(target: "pallet_indra", "Something went wrong during the migration of SessionForValidatorsChange {:?}", why);
			},
		};

		match Validators::<T>::translate(
			|old: Option<Option<Vec<T::AccountId>>>| -> Option<Vec<T::AccountId>> {
				log::info!(target: "pallet_indra", "Current storage value for Validators {:?}", old);
				match old {
					Some(Some(x)) => Some(x),
					_ => None,
				}
			},
		) {
			Ok(_) => {
				writes += 1;
				log::info!(target: "pallet_indra", "Successfully migrated storage for Validators");
			},
			Err(why) => {
				log::error!(target: "pallet_indra", "Something went wrong during the migration of Validators storage {:?}", why);
			},
		};

		// store new version
		StorageVersion::new(1).put::<P>();
		writes += 1;

		T::DbWeight::get().reads(2) + T::DbWeight::get().writes(writes)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		#[storage_alias]
		type SessionForValidatorsChange = StorageValue<Indra, Option<SessionIndex>>;
		#[storage_alias]
		type Validators<T> = StorageValue<Indra, Option<Accounts<T>>>;

		ensure_storage_version::<P>(0)?;

		let session = SessionForValidatorsChange::get();
		let validators = Validators::<T>::get();

		Ok(MigrationChecksState::<T> { session, validators }.encode())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(state: Vec<u8>) -> Result<(), &'static str> {
		ensure_storage_version::<P>(1)?;

		let MigrationChecksState { session: old_session, validators: old_validators } =
			<MigrationChecksState<T>>::decode(&mut &*state)
				.map_err(|_| "Failed to decode old state")?;

		let new_session = SessionForValidatorsChange::get();

		match old_session {
			Some(Some(session)) =>
				ensure!(Some(session) == new_session, "Mismatch on `SessionForValidatorsChange`",),
			_ =>
				ensure!(new_session.is_none(), "New `SessionForValidatorsChange` should be `None`"),
		};

		let new_validators = Validators::<T>::get();

		match old_validators {
			Some(Some(validators)) =>
				ensure!(Some(validators) == new_validators, "Mismatch on `Validators`",),
			_ => ensure!(new_validators.is_none(), "New `Validators` should be `None`"),
		};

		Ok(())
	}
}
