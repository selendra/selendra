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
	input::{Input, InputPricer, InputT, Output},
	target_gas_limit,
	weights::PrecompileWeights,
};
use crate::evm::WeightToGas;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use pallet_evm::{
	precompiles::Precompile,
	runner::state::{PrecompileFailure, PrecompileOutput, PrecompileResult},
	Context, ExitError, ExitRevert, ExitSucceed, WeightInfo,
};
use pallets_support::EVMManager;
use selendra_primitives::Balance;
use sp_runtime::{traits::Convert, RuntimeDebug};
use sp_std::{marker::PhantomData, prelude::*};

/// The `EVM` impl precompile.
///
/// `input` data starts with `action`.
///
/// Actions:
/// - QueryNewContractExtraBytes.
/// - QueryStorageDepositPerByte.
/// - QueryMaintainer.
/// - QueryDeveloperDeposit.
/// - QueryPublicationFee.
/// - TransferMaintainer. Rest `input` bytes: `from`, `contract`, `new_maintainer`.
pub struct EVMPrecompile<R>(PhantomData<R>);

#[pallet_evm_utility_macro::generate_function_selector]
#[derive(RuntimeDebug, Eq, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum Action {
	QueryNewContractExtraBytes = "newContractExtraBytes()",
	QueryStorageDepositPerByte = "storageDepositPerByte()",
	QueryMaintainer = "maintainerOf(address)",
	QueryDeveloperDeposit = "developerDeposit()",
	QueryPublicationFee = "publicationFee()",
	TransferMaintainer = "transferMaintainer(address,address,address)",
	EnableDeveloperAccount = "developerEnable(address)",
	DisableDeveloperAccount = "developerDisable(address)",
	QueryDeveloperStatus = "developerStatus(address)",
	PublishContract = "publishContract(address,address)",
}

impl<Runtime> Precompile for EVMPrecompile<Runtime>
where
	Runtime: pallet_evm::Config,
	pallet_evm::Pallet<Runtime>: EVMManager<Runtime::AccountId, Balance>,
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
			Action::QueryNewContractExtraBytes => {
				let output = pallet_evm::Pallet::<Runtime>::query_new_contract_extra_bytes();
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_uint(output),
					logs: Default::default(),
				})
			},
			Action::QueryStorageDepositPerByte => {
				let deposit = pallet_evm::Pallet::<Runtime>::query_storage_deposit_per_byte();
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_uint(deposit),
					logs: Default::default(),
				})
			},
			Action::QueryMaintainer => {
				let contract = input.evm_address_at(1)?;

				let maintainer = pallet_evm::Pallet::<Runtime>::query_maintainer(contract)
					.map_err(|e| PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: Into::<&str>::into(e).as_bytes().to_vec(),
						cost: target_gas_limit(target_gas).unwrap_or_default(),
					})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_address(maintainer),
					logs: Default::default(),
				})
			},
			Action::QueryDeveloperDeposit => {
				let deposit = pallet_evm::Pallet::<Runtime>::query_developer_deposit();
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_uint(deposit),
					logs: Default::default(),
				})
			},
			Action::QueryPublicationFee => {
				let fee = pallet_evm::Pallet::<Runtime>::query_publication_fee();
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_uint(fee),
					logs: Default::default(),
				})
			},
			Action::TransferMaintainer => {
				let from = input.account_id_at(1)?;
				let contract = input.evm_address_at(2)?;
				let new_maintainer = input.evm_address_at(3)?;

				frame_support::log::debug!(
					target: "evm",
					"evm: from: {:?}, contract: {:?}, new_maintainer: {:?}",
					from, contract, new_maintainer,
				);

				<pallet_evm::Pallet<Runtime> as EVMManager<Runtime::AccountId, Balance>>::transfer_maintainer(
					from,
					contract,
					new_maintainer,
				)
				.map_err(|e| PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: Output::encode_error_msg("Evm TransferMaintainer failed", e),
					cost: target_gas_limit(target_gas).unwrap_or_default(),
				})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: vec![],
					logs: Default::default(),
				})
			},
			Action::PublishContract => {
				let who = input.account_id_at(1)?;
				let contract_address = input.evm_address_at(2)?;
				<pallet_evm::Pallet<Runtime>>::publish_contract_precompile(who, contract_address)
					.map_err(|e| PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: Output::encode_error_msg("Evm PublishContract failed", e),
					cost: target_gas_limit(target_gas).unwrap_or_default(),
				})?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: vec![],
					logs: Default::default(),
				})
			},
			Action::DisableDeveloperAccount => {
				let who = input.account_id_at(1)?;
				<pallet_evm::Pallet<Runtime>>::disable_account_contract_development(who).map_err(
					|e| PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: Output::encode_error_msg("Evm DisableDeveloperAccount failed", e),
						cost: target_gas_limit(target_gas).unwrap_or_default(),
					},
				)?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: vec![],
					logs: Default::default(),
				})
			},
			Action::EnableDeveloperAccount => {
				let who = input.account_id_at(1)?;
				<pallet_evm::Pallet<Runtime>>::enable_account_contract_development(who).map_err(
					|e| PrecompileFailure::Revert {
						exit_status: ExitRevert::Reverted,
						output: Output::encode_error_msg("Evm EnableDeveloperAccount failed", e),
						cost: target_gas_limit(target_gas).unwrap_or_default(),
					},
				)?;

				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: vec![],
					logs: Default::default(),
				})
			},
			Action::QueryDeveloperStatus => {
				let who = input.account_id_at(1)?;
				let developer_status = <pallet_evm::Pallet<Runtime>>::query_developer_status(who);
				Ok(PrecompileOutput {
					exit_status: ExitSucceed::Returned,
					cost: gas_cost,
					output: Output::encode_bool(developer_status),
					logs: Default::default(),
				})
			},
		}
	}
}

