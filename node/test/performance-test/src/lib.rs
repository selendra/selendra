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

//! A Selendra performance tests utilities.

use selendra_erasure_coding::{obtain_chunks, reconstruct};
use selendra_primitives::ExecutorParams;
use std::time::{Duration, Instant};

mod constants;

pub use constants::*;
pub use selendra_node_primitives::VALIDATION_CODE_BOMB_LIMIT;

/// Value used for reference benchmark of erasure-coding.
pub const ERASURE_CODING_N_VALIDATORS: usize = 1024;

pub use selendra_runtime::WASM_BINARY;

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum PerfCheckError {
	#[error("This subcommand is only available in release mode")]
	WrongBuildType,

	#[error("No wasm code found for running the performance test")]
	WasmBinaryMissing,

	#[error("Failed to decompress wasm code")]
	CodeDecompressionFailed,

	#[error(transparent)]
	Wasm(#[from] sc_executor_common::error::WasmError),

	#[error(transparent)]
	ErasureCoding(#[from] selendra_erasure_coding::Error),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(
		"Performance check not passed: exceeded the {limit:?} time limit, elapsed: {elapsed:?}"
	)]
	TimeOut { elapsed: Duration, limit: Duration },
}

/// Measures the time it takes to compile arbitrary wasm code.
pub fn measure_pvf_prepare(wasm_code: &[u8]) -> Result<Duration, PerfCheckError> {
	let start = Instant::now();

	let code = sp_maybe_compressed_blob::decompress(wasm_code, VALIDATION_CODE_BOMB_LIMIT)
		.or(Err(PerfCheckError::CodeDecompressionFailed))?;

	// Recreate the pipeline from the pvf prepare worker.
	let blob = selendra_node_core_pvf_prepare_worker::prevalidate(code.as_ref())
		.map_err(PerfCheckError::from)?;
	selendra_node_core_pvf_prepare_worker::prepare(blob, &ExecutorParams::default())
		.map_err(PerfCheckError::from)?;

	Ok(start.elapsed())
}

/// Measure the time it takes to break arbitrary data into chunks and reconstruct it back.
pub fn measure_erasure_coding(
	n_validators: usize,
	data: &[u8],
) -> Result<Duration, PerfCheckError> {
	let start = Instant::now();

	let chunks = obtain_chunks(n_validators, &data)?;
	let indexed_chunks = chunks.iter().enumerate().map(|(i, chunk)| (chunk.as_slice(), i));

	let _: Vec<u8> = reconstruct(n_validators, indexed_chunks)?;

	Ok(start.elapsed())
}
