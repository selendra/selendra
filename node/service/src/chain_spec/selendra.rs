// This file is part of Selendra.

// Copyright (C) 2021-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Selendra chain configurations.

use super::{
	authority_keys_from_seed, get_account_id_from_seed, testnet_accounts, AccountId,
	AuthorityDiscoveryId, BabeId, Balance, ChainSpecExtension, GrandpaId, ImOnlineId, TokenInfo,
	DEFAULT_PROTOCOL_ID, TELEMETRY_URL,
};

use hex_literal::hex;
use serde::{Deserialize, Serialize};
use serde_json::map::Map;

use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_core::{crypto::UncheckedInto, sr25519};
use sp_runtime::Perbill;

use selendra_runtime::{
	dollar, AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, Block, CdpEngineConfig,
	CdpTreasuryConfig, CouncilConfig, CouncilMembershipConfig, DexConfig, EVMConfig,
	FinancialCouncilMembershipConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig,
	OperatorMembershipSelendraConfig, OrmlNFTConfig, SS58Prefix, SessionConfig, SessionKeys,
	StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalMembershipConfig, TokensConfig,
	KUSD, SEL, VestingConfig
};

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;

/// Flaming Fir testnet generator
pub fn selendra_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../resources/selendra.json")[..])
}

/// Flaming Fir testnet generator
pub fn testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../resources/testnet.json")[..])
}

fn selendra_properties() -> Properties {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	[SEL, KUSD].iter().for_each(|token| {
		token_symbol.push(token.symbol().unwrap().to_string());
		token_decimals.push(token.decimals().unwrap() as u32);
	});
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	properties
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Some(selendra_properties()),
		Default::default(),
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_selendra_genesis,
		vec![],
		None,
		None,
		None,
		Some(selendra_properties()),
		Default::default(),
	)
}

/// Staging testnet config.
pub fn staging_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Selendra Staging",
		"selendra_staging",
		ChainType::Live,
		testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(selendra_properties()),
		Default::default(),
	)
}

/// Staging testnet config.
pub fn mainnet_staging_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"Selendra",
		"selendra",
		ChainType::Live,
		selendra_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(selendra_properties()),
		Default::default(),
	)
}

