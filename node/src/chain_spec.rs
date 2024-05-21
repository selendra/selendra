// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Substrate chain configurations.

use std::path::PathBuf;

use sp_consensus_grandpa::AuthorityId as GrandpaId;
use selendra_runtime::{
	BabeConfig, BalancesConfig, Block, WASM_BINARY, 
	ImOnlineConfig, IndicesConfig, MaxNominations, SessionConfig, SessionKeys,
	StakerStatus, StakingConfig, SudoConfig, SystemConfig,
};
use selendra_runtime_constants::currency::*;

use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_starknet::genesis_loader::{GenesisData, GenesisLoader};

use sc_chain_spec::ChainSpecExtension;
use sc_service::{BasePath, ChainType};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub use selendra_primitives::{AccountId, Balance, Signature};
pub use selendra_runtime::RuntimeGenesisConfig;

use crate::starknet::constants::DEV_CHAIN_ID;

type AccountPublic = <Signature as Verify>::Signer;

pub const GENESIS_ASSETS_DIR: &str = "genesis-assets/";
pub const GENESIS_ASSETS_FILE: &str = "genesis.json";
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

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

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<RuntimeGenesisConfig, Extensions>;

pub fn selendra_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/selendra.json")[..])
}

pub fn selendra_testnet_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/selendra-testnet.json")[..])
}

/// Returns the properties for the [`SelendraChainSpec`].
pub fn selendra_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"tokenDecimals": 18,
		"tokenSymbol": "SEL"
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn load_genesis(data_path: PathBuf) -> GenesisLoader {
    let genesis_path = data_path.join(GENESIS_ASSETS_DIR).join(GENESIS_ASSETS_FILE);

    log::debug!("🧪 Loading genesis data at : {}", genesis_path.display());
    let genesis_file_content = std::fs::read_to_string(genesis_path)
        .expect("Failed to read genesis file. Please run `madara setup` before opening an issue.");
    let genesis_data: GenesisData = serde_json::from_str(&genesis_file_content).expect("Failed loading genesis");
    GenesisLoader::new(data_path, genesis_data)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create RuntimeGenesisConfig for testing
pub fn testnet_genesis(
	genesis_loader: GenesisLoader,
	wasm_binary: &[u8],
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
) -> RuntimeGenesisConfig {

	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
		]
	});
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.0.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let starknet_genesis_config: selendra_runtime::starknet::pallet_starknet::GenesisConfig<_> = genesis_loader.into();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;

	RuntimeGenesisConfig {
		system: SystemConfig { code: wasm_binary.to_vec(), ..Default::default() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect(),
		},
		indices: IndicesConfig { indices: vec![] },
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
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			epoch_config: Some(selendra_runtime::BABE_GENESIS_EPOCH_CONFIG),
			..Default::default()
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: Default::default(),
		grandpa: Default::default(),
		treasury: Default::default(),
		vesting: Default::default(),
		transaction_payment: Default::default(),
		/// Starknet Genesis configuration.
        starknet: starknet_genesis_config,
	}
}

/// Development config (single validator Alice)
pub fn development_config(base_path: BasePath) -> Result<ChainSpec, String> {
	let chain_id = DEV_CHAIN_ID;
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		move || {
            testnet_genesis(
				load_genesis(base_path.config_dir(&chain_id)),
				wasm_binary,
				vec![authority_keys_from_seed("Alice")],
				vec![],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				None,
            )
        },
		vec![],
		None,
		None,
		None,
		Some(selendra_chain_spec_properties()),
		Default::default(),
	))
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config(base_path: BasePath, chain_id: &str) -> Result<ChainSpec, String>  {
	let owned_chain_id = chain_id.to_owned();
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				load_genesis(base_path.config_dir(&owned_chain_id)),
				wasm_binary,
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				None,
			)
        },
		vec![],
		None,
		None,
		None,
		Some(selendra_chain_spec_properties()),
		Default::default(),
	))
}

