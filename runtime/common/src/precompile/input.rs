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

use frame_support::ensure;
use sp_std::{marker::PhantomData, result::Result, vec::Vec};

use crate::evm::WeightToGas;
use ethabi::Token;
use frame_support::traits::Get;
use pallet_evm::{runner::state::PrecompileFailure, ExitRevert};
use pallets_support::{AddressMapping as AddressMappingT};
use selendra_primitives::{Balance};
use sp_core::{H160, U256};
use sp_runtime::{traits::Convert, DispatchError};
use sp_std::prelude::*;

pub const FUNCTION_SELECTOR_LENGTH: usize = 4;
pub const PER_PARAM_BYTES: usize = 32;
pub const HALF_PARAM_BYTES: usize = PER_PARAM_BYTES / 2;
pub const ACTION_INDEX: usize = 0;

pub trait InputT {
	type Error;
	type Action;
	type AccountId;

	fn nth_param(&self, n: usize, len: Option<usize>) -> Result<&[u8], Self::Error>;
	fn action(&self) -> Result<Self::Action, Self::Error>;

	fn account_id_at(&self, index: usize) -> Result<Self::AccountId, Self::Error>;
	fn evm_address_at(&self, index: usize) -> Result<H160, Self::Error>;

	fn i128_at(&self, index: usize) -> Result<i128, Self::Error>;
	fn u256_at(&self, index: usize) -> Result<U256, Self::Error>;

	fn balance_at(&self, index: usize) -> Result<Balance, Self::Error>;

	fn u64_at(&self, index: usize) -> Result<u64, Self::Error>;
	fn u32_at(&self, index: usize) -> Result<u32, Self::Error>;

	fn bytes_at(&self, start: usize) -> Result<Vec<u8>, Self::Error>;
	fn bytes32_at(&self, start: usize) -> Result<Vec<u8>, Self::Error>;
	fn bool_at(&self, index: usize) -> Result<bool, Self::Error>;
}

pub struct Input<'a, Action, AccountId, AddressMapping> {
	content: &'a [u8],
	target_gas: Option<u64>,
	_marker: PhantomData<(Action, AccountId, AddressMapping)>,
}
impl<'a, Action, AccountId, AddressMapping>
	Input<'a, Action, AccountId, AddressMapping>
{
	pub fn new(content: &'a [u8], target_gas: Option<u64>) -> Self {
		Self {
			content,
			target_gas,
			_marker: PhantomData,
		}
	}
}

impl<Action, AccountId, AddressMapping> InputT
	for Input<'_, Action, AccountId, AddressMapping>
where
	Action: TryFrom<u32>,
	AddressMapping: AddressMappingT<AccountId>,
{
	type Error = PrecompileFailure;
	type Action = Action;
	type AccountId = AccountId;

	fn nth_param(&self, n: usize, len: Option<usize>) -> Result<&[u8], Self::Error> {
		let (start, end) = if n == 0 {
			// ACTION_INDEX
			let start = 0;
			let end = start + FUNCTION_SELECTOR_LENGTH;
			(start, end)
		} else {
			let start = FUNCTION_SELECTOR_LENGTH + PER_PARAM_BYTES * (n - 1);
			let end = start + len.unwrap_or(PER_PARAM_BYTES);
			(start, end)
		};

		ensure!(
			end <= self.content.len(),
			PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: "invalid input".into(),
				cost: self.target_gas.unwrap_or_default(),
			}
		);

		Ok(&self.content[start..end])
	}

	fn action(&self) -> Result<Self::Action, Self::Error> {
		let param = self.nth_param(ACTION_INDEX, None)?;
		let action = u32::from_be_bytes(param.try_into().map_err(|_| PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "invalid action".into(),
			cost: self.target_gas.unwrap_or_default(),
		})?);

		action.try_into().map_err(|_| PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "invalid action".into(),
			cost: self.target_gas.unwrap_or_default(),
		})
	}

	fn account_id_at(&self, index: usize) -> Result<Self::AccountId, Self::Error> {
		let param = self.nth_param(index, None)?;

		let mut address = [0u8; 20];
		address.copy_from_slice(&param[12..]);

		Ok(AddressMapping::get_account_id(&address.into()))
	}

	fn evm_address_at(&self, index: usize) -> Result<H160, Self::Error> {
		let param = self.nth_param(index, None)?;

		let mut address = [0u8; 20];
		address.copy_from_slice(&param[12..]);

		Ok(H160::from_slice(&address))
	}

	fn i128_at(&self, index: usize) -> Result<i128, Self::Error> {
		let param = self.nth_param(index, None)?;
		decode_i128(param).ok_or(PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "failed to decode i128".into(),
			cost: self.target_gas.unwrap_or_default(),
		})
	}

	fn u256_at(&self, index: usize) -> Result<U256, Self::Error> {
		let param = self.nth_param(index, None)?;
		Ok(U256::from_big_endian(param))
	}

	fn balance_at(&self, index: usize) -> Result<Balance, Self::Error> {
		let param = self.u256_at(index)?;
		param.try_into().map_err(|_| PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "failed to convert uint256 into Balance".into(),
			cost: self.target_gas.unwrap_or_default(),
		})
	}

	fn u64_at(&self, index: usize) -> Result<u64, Self::Error> {
		let param = self.u256_at(index)?;
		param.try_into().map_err(|_| PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "failed to convert uint256 into u64".into(),
			cost: self.target_gas.unwrap_or_default(),
		})
	}

	fn u32_at(&self, index: usize) -> Result<u32, Self::Error> {
		let param = self.u256_at(index)?;
		param.try_into().map_err(|_| PrecompileFailure::Revert {
			exit_status: ExitRevert::Reverted,
			output: "failed to convert uint256 into u32".into(),
			cost: self.target_gas.unwrap_or_default(),
		})
	}

	fn bytes_at(&self, index: usize) -> Result<Vec<u8>, Self::Error> {
		let offset = self.u32_at(index)?;
		let data_index = (offset as usize).saturating_div(PER_PARAM_BYTES).saturating_add(1);

		let bytes_len = self.u32_at(data_index)?;
		let bytes = self.nth_param(data_index.saturating_add(1), Some(bytes_len as usize))?;

		Ok(bytes.to_vec())
	}

	fn bytes32_at(&self, index: usize) -> Result<Vec<u8>, Self::Error> {
		let bytes = self.nth_param(index, Some(32))?;

		Ok(bytes.to_vec())
	}

	fn bool_at(&self, index: usize) -> Result<bool, Self::Error> {
		const ONE: U256 = U256([1u64, 0, 0, 0]);
		let param = self.u256_at(index)?;
		if param == ONE {
			Ok(true)
		} else if param.is_zero() {
			Ok(false)
		} else {
			Err(PrecompileFailure::Revert {
				exit_status: ExitRevert::Reverted,
				output: "failed to decode bool".into(),
				cost: self.target_gas.unwrap_or_default(),
			})
		}
	}
}

