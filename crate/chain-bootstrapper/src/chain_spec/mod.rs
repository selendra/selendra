mod cli;
mod commands;
mod constants;
mod keystore;

pub use commands::{BootstrapChainCmd, ConvertChainspecToRawCmd};

use std::{collections::BTreeMap, str::FromStr, string::ToString};
use serde_json::{Number, Value};

use sp_runtime::Perbill;
use sp_core::{H160, U256};

use selendra_primitives::{
	currency::TOKEN_DECIMALS,
	staking::{MIN_NOMINATOR_BOND, MIN_VALIDATOR_BOND},
	AccountId, AlephNodeSessionKeys, Version as FinalityVersion, ADDRESSES_ENCODING,
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
	let total_issuance = 300_000_000u128 * 10u128.pow(TOKEN_DECIMALS);
	// (A0-4258) due to known issue https://github.com/paritytech/polkadot-sdk/pull/2987/files,
	// we need to make sure returned number is in u64 range, otherwise serde_json::json macro fails
	// this is fixed in polkadot-sdk 1.6.0
	total_issuance / (accounts.len() as u128) / 10
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

	let evm_accounts = {
		let mut map = BTreeMap::new();
		map.insert(
			// H160 address of Alice dev account
			// Derived from SS58 (42 prefix) address
			// SS58: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
			// hex: 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
			// Using the full hex key, truncating to the first 20 bytes (the first 40 hex chars)
			H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558")
				.expect("internal H160 is valid; qed"),
			fp_evm::GenesisAccount {
				balance: U256::from_str("0xffffffffffffffffffffffffffffffff")
					.expect("internal U256 is valid; qed"),
				code: Default::default(),
				nonce: Default::default(),
				storage: Default::default(),
			},
		);
		map
	};

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
		"evmChainId": { "chainId": 1941 },
		"evm": { "accounts": evm_accounts },
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