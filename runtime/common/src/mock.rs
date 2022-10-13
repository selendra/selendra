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

//! Mocking utilities for testing.

use crate::traits::Registrar;
use frame_support::{
	dispatch::{DispatchError, DispatchResult},
	weights::Weight,
};
use parity_scale_codec::{Decode, Encode};
use primitives::v2::{HeadData, Id as IndraId, ValidationCode};
use sp_runtime::{traits::SaturatedConversion, Permill};
use std::{cell::RefCell, collections::HashMap};

thread_local! {
	static OPERATIONS: RefCell<Vec<(IndraId, u32, bool)>> = RefCell::new(Vec::new());
	static INDRACORES: RefCell<Vec<IndraId>> = RefCell::new(Vec::new());
	static INDRABASES: RefCell<Vec<IndraId>> = RefCell::new(Vec::new());
	static LOCKS: RefCell<HashMap<IndraId, bool>> = RefCell::new(HashMap::new());
	static MANAGERS: RefCell<HashMap<IndraId, Vec<u8>>> = RefCell::new(HashMap::new());
}

pub struct TestRegistrar<T>(sp_std::marker::PhantomData<T>);

impl<T: frame_system::Config> Registrar for TestRegistrar<T> {
	type AccountId = T::AccountId;

	fn manager_of(id: IndraId) -> Option<Self::AccountId> {
		MANAGERS.with(|x| x.borrow().get(&id).and_then(|v| T::AccountId::decode(&mut &v[..]).ok()))
	}

	fn indracores() -> Vec<IndraId> {
		INDRACORES.with(|x| x.borrow().clone())
	}

	fn is_indrabase(id: IndraId) -> bool {
		INDRABASES.with(|x| x.borrow().binary_search(&id).is_ok())
	}

	fn apply_lock(id: IndraId) {
		LOCKS.with(|x| x.borrow_mut().insert(id, true));
	}

	fn remove_lock(id: IndraId) {
		LOCKS.with(|x| x.borrow_mut().insert(id, false));
	}

	fn register(
		manager: Self::AccountId,
		id: IndraId,
		_genesis_head: HeadData,
		_validation_code: ValidationCode,
	) -> DispatchResult {
		// Should not be indracore.
		INDRACORES.with(|x| {
			let indracores = x.borrow_mut();
			match indracores.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("Already Indracore")),
				Err(_) => Ok(()),
			}
		})?;
		// Should not be indrabase, then make it.
		INDRABASES.with(|x| {
			let mut indrabases = x.borrow_mut();
			match indrabases.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("Already Indrabase")),
				Err(i) => {
					indrabases.insert(i, id);
					Ok(())
				},
			}
		})?;
		MANAGERS.with(|x| x.borrow_mut().insert(id, manager.encode()));
		Ok(())
	}

	fn deregister(id: IndraId) -> DispatchResult {
		// Should not be indracore.
		INDRACORES.with(|x| {
			let indracores = x.borrow_mut();
			match indracores.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("cannot deregister indracore")),
				Err(_) => Ok(()),
			}
		})?;
		// Remove from indrabase.
		INDRABASES.with(|x| {
			let mut indrabases = x.borrow_mut();
			match indrabases.binary_search(&id) {
				Ok(i) => {
					indrabases.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not indrabase, so cannot `deregister`")),
			}
		})?;
		MANAGERS.with(|x| x.borrow_mut().remove(&id));
		Ok(())
	}

	fn make_indracore(id: IndraId) -> DispatchResult {
		INDRABASES.with(|x| {
			let mut indrabases = x.borrow_mut();
			match indrabases.binary_search(&id) {
				Ok(i) => {
					indrabases.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not indrabase, so cannot `make_indracore`")),
			}
		})?;
		INDRACORES.with(|x| {
			let mut indracores = x.borrow_mut();
			match indracores.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("already indracore, so cannot `make_indracore`")),
				Err(i) => {
					indracores.insert(i, id);
					Ok(())
				},
			}
		})?;
		OPERATIONS.with(|x| {
			x.borrow_mut().push((
				id,
				frame_system::Pallet::<T>::block_number().saturated_into(),
				true,
			))
		});
		Ok(())
	}
	fn make_indrabase(id: IndraId) -> DispatchResult {
		INDRACORES.with(|x| {
			let mut indracores = x.borrow_mut();
			match indracores.binary_search(&id) {
				Ok(i) => {
					indracores.remove(i);
					Ok(())
				},
				Err(_) => Err(DispatchError::Other("not indracore, so cannot `make_indrabase`")),
			}
		})?;
		INDRABASES.with(|x| {
			let mut indrabases = x.borrow_mut();
			match indrabases.binary_search(&id) {
				Ok(_) => Err(DispatchError::Other("already indrabase, so cannot `make_indrabase`")),
				Err(i) => {
					indrabases.insert(i, id);
					Ok(())
				},
			}
		})?;
		OPERATIONS.with(|x| {
			x.borrow_mut().push((
				id,
				frame_system::Pallet::<T>::block_number().saturated_into(),
				false,
			))
		});
		Ok(())
	}

	#[cfg(test)]
	fn worst_head_data() -> HeadData {
		vec![0u8; 1000].into()
	}

	#[cfg(test)]
	fn worst_validation_code() -> ValidationCode {
		let validation_code = vec![0u8; 1000];
		validation_code.into()
	}

	#[cfg(test)]
	fn execute_pending_transitions() {}
}

impl<T: frame_system::Config> TestRegistrar<T> {
	pub fn operations() -> Vec<(IndraId, T::BlockNumber, bool)> {
		OPERATIONS
			.with(|x| x.borrow().iter().map(|(p, b, c)| (*p, (*b).into(), *c)).collect::<Vec<_>>())
	}

	#[allow(dead_code)]
	pub fn indracores() -> Vec<IndraId> {
		INDRACORES.with(|x| x.borrow().clone())
	}

	#[allow(dead_code)]
	pub fn indrabases() -> Vec<IndraId> {
		INDRABASES.with(|x| x.borrow().clone())
	}

	#[allow(dead_code)]
	pub fn clear_storage() {
		OPERATIONS.with(|x| x.borrow_mut().clear());
		INDRACORES.with(|x| x.borrow_mut().clear());
		INDRABASES.with(|x| x.borrow_mut().clear());
		MANAGERS.with(|x| x.borrow_mut().clear());
	}
}

/// A very dumb implementation of `EstimateNextSessionRotation`. At the moment of writing, this
/// is more to satisfy type requirements rather than to test anything.
pub struct TestNextSessionRotation;

impl frame_support::traits::EstimateNextSessionRotation<u32> for TestNextSessionRotation {
	fn average_session_length() -> u32 {
		10
	}

	fn estimate_current_session_progress(_now: u32) -> (Option<Permill>, Weight) {
		(None, 0)
	}

	fn estimate_next_session_rotation(_now: u32) -> (Option<u32>, Weight) {
		(None, 0)
	}
}
