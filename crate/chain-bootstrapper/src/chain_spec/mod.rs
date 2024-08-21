mod cli;
mod commands;
mod constants;
mod keystore;

pub use commands::{BootstrapChainCmd, ConvertChainspecToRawCmd};

use serde_json::{Number, Value};
use std::string::ToString;

use sp_runtime::Perbill;

use selendra_primitives::{
	currency::TOKEN_DECIMALS,
	staking::{MIN_NOMINATOR_BOND, MIN_VALIDATOR_BOND},
	AccountId, AlephNodeSessionKeys, Balance, Version as FinalityVersion, ADDRESSES_ENCODING,
};
use selendra_runtime::WASM_BINARY;

use crate::chain_spec::{
	cli::ChainSpecParams, constants::SelendraNodeChainSpec, keystore::AccountSessionKeys,
};
use pallet_staking::{Forcing, StakerStatus};

fn to_account_ids(authorities: &[AccountSessionKeys]) -> impl Iterator<Item = AccountId> + '_ {
	authorities.iter().map(|auth| auth.account_id.clone())
}

fn system_properties(token_symbol: String) -> serde_json::map::Map<String, Value> {
	[
		("tokenSymbol".to_string(), Value::String(token_symbol)),
		("tokenDecimals".to_string(), Value::Number(Number::from(TOKEN_DECIMALS))),
		("ss58Format".to_string(), Value::Number(Number::from(ADDRESSES_ENCODING))),
	]
	.iter()
	.cloned()
	.collect()
}

/// Generate chain spec for new AlephNode chains
pub fn build_chain_spec(
	chain_params: &ChainSpecParams,
	account_session_keys: Vec<AccountSessionKeys>,
) -> Result<SelendraNodeChainSpec, String> {
	let token_symbol = String::from(chain_params.token_symbol());
	let sudo_account = chain_params.sudo_account_id();
	let rich_accounts = chain_params.rich_account_ids();
	let finality_version = chain_params.finality_version();

	Ok(SelendraNodeChainSpec::builder(
		WASM_BINARY.ok_or("AlephNode development wasm not available")?,
		Default::default(),
	)
	.with_name(chain_params.chain_name())
	.with_id(chain_params.chain_id())
	.with_chain_type(chain_params.chain_type())
	.with_genesis_config_patch(generate_genesis_config(
		account_session_keys,
		sudo_account,
		rich_accounts,
		finality_version,
	))
	.with_properties(system_properties(token_symbol))
	.build())
}

/// Calculate initial endowments such that total issuance is kept approximately constant.
fn calculate_initial_endowment(accounts: &[AccountId]) -> u128 {
	let total_issuance: Balance = 227_887_547u128 * 10u128.pow(TOKEN_DECIMALS);
	total_issuance / (accounts.len() as u128)
}

/// Configure initial storage state for FRAME modules.
fn generate_genesis_config(
	account_session_keys: Vec<AccountSessionKeys>,
	sudo_account: AccountId,
	rich_accounts: Option<Vec<AccountId>>,
	finality_version: FinalityVersion,
) -> serde_json::Value {
	let mut endowed_accounts = to_account_ids(&account_session_keys)
		.chain(rich_accounts.unwrap_or_default().into_iter().chain([sudo_account.clone()]))
		.collect::<Vec<_>>();
	endowed_accounts.sort();
	endowed_accounts.dedup();

	let initial_endowement = calculate_initial_endowment(&endowed_accounts);

	serde_json::json!({
		"balances": {
			"balances": endowed_accounts
						.into_iter()
						.map(|account| (account, initial_endowement))
						.collect::<Vec<_>>(),
		},
		"sudo": {
			"key": Some(sudo_account),
		},
		"elections": {
			"reservedValidators": to_account_ids(&account_session_keys).collect::<Vec<_>>(),
		},
		"session": {
		   "keys": account_session_keys
					.iter()
					.map(|auth| {
						(
							auth.account_id.clone(),
							auth.account_id.clone(),
							AlephNodeSessionKeys {
								aura: auth.aura_key.clone(),
								aleph: auth.aleph_key.clone(),
							},
						)
					})
					.collect::<Vec<_>>(),
		},
		"staking": {
			"forceEra": Forcing::NotForcing,
			"validatorCount":  account_session_keys.len() as u32,
			"minimumValidatorCount": 4,
			"slashRewardFraction": Perbill::from_percent(10),
			"stakers": account_session_keys
						.iter()
						.enumerate()
						.map(|(validator_idx, validator)| {
							(
								validator.account_id.clone(),
								// this is controller account but in Substrate 1.0.0, it is omitted anyway,
								// so it does not matter what we pass in the below line as always stash == controller
								validator.account_id.clone(),
								(validator_idx + 1) as u128 * MIN_VALIDATOR_BOND,
								StakerStatus::<AccountId>::Validator,
							)
						})
						.collect::<Vec<_>>(),
			"minValidatorBond": MIN_VALIDATOR_BOND,
			"minNominatorBond": MIN_NOMINATOR_BOND,
		},
		"aleph": {
			"finalityVersion": finality_version,
		},
		"committeeManagement": {
			"sessionValidators": {
				"committee": to_account_ids(&account_session_keys).collect::<Vec<_>>(),
			},
		},
		"evmChainId": { "chainId": 1961 }, // Mainnet 1961 - Testnet 1953
	})
}

pub fn build_chain_spec_json(
	is_raw_chainspec: bool,
	chain_params: &ChainSpecParams,
	account_session_keys: Vec<AccountSessionKeys>,
) -> sc_service::error::Result<String> {
	let chain_spec = build_chain_spec(chain_params, account_session_keys)?;
	sc_service::chain_ops::build_spec(&chain_spec, is_raw_chainspec)
}
