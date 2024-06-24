//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![allow(clippy::type_complexity, clippy::too_many_arguments, clippy::large_enum_variant)]
#![cfg_attr(feature = "runtime-benchmarks", warn(unused_crate_dependencies))]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod cli;
mod command;
mod eth;
mod executor;
mod rpc;
mod service;
mod chain_spec;
mod config;

pub use cli::{Cli, Subcommand};
pub use config::Validator as ConfigValidator;
#[cfg(any(
    feature = "runtime-benchmarks",
    feature = "aleph-native-runtime",
    feature = "try-runtime"
))]
pub use executor::aleph_executor::ExecutorDispatch;
pub use service::{new_authority, new_partial, ServiceComponents};

fn main() -> sc_cli::Result<()> {
	command::run()
}