struct Pricer<R>(PhantomData<R>);

impl<Runtime> Pricer<Runtime>
where
	Runtime: pallet_evm::Config,
{
	const BASE_COST: u64 = 50;

	fn cost(
		input: &Input<Action, Runtime::AccountId, Runtime::AddressMapping>,
	) -> Result<u64, PrecompileFailure> {
		let action = input.action()?;
		let cost = match action {
			Action::QueryNewContractExtraBytes => {
				let weight = PrecompileWeights::<Runtime>::evm_query_new_contract_extra_bytes();
				WeightToGas::convert(weight)
			},
			Action::QueryStorageDepositPerByte => {
				let weight = PrecompileWeights::<Runtime>::evm_query_storage_deposit_per_byte();
				WeightToGas::convert(weight)
			},
			Action::QueryMaintainer => {
				let weight = PrecompileWeights::<Runtime>::evm_query_maintainer();
				WeightToGas::convert(weight)
			},
			Action::QueryDeveloperDeposit => {
				let weight = PrecompileWeights::<Runtime>::evm_query_developer_deposit();
				WeightToGas::convert(weight)
			},
			Action::QueryPublicationFee => {
				let weight = PrecompileWeights::<Runtime>::evm_query_publication_fee();
				WeightToGas::convert(weight)
			},
			Action::TransferMaintainer => {
				let read_accounts = InputPricer::<Runtime>::read_accounts(1);
				let weight = <Runtime as pallet_evm::Config>::WeightInfo::transfer_maintainer();
				Self::BASE_COST
					.saturating_add(read_accounts)
					.saturating_add(WeightToGas::convert(weight))
			},
			Action::PublishContract => {
				let read_accounts = InputPricer::<Runtime>::read_accounts(1);
				let weight = <Runtime as pallet_evm::Config>::WeightInfo::publish_contract();
				Self::BASE_COST
					.saturating_add(read_accounts)
					.saturating_add(WeightToGas::convert(weight))
			},
			Action::DisableDeveloperAccount => {
				let read_accounts = InputPricer::<Runtime>::read_accounts(1);
				let weight =
					<Runtime as pallet_evm::Config>::WeightInfo::disable_contract_development();
				Self::BASE_COST
					.saturating_add(read_accounts)
					.saturating_add(WeightToGas::convert(weight))
			},
			Action::EnableDeveloperAccount => {
				let read_accounts = InputPricer::<Runtime>::read_accounts(1);
				let weight =
					<Runtime as pallet_evm::Config>::WeightInfo::enable_contract_development();
				Self::BASE_COST
					.saturating_add(read_accounts)
					.saturating_add(WeightToGas::convert(weight))
			},
			Action::QueryDeveloperStatus => {
				let weight = PrecompileWeights::<Runtime>::evm_query_developer_status();
				WeightToGas::convert(weight)
			},
		};
		Ok(cost)
	}
}
