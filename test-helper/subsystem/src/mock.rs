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

use std::sync::Arc;

use sc_keystore::LocalKeystore;
use sp_application_crypto::AppKey;
use sp_keyring::Sr25519Keyring;
use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr};

use selendra_primitives::v2::{AuthorityDiscoveryId, ValidatorId};

/// Get mock keystore with `Ferdie` key.
pub fn make_ferdie_keystore() -> SyncCryptoStorePtr {
	let keystore: SyncCryptoStorePtr = Arc::new(LocalKeystore::in_memory());
	SyncCryptoStore::sr25519_generate_new(
		&*keystore,
		ValidatorId::ID,
		Some(&Sr25519Keyring::Ferdie.to_seed()),
	)
	.expect("Insert key into keystore");
	SyncCryptoStore::sr25519_generate_new(
		&*keystore,
		AuthorityDiscoveryId::ID,
		Some(&Sr25519Keyring::Ferdie.to_seed()),
	)
	.expect("Insert key into keystore");
	keystore
}
