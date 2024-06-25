pub fn testnet_chainspec() -> &'static [u8] {
	include_bytes!("../resources/testnet.json")
}

pub type SelendraNodeChainSpec = sc_service::GenericChainSpec<()>;

pub fn testnet_config() -> Result<SelendraNodeChainSpec, String> {
	SelendraNodeChainSpec::from_json_bytes(testnet_chainspec())
}