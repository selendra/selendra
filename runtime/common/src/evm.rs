use super::{
	dollar, gas_to_weight_ratio, is_system_contract, BTreeMap, Balance, Bytes, Convert,
	DispatchClass, GenesisAccount, Get, Nonce, PhantomData, PrecompileCallerFilter, Weight, H160,
	SEL,
};

#[cfg(feature = "std")]
pub use sp_core::bytes::from_hex;
#[cfg(feature = "std")]
pub use std::str::FromStr;

/// The call is allowed only if caller is a system contract.
pub struct SystemContractsFilter;
impl PrecompileCallerFilter for SystemContractsFilter {
	fn is_allowed(caller: H160) -> bool {
		is_system_contract(caller)
	}
}

/// Convert gas to weight
pub struct GasToWeight;
impl Convert<u64, Weight> for GasToWeight {
	fn convert(gas: u64) -> Weight {
		gas.saturating_mul(gas_to_weight_ratio::RATIO)
	}
}

/// Convert weight to gas
pub struct WeightToGas;
impl Convert<Weight, u64> for WeightToGas {
	fn convert(weight: Weight) -> u64 {
		weight
			.checked_div(gas_to_weight_ratio::RATIO)
			.expect("Compile-time constant is not zero; qed;")
	}
}

pub struct EvmLimits<T>(PhantomData<T>);
impl<T> EvmLimits<T>
where
	T: frame_system::Config,
{
	pub fn max_gas_limit() -> u64 {
		let weights = T::BlockWeights::get();
		let normal_weight = weights.get(DispatchClass::Normal);
		WeightToGas::convert(normal_weight.max_extrinsic.unwrap_or(weights.max_block))
	}

	pub fn max_storage_limit() -> u32 {
		let length = T::BlockLength::get();
		*length.max.get(DispatchClass::Normal)
	}
}

#[cfg(feature = "std")]
/// Returns `evm_genesis_accounts`
pub fn evm_genesis(evm_accounts: Vec<H160>) -> BTreeMap<H160, GenesisAccount<Balance, Nonce>> {
	let contracts_json =
		&include_bytes!("../../../predeploy-contracts/resources/bytecodes.json")[..];
	let contracts: Vec<(String, String, String)> = serde_json::from_slice(contracts_json).unwrap();
	let mut accounts = BTreeMap::new();
	for (_, address, code_string) in contracts {
		let account = GenesisAccount {
			nonce: 0u32,
			balance: 0u128,
			storage: BTreeMap::new(),
			code: Bytes::from_str(&code_string).unwrap().0,
			enable_contract_development: false,
		};

		let addr = H160::from_slice(
			from_hex(address.as_str())
				.expect("predeploy-contracts must specify address")
				.as_slice(),
		);
		accounts.insert(addr, account);
	}

	for dev_acc in evm_accounts {
		let account = GenesisAccount {
			nonce: 0u32,
			balance: 1000 * dollar(SEL),
			storage: BTreeMap::new(),
			code: vec![],
			enable_contract_development: true,
		};
		accounts.insert(dev_acc, account);
	}

	accounts
}

#[cfg(test)]
mod tests {
	use super::*;
	use primitives::evm::SYSTEM_CONTRACT_ADDRESS_PREFIX;

	#[test]
	fn system_contracts_filter_works() {
		assert!(SystemContractsFilter::is_allowed(H160::from_low_u64_be(1)));

		let mut max_allowed_addr = [0u8; 20];
		max_allowed_addr[SYSTEM_CONTRACT_ADDRESS_PREFIX.len()] = 127u8;
		assert!(SystemContractsFilter::is_allowed(max_allowed_addr.into()));

		let mut min_blocked_addr = [0u8; 20];
		min_blocked_addr[SYSTEM_CONTRACT_ADDRESS_PREFIX.len() - 1] = 1u8;
		assert!(!SystemContractsFilter::is_allowed(min_blocked_addr.into()));
	}
}
