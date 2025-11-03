use primitives::{CommitteeSeats, EraValidators};
use rand::{seq::SliceRandom, SeedableRng};
use rand_pcg::Pcg32;
use sp_staking::EraIndex;
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};

use crate::{
    traits::ValidatorProvider, CommitteeSize, Config, CurrentEraValidators, NextEraCommitteeSize,
    NextEraNonReservedValidators, NextEraReservedValidators, Openness, Pallet,
};

impl<T> Pallet<T>
where
    T: Config,
{
    fn populate_next_era_validators_on_next_era_start(era: EraIndex) {
        let parent_hash = frame_system::Pallet::<T>::parent_hash();
        let mut bytes = [0u8; 8];
        bytes.clone_from_slice(&parent_hash.as_ref()[..8]);
        let seed = u64::from_le_bytes(bytes);

        let mut rng = Pcg32::seed_from_u64(seed);
        
        let reserved_validators = NextEraReservedValidators::<T>::get();
        let non_reserved_validators = NextEraNonReservedValidators::<T>::get();
        let committee_size = NextEraCommitteeSize::<T>::get();

        // In Permissionless mode, NextEraNonReservedValidators already contains the correct
        // set of validators from the election. We should not filter them again by the previous
        // era's elected_committee, as that would prevent new validators from joining.
        let openness = Openness::<T>::get();
        
        let (reserved_vec, non_reserved_vec) = match openness {
            primitives::ElectionOpenness::Permissionless => {
                // In Permissionless mode, just shuffle without filtering
                let mut reserved_vec: Vec<_> = reserved_validators.to_vec();
                let mut non_reserved_vec: Vec<_> = non_reserved_validators.to_vec();
                reserved_vec.shuffle(&mut rng);
                non_reserved_vec.shuffle(&mut rng);
                (reserved_vec, non_reserved_vec)
            }
            primitives::ElectionOpenness::Permissioned => {
                // In Permissioned mode, filter by elected_committee as before
                let elected_committee = BTreeSet::from_iter(T::ValidatorProvider::elected_validators(era));
                
                let mut retain_shuffle_elected = |vals: Vec<T::AccountId>| -> Vec<T::AccountId> {
                    let mut vals: Vec<_> = vals
                        .into_iter()
                        .filter(|v| elected_committee.contains(v))
                        .collect();
                    vals.shuffle(&mut rng);
                    vals
                };
                
                let reserved_vec = retain_shuffle_elected(reserved_validators.to_vec());
                let non_reserved_vec = retain_shuffle_elected(non_reserved_validators.to_vec());
                (reserved_vec, non_reserved_vec)
            }
        };

        CurrentEraValidators::<T>::put(EraValidators {
            reserved: reserved_vec.try_into().expect("Too many validators"),
            non_reserved: non_reserved_vec.try_into().expect("Too many validators"),
        });
        CommitteeSize::<T>::put(committee_size);
    }
}

impl<T: Config> primitives::EraManager for Pallet<T> {
    fn on_new_era(era: EraIndex) {
        Self::populate_next_era_validators_on_next_era_start(era);
    }
}

impl<T: Config> primitives::BanHandler for Pallet<T> {
    type AccountId = T::AccountId;
    fn can_ban(account_id: &Self::AccountId) -> bool {
        !NextEraReservedValidators::<T>::get().contains(account_id)
    }
}

impl<T: Config + pallet_staking::Config> primitives::ValidatorProvider for Pallet<T> {
    type AccountId = T::AccountId;
    type MaxValidators = T::MaxValidators;
    fn current_era_validators() -> EraValidators<Self::AccountId, Self::MaxValidators> {
        CurrentEraValidators::<T>::get()
    }
    fn current_era_committee_size() -> CommitteeSeats {
        CommitteeSize::<T>::get()
    }
}
