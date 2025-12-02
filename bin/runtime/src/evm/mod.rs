// Evm palllet implement

mod precompiles;

use crate::{
	Aura, Balances, DynamicEvmBaseFee, Runtime, RuntimeEvent, Timestamp,
	NORMAL_DISPATCH_RATIO
};

use pallet_transaction_payment::Multiplier;
use sp_core::{crypto::ByteArray, Get, H160, U256};
use sp_runtime::{
	ConsensusEngineId, Perquintill,
};
use sp_std::{marker::PhantomData, prelude::*};

use frame_support::{
	parameter_types,
	traits::{ConstU32, FindAuthor},
	weights::{constants::WEIGHT_REF_TIME_PER_SECOND, Weight},
};
use pallet_ethereum::PostLogContent;
// use pallet_evm::{EnsureAccountId20, IdentityAddressMapping};

use precompiles::FrontierPrecompiles;
use primitives::{
	TOKEN, AccountId, Balance, BlakeTwo256,
	evm::HashedDefaultMappings,
};

pub struct FindAuthorTruncated<F>(PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Aura::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.to_raw_vec()[4..24]));
		}
		None
	}
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;
/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_REF_TIME_PER_SECOND.saturating_div(GAS_PER_SECOND);

parameter_types! {
	/// EVM gas limit
	pub BlockGasLimit: U256 = U256::from(
		NORMAL_DISPATCH_RATIO * WEIGHT_REF_TIME_PER_SECOND / WEIGHT_PER_GAS
	);
	pub PrecompilesValue: FrontierPrecompiles<Runtime> = FrontierPrecompiles::<_>::new();
	pub WeightPerGas: Weight = Weight::from_parts(WEIGHT_PER_GAS, 0);
	pub ChainId: u64 = 1961;
	/// The amount of gas per pov size: BLOCK_GAS_LIMIT / MAX_POV_SIZE
	pub const GasLimitPovSizeRatio: u64 = 16;
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = DynamicEvmBaseFee;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = crate::UnifiedAccounts;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = FrontierPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = ChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated<Aura>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type SuicideQuickClearLimit = ConstU32<0>;
	type Timestamp = Timestamp;
	type WeightInfo = pallet_evm::weights::SubstrateWeight<Self>;
}

parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ConstU32<30>;
}

parameter_types! {
	pub const AccountMappingStorageFee: Balance = TOKEN / 100; // 0.01 SEL storage fee
}

impl pallet_unified_accounts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type DefaultMappings = HashedDefaultMappings<BlakeTwo256>;
	type ChainId = ChainId;
	type AccountMappingStorageFee = AccountMappingStorageFee;
	type WeightInfo = pallet_unified_accounts::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub DefaultBaseFeePerGas: U256 = U256::from(10_000_000_000_u128);
	pub MinBaseFeePerGas: U256 = U256::from(100_000_000_u128);
	pub MaxBaseFeePerGas: U256 = U256::from(10_000_000_000_000_u128);
	pub StepLimitRatio: Perquintill = Perquintill::from_rational(93_u128, 1_000_000);
}

/// Simple wrapper for fetching current native transaction fee weight fee multiplier.
pub struct AdjustmentFactorGetter;
impl Get<Multiplier> for AdjustmentFactorGetter {
	fn get() -> Multiplier {
		pallet_transaction_payment::NextFeeMultiplier::<Runtime>::get()
	}
}

impl pallet_dynamic_evm_base_fee::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type MinBaseFeePerGas = MinBaseFeePerGas;
	type MaxBaseFeePerGas = MaxBaseFeePerGas;
	type AdjustmentFactor = AdjustmentFactorGetter;
	type WeightFactor = ();
	type StepLimitRatio = StepLimitRatio;
	type WeightInfo = pallet_dynamic_evm_base_fee::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	/// Weight limit for checked transactions (user calls)
	pub CheckedTxWeightLimit: Weight = Weight::from_parts(u64::MAX / 2, 0);
	/// Weight limit for XVM transactions
	pub XvmTxWeightLimit: Weight = Weight::from_parts(u64::MAX / 4, 0);
}

impl pallet_ethereum_checked::Config for Runtime {
	type CheckedTxWeightLimit = CheckedTxWeightLimit;
	type XvmTxWeightLimit = XvmTxWeightLimit;
	type InvalidEvmTransactionError = pallet_ethereum::InvalidTransactionWrapper;
	type ValidatedTransaction = pallet_ethereum::ValidatedTransaction<Self>;
	type AddressMapper = crate::UnifiedAccounts;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_ethereum_checked::weights::SubstrateWeight<Runtime>;
}

impl pallet_xvm::Config for Runtime {
	type AddressMapper = crate::UnifiedAccounts;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Runtime>;
	type EthereumTransact = crate::EthereumChecked;
	type WeightInfo = pallet_xvm::weights::SubstrateWeight<Runtime>;
}
