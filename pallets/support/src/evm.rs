use codec::{Decode, Encode};

use frame_support::{
	transactional,
};
use sp_core::{H160, U256};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, MaybeSerializeDeserialize},
	DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::{
	cmp::{Eq, PartialEq},
	prelude::*,
};

use selendra_primitives::{
	evm::{CallInfo, EvmAddress}
};

/// Return true if the call of EVM precompile contract is allowed.
pub trait PrecompileCallerFilter {
	fn is_allowed(caller: H160) -> bool;
}

/// Return true if the EVM precompile is paused.
pub trait PrecompilePauseFilter {
	fn is_paused(address: H160) -> bool;
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug)]
pub enum ExecutionMode {
	Execute,
	/// Discard any state changes
	View,
	/// Also discard any state changes and use estimate gas mode for evm config
	EstimateGas,
}

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug)]
pub struct InvokeContext {
	pub contract: EvmAddress,
	/// similar to msg.sender
	pub sender: EvmAddress,
	/// similar to tx.origin
	pub origin: EvmAddress,
}

// pub trait TransactionPayment<AccountId, Balance, NegativeImbalance> {
// 	fn reserve_fee(
// 		who: &AccountId,
// 		fee: Balance,
// 		named: Option<ReserveIdentifier>,
// 	) -> Result<Balance, DispatchError>;
// 	fn unreserve_fee(who: &AccountId, fee: Balance, named: Option<ReserveIdentifier>) -> Balance;
// 	fn unreserve_and_charge_fee(
// 		who: &AccountId,
// 		weight: Weight,
// 	) -> Result<(Balance, NegativeImbalance), TransactionValidityError>;
// 	fn refund_fee(
// 		who: &AccountId,
// 		weight: Weight,
// 		payed: NegativeImbalance,
// 	) -> Result<(), TransactionValidityError>;
// 	fn charge_fee(
// 		who: &AccountId,
// 		len: u32,
// 		weight: Weight,
// 		tip: Balance,
// 		pays_fee: Pays,
// 		class: DispatchClass,
// 	) -> Result<(), TransactionValidityError>;
// 	fn weight_to_fee(weight: Weight) -> Balance;
// 	fn apply_multiplier_to_fee(fee: Balance, multiplier: Option<Multiplier>) -> Balance;
// }

pub trait TransferAll<AccountId> {
	fn transfer_all(source: &AccountId, dest: &AccountId) -> DispatchResult;
}

#[impl_trait_for_tuples::impl_for_tuples(5)]
impl<AccountId> TransferAll<AccountId> for Tuple {
	#[transactional]
	fn transfer_all(source: &AccountId, dest: &AccountId) -> DispatchResult {
		for_tuples!( #( {
			Tuple::transfer_all(source, dest)?;
		} )* );
		Ok(())
	}
}

/// An abstraction of EVMManager
pub trait EVMManager<AccountId, Balance> {
	/// Query the constants `NewContractExtraBytes` value from evm module.
	fn query_new_contract_extra_bytes() -> u32;
	/// Query the constants `StorageDepositPerByte` value from evm module.
	fn query_storage_deposit_per_byte() -> Balance;
	/// Query the maintainer address from the ERC20 contract.
	fn query_maintainer(contract: H160) -> Result<H160, DispatchError>;
	/// Query the constants `DeveloperDeposit` value from evm module.
	fn query_developer_deposit() -> Balance;
	/// Query the constants `PublicationFee` value from evm module.
	fn query_publication_fee() -> Balance;
	/// Transfer the maintainer of the contract address.
	fn transfer_maintainer(from: AccountId, contract: H160, new_maintainer: H160)
		-> DispatchResult;
	/// Publish contract
	fn publish_contract_precompile(who: AccountId, contract: H160) -> DispatchResult;
	/// Query the developer status of an account
	fn query_developer_status(who: AccountId) -> bool;
	/// Enable developer mode
	fn enable_account_contract_development(who: AccountId) -> DispatchResult;
	/// Disable developer mode
	fn disable_account_contract_development(who: AccountId) -> DispatchResult;
}

/// An abstraction of EVMAccountsManager
pub trait EVMAccountsManager<AccountId> {
	/// Returns the AccountId used to generate the given EvmAddress.
	fn get_account_id(address: &EvmAddress) -> AccountId;
	/// Returns the EvmAddress associated with a given AccountId or the underlying EvmAddress of the
	/// AccountId.
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	/// Claim account mapping between AccountId and a generated EvmAddress based off of the
	/// AccountId.
	fn claim_default_evm_address(account_id: &AccountId) -> Result<EvmAddress, DispatchError>;
}

/// A mapping between `AccountId` and `EvmAddress`.
pub trait AddressMapping<AccountId> {
	/// Returns the AccountId used go generate the given EvmAddress.
	fn get_account_id(evm: &EvmAddress) -> AccountId;
	/// Returns the EvmAddress associated with a given AccountId or the
	/// underlying EvmAddress of the AccountId.
	/// Returns None if there is no EvmAddress associated with the AccountId
	/// and there is no underlying EvmAddress in the AccountId.
	fn get_evm_address(account_id: &AccountId) -> Option<EvmAddress>;
	/// Returns the EVM address associated with an account ID and generates an
	/// account mapping if no association exists.
	fn get_or_create_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns the default EVM address associated with an account ID.
	fn get_default_evm_address(account_id: &AccountId) -> EvmAddress;
	/// Returns true if a given AccountId is associated with a given EvmAddress
	/// and false if is not.
	fn is_linked(account_id: &AccountId, evm: &EvmAddress) -> bool;
}

/// An abstraction of EVM for EVMBridge
pub trait EVM<AccountId> {
	type Balance: AtLeast32BitUnsigned + Copy + MaybeSerializeDeserialize + Default;

	fn execute(
		context: InvokeContext,
		input: Vec<u8>,
		value: Self::Balance,
		gas_limit: u64,
		storage_limit: u32,
		mode: ExecutionMode,
	) -> Result<CallInfo, sp_runtime::DispatchError>;

	/// Get the real origin account and charge storage rent from the origin.
	fn get_origin() -> Option<AccountId>;
	/// Set the EVM origin
	fn set_origin(origin: AccountId);
	/// Kill the EVM origin
	fn kill_origin();
	/// Get the real origin account or xcm origin and charge storage rent from the origin.
	fn get_real_origin() -> Option<AccountId>;
}

/// Convert any type that implements Into<U256> into byte representation ([u8, 32])
pub fn to_bytes<T: Into<U256>>(value: T) -> [u8; 32] {
	Into::<[u8; 32]>::into(value.into())
}

pub mod limits {
	pub struct Limit {
		pub gas: u64,
		pub storage: u32,
	}

	impl Limit {
		pub const fn new(gas: u64, storage: u32) -> Self {
			Self { gas, storage }
		}
	}

	pub mod erc20 {
		use super::*;

		pub const NAME: Limit = Limit::new(100_000, 0);
		pub const SYMBOL: Limit = Limit::new(100_000, 0);
		pub const DECIMALS: Limit = Limit::new(100_000, 0);
		pub const TOTAL_SUPPLY: Limit = Limit::new(100_000, 0);
		pub const BALANCE_OF: Limit = Limit::new(100_000, 0);
		pub const TRANSFER: Limit = Limit::new(200_000, 960);
	}
}
