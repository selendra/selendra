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
	log,
	sp_runtime::{traits::OpaqueKeys, RuntimeAppPublic},
};
use selendra_primitives::{AuthorityId, SessionIndex};
use sp_std::prelude::*;

use crate::Config;

/// Information provider from `pallet_session`. Loose pallet coupling via traits.
pub trait SessionInfoProvider {
	fn current_session() -> SessionIndex;
}

/// Authorities provider, used only as default value in case of missing this information in our pallet. This can
/// happen for the session after runtime upgraded.
pub trait NextSessionAuthorityProvider<T: Config> {
	fn next_authorities() -> Vec<T::AuthorityId>;
}

impl<T> SessionInfoProvider for pallet_session::Pallet<T>
where
	T: pallet_session::Config,
{
	fn current_session() -> SessionIndex {
		pallet_session::CurrentIndex::<T>::get()
	}
}

impl<T> NextSessionAuthorityProvider<T> for pallet_session::Pallet<T>
where
	T: Config + pallet_session::Config,
{
	fn next_authorities() -> Vec<T::AuthorityId> {
		let next: Option<Vec<_>> = pallet_session::Pallet::<T>::queued_keys()
			.iter()
			.map(|(_, key)| key.get(AuthorityId::ID))
			.collect();

		next.unwrap_or_else(|| {
			log::error!(target: "pallet_indra", "Missing next session keys");
			vec![]
		})
	}
}
