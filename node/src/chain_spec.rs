pub fn selendra_chainspec() -> &'static [u8] {
	include_bytes!("../resources/selendra.json")
}

pub type SelendraNodeChainSpec = sc_service::GenericChainSpec<()>;

pub fn selendra_config() -> Result<SelendraNodeChainSpec, String> {
	SelendraNodeChainSpec::from_json_bytes(selendra_chainspec())
}
