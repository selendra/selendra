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

//! Interface to the Substrate Executor

use selendra_node_core_pvf_common::executor_intf::params_to_wasmtime_semantics;
use selendra_primitives::ExecutorParams;
use sc_executor_common::runtime_blob::RuntimeBlob;

/// Runs the prevalidation on the given code. Returns a [`RuntimeBlob`] if it succeeds.
pub fn prevalidate(code: &[u8]) -> Result<RuntimeBlob, sc_executor_common::error::WasmError> {
	let blob = RuntimeBlob::new(code)?;
	// It's assumed this function will take care of any prevalidation logic
	// that needs to be done.
	//
	// Do nothing for now.
	Ok(blob)
}

/// Runs preparation on the given runtime blob. If successful, it returns a serialized compiled
/// artifact which can then be used to pass into `Executor::execute` after writing it to the disk.
pub fn prepare(
	blob: RuntimeBlob,
	executor_params: &ExecutorParams,
) -> Result<Vec<u8>, sc_executor_common::error::WasmError> {
	let semantics = params_to_wasmtime_semantics(executor_params)
		.map_err(|e| sc_executor_common::error::WasmError::Other(e))?;
	sc_executor_wasmtime::prepare_runtime_artifact(blob, &semantics)
}
