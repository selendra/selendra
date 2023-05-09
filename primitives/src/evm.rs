use crate::Balance;

use core::ops::Range;
use hex_literal::hex;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

pub use ethereum::{AccessListItem, Log};
use evm::ExitReason;

use sp_core::{H160, U256};
use sp_runtime::{traits::Zero, RuntimeDebug};
use sp_std::prelude::*;

// GAS MASK
const GAS_MASK: u64 = 100_000u64;
// STORAGE MASK
const STORAGE_MASK: u64 = 100u64;
// GAS LIMIT CHUNK
const GAS_LIMIT_CHUNK: u64 = 30_000u64;

pub const MIRRORED_NFT_ADDRESS_START: u64 = 0x2000000;
pub const MIRRORED_TOKENS_ADDRESS_START: EvmAddress =
	H160(hex!("0000000000000000000100000000000000000000"));

/// System contract address prefix
pub const SYSTEM_CONTRACT_ADDRESS_PREFIX: [u8; 9] = [0u8; 9];

pub const H160_POSITION_CURRENCY_ID_TYPE: usize = 9;
pub const H160_POSITION_TOKEN: usize = 19;
pub const H160_POSITION_TOKEN_NFT: Range<usize> = 16..20;

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	MaxEncodedLen,
	TypeInfo,
)]
#[repr(u8)]
pub enum ReserveIdentifier {
	EvmStorageDeposit,
	EvmDeveloperDeposit,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// External input from the transaction.
pub struct Vicinity {
	/// Current transaction gas price.
	pub gas_price: U256,
	/// Origin of the transaction.
	pub origin: EvmAddress,
	/// Environmental coinbase.
	pub block_coinbase: Option<EvmAddress>,
	/// Environmental block gas limit. Used only for testing
	pub block_gas_limit: Option<U256>,
	/// Environmental block difficulty. Used only for testing
	pub block_difficulty: Option<U256>,
	/// Environmental base fee per gas.
	pub block_base_fee_per_gas: Option<U256>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BlockLimits {
	/// Max gas limit
	pub max_gas_limit: u64,
	/// Max storage limit
	pub max_storage_limit: u32,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct EstimateResourcesRequest {
	/// From
	pub from: Option<H160>,
	/// To
	pub to: Option<H160>,
	/// Gas Limit
	pub gas_limit: Option<u64>,
	/// Storage Limit
	pub storage_limit: Option<u32>,
	/// Value
	pub value: Option<Balance>,
	/// Data
	pub data: Option<Vec<u8>>,
	/// AccessList
	pub access_list: Option<Vec<AccessListItem>>,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ExecutionInfo<T> {
	pub exit_reason: ExitReason,
	pub value: T,
	pub used_gas: U256,
	pub used_storage: i32,
	pub logs: Vec<Log>,
}

pub type CallInfo = ExecutionInfo<Vec<u8>>;
pub type CreateInfo = ExecutionInfo<H160>;

/// Evm Address.
pub type EvmAddress = sp_core::H160;

pub fn decode_gas_limit(gas_limit: u64) -> (u64, u32) {
	let gas_and_storage: u64 = gas_limit.checked_rem(GAS_MASK).expect("constant never failed; qed");
	let actual_gas_limit: u64 = gas_and_storage
		.checked_div(STORAGE_MASK)
		.expect("constant never failed; qed")
		.saturating_mul(GAS_LIMIT_CHUNK);
	let storage_limit_number: u32 = gas_and_storage
		.checked_rem(STORAGE_MASK)
		.expect("constant never failed; qed")
		.try_into()
		.expect("STORAGE_MASK is 100, the result maximum is 99; qed");

	let actual_storage_limit = if storage_limit_number.is_zero() {
		Default::default()
	} else {
		2u32.saturating_pow(storage_limit_number)
	};

	(actual_gas_limit, actual_storage_limit)
}
