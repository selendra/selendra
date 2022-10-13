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
//! Runtime modules for indracores code.
//!
//! It is crucial to include all the modules from this crate in the runtime, in
//! particular the `Initializer` module, as it is responsible for initializing the state
//! of the other modules.

#![cfg_attr(feature = "runtime-benchmarks", recursion_limit = "256")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod configuration;
pub mod disputes;
pub mod dmp;
pub mod hrmp;
pub mod inclusion;
pub mod indras;
pub mod indras_inherent;
pub mod initializer;
pub mod metrics;
pub mod origin;
pub mod reward_points;
pub mod runtime_api_impl;
pub mod scheduler;
pub mod session_info;
pub mod shared;
pub mod ump;

mod util;

#[cfg(any(feature = "runtime-benchmarks", test))]
mod builder;
#[cfg(test)]
mod mock;

pub use indras::IndraLifecycle;
pub use origin::{ensure_indracore, Origin};
use primitives::v2::Id as IndraId;

/// Schedule a indra to be initialized at the start of the next session with the given genesis data.
///
/// See [`indras::Pallet::schedule_indra_initialize`] for more details.
pub fn schedule_indra_initialize<T: indras::Config>(
	id: IndraId,
	genesis: indras::IndraGenesisArgs,
) -> Result<(), ()> {
	<indras::Pallet<T>>::schedule_indra_initialize(id, genesis).map_err(|_| ())
}

/// Schedule a indra to be cleaned up at the start of the next session.
///
/// See [`indras::Pallet::schedule_indra_cleanup`] for more details.
pub fn schedule_indra_cleanup<T: indras::Config>(id: primitives::v2::Id) -> Result<(), ()> {
	<indras::Pallet<T>>::schedule_indra_cleanup(id).map_err(|_| ())
}

/// Schedule a indrabase to be upgraded to a indracore.
pub fn schedule_indrabase_upgrade<T: indras::Config>(id: IndraId) -> Result<(), ()> {
	indras::Pallet::<T>::schedule_indrabase_upgrade(id).map_err(|_| ())
}

/// Schedule a indracore to be downgraded to a indrabase.
pub fn schedule_indracore_downgrade<T: indras::Config>(id: IndraId) -> Result<(), ()> {
	indras::Pallet::<T>::schedule_indracore_downgrade(id).map_err(|_| ())
}
