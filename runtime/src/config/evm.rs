use crate::{
	origin::EnsureRootOrHalfCouncil, Aura, Balances, IdleScheduler, Runtime, RuntimeEvent, System,
	TreasuryPalletId, WeightToFee, MILLI_CENT,
};

use codec::{Decode, Encode};
use scale_info::TypeInfo;

use frame_support::{
	pallet_prelude::Weight, parameter_types, traits::ReservableCurrency, transactional,
};

use sp_core::{ConstU32, H160};
use sp_runtime::{
	traits::{AccountIdConversion, BlockNumberProvider},
	DispatchResult, RuntimeDebug,
};

use pallet_evm::{EvmChainId, EvmTask, TransferAll};
use pallet_evm_accounts::EvmAddressMapping;
use pallet_idle_scheduler::DispatchableTask;
use selendra_primitives::{define_combined_task, evm::task::TaskResult, AccountId, Balance};
use selendra_runtime_common::{
	evm::GasToWeight, impls::DealWithFees, precompile::AllPrecompiles, BlockWeights,
};

define_combined_task! {
	#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo)]
	pub enum ScheduledTasks {
		EvmTask(EvmTask<Runtime>),
	}
}

pub struct CurrentBlockNumberProvider;

impl BlockNumberProvider for CurrentBlockNumberProvider {
	type BlockNumber = u32;

	fn current_block_number() -> Self::BlockNumber {
		System::block_number().try_into().unwrap()
	}
}

parameter_types!(
	// At least 2% of max block weight should remain before idle tasks are dispatched.
	pub MinimumWeightRemainInBlock: Weight = BlockWeights::get().max_block / 50;
);

impl pallet_idle_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type BlockNumberProvider = CurrentBlockNumberProvider;
	type DisableBlockThreshold = ConstU32<5>;
}

parameter_types! {
	pub const NewContractExtraBytes: u32 = 10_000;
	pub NetworkContractSource: H160 = H160::from_low_u64_be(0);
	pub DeveloperDeposit: Balance = 50 * MILLI_CENT;
	pub PublicationFee: Balance = 10 * MILLI_CENT;
	pub PrecompilesValue: AllPrecompiles<Runtime> = AllPrecompiles::<_>::selendra();
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct StorageDepositPerByte;
impl<I: From<Balance>> frame_support::traits::Get<I> for StorageDepositPerByte {
	fn get() -> I {
		I::from(100_000_000_000_000)
	}
}

// TODO: remove
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct TxFeePerGas;
impl<I: From<Balance>> frame_support::traits::Get<I> for TxFeePerGas {
	fn get() -> I {
		// NOTE: 200 GWei
		// ensure suffix is 0x0000
		I::from(200u128.saturating_mul(10u128.saturating_pow(9)) & !0xffff)
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct TxFeePerGasV2;
impl<I: From<Balance>> frame_support::traits::Get<I> for TxFeePerGasV2 {
	fn get() -> I {
		// NOTE: 100 GWei
		I::from(100_000_000_000u128)
	}
}

parameter_types! {
	pub SelendraTreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl pallet_evm::Config for Runtime {
	type AddressMapping = EvmAddressMapping<Runtime>;
	type Currency = Balances;
	type TransferAll = TransferAllEvm;
	type NewContractExtraBytes = NewContractExtraBytes;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = TxFeePerGas;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = AllPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type GasToWeight = GasToWeight;
	type OnTransactionPayment = DealWithFees<Runtime>;
	type NetworkContractOrigin = EnsureRootOrHalfCouncil;
	type NetworkContractSource = NetworkContractSource;
	type DeveloperDeposit = DeveloperDeposit;
	type PublicationFee = PublicationFee;
	type TreasuryAccount = SelendraTreasuryAccount;
	type FreePublicationOrigin = EnsureRootOrHalfCouncil;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type Task = ScheduledTasks;
	type IdleScheduler = IdleScheduler;
	type WeightToFee = WeightToFee;
	type WeightInfo = pallet_evm::weights::SelendraWeight<Runtime>;
}

impl pallet_evm_accounts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type AddressMapping = EvmAddressMapping<Runtime>;
	type TransferAll = TransferAllEvm;
	type ChainId = EvmChainId<Runtime>;
	type WeightInfo = pallet_evm_accounts::weights::SelendraWeight<Runtime>;
}

pub struct TransferAllEvm;
impl TransferAll<AccountId> for TransferAllEvm {
	#[transactional]
	fn transfer_all(source: &AccountId, dest: &AccountId) -> DispatchResult {
		// unreserve all reserved currency
		<Balances as ReservableCurrency<_>>::unreserve(source, Balances::reserved_balance(source));

		// transfer all free to dest
		match Balances::transfer(
			Some(source.clone()).into(),
			dest.clone().into(),
			Balances::free_balance(source),
		) {
			Ok(_) => Ok(()),
			Err(e) => Err(e.error),
		}
	}
}
