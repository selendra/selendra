use super::{
	account_id_from_string, calculate_initial_endowment, configure_chain_spec_fields, deduplicate,
	get_account_id_from_seed, selendra_chain_spec_properties, to_account_ids, AuthorityKeys,
	ChainSpec,
};

use pallet_staking::Forcing;
use sc_chain_spec::ChainType;
use sp_core::sr25519;
use sp_runtime::Perbill;

use selendra_primitives::{
	AccountId, Version as FinalityVersion, LEGACY_FINALITY_VERSION, MIN_NOMINATOR_BOND,
	MIN_VALIDATOR_BOND,
};
use selendra_runtime::{
	AuraConfig, BalancesConfig, ElectionsConfig, GenesisConfig, IndraConfig, SessionConfig,
	StakingConfig, SudoConfig, SystemConfig, VestingConfig, WASM_BINARY,
};

/// Generate chain spec for local runs.
/// Controller accounts are generated for the specified authorities.
pub fn testnet_config(authorities: Vec<AuthorityKeys>) -> Result<ChainSpec, String> {
	let controller_accounts: Vec<AccountId> = to_account_ids(&authorities)
		.into_iter()
		.enumerate()
		.map(|(index, _account)| {
			account_id_from_string(format!("//{}//Controller", index).as_str())
		})
		.collect();
	testnet_chain_spec_config(authorities, controller_accounts)
}

fn testnet_chain_spec_config(
	authorities: Vec<AuthorityKeys>,
	controller_accounts: Vec<AccountId>,
) -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;
	let sudo_account = get_account_id_from_seed::<sr25519::Public>("Alice");
	let faucet_account = get_account_id_from_seed::<sr25519::Public>("Alice");
	let min_validator_count = 1;
	let finality_version = LEGACY_FINALITY_VERSION as u32;

	Ok(ChainSpec::from_genesis(
		"Selendra Testnet",
		"selendra_testnet",
		ChainType::Development,
		move || {
			selendra_genesis(
				wasm_binary,
				authorities.clone(),          // Initial PoA authorities, will receive funds
				sudo_account.clone(),         // Sudo account, will also be pre funded
				Some(faucet_account.clone()), // Pre-funded faucet account
				controller_accounts.clone(),  // Controller accounts for staking.
				min_validator_count,
				finality_version,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		// Fork ID
		None,
		// Properties
		Some(selendra_chain_spec_properties()),
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn selendra_genesis(
	wasm_binary: &[u8],
	authorities: Vec<AuthorityKeys>,
	sudo_account: AccountId,
	endowed_accounts: Option<AccountId>,
	controller_accounts: Vec<AccountId>,
	min_validator_count: u32,
	finality_version: FinalityVersion,
) -> GenesisConfig {
	let special_accounts = match endowed_accounts {
		Some(endowed_id) => vec![sudo_account.clone(), endowed_id],
		None => vec![sudo_account.clone()],
	};

	// NOTE: some combinations of bootstrap chain arguments can potentially
	// lead to duplicated rich accounts, e.g. if a sudo account is also an authority
	// which is why we remove the duplicates if any here
	let unique_accounts =
		deduplicate(to_account_ids(&authorities).chain(special_accounts).collect());

	let endowment = calculate_initial_endowment(&unique_accounts);

	let unique_accounts_balances = unique_accounts
		.into_iter()
		.map(|account| (account, endowment))
		.collect::<Vec<_>>();

	let validator_count = authorities.len() as u32;
	let accounts_config =
		configure_chain_spec_fields(unique_accounts_balances, authorities, controller_accounts);

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: accounts_config.balances,
		},
		aura: AuraConfig { authorities: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(sudo_account),
		},
		elections: ElectionsConfig {
			reserved_validators: accounts_config.members.clone(),
			non_reserved_validators: vec![],
			committee_seats: Default::default(),
			committee_ban_config: Default::default(),
		},
		session: SessionConfig { keys: accounts_config.keys },
		staking: StakingConfig {
			force_era: Forcing::NotForcing,
			validator_count,
			// to satisfy some e2e tests as this cannot be changed during runtime
			minimum_validator_count: min_validator_count,
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: accounts_config.stakers,
			min_validator_bond: MIN_VALIDATOR_BOND,
			min_nominator_bond: MIN_NOMINATOR_BOND,
			..Default::default()
		},
		indra: IndraConfig { finality_version, ..Default::default() },
		treasury: Default::default(),
		vesting: VestingConfig { vesting: vec![] },
		nomination_pools: Default::default(),
		transaction_payment: Default::default(),
	}
}
