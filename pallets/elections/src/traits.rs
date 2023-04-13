// Copyright 2023 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the Apache License as published by
// the Free Software Foundation

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Apache License for more details.

// You should have received a copy of the Apache License
// along with Selendra.  If not, see <https://www.apache.org/licenses/LICENSE-2.0>.

use frame_support::{pallet_prelude::Get, traits::Currency};
use sp_staking::{EraIndex, SessionIndex};
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

pub trait SessionInfoProvider<T: frame_system::Config> {
    /// Returns set containing validators that in the current session produce&finalize blocks.
    fn current_committee() -> BTreeSet<T::AccountId>;
}

impl<T> SessionInfoProvider<T> for pallet_session::Pallet<T>
where
    T: pallet_session::Config,
    T::ValidatorId: Into<T::AccountId>,
{
    fn current_committee() -> BTreeSet<T::AccountId> {
        pallet_session::Validators::<T>::get()
            .into_iter()
            .map(|a| a.into())
            .collect()
    }
}

pub trait ValidatorRewardsHandler<T: frame_system::Config> {
    /// Returns total exposure of validators for the `era`
    fn validator_totals(era: EraIndex) -> Vec<(T::AccountId, u128)>;
    /// Add reward for validators
    fn add_rewards(rewards: impl IntoIterator<Item = (T::AccountId, u32)>);
}

impl<T> ValidatorRewardsHandler<T> for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
    <T::Currency as Currency<T::AccountId>>::Balance: Into<u128>,
{
    fn validator_totals(era: EraIndex) -> Vec<(T::AccountId, u128)> {
        pallet_staking::ErasStakers::<T>::iter_prefix(era)
            .map(|(validator, exposure)| (validator, exposure.total.into()))
            .collect()
    }

    fn add_rewards(rewards: impl IntoIterator<Item = (T::AccountId, u32)>) {
        pallet_staking::Pallet::<T>::reward_by_ids(rewards);
    }
}

pub trait EraInfoProvider {
    type AccountId;

    /// Returns `Some(idx)` where idx is the active era index otherwise
    /// if no era is active returns `None`.
    fn active_era() -> Option<EraIndex>;
    /// Returns `Some(idx)` where idx is the current era index which is latest
    /// planed era otherwise if no era has started returns `None`.
    fn current_era() -> Option<EraIndex>;
    /// Returns the index of the starting session of the `era` if possible. Otherwise returns `None`.
    fn era_start_session_index(era: EraIndex) -> Option<SessionIndex>;
    /// Returns how many sessions are in single era.
    fn sessions_per_era() -> SessionIndex;
    /// Returns the elected authorities for provided era.
    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId>;
}

impl<T> EraInfoProvider for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
{
    type AccountId = T::AccountId;

    fn active_era() -> Option<EraIndex> {
        pallet_staking::ActiveEra::<T>::get().map(|ae| ae.index)
    }

    fn current_era() -> Option<EraIndex> {
        pallet_staking::CurrentEra::<T>::get()
    }

    fn era_start_session_index(era: EraIndex) -> Option<SessionIndex> {
        pallet_staking::ErasStartSessionIndex::<T>::get(era)
    }

    fn sessions_per_era() -> SessionIndex {
        T::SessionsPerEra::get()
    }

    fn elected_validators(era: EraIndex) -> Vec<Self::AccountId> {
        pallet_staking::ErasStakers::<T>::iter_key_prefix(era).collect()
    }
}

pub trait ValidatorExtractor {
    type AccountId;

    /// Removes given validator from pallet's staking validators list
    fn remove_validator(who: &Self::AccountId);
}

impl<T> ValidatorExtractor for pallet_staking::Pallet<T>
where
    T: pallet_staking::Config,
{
    type AccountId = T::AccountId;

    fn remove_validator(who: &Self::AccountId) {
        pallet_staking::Pallet::<T>::do_remove_validator(who);
    }
}

