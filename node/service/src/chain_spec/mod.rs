pub mod genesis;

use libp2p::PeerId;
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashSet, str::FromStr};

use pallet_staking::StakerStatus;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};

use selendra_primitives::{
	AccountId, AccountPublic, AuthorityId as SelendraId, Balance, IdentifyAccount,
	MIN_VALIDATOR_BOND, TOKEN,
};
use selendra_runtime::{GenesisConfig, SessionKeys};

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

// total issuance of 300M (for devnet/tests/local runs only)
const TOTAL_ISSUANCE: Balance = 300_000_000 * TOKEN;

pub const DEFAULT_BACKUP_FOLDER: &str = "backup-stash";

/// Calculate initial endowments such that total issuance is kept approximately constant.
fn calculate_initial_endowment(accounts: &[AccountId]) -> Balance {
	TOTAL_ISSUANCE / (accounts.len() as Balance)
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

fn to_account_ids(authorities: &[AuthorityKeys]) -> impl Iterator<Item = AccountId> + '_ {
	authorities.iter().map(|auth| auth.account_id.clone())
}

/// Given a Vec<AccountIds> returns a unique collection
fn deduplicate(accounts: Vec<AccountId>) -> Vec<AccountId> {
	let set: HashSet<_> = accounts.into_iter().collect();
	set.into_iter().collect()
}

/// Generate an account ID from seed.
pub fn account_id_from_string(seed: &str) -> AccountId {
	AccountId::from(
		sr25519::Pair::from_string(seed, None)
			.expect("Can't create pair from seed value")
			.public(),
	)
}

/// Returns the properties for the [`SelendraChainSpec`].
pub fn selendra_chain_spec_properties() -> serde_json::map::Map<String, serde_json::Value> {
	serde_json::json!({
		"tokenSymbol": "SEL",
		"tokenDecimals": 18,
	})
	.as_object()
	.expect("Map given; qed")
	.clone()
}

#[derive(Clone)]
pub struct SerializablePeerId {
	inner: PeerId,
}

impl SerializablePeerId {
	pub fn new(inner: PeerId) -> SerializablePeerId {
		SerializablePeerId { inner }
	}
}

impl Serialize for SerializablePeerId {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let s: String = format!("{}", self.inner);
		serializer.serialize_str(&s)
	}
}

impl<'de> Deserialize<'de> for SerializablePeerId {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let inner = PeerId::from_str(&s)
			.map_err(|_| D::Error::custom(format!("Could not deserialize as PeerId: {}", s)))?;
		Ok(SerializablePeerId { inner })
	}
}

#[derive(Clone, Deserialize, Serialize)]
pub struct AuthorityKeys {
	pub account_id: AccountId,
	pub aura_key: AuraId,
	pub selendra_key: SelendraId,
	pub peer_id: SerializablePeerId,
}

/// Provides configuration for staking by defining balances, members, keys and stakers.
struct AccountsConfig {
	balances: Vec<(AccountId, Balance)>,
	members: Vec<AccountId>,
	keys: Vec<(AccountId, AccountId, SessionKeys)>,
	stakers: Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
}

/// Provides accounts for GenesisConfig setup based on distinct staking accounts.
/// Assumes validator == stash, but controller is a distinct account
fn configure_chain_spec_fields(
	unique_accounts_balances: Vec<(AccountId, Balance)>,
	authorities: Vec<AuthorityKeys>,
	controllers: Vec<AccountId>,
) -> AccountsConfig {
	let balances = unique_accounts_balances
		.into_iter()
		.chain(controllers.clone().into_iter().map(|account| (account, TOKEN)))
		.collect();

	let keys = authorities
		.iter()
		.map(|auth| {
			(
				auth.account_id.clone(),
				auth.account_id.clone(),
				SessionKeys { aura: auth.aura_key.clone(), indra: auth.selendra_key.clone() },
			)
		})
		.collect();

	let stakers = authorities
		.iter()
		.zip(controllers)
		.enumerate()
		.map(|(validator_idx, (validator, controller))| {
			(
				validator.account_id.clone(),
				controller,
				(validator_idx + 1) as Balance * MIN_VALIDATOR_BOND,
				StakerStatus::Validator,
			)
		})
		.collect();

	let members = to_account_ids(&authorities).collect();

	AccountsConfig { balances, members, keys, stakers }
}
