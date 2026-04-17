use sc_cli::{
    clap::{self, Parser, Subcommand as ClapSubcommand},
    PurgeChainCmd, RunCmd, SubstrateCli,
};

pub type SelendraNodeChainSpec = sc_service::GenericChainSpec<Option<()>>;

use crate::{
    aleph_cli::AlephCli,
    resources::{mainnet_chainspec, testnet_chainspec},
    eth::EthConfiguration
};

#[derive(Debug, Parser)]
#[command(subcommand_negates_reqs(true), version(env!("SUBSTRATE_CLI_IMPL_VERSION")))]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[command(flatten)]
    pub aleph: AlephCli,

    #[command(flatten)]
    pub run: RunCmd,

    #[command(flatten)]
	pub eth: EthConfiguration,
}

pub fn mainnet_config() -> Result<SelendraNodeChainSpec, String> {
    SelendraNodeChainSpec::from_json_bytes(mainnet_chainspec())
}

pub fn testnet_config() -> Result<SelendraNodeChainSpec, String> {
    SelendraNodeChainSpec::from_json_bytes(testnet_chainspec())
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
        "docs.selendra.org".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> Result<Box<dyn sc_service::ChainSpec>, String> {
        let default_chain = "testnet";
        let id = id.trim();
        let id = if id.is_empty() { default_chain } else { id };

        let chainspec = match id {
            "mainnet" | "selendra" => mainnet_config(),

            "testnet" => testnet_config(),
            _ => SelendraNodeChainSpec::from_json_file(id.into()),
        };
        Ok(Box::new(chainspec?))
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, ClapSubcommand)]
pub enum Subcommand {
    /// Key management cli utilities
    #[command(subcommand)]
    Key(sc_cli::KeySubcommand),

    /// Validate blocks.
    CheckBlock(sc_cli::CheckBlockCmd),

    /// Export blocks.
    ExportBlocks(sc_cli::ExportBlocksCmd),

    /// Export the state of a given block into a chain spec.
    ExportState(sc_cli::ExportStateCmd),

    /// Import blocks.
    ImportBlocks(sc_cli::ImportBlocksCmd),

    /// Remove the whole chain.
    PurgeChain(PurgeChainCmd),

    /// Revert the chain to a previous state.
    Revert(sc_cli::RevertCmd),

    /// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

    /// Db meta columns information.
	FrontierDb(fc_cli::FrontierDbCmd),

    /// The custom benchmark subcommand benchmarking runtime pallets.
    #[cfg(feature = "runtime-benchmarks")]
    #[clap(subcommand)]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),

    /// The custom benchmark subcommand benchmarking runtime pallets. Note: `runtime-benchmarks`
    /// feature must be enabled.
    #[cfg(not(feature = "runtime-benchmarks"))]
    Benchmark,
}