fn development_config_genesis() -> GenesisConfig {
	let wasm_binary = selendra_runtime::WASM_BINARY.unwrap_or_default();

	selendra_development_genesis(
		wasm_binary,
		vec![authority_keys_from_seed("Alice")],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

fn local_selendra_genesis() -> GenesisConfig {
	let wasm_binary = selendra_runtime::WASM_BINARY.unwrap_or_default();

	selendra_development_genesis(
		wasm_binary,
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
			authority_keys_from_seed("Charlie"),
			authority_keys_from_seed("Dave"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

fn testnet_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5EFWY51UxSopvC8BrzTujUD5fCv1eEYVyGV5NJs7a3fVFB2P
			hex!["60b60ebdcce971da7cc8ae0ce7368ee2c7c4a24cd28f145b8ff3633b7b66db56"].into(),
			// 5GmrwtGam5oZxfi4CdAxUs2aSt91bLRqHwRTyuvwtwGKErMX
			hex!["d05498468ffcff39db873e9ea1a0df8dad2e2660db27d59c971383578309de3c"].into(),
			// 5HAAUgq56DWpfGhjqfAuzNAU3cNH3v8SwrG64xg41vpUicwk
			hex!["e1570233fa258d316457a3e61df77c7d47421947c63bf92864b799011b79eeff"]
				.unchecked_into(),
			// 5GguKWk8Miysxevoazr66Lz2LRohRPXQ7ZhXF3FzyzK9QrND
			hex!["cc8c5b119fcd5058592ef44b8c25a109ed35cb74463843f14ee723f3948fe200"]
				.unchecked_into(),
			// 5GguKWk8Miysxevoazr66Lz2LRohRPXQ7ZhXF3FzyzK9QrND
			hex!["cc8c5b119fcd5058592ef44b8c25a109ed35cb74463843f14ee723f3948fe200"]
				.unchecked_into(),
			// 5GguKWk8Miysxevoazr66Lz2LRohRPXQ7ZhXF3FzyzK9QrND
			hex!["cc8c5b119fcd5058592ef44b8c25a109ed35cb74463843f14ee723f3948fe200"]
				.unchecked_into(),
		),
		(
			// 5G9AmTwyK1qtBuVUxvyhTG4sXYgYsmgHvMv9YTBwk5qpyGHP
			hex!["b45880e7830c7a3bd7bf2aefef38454514c3060f52c08adef7fbba8fed94206f"].into(),
			// 5CJ6XQsmFEb4XR4JpPor8mCMWKbhVAR3T5G6BSrm66eknhSc
			hex!["0a367f5e1ca979234e67baabbdcaf24c8a0b33472930bfb915d9c7730a449e67"].into(),
			// 5CKBpJi8EConBhyQwr7viNzR62JswFUomXr8ZeKe5joaEmkY
			hex!["0b0b8e106f8db92ba44c441ea61a1877d7df51a4569ee67fe390530cf0c60923"]
				.unchecked_into(),
			// 5CPhhXA3bYy3AAKBMNfA4fbA4UCyUx6Nb4WQrbFTExYXDmGk
			hex!["0e7d21b970155d93584c3e293ffac20bad264cf30c7d4cd564f68b2f7a818942"]
				.unchecked_into(),
			// 5CPhhXA3bYy3AAKBMNfA4fbA4UCyUx6Nb4WQrbFTExYXDmGk
			hex!["0e7d21b970155d93584c3e293ffac20bad264cf30c7d4cd564f68b2f7a818942"]
				.unchecked_into(),
			// 5CPhhXA3bYy3AAKBMNfA4fbA4UCyUx6Nb4WQrbFTExYXDmGk
			hex!["0e7d21b970155d93584c3e293ffac20bad264cf30c7d4cd564f68b2f7a818942"]
				.unchecked_into(),
		),
		(
			// 5G6TLUeS8Y3UKZQ5BbtbQVJRkshFSQAdqo9tcJKaQJPNPc36
			hex!["b2468b4bf038fc09bd7ef2b0373b4fa68a3a742cb092e8670c6cd181adc81142"].into(),
			// 5GRV9vKCYuXeEnqPsiJqBWvECZP2Af7hpsKdwhjL5FNaSSFf
			hex!["c0ca53a000d89e74b6091b5089656be9fc1760dd81e45828419a0de620a6f549"].into(),
			// 5CusfKW7pJRR1zRAJvkW6swJvTDbrA3KbuVk2QJRT1S5h3MV
			hex!["25800774d0bbec2cac526ac6f26c5ab5e5f517d055aae4687faf9e63c064c39e"]
				.unchecked_into(),
			// 5CFvLRva9hhamARHaDA8CkN1P7GZrcTYaahnWCLV2h6DduhP
			hex!["088db4d2412dc174a70b01fe0113eef9edff00beb5015343a810683e3b2f9e79"]
				.unchecked_into(),
			// 5CFvLRva9hhamARHaDA8CkN1P7GZrcTYaahnWCLV2h6DduhP
			hex!["088db4d2412dc174a70b01fe0113eef9edff00beb5015343a810683e3b2f9e79"]
				.unchecked_into(),
			// 5CFvLRva9hhamARHaDA8CkN1P7GZrcTYaahnWCLV2h6DduhP
			hex!["088db4d2412dc174a70b01fe0113eef9edff00beb5015343a810683e3b2f9e79"]
				.unchecked_into(),
		),
		(
			// 5DhhuzDuSi1Fu5fU5PBM3F1dEyFUZuw3zTL1yibnBZ4w7cin
			hex!["4874818b10ccb74ba077a22dc79f7476a9ad1e669192d5ccb41aa6e98457e46f"].into(),
			// 5C7chbGxavPzeLRqyzrSSTmWWtjsMoGcSX9zTnzDdawJ6vaz
			hex!["023860bd3ee424d1025c4c15a6909557845546863f0e25b438598e2d6c444d38"].into(),
			// 5Eihx1pfEqVuqd1URWcG6jTA7xd1y8GdBsiDrS9Jyo4UG5Cu
			hex!["75741a41b673fd9e8827b66fa07e06264fcef545c5dba35f23a31901b434e989"]
				.unchecked_into(),
			// 5Ent46fLfj63aa5jj71gwd4Eq6FDh6uravC3FYKiB5S3rSbr
			hex!["78a319aae5f1f60589eea97569d3e3db14c3dbfd766d23ab0b308d6348182d48"]
				.unchecked_into(),
			// 5Ent46fLfj63aa5jj71gwd4Eq6FDh6uravC3FYKiB5S3rSbr
			hex!["78a319aae5f1f60589eea97569d3e3db14c3dbfd766d23ab0b308d6348182d48"]
				.unchecked_into(),
			// 5Ent46fLfj63aa5jj71gwd4Eq6FDh6uravC3FYKiB5S3rSbr
			hex!["78a319aae5f1f60589eea97569d3e3db14c3dbfd766d23ab0b308d6348182d48"]
				.unchecked_into(),
		),
	];

	let root_key: AccountId = hex![
		// 5DiQQJEKjLf9ucHzitkVehzHtZmj8rvYBzGiqbfwcqVa6mpg
		"48fccd18e5901ad0a484cfc4859f57c163219fa6911c3ba824c2d9743167795f"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];
	let wasm_binary = selendra_runtime::WASM_BINARY.unwrap_or_default();

	selendra_genesis(wasm_binary, initial_authorities, root_key, Some(endowed_accounts))
}

fn selendra_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5GN7hUZdJMU9tWiEDvVCgcif58vwDtVqQdCaruBZ652jhmCR
			hex!["be385d0437ac1f0a0cfbdc656aa268e33a2734ef7f07167bc594b33bdd695e1f"].into(),
			// 5D7o9uWzzxR99SixNMW8NrCBVFuKU2E2vnGSehF9uJJgYUbK
			hex!["2e97cd376dd1f8c2d8b10669acf119cfa59f4d9a0be706b1c775c180a1d28875"].into(),
			// 5EKocoqq3nYDAgoeHQMRJBSanZqgDpwnjgxZryDTA9jWiADK
			hex!["63fc8a9432ae3074ec280a74dfe46be74308de401ea913925136852e06b639e1"]
				.unchecked_into(),
			// 5F4UfYBPSSYL8QfU6ZTUuf9UiU7nuPDvQtf9c4Wrakz7oCRf
			hex!["84884ade5bc72827b2d66ab8e83763ec41ad55e98ad64b15739af97e2e3cc476"]
				.unchecked_into(),
			// 5F4UfYBPSSYL8QfU6ZTUuf9UiU7nuPDvQtf9c4Wrakz7oCRf
			hex!["84884ade5bc72827b2d66ab8e83763ec41ad55e98ad64b15739af97e2e3cc476"]
				.unchecked_into(),
			// 5F4UfYBPSSYL8QfU6ZTUuf9UiU7nuPDvQtf9c4Wrakz7oCRf
			hex!["84884ade5bc72827b2d66ab8e83763ec41ad55e98ad64b15739af97e2e3cc476"]
				.unchecked_into(),
		),
		(
			// 5CFktyx3ztxa8Ga1c2gdbVvkgXeWjATkRzgq9s2mDx19qThx
			hex!["086deec2141596b8a5918a4218d74c23549e0082c3371718eef53fd4650f817c"].into(),
			// 5C4s55jCYzvYkyMYvLG5tcE81bwQ4VrRy5GJT3UDAU2D4A3q
			hex!["001f0465ed847407f2d987679caa00b9959c9d6abb29323927f4c02f9131a919"].into(),
			// 5Dv3y5SqVrpuGpsLQ9bfiqaKfi8bL1aENxEfZzpvbiAXafNv
			hex!["51def4cb548a5f6669cd302a19c865adb0a7744c250c1d0819910e6c8a10a52f"]
				.unchecked_into(),
			// 5GR7yG1sh48BfyTWrfrPHwM1FGRrTK9rvUofwHtqh58dTTRk
			hex!["c0830400cc20b9b8ddc097990c40654be2b81751936ecdffc6f0fdeb3104f94d"]
				.unchecked_into(),
			// 5GR7yG1sh48BfyTWrfrPHwM1FGRrTK9rvUofwHtqh58dTTRk
			hex!["c0830400cc20b9b8ddc097990c40654be2b81751936ecdffc6f0fdeb3104f94d"]
				.unchecked_into(),
			// 5GR7yG1sh48BfyTWrfrPHwM1FGRrTK9rvUofwHtqh58dTTRk
			hex!["c0830400cc20b9b8ddc097990c40654be2b81751936ecdffc6f0fdeb3104f94d"]
				.unchecked_into(),
		),
		(
			// 5DhjiiK5cmHwEuUP4n6i2PcoeKkdmENsvEc9XxFW3qMkLobu
			hex!["487a9592c0757c24f56584706a8cf570530d37a0e9ebb13d8ff8558978c31431"].into(),
			// 5EjbQqcoTiCvzvpfDXxjdZnkPQgj4mzqp4GvLd7mj5qJB1ZN
			hex!["76215739f8bc31a795783a04119f1dd57e9c2c2d6413a754f2647f30b5a47062"].into(),
			// 5DQf6o1CCN9e2gFERXkRgjmbHybo5VWvHjL9LauSJQMJby1f
			hex!["3b73dcca64144574e60e5c21446dcc3993d7b9d83b54e16580874f48cf049cb0"]
				.unchecked_into(),
			// 5HTGLMDJ7qaUQz8LzfVXfUBQPNk1Bj1x6E6vikUNWZoKFCGz
			hex!["ee61e4f2fd572d1a17154382dad88d6f31669817272215268f3b2090c4f01b67"]
				.unchecked_into(),
			// 5HTGLMDJ7qaUQz8LzfVXfUBQPNk1Bj1x6E6vikUNWZoKFCGz
			hex!["ee61e4f2fd572d1a17154382dad88d6f31669817272215268f3b2090c4f01b67"]
				.unchecked_into(),
			// 5HTGLMDJ7qaUQz8LzfVXfUBQPNk1Bj1x6E6vikUNWZoKFCGz
			hex!["ee61e4f2fd572d1a17154382dad88d6f31669817272215268f3b2090c4f01b67"]
				.unchecked_into(),
		),
		(
			// 5FUg8njCL72Q59D2JsVvrRmyENdzT1RCfQzGTvpW8rpaQh4s
			hex!["96fccb07f2a21dcc61d62cf1c5af3b3579eb7a8fccf1861ace49609ef63dd60f"].into(),
			// 5FCAFnwZwdpF1VhENMaCGuPLu8JAJoCX1ijLRZ8UqmFJtfaY
			hex!["8a6448bb5ec5ce679a22e517baf3d99b33a6a00507922436503caa747bb0a232"].into(),
			// 5Dp6sGi38AdXEg8y91f3W4rTHeGxs62ADrkAPKSfKmtH5mYb
			hex!["4d553ee86372a9e3e1ca2c8fa87e7c3c6a1eae231474252f134e60f5e46cbc9e"]
				.unchecked_into(),
			// 5D72pBUAv3rGWnXCVvDmdYHwWt3rjjxU2CfVEVVrCkKHye8R
			hex!["2e028a22bfa0628996d8a3794b770e5505aee386c64ef8d02c1bc9b808d95f69"]
				.unchecked_into(),
			// 5D72pBUAv3rGWnXCVvDmdYHwWt3rjjxU2CfVEVVrCkKHye8R
			hex!["2e028a22bfa0628996d8a3794b770e5505aee386c64ef8d02c1bc9b808d95f69"]
				.unchecked_into(),
			// 5D72pBUAv3rGWnXCVvDmdYHwWt3rjjxU2CfVEVVrCkKHye8R
			hex!["2e028a22bfa0628996d8a3794b770e5505aee386c64ef8d02c1bc9b808d95f69"]
				.unchecked_into(),
		),
		(
			// 5HQwTy6rFvKG1ffX6ujqWwxcLCucJxKXip9sVYW6E89WcN32
			hex!["ec9be2d1c917426e923cfc686302140f28e44ebd67f4c46c9da37f8ef35d663e"].into(),
			// 5D9kprYDUCLo8xA4PzUxgJxuLeYgML1wFRJfc683AhftPWV8
			hex!["3016743ff73e7e981a3532d83cd85e495e8b029be89a4cb9779b43f664c08f27"].into(),
			// 5GXDeatvoZ3pPSMupH2yxAG5MJtXQ5ayTk8ur5c9iT6iNqiY
			hex!["c5299a1ce614ed930d3c60598ec11497e9d62f292e4b5055258e11b677a0ad45"]
				.unchecked_into(),
			// 5HHGwiKuAdcsfbVP656WYvjQ4na57TWiTWSCjxt62iRCVWkV
			hex!["e6c37e287e779660cd7ea7b243a61963fd888c2252ebea6c8b2401015f795a67"]
				.unchecked_into(),
			// 5HHGwiKuAdcsfbVP656WYvjQ4na57TWiTWSCjxt62iRCVWkV
			hex!["e6c37e287e779660cd7ea7b243a61963fd888c2252ebea6c8b2401015f795a67"]
				.unchecked_into(),
			// 5HHGwiKuAdcsfbVP656WYvjQ4na57TWiTWSCjxt62iRCVWkV
			hex!["e6c37e287e779660cd7ea7b243a61963fd888c2252ebea6c8b2401015f795a67"]
				.unchecked_into(),
		),
	];

	let root_key: AccountId = hex![
		// 5CkQtjERkXaTRr6Kioi8VfDZ52J72K6QAo2THNAfzg1ybUXH
		"1e48b569d594871628099074a611bddc8177ca642ed6a2e64becdd025c975e41"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];
	let wasm_binary = selendra_runtime::WASM_BINARY.unwrap_or_default();

	selendra_genesis(wasm_binary, initial_authorities, root_key, Some(endowed_accounts))
}

