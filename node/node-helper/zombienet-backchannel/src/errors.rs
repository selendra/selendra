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

//! Selendra Zombienet Backchannel error definitions.

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
pub enum BackchannelError {
	#[error("Error connecting websocket server")]
	CantConnectToWS,

	#[error("Backchannel not initialized yet")]
	Uninitialized,

	#[error("Backchannel already initialized")]
	AlreadyInitialized,

	#[error("Error sending new value to backchannel")]
	SendItemFail,

	#[error("Invalid host for connection backchannel")]
	InvalidHost,

	#[error("Invalid port for connection backchannel")]
	InvalidPort,
}
