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

//! Selendra CLI library.

use clap::Parser;
use sc_cli::SubstrateCli;

/// Sub-commands supported by the collator.
#[derive(Debug, Parser)]
pub enum Subcommand {
	/// Export the genesis state of the parachain.
	#[command(name = "export-genesis-state")]
	ExportGenesisState(ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	#[command(name = "export-genesis-wasm")]
	ExportGenesisWasm(ExportGenesisWasmCommand),
}

/// Command for exporting the genesis state of the parachain
#[derive(Debug, Parser)]
pub struct ExportGenesisStateCommand {}

/// Command for exporting the genesis wasm file.
#[derive(Debug, Parser)]
pub struct ExportGenesisWasmCommand {}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
#[group(skip)]
pub struct RunCmd {
	#[allow(missing_docs)]
	#[clap(flatten)]
	pub base: sc_cli::RunCmd,

	/// Id of the parachain this collator collates for.
	#[arg(long)]
	pub parachain_id: Option<u32>,
}

#[allow(missing_docs)]
#[derive(Debug, Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[clap(flatten)]
	pub run: RunCmd,
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"SmallWorld Selendra".into()
	}

	fn impl_version() -> String {
		"0.0.0".into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/selendra/selendra/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2022
	}

	fn executable_name() -> String {
		"adder-collator".into()
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let id = if id.is_empty() { "selendra" } else { id };
		Ok(match id {
			"selendra-staging" =>
				Box::new(selendra_service::chain_spec::selendra_staging_testnet_config()?),
			"selendra-local" =>
				Box::new(selendra_service::chain_spec::selendra_local_testnet_config()?),
			"selendra" => Box::new(selendra_service::chain_spec::selendra_config()?),
			path => {
				let path = std::path::PathBuf::from(path);
				Box::new(selendra_service::SelendraChainSpec::from_json_file(path)?)
			},
		})
	}
}
