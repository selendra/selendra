// This file is part of Selendra.

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

use crate::{Cli, Subcommand};
use frame_benchmarking_cli::*;
use sc_cli::{ChainSpec, Result, RuntimeVersion, SubstrateCli};
use sc_service::PartialComponents;
use selendra_primitives::Block;
use service::{chain_spec, new_partial, ExecutorDispatch, FullClient};
use sp_core::crypto::Ss58AddressFormat;

use selendra_runtime::RuntimeApi;

use std::sync::Arc;

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Selendra Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
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
		2021
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		let spec = match id {
			"" | "selendra" => Box::new(chain_spec::selendra::selendra_config()?),
			"dev" | "selendra-dev" => Box::new(chain_spec::selendra::development_config()),
			"selendra-local" => Box::new(chain_spec::selendra::local_testnet_config()),
			"selendra-mainnet" => Box::new(chain_spec::selendra::mainnet_staging_config()),
			"selendra-staging" => Box::new(chain_spec::selendra::staging_config()),
			"selendra-testnet" | "testnet" => Box::new(chain_spec::selendra::testnet_config()?),
			path =>
				Box::new(chain_spec::ChainSpec::from_json_file(std::path::PathBuf::from(path))?),
		};
		Ok(spec)
	}

	fn native_runtime_version(_: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		&selendra_runtime::VERSION
	}
}

fn set_default_ss58_version(_spec: &Box<dyn sc_service::ChainSpec>) {
	let ss58_version = Ss58AddressFormat::custom(204);
	sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		None => {
			let runner = cli.create_runner(&cli.run)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.run_node_until_exit(|config| async move {
				service::new_full(config, cli.no_hardware_benchmarks)
					.map_err(sc_cli::Error::Service)
			})
		},
		Some(Subcommand::Inspect(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| cmd.run::<Block, RuntimeApi, ExecutorDispatch>(config))
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| {
				// This switch needs to be in the client, since the client decides
				// which sub-commands it wants to support.
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err(
								"Runtime benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
									.into(),
							)
						}

						cmd.run::<Block, ExecutorDispatch>(config)
					},
					BenchmarkCmd::Block(cmd) => {
						let PartialComponents { client, .. } = new_partial(&config)?;
						cmd.run(client)
					},
					BenchmarkCmd::Storage(cmd) => {
						let PartialComponents { client, backend, .. } = new_partial(&config)?;
						let db = backend.expose_db();
						let storage = backend.expose_storage();

						cmd.run(config, client, db, storage)
					},
					BenchmarkCmd::Overhead(_) => todo!(),
					BenchmarkCmd::Machine(cmd) =>
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()),
				}
			})
		},
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			// let chain_spec = &runner.config().chain_spec;
			// set_default_ss58_version(chain_spec);

			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				let PartialComponents { client, task_manager, .. } = new_partial(&config)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				let PartialComponents { client, task_manager, import_queue, .. } =
					new_partial(&config)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| cmd.run(config.database))
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				let PartialComponents { client, task_manager, backend, .. } = new_partial(&config)?;
				let aux_revert = Box::new(|client: Arc<FullClient>, backend, blocks| {
					sc_consensus_babe::revert(client.clone(), backend, blocks)?;
					grandpa::revert(client, blocks)?;
					Ok(())
				});
				Ok((cmd.run(client, backend, Some(aux_revert)), task_manager))
			})
		},
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.async_run(|config| {
				// we don't need any of the components of new_partial, just a runtime, or a task
				// manager to do `async_run`.
				let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
				let task_manager =
					sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
						.map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

				Ok((cmd.run::<Block, ExecutorDispatch>(config), task_manager))
			})
		},
		#[cfg(not(feature = "try-runtime"))]
		Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
			.into()),
		Some(Subcommand::ChainInfo(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			let chain_spec = &runner.config().chain_spec;
			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| cmd.run::<Block>(&config))
		},
	}
}
