// Copyright 2022 Smallworld Selendra
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

use selendra_node_subsystem::SubsystemError;
pub(crate) use selendra_overseer::OverseerError;

#[fatality::fatality(splitable)]
pub(crate) enum Error {
	/// Received error from overseer:
	#[fatal]
	#[error(transparent)]
	SubsystemError(#[from] SubsystemError),
	/// The stream of incoming events concluded.
	#[fatal]
	#[error("Event stream closed unexpectedly")]
	EventStreamConcluded,
}

impl From<OverseerError> for Error {
	fn from(e: OverseerError) -> Self {
		Error::SubsystemError(SubsystemError::from(e))
	}
}