pub struct Output;

impl Output {
	pub fn encode_bool(b: bool) -> Vec<u8> {
		ethabi::encode(&[Token::Bool(b)])
	}

	pub fn encode_uint<T>(b: T) -> Vec<u8>
	where
		U256: From<T>,
	{
		ethabi::encode(&[Token::Uint(U256::from(b))])
	}

	pub fn encode_uint_tuple<T>(b: Vec<T>) -> Vec<u8>
	where
		U256: From<T>,
	{
		ethabi::encode(&[Token::Tuple(b.into_iter().map(U256::from).map(Token::Uint).collect())])
	}

	pub fn encode_uint_array<T>(b: Vec<T>) -> Vec<u8>
	where
		U256: From<T>,
	{
		ethabi::encode(&[Token::Array(b.into_iter().map(U256::from).map(Token::Uint).collect())])
	}

	pub fn encode_bytes(b: &[u8]) -> Vec<u8> {
		ethabi::encode(&[Token::Bytes(b.to_vec())])
	}

	pub fn encode_bytes_tuple(b: Vec<&[u8]>) -> Vec<u8> {
		ethabi::encode(&[Token::Tuple(b.into_iter().map(|v| Token::Bytes(v.to_vec())).collect())])
	}

	pub fn encode_fixed_bytes(b: &[u8]) -> Vec<u8> {
		ethabi::encode(&[Token::FixedBytes(b.to_vec())])
	}

	pub fn encode_address(b: H160) -> Vec<u8> {
		ethabi::encode(&[Token::Address(b)])
	}

	pub fn encode_address_tuple(b: Vec<H160>) -> Vec<u8> {
		ethabi::encode(&[Token::Tuple(b.into_iter().map(Token::Address).collect())])
	}

	pub fn encode_address_array(b: Vec<H160>) -> Vec<u8> {
		ethabi::encode(&[Token::Array(b.into_iter().map(Token::Address).collect())])
	}

	pub fn encode_error_msg(info: &str, err: DispatchError) -> Vec<u8> {
		let mut msg = Vec::new();
		msg.extend_from_slice(info.as_bytes());
		msg.extend_from_slice(": ".as_bytes());
		msg.extend_from_slice(Into::<&str>::into(err).as_bytes());
		msg
	}
}

pub struct InputPricer<T>(PhantomData<T>);

impl<T> InputPricer<T>
where
	T: frame_system::Config,
{
	pub(crate) fn read_accounts(count: u64) -> u64 {
		// EvmAccounts::Accounts
		WeightToGas::convert(T::DbWeight::get().reads(count))
	}
}

fn decode_i128(bytes: &[u8]) -> Option<i128> {
	if bytes[0..HALF_PARAM_BYTES] == [0xff; HALF_PARAM_BYTES] {
		if let Ok(v) = i128::try_from(!U256::from(bytes)) {
			if let Some(v) = v.checked_neg() {
				return v.checked_sub(1);
			}
		}
		return None;
	} else if bytes[0..HALF_PARAM_BYTES] == [0x00; HALF_PARAM_BYTES] {
		return i128::try_from(U256::from_big_endian(bytes)).ok();
	}
	None
}