/// Helper function to create GenesisConfig for testing
pub fn selendra_development_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let endowment: Balance = 10_000_000 * dollar(SEL);
	let stash: Balance = endowment / 1000;
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	GenesisConfig {
		system: SystemConfig { code: wasm_binary.to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, endowment)).collect(),
		},
		council: CouncilConfig::default(),
		council_membership: CouncilMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		financial_council: Default::default(),
		financial_council_membership: FinancialCouncilMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		operator_membership_selendra: OperatorMembershipSelendraConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), stash, StakerStatus::Validator))
				.collect(),
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(selendra_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		vesting: VestingConfig { vesting: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		nomination_pools: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		democracy: Default::default(),
		treasury: Default::default(),
		tokens: TokensConfig { balances: vec![] },
		cdp_treasury: CdpTreasuryConfig { expected_collateral_auction_size: vec![] },
		cdp_engine: CdpEngineConfig { collaterals_params: vec![] },
		asset_registry: Default::default(),
		evm: EVMConfig { chain_id: 200u64, accounts: Default::default() },
		dex: DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: vec![],
			initial_added_liquidity_pools: vec![],
		},
		orml_nft: OrmlNFTConfig { tokens: vec![] },
		sudo: SudoConfig { key: Some(root_key) },
	}
}

