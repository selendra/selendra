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

use crate::artifacts::ArtifactId;
use selendra_indracore::primitives::ValidationCodeHash;
use sp_core::blake2_256;
use std::{fmt, sync::Arc};

/// A struct that carries code of a indracore validation function and it's hash.
///
/// Should be cheap to clone.
#[derive(Clone)]
pub struct Pvf {
	pub(crate) code: Arc<Vec<u8>>,
	pub(crate) code_hash: ValidationCodeHash,
}

impl fmt::Debug for Pvf {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Pvf {{ code, code_hash: {:?} }}", self.code_hash)
	}
}

impl Pvf {
	/// Returns an instance of the PVF out of the given PVF code.
	pub fn from_code(code: Vec<u8>) -> Self {
		let code = Arc::new(code);
		let code_hash = blake2_256(&code).into();
		Self { code, code_hash }
	}

	/// Creates a new PVF which artifact id can be uniquely identified by the given number.
	#[cfg(test)]
	pub(crate) fn from_discriminator(num: u32) -> Self {
		let descriminator_buf = num.to_le_bytes().to_vec();
		Pvf::from_code(descriminator_buf)
	}

	/// Returns the artifact ID that corresponds to this PVF.
	pub(crate) fn as_artifact_id(&self) -> ArtifactId {
		ArtifactId::new(self.code_hash)
	}
}
