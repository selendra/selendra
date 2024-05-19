use sc_cli::{Result, RpcMethods, RunCmd, SubstrateCli};
use sc_service::BasePath;

use crate::{cli::Cli, service};
#[derive(Clone, Debug, clap::Args)]
pub struct ExtendedRunCmd {
    #[clap(flatten)]
    pub base: RunCmd,
}

impl ExtendedRunCmd {
    /// The substrate base directory on your machine
    ///
    /// Will be different depending on your OS
    pub fn base_path(&self) -> Result<BasePath> {
        Ok(self
            .base
            .shared_params
            .base_path()?
            .unwrap_or_else(|| BasePath::from_project("", "", &<Cli as SubstrateCli>::executable_name())))
    }
}

pub fn run_node(mut cli: Cli) -> Result<()> {
    if cli.run.base.shared_params.dev {
        override_dev_environment(&mut cli.run);
    }
    let runner = cli.create_runner(&cli.run.base)?;

    runner.run_node_until_exit(|config| async move {
        service::new_full(config, cli).map_err(sc_cli::Error::Service)
    })
}

fn override_dev_environment(cmd: &mut ExtendedRunCmd) {
    // create a reproducible dev environment
    // by disabling the default substrate `dev` behaviour
    cmd.base.shared_params.dev = false;
    cmd.base.shared_params.chain = Some("dev".to_string());

    cmd.base.force_authoring = true;
    cmd.base.alice = true;
    cmd.base.tmp = true;

    // we can't set `--rpc-cors=all`, so it needs to be set manually if we want to connect with external
    // hosts
    cmd.base.rpc_external = true;
    cmd.base.rpc_methods = RpcMethods::Unsafe;
}