use crate::{config::TxFeePerGasV2, RuntimeCall, SignedExtra, System, EVM};

use codec::{Decode, Encode};
pub use frame_support::{pallet_prelude::InvalidTransaction, RuntimeDebug};
use pallet_evm::decode_gas_limit;
use selendra_primitives::evm::{decode_gas_price, EthereumTransactionMessage};
use sp_core::Get;
use sp_runtime::traits::Convert;

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct ConvertEthereumTx;

impl
	Convert<
		(RuntimeCall, SignedExtra),
		Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction>,
	> for ConvertEthereumTx
{
	fn convert(
		(call, mut extra): (RuntimeCall, SignedExtra),
	) -> Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction> {
		match call {
			RuntimeCall::EVM(pallet_evm::Call::eth_call {
				action,
				input,
				value,
				gas_limit,
				storage_limit,
				access_list,
				valid_until,
			}) => {
				if System::block_number() > valid_until {
					return Err(InvalidTransaction::Stale)
				}

				let (_, _, _, _, mortality, check_nonce, _, _, _) = extra.clone();

				if mortality != frame_system::CheckEra::from(sp_runtime::generic::Era::Immortal) {
					// require immortal
					return Err(InvalidTransaction::BadProof)
				}

				let nonce = check_nonce.nonce;
				let tip = 0;

				extra.5.mark_as_ethereum_tx(valid_until);

				Ok((
					EthereumTransactionMessage {
						chain_id: EVM::chain_id(),
						genesis: System::block_hash(0),
						nonce,
						tip,
						gas_price: Default::default(),
						gas_limit,
						storage_limit,
						action,
						value,
						input,
						valid_until,
						access_list,
					},
					extra,
				))
			},
			RuntimeCall::EVM(pallet_evm::Call::eth_call_v2 {
				action,
				input,
				value,
				gas_price,
				gas_limit,
				access_list,
			}) => {
				let (tip, valid_until) =
					decode_gas_price(gas_price, gas_limit, TxFeePerGasV2::get())
						.ok_or(InvalidTransaction::Stale)?;

				if System::block_number() > valid_until {
					return Err(InvalidTransaction::Stale)
				}

				let (_, _, _, _, mortality, check_nonce, _, _, _) = extra.clone();

				if mortality != frame_system::CheckEra::from(sp_runtime::generic::Era::Immortal) {
					// require immortal
					return Err(InvalidTransaction::BadProof)
				}

				let nonce = check_nonce.nonce;
				extra.5.mark_as_ethereum_tx(valid_until);

				let storage_limit = decode_gas_limit(gas_limit).1;

				Ok((
					EthereumTransactionMessage {
						chain_id: EVM::chain_id(),
						genesis: System::block_hash(0),
						nonce,
						tip,
						gas_price,
						gas_limit,
						storage_limit,
						action,
						value,
						input,
						valid_until,
						access_list,
					},
					extra,
				))
			},
			_ => Err(InvalidTransaction::BadProof),
		}
	}
}
