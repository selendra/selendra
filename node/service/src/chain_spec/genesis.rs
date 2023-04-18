use super::{AuthorityKeys, configure_chain_spec_fields, deduplicate, to_account_ids, calculate_initial_endowment};

use sp_runtime::Perbill;
use pallet_staking::Forcing;

use selendra_primitives::{
	AccountId, Version as FinalityVersion,
	MIN_NOMINATOR_BOND, MIN_VALIDATOR_BOND,
};
use selendra_runtime::{
	AuraConfig, BalancesConfig, ElectionsConfig, GenesisConfig, IndraConfig, SessionConfig,
	StakingConfig, SudoConfig, SystemConfig, VestingConfig,
};

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
    let unique_accounts = deduplicate(
        to_account_ids(&authorities)
            .chain(special_accounts)
            .collect(),
    );

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
			balances:  accounts_config.balances,
		},
		aura: AuraConfig {
			authorities: vec![],
		},
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
		session: SessionConfig {
            keys: accounts_config.keys,
        },
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
		indra: IndraConfig {
            finality_version,
            ..Default::default()
        },
		treasury: Default::default(),
		vesting: VestingConfig { vesting: vec![] },
        nomination_pools: Default::default(),
		transaction_payment: Default::default(),
	}
}
