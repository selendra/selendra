use crate::Runtime;
/// All migrations that will run on the next runtime upgrade.
///
/// This contains the combined migrations of the last 10 releases. It allows to skip runtime
/// upgrades in case governance decides to do so. THE ORDER IS IMPORTANT.
pub type Migrations = migrations::Unreleased;

/// The runtime migrations per release.
#[allow(deprecated, missing_docs)]
pub mod migrations {
	use super::*;
	const IDENTITY_MIGRATION_KEY_LIMIT: u64 = u64::MAX;

	/// Unreleased migrations. Add new ones here:
	pub type Unreleased = (
		pallet_staking::migrations::v14::MigrateToV14<Runtime>,
		pallet_referenda::migration::v1::MigrateV0ToV1<Runtime, ()>,
		pallet_im_online::migration::v1::Migration<Runtime>,
		pallet_grandpa::migrations::MigrateV4ToV5<Runtime>,
		// Migrate Identity pallet for Usernames
		pallet_identity::migration::versioned::V0ToV1<Runtime, IDENTITY_MIGRATION_KEY_LIMIT>,
	);
}

