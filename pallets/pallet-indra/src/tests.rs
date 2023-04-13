#![cfg(test)]

use frame_support::{storage_alias, traits::OneSessionHandler};
use primitives::VersionChange;

use crate::mock::*;

#[storage_alias]
type SessionForValidatorsChange = StorageValue<Indra, u32>;

#[storage_alias]
type Validators<T> = StorageValue<Indra, Vec<<T as frame_system::Config>::AccountId>>;

#[cfg(feature = "try-runtime")]
mod migration_tests {
    use frame_support::{storage::migration::put_storage_value, traits::StorageVersion};
    use pallets_support::StorageMigration;

    use crate::{migrations, mock::*, Pallet};

    const MODULE: &[u8] = b"Indra";

    #[test]
    fn migration_from_v0_to_v1_works() {
        new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
            StorageVersion::new(0).put::<Pallet<Test>>();

            put_storage_value(MODULE, b"SessionForValidatorsChange", &[], Some(7u32));
            put_storage_value(MODULE, b"Validators", &[], Some(vec![0u64, 1u64]));

            let _weight = migrations::v0_to_v1::Migration::<Test, Indra>::migrate();
        })
    }

    #[test]
    fn migration_from_v1_to_v2_works() {
        new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
            StorageVersion::new(1).put::<Pallet<Test>>();

            put_storage_value(MODULE, b"SessionForValidatorsChange", &[], ());
            put_storage_value(MODULE, b"Validators", &[], ());
            put_storage_value(MODULE, b"MillisecsPerBlock", &[], ());
            put_storage_value(MODULE, b"SessionPeriod", &[], ());

            let _weight = migrations::v1_to_v2::Migration::<Test, Indra>::migrate();
        })
    }
}

#[test]
fn test_update_authorities() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        initialize_session();
        run_session(1);

        let authorities = to_authorities(&[2, 3, 4]);

        Indra::update_authorities(authorities.as_slice(), authorities.as_slice());

        assert_eq!(Indra::authorities(), to_authorities(&[2, 3, 4]));
        assert_eq!(Indra::next_authorities(), to_authorities(&[2, 3, 4]));
    });
}

#[test]
fn test_initialize_authorities() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        assert_eq!(Indra::authorities(), to_authorities(&[1, 2]));
        assert_eq!(Indra::next_authorities(), to_authorities(&[1, 2]));
    });
}

#[test]
fn fails_to_initialize_again_authorities() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        let authorities = to_authorities(&[1, 2, 3]);
        Indra::initialize_authorities(&authorities, &authorities);

        // should not update storage
        assert_eq!(Indra::authorities(), to_authorities(&[1, 2]));
    });
}

#[test]
fn test_current_authorities() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        initialize_session();

        run_session(1);

        let authorities = to_authorities(&[2, 3, 4]);

        Indra::update_authorities(&authorities, &authorities);

        assert_eq!(Indra::authorities(), to_authorities(&[2, 3, 4]));
        assert_eq!(Indra::next_authorities(), to_authorities(&[2, 3, 4]));

        run_session(2);

        let authorities = to_authorities(&[1, 2, 3]);
        Indra::update_authorities(&authorities, &authorities);

        assert_eq!(Indra::authorities(), to_authorities(&[1, 2, 3]));
        assert_eq!(Indra::next_authorities(), to_authorities(&[1, 2, 3]));
    })
}

#[test]
fn test_session_rotation() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        initialize_session();
        run_session(1);

        let new_validators = new_session_validators(&[3u64, 4u64]);
        let queued_validators = new_session_validators(&[5, 6]);
        Indra::on_new_session(true, new_validators, queued_validators);
        assert_eq!(Indra::authorities(), to_authorities(&[3, 4]));
        assert_eq!(Indra::next_authorities(), to_authorities(&[5, 6]));
    })
}

#[test]
fn test_emergency_signer() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        initialize_session();

        run_session(1);

        Indra::set_next_emergency_finalizer(to_authority(&21));

        assert_eq!(Indra::emergency_finalizer(), None);
        assert_eq!(Indra::queued_emergency_finalizer(), None);

        run_session(2);

        Indra::set_next_emergency_finalizer(to_authority(&37));

        assert_eq!(Indra::emergency_finalizer(), None);
        assert_eq!(Indra::queued_emergency_finalizer(), Some(to_authority(&21)));

        run_session(3);

        assert_eq!(Indra::emergency_finalizer(), Some(to_authority(&21)));
        assert_eq!(Indra::queued_emergency_finalizer(), Some(to_authority(&37)));
    })
}

#[test]
fn test_finality_version_scheduling() {
    new_test_ext(&[(1u64, 1u64), (2u64, 2u64)]).execute_with(|| {
        initialize_session();

        run_session(1);

        let version_to_schedule = VersionChange {
            version_incoming: 1,
            session: 4,
        };

        let scheduling_result =
            Indra::do_schedule_finality_version_change(version_to_schedule.clone());
        assert_eq!(scheduling_result, Ok(()));

        let scheduled_version_change = Indra::finality_version_change();
        assert_eq!(scheduled_version_change, Some(version_to_schedule.clone()));

        run_session(4);

        let current_version = Indra::finality_version();
        assert_eq!(current_version, version_to_schedule.version_incoming);

        let scheduled_version_change = Indra::finality_version_change();
        assert_eq!(scheduled_version_change, None);

        let version_to_schedule = VersionChange {
            version_incoming: 1,
            session: 5,
        };

        let scheduling_result = Indra::do_schedule_finality_version_change(version_to_schedule);
        assert!(scheduling_result.is_err());
    })
}
