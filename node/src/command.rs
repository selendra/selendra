// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use futures::TryFutureExt;
use log::info;
// Substrate
use sc_cli::SubstrateCli;
use sc_network::config::Role;
use sc_service::{Configuration, DatabaseSource};

// Frontier
use fc_db::kv::frontier_database_dir;

use selendra_primitives::HEAP_PAGES;

use crate::{
	chain_spec::testnet_chainspec,
	cli::{Cli, Subcommand},
	eth::db_config_dir,
	new_partial, service, ConfigValidator, ServiceComponents,
};

#[cfg(feature = "runtime-benchmarks")]
use crate::chain_spec::get_account_id_from_seed;

fn enforce_heap_pages(config: &mut Configuration) {
	config.default_heap_pages = Some(HEAP_PAGES);
}

pub type AlephNodeChainSpec = sc_service::GenericChainSpec<()>;

pub fn testnet_config() -> Result<AlephNodeChainSpec, String> {
	AlephNodeChainSpec::from_json_bytes(testnet_chainspec())
}

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
		2022
	}

	fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
		let default_chain = "testnet";
		let id = id.trim();
		let id = if id.is_empty() { default_chain } else { id };

		let chainspec = match id {
			"mainnet" => testnet_config(),

			_ => AlephNodeChainSpec::from_json_file(id.into()),
		};
		Ok(Box::new(chainspec?))
	}
}

/// Parse and run command line arguments
pub fn run() -> sc_cli::Result<()> {
	let mut cli = Cli::from_args();

	let config_validation_result = ConfigValidator::process(&mut cli);

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents { client, task_manager, import_queue, .. } =
					new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents { client, task_manager, .. } =
					new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents { client, task_manager, .. } =
					new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents { client, task_manager, import_queue, .. } =
					new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		},
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				// Remove Frontier offchain db
				let db_config_dir = db_config_dir(&config);
				match cli.eth.frontier_backend_type {
					crate::eth::BackendType::KeyValue => {
						let frontier_database_config = match config.database {
							DatabaseSource::RocksDb { .. } => DatabaseSource::RocksDb {
								path: frontier_database_dir(&db_config_dir, "db"),
								cache_size: 0,
							},
							DatabaseSource::ParityDb { .. } => DatabaseSource::ParityDb {
								path: frontier_database_dir(&db_config_dir, "paritydb"),
							},
							_ => {
								return Err(format!(
									"Cannot purge `{:?}` database",
									config.database
								)
								.into())
							},
						};
						cmd.run(frontier_database_config)?;
					},
					crate::eth::BackendType::Sql => {
						let db_path = db_config_dir.join("sql");
						match std::fs::remove_dir_all(&db_path) {
							Ok(_) => {
								println!("{:?} removed.", &db_path);
							},
							Err(ref err) if err.kind() == std::io::ErrorKind::NotFound => {
								eprintln!("{:?} did not exist.", &db_path);
							},
							Err(err) => {
								return Err(format!(
									"Cannot purge `{:?}` database: {:?}",
									db_path, err,
								)
								.into())
							},
						};
					},
				};
				cmd.run(config.database)
			})
		},
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents { client, task_manager, backend, .. } =
					new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, backend, None), task_manager))
			})
		},
		#[cfg(feature = "runtime-benchmarks")]
		Some(Subcommand::Benchmark(cmd)) => {
			use crate::benchmarking::{
				inherent_benchmark_data, RemarkBuilder, TransferKeepAliveBuilder,
			};
			use frame_benchmarking_cli::{
				BenchmarkCmd, ExtrinsicFactory, SUBSTRATE_REFERENCE_HARDWARE,
			};
			use selendra_runtime::{Block, ExistentialDeposit};

			let runner = cli.create_runner(cmd)?;
			match cmd {
				BenchmarkCmd::Pallet(cmd) => runner.sync_run(|config| cmd.run::<Block, ()>(config)),
				BenchmarkCmd::Block(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					cmd.run(client)
				}),
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|mut config| {
					let (client, backend, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					let db = backend.expose_db();
					let storage = backend.expose_storage();
					cmd.run(config, client, db, storage)
				}),
				BenchmarkCmd::Overhead(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					let ext_builder = RemarkBuilder::new(client.clone());
					cmd.run(config, client, inherent_benchmark_data()?, Vec::new(), &ext_builder)
				}),
				BenchmarkCmd::Extrinsic(cmd) => runner.sync_run(|mut config| {
					let (client, _, _, _, _) = service::new_chain_ops(&mut config, &cli.eth)?;
					// Register the *Remark* and *TKA* builders.
					let ext_factory = ExtrinsicFactory(vec![
						Box::new(RemarkBuilder::new(client.clone())),
						Box::new(TransferKeepAliveBuilder::new(
							client.clone(),
							get_account_id_from_seed::<sp_core::ecdsa::Public>("Alice"),
							ExistentialDeposit::get(),
						)),
					]);

					cmd.run(client, inherent_benchmark_data()?, Vec::new(), &ext_factory)
				}),
				BenchmarkCmd::Machine(cmd) => {
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
				},
			}
		},
		#[cfg(not(feature = "runtime-benchmarks"))]
		Some(Subcommand::Benchmark) => {
			Err("Benchmarking wasn't enabled when building the node. selendra_primitives
			You can enable it with `--features runtime-benchmarks`."
				.into())
		},
		Some(Subcommand::FrontierDb(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|mut config| {
				let (client, _, _, _, frontier_backend) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				let frontier_backend = match frontier_backend {
					fc_db::Backend::KeyValue(kv) => std::sync::Arc::new(kv),
					_ => panic!("Only fc_db::Backend::KeyValue supported"),
				};
				cmd.run(client, frontier_backend)
			})
		},
		None => {
			let runner = cli.create_runner(&cli.run)?;

			config_validation_result.report();
			let mut aleph_cli_config = cli.aleph;

			runner.run_node_until_exit(|mut config| async move {
				if matches!(config.role, Role::Full) {
					if !aleph_cli_config.external_addresses().is_empty() {
						panic!(
							"A non-validator node cannot be run with external addresses specified."
						);
					}
					// We ensure that external addresses for non-validator nodes are set, but to a
					// value that is not routable. This will no longer be neccessary once we have
					// proper support for non-validator nodes, but this requires a major
					// refactor.
					info!(
						"Running as a non-validator node, setting dummy addressing configuration."
					);
					aleph_cli_config.set_dummy_external_addresses();
				}
				enforce_heap_pages(&mut config);
				service::build_full(config, aleph_cli_config, cli.eth).map_err(Into::into).await
			})
		},
	}
}
