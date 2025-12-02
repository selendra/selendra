use futures::TryFutureExt;
use selendra_node::{
	new_partial, Cli, ConfigValidator, ServiceComponents, Subcommand,
	eth, service
};
use log::info;
use primitives::HEAP_PAGES;
use sc_cli::{clap::Parser, SubstrateCli};
use sc_network::config::Role;
use sc_service::{Configuration, DatabaseSource};
use fc_db::kv::frontier_database_dir;
use std::sync::Arc;

fn enforce_heap_pages(config: &mut Configuration) {
    config.default_heap_pages = Some(HEAP_PAGES);
}

fn main() -> sc_cli::Result<()> {
    let mut cli = Cli::parse();

	let config_validation_result = ConfigValidator::process(&mut cli);

	match &cli.subcommand {
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.database), task_manager))
			})
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents {
                    client,
                    task_manager,
                    ..
                } = new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, config.chain_spec), task_manager))
			})
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.async_run(|mut config| {
				let ServiceComponents {
                    client,
                    task_manager,
                    import_queue,
                    ..
                } = new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, import_queue), task_manager))
			})
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				// Remove Frontier offchain db
				let db_config_dir =eth::db_config_dir(&config);
				match cli.eth.frontier_backend_type {
					eth::BackendType::KeyValue => {
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
					eth::BackendType::Sql => {
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
				let ServiceComponents {
                    client,
                    task_manager,
                    backend,
                    ..
                } = new_partial(&mut config, &cli.eth)?;
				Ok((cmd.run(client, backend, None), task_manager))
			})
		}
        Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::FrontierDb(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|mut config| {
				let (client, _, _, _, frontier_backend) =
					service::new_chain_ops(&mut config, &cli.eth)?;
				let frontier_backend = match &*frontier_backend {
					fc_db::Backend::KeyValue(kv) => Arc::new(kv.clone()),
					_ => panic!("Only fc_db::Backend::KeyValue supported"),
				};
				cmd.run(client, frontier_backend)
			})
		}
        #[cfg(feature = "runtime-benchmarks")]
        Some(Subcommand::Benchmark(cmd)) => {
            use selendra_node::ExecutorDispatch;
            use primitives::Block;
            use sc_executor::NativeExecutionDispatch;

            let runner = cli.create_runner(cmd)?;
            runner.sync_run(|config| {
                if let frame_benchmarking_cli::BenchmarkCmd::Pallet(cmd) = cmd {
                    cmd.run::<Block, <ExecutorDispatch as NativeExecutionDispatch>::ExtendHostFunctions>(config)
                } else {
                    Err(sc_cli::Error::Input("Wrong subcommand".to_string()))
                }
            })
        }
        #[cfg(not(feature = "runtime-benchmarks"))]
        Some(Subcommand::Benchmark) => Err(
            "Benchmarking wasn't enabled when building the node. You can enable it with \
                    `--features runtime-benchmarks`."
                .into(),
        ),
		None => {
			let runner = cli.create_runner(&cli.run)?;

			config_validation_result.report();

            let mut selendra_cli_config = cli.aleph;
			runner.run_node_until_exit(|mut config| async move {
				if matches!(config.role, Role::Full) {
					if !selendra_cli_config.external_addresses().is_empty() {
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
					selendra_cli_config.set_dummy_external_addresses();
				}
				enforce_heap_pages(&mut config);
                service::build_full(config, selendra_cli_config, cli.eth).map_err(sc_cli::Error::Service).await
			})
        }
	}
}
