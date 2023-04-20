// Copyright 2023 Selendra by SmallWorld Cambodia
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
#[cfg(feature = "try-runtime")]
use {frame_support::ensure, pallets_support::ensure_storage_version, sp_std::vec::Vec};

use crate::Config;

#[storage_alias]
type SessionForValidatorsChange = StorageValue<Indra, ()>;

#[storage_alias]
type MillisecsPerBlock = StorageValue<Indra, ()>;

#[storage_alias]
type SessionPeriod = StorageValue<Indra, ()>;

#[storage_alias]
type Validators = StorageValue<Indra, ()>;

/// Removes:
///   - SessionForValidatorsChange
///   - MillisecsPerBlock
///   - SessionPeriod
///   - Validators
pub struct Migration<T, P>(sp_std::marker::PhantomData<(T, P)>);

impl<T: Config, P: PalletInfoAccess> OnRuntimeUpgrade for Migration<T, P> {
	fn on_runtime_upgrade() -> Weight {
		let mut writes = 0;
		let mut reads = 0;
		log::info!(target: "pallet_indra", "Running migration from STORAGE_VERSION 1 to 2");

		if !SessionForValidatorsChange::exists() {
			log::info!(target: "pallet_indra", "Storage item SessionForValidatorsChange does not exist!");
		} else {
			writes += 1;
		}
		SessionForValidatorsChange::kill();
		reads += 1;

		if !MillisecsPerBlock::exists() {
			log::info!(target: "pallet_indra", "Storage item MillisecsPerBlock does not exist!");
		} else {
			writes += 1;
		}
		MillisecsPerBlock::kill();
		reads += 1;

		if !SessionPeriod::exists() {
			log::info!(target: "pallet_indra", "Storage item SessionPeriod does not exist!");
		} else {
			writes += 1;
		}
		SessionPeriod::kill();
		reads += 1;

		if !Validators::exists() {
			log::info!(target: "pallet_indra", "Storage item Validators does not exist!");
		} else {
			writes += 1;
		}
		Validators::kill();
		reads += 1;

		// store new version
		StorageVersion::new(2).put::<P>();
		writes += 1;

		T::DbWeight::get().reads(reads) + T::DbWeight::get().writes(writes)
	}

	#[cfg(feature = "try-runtime")]
	fn pre_upgrade() -> Result<Vec<u8>, &'static str> {
		ensure_storage_version::<P>(1)?;
		Ok(Vec::new())
	}

	#[cfg(feature = "try-runtime")]
	fn post_upgrade(_state: Vec<u8>) -> Result<(), &'static str> {
		ensure_storage_version::<P>(2)?;

		ensure!(
			SessionForValidatorsChange::get().is_none(),
			"`SessionForValidatorsChange` should be removed"
		);
		ensure!(MillisecsPerBlock::get().is_none(), "`MillisecsPerBlock` should be removed");
		ensure!(SessionPeriod::get().is_none(), "`SessionPeriod` should be removed");
		ensure!(Validators::get().is_none(), "`Validators` should be removed");

		Ok(())
	}
}
