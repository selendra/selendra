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

use selendra_primitives::SessionIndex;
use sp_std::vec::Vec;

use crate::{Config, Event, FinalityScheduledVersionChange, FinalityVersion, Pallet};

impl<T> pallet_session::SessionManager<T::AccountId> for Pallet<T>
where
    T: Config,
{
    fn new_session(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        <T as Config>::SessionManager::new_session(new_index)
    }

    fn new_session_genesis(new_index: SessionIndex) -> Option<Vec<T::AccountId>> {
        <T as Config>::SessionManager::new_session_genesis(new_index)
    }

    fn end_session(end_index: SessionIndex) {
        <T as Config>::SessionManager::end_session(end_index);
    }

    fn start_session(start_index: SessionIndex) {
        <T as Config>::SessionManager::start_session(start_index);
        Self::update_version_change_history();
    }
}

impl<T> Pallet<T>
where
    T: Config,
{
    // Check if a schedule version change has moved into the past. Update history, even if there is
    // no change. Resets the scheduled version.
    fn update_version_change_history() {
        let current_session = Self::current_session();

        if let Some(scheduled_version_change) = <FinalityScheduledVersionChange<T>>::get() {
            let scheduled_session = scheduled_version_change.session;
            let scheduled_version = scheduled_version_change.version_incoming;

            // Record the scheduled version as the current version as it moves into the past.
            if scheduled_session == current_session {
                <FinalityVersion<T>>::put(scheduled_version);

                // Reset the scheduled version.
                <FinalityScheduledVersionChange<T>>::kill();

                Self::deposit_event(Event::FinalityVersionChange(scheduled_version_change));
            }
        }
    }
}
