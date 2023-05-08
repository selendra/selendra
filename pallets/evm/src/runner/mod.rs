use crate::{BalanceOf, CallInfo, Config, CreateInfo};

use frame_support::dispatch::DispatchError;
use sp_core::{H160, H256};
use sp_std::vec::Vec;

use pallet_evm_utility::evm;
pub use primitives::evm::{EvmAddress, Vicinity};

pub mod stack;
pub mod state;
pub mod storage_meter;

pub trait Runner<T: Config> {
	fn call(
		source: H160,
		origin: H160,
		target: H160,
		input: Vec<u8>,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CallInfo, DispatchError>;

	fn create(
		source: H160,
		init: Vec<u8>,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CreateInfo, DispatchError>;

	fn create2(
		source: H160,
		init: Vec<u8>,
		salt: H256,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CreateInfo, DispatchError>;

	fn create_at_address(
		source: H160,
		address: H160,
		init: Vec<u8>,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CreateInfo, DispatchError>;
}

pub trait RunnerExtended<T: Config>: Runner<T> {
	fn rpc_call(
		source: H160,
		origin: H160,
		target: H160,
		input: Vec<u8>,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CallInfo, DispatchError>;

	fn rpc_create(
		source: H160,
		init: Vec<u8>,
		value: BalanceOf<T>,
		gas_limit: u64,
		storage_limit: u32,
		access_list: Vec<(H160, Vec<H256>)>,
		config: &evm::Config,
	) -> Result<CreateInfo, DispatchError>;
}
