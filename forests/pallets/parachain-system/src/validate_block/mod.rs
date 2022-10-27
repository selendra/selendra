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

//! A module that enables a runtime to work as parachain.

use selendra_parachain::primitives::ValidationParams;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub mod implementation;
#[cfg(test)]
mod tests;

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use selendra_parachain;
#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use sp_runtime::traits::GetRuntimeBlockType;

// Stores the [`ValidationParams`] that are being passed to `validate_block`.
//
// This value will only be set when a parachain validator validates a given `PoV`.
environmental::environmental!(VALIDATION_PARAMS: ValidationParams);

/// Execute the given closure with the [`ValidationParams`].
///
/// Returns `None` if the [`ValidationParams`] are not set, because the code is currently not being
/// executed in the context of `validate_block`.
pub(crate) fn with_validation_params<R>(f: impl FnOnce(&ValidationParams) -> R) -> Option<R> {
	VALIDATION_PARAMS::with(|v| f(v))
}

/// Set the [`ValidationParams`] for the local context and execute the given closure in this context.
#[cfg(not(feature = "std"))]
fn set_and_run_with_validation_params<R>(mut params: ValidationParams, f: impl FnOnce() -> R) -> R {
	VALIDATION_PARAMS::using(&mut params, f)
}