/// Helper function to create GenesisConfig for testing
pub fn selendra_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> GenesisConfig {
	let endowment: Balance = 527_922_923 * dollar(SEL);
	let stash: Balance = 100 * dollar(SEL);
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	GenesisConfig {
		system: SystemConfig { code: wasm_binary.to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), endowment))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), stash)))
				.collect(),
		},
		council: CouncilConfig::default(),
		council_membership: CouncilMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		financial_council: Default::default(),
		financial_council_membership: FinancialCouncilMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		technical_committee: Default::default(),
		technical_membership: TechnicalMembershipConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		operator_membership_selendra: OperatorMembershipSelendraConfig {
			members: vec![].try_into().unwrap(),
			phantom: Default::default(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), stash, StakerStatus::Validator))
				.collect(),
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		},
		phragmen_election: Default::default(),
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(selendra_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		vesting: VestingConfig { vesting: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		indices: IndicesConfig { indices: vec![] },
		nomination_pools: Default::default(),
		democracy: Default::default(),
		treasury: Default::default(),
		tokens: TokensConfig { balances: vec![] },
		cdp_treasury: CdpTreasuryConfig { expected_collateral_auction_size: vec![] },
		cdp_engine: CdpEngineConfig { collaterals_params: vec![] },
		asset_registry: Default::default(),
		evm: EVMConfig { chain_id: 204u64, accounts: Default::default() },
		dex: DexConfig {
			initial_listing_trading_pairs: vec![],
			initial_enabled_trading_pairs: vec![],
			initial_added_liquidity_pools: vec![],
		},
		orml_nft: OrmlNFTConfig { tokens: vec![] },
		sudo: SudoConfig { key: Some(root_key) },
	}
}
