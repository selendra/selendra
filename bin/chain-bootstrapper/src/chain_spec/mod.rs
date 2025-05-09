mod builder;
mod cli;
pub mod commands;
mod keystore;

pub use commands::{BootstrapChainCmd, ConvertChainspecToRawCmd};

pub const CHAINTYPE_DEV: &str = "dev";
pub const CHAINTYPE_LOCAL: &str = "local";
pub const CHAINTYPE_LIVE: &str = "live";

pub const DEFAULT_CHAIN_ID: &str = "selnet";
pub const DEFAULT_SUDO_ACCOUNT_ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

pub type SelendraNodeChainSpec = sc_service::GenericChainSpec<()>;

use primitives::AccountId;
use sc_chain_spec::ChainType;
use sc_cli::Error;
use sp_application_crypto::Ss58Codec;

fn parse_chaintype(s: &str) -> Result<ChainType, Error> {
    Ok(match s {
        CHAINTYPE_DEV => ChainType::Development,
        CHAINTYPE_LOCAL => ChainType::Local,
        CHAINTYPE_LIVE => ChainType::Live,
        s => panic!("Wrong chain type {s} Possible values: dev local live"),
    })
}

/// Generate AccountId based on string command line argument.
fn parse_account_id(s: &str) -> Result<AccountId, Error> {
    Ok(AccountId::from_string(s).expect("Passed string is not a hex encoding of a public key"))
}
