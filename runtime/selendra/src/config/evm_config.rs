use crate::{
	config::utility_config::ScheduledTasks, dollar, parameter_types, weights, AllPrecompiles, Babe,
	Balance, Balances, Currencies, Event, GasToWeight, IdleScheduler, Runtime, RuntimeDebug,
	TreasuryAccount, EVM, H160, SEL, Call, SignedExtra, System, SignedPayload, Verify, Convert
};
use runtime_common::{EnsureRootOrHalfCouncil, EnsureRootOrTwoThirdsTechnicalCommittee};
use primitives::evm::EthereumTransactionMessage;

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::{
	pallet_prelude::InvalidTransaction,
};

pub use module_evm::{EvmChainId, EvmTask};
use module_evm_accounts::EvmAddressMapping;

impl module_evm_accounts::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type AddressMapping = EvmAddressMapping<Runtime>;
	type TransferAll = Currencies;
	type ChainId = EvmChainId<Runtime>;
	type WeightInfo = weights::module_evm_accounts::WeightInfo<Runtime>;
}

parameter_types! {
	pub NetworkContractSource: H160 = H160::from_low_u64_be(0);
	pub PrecompilesValue: AllPrecompiles<Runtime> = AllPrecompiles::<_>::selendra();
}

#[cfg(feature = "with-ethereum-compatibility")]
parameter_types! {
	pub const NewContractExtraBytes: u32 = 0;
	pub const DeveloperDeposit: Balance = 0;
	pub const PublicationFee: Balance = 0;
}

#[cfg(not(feature = "with-ethereum-compatibility"))]
parameter_types! {
	pub const NewContractExtraBytes: u32 = 10_000;
	pub DeveloperDeposit: Balance = dollar(SEL);
	pub PublicationFee: Balance = 10 * dollar(SEL);
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct StorageDepositPerByte;
impl<I: From<Balance>> frame_support::traits::Get<I> for StorageDepositPerByte {
	fn get() -> I {
		#[cfg(not(feature = "with-ethereum-compatibility"))]
		// NOTE: SEL decimals is 12, convert to 18.
		// 10 * millicent(SEL) * 10^6
		#[rustfmt::skip]
		return I::from(100_000_000_000_000);
		#[cfg(feature = "with-ethereum-compatibility")]
		return I::from(0)
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct TxFeePerGas;
impl<I: From<Balance>> frame_support::traits::Get<I> for TxFeePerGas {
	fn get() -> I {
		// NOTE: 200 GWei
		// ensure suffix is 0x0000
		I::from(200u128.saturating_mul(10u128.saturating_pow(9)) & !0xffff)
	}
}

#[cfg(feature = "with-ethereum-compatibility")]
static LONDON_CONFIG: module_evm_utility::evm::Config = module_evm_utility::evm::Config::london();

impl module_evm::Config for Runtime {
	type AddressMapping = EvmAddressMapping<Runtime>;
	type Currency = Balances;
	type TransferAll = Currencies;
	type NewContractExtraBytes = NewContractExtraBytes;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = TxFeePerGas;
	type Event = Event;
	type PrecompilesType = AllPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type GasToWeight = GasToWeight;
	type ChargeTransactionPayment = module_transaction_payment::ChargeTransactionPayment<Runtime>;
	type NetworkContractOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type NetworkContractSource = NetworkContractSource;
	type DeveloperDeposit = DeveloperDeposit;
	type PublicationFee = PublicationFee;
	type TreasuryAccount = TreasuryAccount;
	type FreePublicationOrigin = EnsureRootOrHalfCouncil;
	type Runner = module_evm::runner::stack::Runner<Self>;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type Task = ScheduledTasks;
	type IdleScheduler = IdleScheduler;
	type WeightInfo = weights::module_evm::WeightInfo<Runtime>;

	#[cfg(feature = "with-ethereum-compatibility")]
	fn config() -> &'static module_evm_utility::evm::Config {
		&LONDON_CONFIG
	}
}

impl module_evm_bridge::Config for Runtime {
	type EVM = EVM;
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct ConvertEthereumTx;

impl
	Convert<
		(Call, SignedExtra),
		Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction>,
	> for ConvertEthereumTx
{
	fn convert(
		(call, mut extra): (Call, SignedExtra),
	) -> Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction> {
		match call {
			Call::EVM(module_evm::Call::eth_call {
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

				let (_, _, _, _, mortality, check_nonce, _, charge, ..) = extra.clone();

				if mortality != frame_system::CheckEra::from(sp_runtime::generic::Era::Immortal) {
					// require immortal
					return Err(InvalidTransaction::BadProof)
				}

				let nonce = check_nonce.nonce;
				let tip = charge.0;

				extra.5.mark_as_ethereum_tx(valid_until);

				Ok((
					EthereumTransactionMessage {
						chain_id: EVM::chain_id(),
						genesis: System::block_hash(0),
						nonce,
						tip,
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

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct PayerSignatureVerification;

impl Convert<(Call, SignedExtra), Result<(), InvalidTransaction>> for PayerSignatureVerification {
	fn convert((call, extra): (Call, SignedExtra)) -> Result<(), InvalidTransaction> {
		if let Call::TransactionPayment(module_transaction_payment::Call::with_fee_paid_by {
			call,
			payer_addr,
			payer_sig,
		}) = call
		{
			let payer_account: [u8; 32] = payer_addr
				.encode()
				.as_slice()
				.try_into()
				.map_err(|_| InvalidTransaction::BadSigner)?;
			// payer signature is aim at inner call of `with_fee_paid_by` call.
			let raw_payload =
				SignedPayload::new(*call, extra).map_err(|_| InvalidTransaction::BadSigner)?;
			if !raw_payload
				.using_encoded(|payload| payer_sig.verify(payload, &payer_account.into()))
			{
				return Err(InvalidTransaction::BadProof)
			}
		}
		Ok(())
	}
}
