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

use assert_cmd::cargo::cargo_bin;
use std::{process::Command, result::Result};

static RUNTIMES: [&'static str; 1] = ["selendra"];

static EXTRINSICS: [(&'static str, &'static str); 2] =
	[("system", "remark"), ("balances", "transfer_keep_alive")];

/// `benchmark extrinsic` works for all dev runtimes and some extrinsics.
#[test]
fn benchmark_extrinsic_works() {
	for runtime in RUNTIMES {
		for (pallet, extrinsic) in EXTRINSICS {
			let runtime = format!("{}-dev", runtime);
			assert!(benchmark_extrinsic(&runtime, pallet, extrinsic).is_ok());
		}
	}
}

/// `benchmark extrinsic` rejects all non-dev runtimes.
#[test]
fn benchmark_extrinsic_rejects_non_dev_runtimes() {
	for runtime in RUNTIMES {
		assert!(benchmark_extrinsic(runtime, "system", "remark").is_err());
	}
}

fn benchmark_extrinsic(runtime: &str, pallet: &str, extrinsic: &str) -> Result<(), String> {
	let status = Command::new(cargo_bin("selendra"))
		.args(["benchmark", "extrinsic", "--chain", &runtime])
		.args(&["--pallet", pallet, "--extrinsic", extrinsic])
		// Run with low repeats for faster execution.
		.args(["--repeat=1", "--warmup=1", "--max-ext-per-block=1"])
		.status()
		.map_err(|e| format!("command failed: {:?}", e))?;

	if !status.success() {
		return Err("Command failed".into())
	}

	Ok(())
}
