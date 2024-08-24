// Evm palllet implement

mod precompiles;

use crate::{
	Aura, Balances, DynamicEvmBaseFee, EVMChainId, Runtime, RuntimeCall, RuntimeEvent, Timestamp,
	WeightFeeFactor,
};

use pallet_transaction_payment::Multiplier;
use sp_core::{crypto::ByteArray, Get, H160, U256};
use sp_runtime::{
	traits::Verify, transaction_validity::TransactionPriority, ConsensusEngineId, Perquintill,
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
use selendra_primitives::{
	common::NORMAL_DISPATCH_RATIO, currency::TOKEN, AccountId, Balance, BlakeTwo256, Signature,
};

impl pallet_evm_chain_id::Config for Runtime {}

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
	pub const GasLimitPovSizeRatio: u64 = 16;
	pub SuicideQuickClearLimit: u32 = 0;
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = DynamicEvmBaseFee;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Self>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = pallet_evm::HashedAddressMapping<BlakeTwo256>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = FrontierPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type ChainId = EVMChainId;
	type BlockGasLimit = BlockGasLimit;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type OnChargeTransaction = ();
	type OnCreate = ();
	type FindAuthor = FindAuthorTruncated<Aura>;
	type GasLimitPovSizeRatio = GasLimitPovSizeRatio;
	type SuicideQuickClearLimit = SuicideQuickClearLimit;
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
	pub const EcdsaUnsignedPriority: TransactionPriority = TransactionPriority::MAX / 2;
	pub const CallFee: Balance = TOKEN / 10;
	pub const CallMagicNumber: u16 = 0x0250;
}

impl pallet_custom_signatures::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Signature = pallet_custom_signatures::ethereum::EthereumSignature;
	type Signer = <Signature as Verify>::Signer;
	type CallMagicNumber = CallMagicNumber;
	type Currency = Balances;
	type CallFee = CallFee;
	type OnChargeTransaction = ();
	type UnsignedPriority = EcdsaUnsignedPriority;
}

parameter_types! {
	pub DefaultBaseFeePerGas: U256 = U256::from(10_000_000_000_u128);
	pub MinBaseFeePerGas: U256 = U256::from(80_000_000_000_u128);
	pub MaxBaseFeePerGas: U256 = U256::from(8_000_000_000_000_u128);
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
	type WeightFactor = WeightFeeFactor;
	type StepLimitRatio = StepLimitRatio;
	type WeightInfo = pallet_dynamic_evm_base_fee::weights::SubstrateWeight<Runtime>;
}
