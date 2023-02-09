//! Make the set of voting bag thresholds to be used in `voter_bags.rs`.
//!
//! Generally speaking this script can be run once per runtime and never
//! touched again. It can be reused to regenerate a wholly different
//! quantity of bags, or if the existential deposit changes, etc.

use clap::{Parser, ValueEnum};
use generate_bags::generate_thresholds;
use selendra_runtime::Runtime as SelendraRuntime;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, ValueEnum)]
#[value(rename_all = "PascalCase")]
enum Runtime {
	Selendra,
}

impl Runtime {
	fn generate_thresholds_fn(
		&self,
	) -> Box<dyn FnOnce(usize, &Path, u128, u128) -> Result<(), std::io::Error>> {
		match self {
			Runtime::Selendra => Box::new(generate_thresholds::<SelendraRuntime>),
		}
	}
}

#[derive(Debug, Parser)]
struct Opt {
	/// How many bags to generate.
	#[arg(long, default_value_t = 200)]
	n_bags: usize,

	/// Which runtime to generate.
	#[arg(long, ignore_case = true, value_enum, default_value_t = Runtime::Selendra)]
	runtime: Runtime,

	/// Where to write the output.
	output: PathBuf,

	/// The total issuance of the native currency.
	#[arg(short, long)]
	total_issuance: u128,

	/// The minimum account balance (i.e. existential deposit) for the native currency.
	#[arg(short, long)]
	minimum_balance: u128,
}

fn main() -> Result<(), std::io::Error> {
	let Opt { n_bags, output, runtime, total_issuance, minimum_balance } = Opt::parse();

	runtime.generate_thresholds_fn()(n_bags, &output, total_issuance, minimum_balance)
}
