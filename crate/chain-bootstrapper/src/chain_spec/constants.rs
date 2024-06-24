pub const CHAINTYPE_DEV: &str = "dev";
pub const CHAINTYPE_LOCAL: &str = "local";
pub const CHAINTYPE_LIVE: &str = "live";

pub const DEFAULT_CHAIN_ID: &str = "selendranet1";
pub const DEFAULT_SUDO_ACCOUNT_ALICE: &str = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

pub type SelendraNodeChainSpec = sc_service::GenericChainSpec<()>;

use sc_chain_spec::ChainType;
use sc_cli::Error;
use selendra_primitives::AccountId;
use sp_application_crypto::Ss58Codec;

pub fn parse_chaintype(s: &str) -> Result<ChainType, Error> {
	Ok(match s {
		CHAINTYPE_DEV => ChainType::Development,
		CHAINTYPE_LOCAL => ChainType::Local,
		CHAINTYPE_LIVE => ChainType::Live,
		s => panic!("Wrong chain type {s} Possible values: dev local live"),
	})
}

/// Generate AccountId based on string command line argument.
pub fn parse_account_id(s: &str) -> Result<AccountId, Error> {
	Ok(AccountId::from_string(s).expect("Passed string is not a hex encoding of a public key"))
}
