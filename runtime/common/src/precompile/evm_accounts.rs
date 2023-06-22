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

use super::{
	input::{Input, InputT, Output},
	target_gas_limit,
};
use crate::evm::WeightToGas;
use frame_support::{pallet_prelude::IsType, traits::Get};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use pallet_evm::{
	precompiles::Precompile,
	runner::state::{PrecompileFailure, PrecompileOutput, PrecompileResult},
	Context, ExitError, ExitRevert, ExitSucceed,
};
use pallet_evm_accounts::WeightInfo;
use pallets_support::EVMAccountsManager;
use sp_runtime::{traits::Convert, AccountId32, RuntimeDebug};
use sp_std::{marker::PhantomData, prelude::*};

/// The `EVMAccounts` impl precompile.
///
/// `input` data starts with `action`.
///
/// Actions:
/// - GetAccountId.
/// - GetEvmAddress.
pub struct EVMAccountsPrecompile<R>(PhantomData<R>);

#[pallet_evm_utility_macro::generate_function_selector]
#[derive(RuntimeDebug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	GetAccountId = "getAccountId(address)",
	GetEvmAddress = "getEvmAddress(bytes32)",
	ClaimDefaultEvmAddress = "claimDefaultEvmAddress(bytes32)",
}

impl<Runtime> Precompile for EVMAccountsPrecompile<Runtime>
where
	Runtime::AccountId: IsType<AccountId32>,
	Runtime: pallet_evm_accounts::Config,
	pallet_evm_accounts::Pallet<Runtime>: EVMAccountsManager<Runtime::AccountId>,
{
	fn execute(
		input: &[u8],
		target_gas: Option<u64>,
		_context: &Context,
		_is_static: bool,
	) -> PrecompileResult {
		let input = Input::<Action, Runtime::AccountId, Runtime::AddressMapping>::new(
			input,
			target_gas_limit(target_gas),
		);

		let gas_cost = Pricer::<Runtime>::cost(&input)?;

		if let Some(gas_limit) = target_gas {
			if gas_limit < gas_cost {
				return Err(PrecompileFailure::Error { exit_status: ExitError::OutOfGas })
			}
		}

		let action = input.action()?;

		match action {
			Action::GetAccountId => {
				let address = input.evm_address_at(1)?;

				let output = pallet_evm_accounts::Pallet::<Runtime>::get_account_id(&address);
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_fixed_bytes(output.into().as_ref()),
					logs: Default::default(),
				})
			},
			Action::GetEvmAddress => {
				// bytes32
				let input_data = input.bytes32_at(1)?;

				let mut buf = [0u8; 32];
				buf.copy_from_slice(&input_data[..]);
				let account_id: Runtime::AccountId = AccountId32::from(buf).into();

				// If it does not exist, return address(0x0). Keep the behavior the same as mapping[key]
				let address = pallet_evm_accounts::Pallet::<Runtime>::get_evm_address(&account_id)
					.unwrap_or_default();

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_address(address),
					logs: Default::default(),
				})
			},
			Action::ClaimDefaultEvmAddress => {
				// bytes32
				let input_data = input.bytes32_at(1)?;

				let mut buf = [0u8; 32];
				buf.copy_from_slice(&input_data[..]);
				let account_id: Runtime::AccountId = AccountId32::from(buf).into();

				let address =
					pallet_evm_accounts::Pallet::<Runtime>::claim_default_evm_address(&account_id)
						.map_err(|e| PrecompileFailure::Revert {
							exit_status: ExitRevert::Reverted,
							output: Output::encode_error_msg(
								"EvmAccounts ClaimDefaultEvmAddress failed",
								e,
							),
							cost: target_gas_limit(target_gas).unwrap_or_default(),
						})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_address(address),
					logs: Default::default(),
				})
			},
		}
	}
}

struct Pricer<R>(PhantomData<R>);

impl<Runtime> Pricer<Runtime>
where
	Runtime: pallet_evm_accounts::Config,
{
	const BASE_COST: u64 = 200;

	fn cost(
		input: &Input<Action, Runtime::AccountId, Runtime::AddressMapping>,
	) -> Result<u64, PrecompileFailure> {
		let action = input.action()?;
		let cost = match action {
			Action::GetAccountId => {
				// EVMAccounts::Accounts (r: 1)
				WeightToGas::convert(<Runtime as frame_system::Config>::DbWeight::get().reads(1))
			},
			Action::GetEvmAddress => {
				// EVMAccounts::EvmAddresses (r: 1)
				WeightToGas::convert(<Runtime as frame_system::Config>::DbWeight::get().reads(1))
			},
			Action::ClaimDefaultEvmAddress => {
				// claim_default_account weight
				let weight =
					<Runtime as pallet_evm_accounts::Config>::WeightInfo::claim_default_account();

				WeightToGas::convert(weight)
			},
		};
		Ok(Self::BASE_COST.saturating_add(cost))
	}
}