/// Staging testnet config.
#[rustfmt::skip]
pub fn staging_testnet_config(base_path: BasePath, chain_id: &str) -> Result<ChainSpec, String> {
	let boot_nodes = vec![];
	let owned_chain_id = chain_id.to_owned();
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	
	Ok(ChainSpec::from_genesis(
		"Staging Testnet",
		"staging_testnet",
		ChainType::Live,
		move || {
			testnet_genesis(
				load_genesis(base_path.config_dir(&owned_chain_id)), 
				wasm_binary,
				vec![
					(
						// 5Fbsd6WXDGiLTxunqeK5BATNiocfCqu9bS1yArVjCgeBLkVy
						array_bytes::hex_n_into_unchecked("9c7a2ee14e565db0c69f78c7b4cd839fbf52b607d867e9e9c5a79042898a0d12"),
						// 5EnCiV7wSHeNhjW3FSUwiJNkcc2SBkPLn5Nj93FmbLtBjQUq
						array_bytes::hex_n_into_unchecked("781ead1e2fa9ccb74b44c19d29cb2a7a4b5be3972927ae98cd3877523976a276"),
						// 5Fb9ayurnxnaXj56CjmyQLBiadfRCqUbL2VWNbbe1nZU6wiC
						array_bytes::hex2array_unchecked("9becad03e6dcac03cee07edebca5475314861492cdfc96a2144a67bbe9699332")
							.unchecked_into(),
						// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
						array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
							.unchecked_into(),
						// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
						array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
							.unchecked_into(),
						// 5EZaeQ8djPcq9pheJUhgerXQZt9YaHnMJpiHMRhwQeinqUW8
						array_bytes::hex2array_unchecked("6e7e4eb42cbd2e0ab4cae8708ce5509580b8c04d11f6758dbf686d50fe9f9106")
							.unchecked_into(),
					),
					(
						// 5ERawXCzCWkjVq3xz1W5KGNtVx2VdefvZ62Bw1FEuZW4Vny2
						array_bytes::hex_n_into_unchecked("68655684472b743e456907b398d3a44c113f189e56d1bbfd55e889e295dfde78"),
						// 5Gc4vr42hH1uDZc93Nayk5G7i687bAQdHHc9unLuyeawHipF
						array_bytes::hex_n_into_unchecked("c8dc79e36b29395413399edaec3e20fcca7205fb19776ed8ddb25d6f427ec40e"),
						// 5EockCXN6YkiNCDjpqqnbcqd4ad35nU4RmA1ikM4YeRN4WcE
						array_bytes::hex2array_unchecked("7932cff431e748892fa48e10c63c17d30f80ca42e4de3921e641249cd7fa3c2f")
							.unchecked_into(),
						// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
						array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
							.unchecked_into(),
						// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
						array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
							.unchecked_into(),
						// 5DhLtiaQd1L1LU9jaNeeu9HJkP6eyg3BwXA7iNMzKm7qqruQ
						array_bytes::hex2array_unchecked("482dbd7297a39fa145c570552249c2ca9dd47e281f0c500c971b59c9dcdcd82e")
							.unchecked_into(),
					),
					(
						// 5DyVtKWPidondEu8iHZgi6Ffv9yrJJ1NDNLom3X9cTDi98qp
						array_bytes::hex_n_into_unchecked("547ff0ab649283a7ae01dbc2eb73932eba2fb09075e9485ff369082a2ff38d65"),
						// 5FeD54vGVNpFX3PndHPXJ2MDakc462vBCD5mgtWRnWYCpZU9
						array_bytes::hex_n_into_unchecked("9e42241d7cd91d001773b0b616d523dd80e13c6c2cab860b1234ef1b9ffc1526"),
						// 5E1jLYfLdUQKrFrtqoKgFrRvxM3oQPMbf6DfcsrugZZ5Bn8d
						array_bytes::hex2array_unchecked("5633b70b80a6c8bb16270f82cca6d56b27ed7b76c8fd5af2986a25a4788ce440")
							.unchecked_into(),
						// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
						array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
							.unchecked_into(),
						// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
						array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
							.unchecked_into(),
						// 5DhKqkHRkndJu8vq7pi2Q5S3DfftWJHGxbEUNH43b46qNspH
						array_bytes::hex2array_unchecked("482a3389a6cf42d8ed83888cfd920fec738ea30f97e44699ada7323f08c3380a")
							.unchecked_into(),
					),
					(
						// 5HYZnKWe5FVZQ33ZRJK1rG3WaLMztxWrrNDb1JRwaHHVWyP9
						array_bytes::hex_n_into_unchecked("f26cdb14b5aec7b2789fd5ca80f979cef3761897ae1f37ffb3e154cbcc1c2663"),
						// 5EPQdAQ39WQNLCRjWsCk5jErsCitHiY5ZmjfWzzbXDoAoYbn
						array_bytes::hex_n_into_unchecked("66bc1e5d275da50b72b15de072a2468a5ad414919ca9054d2695767cf650012f"),
						// 5DMa31Hd5u1dwoRKgC4uvqyrdK45RHv3CpwvpUC1EzuwDit4
						array_bytes::hex2array_unchecked("3919132b851ef0fd2dae42a7e734fe547af5a6b809006100f48944d7fae8e8ef")
							.unchecked_into(),
						// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
						array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
							.unchecked_into(),
						// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
						array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
							.unchecked_into(),
						// 5C4vDQxA8LTck2xJEy4Yg1hM9qjDt4LvTQaMo4Y8ne43aU6x
						array_bytes::hex2array_unchecked("00299981a2b92f878baaf5dbeba5c18d4e70f2a1fcd9c61b32ea18daf38f4378")
							.unchecked_into(),
					),
				],
				vec![],
				array_bytes::hex_n_into_unchecked(
					// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
					"9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809",
				), 
				Some(vec![array_bytes::hex_n_into_unchecked(
					// 5Ff3iXP75ruzroPWRP2FYBHWnmGGBSb63857BgnzCoXNxfPo
					"9ee5e5bdc0ec239eb164f865ecc345ce4c88e76ee002e0f7e318097347471809",
				)]))
        },
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		Some(selendra_chain_spec_properties()),
		Default::default(),
	))
}
