//! Configuration of the pallets used in the runtime.
//! The pallets used in the runtime are configured here.
//! This file is used to generate the `construct_runtime!` macro.
use std::num::NonZeroU128;

use blockifier::blockifier::block::GasPrices;
pub use frame_support::traits::{
    ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, OnTimestampSet, Randomness, StorageInfo,
};
pub use frame_support::weights::constants::{
    BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
};
pub use frame_support::weights::{IdentityFee, Weight};
pub use frame_support::{construct_runtime, parameter_types, StorageValue};
pub use frame_system::Call as SystemCall;
pub use mp_chain_id::SN_GOERLI_CHAIN_ID;
pub use mp_program_hash::SN_OS_PROGRAM_HASH;
/// Import the StarkNet pallet.
pub use pallet_starknet;
pub use pallet_timestamp::Call as TimestampCall;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::traits::{AccountIdLookup, BlakeTwo256};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};
use sp_std::marker::PhantomData;

parameter_types! {
    pub const UnsignedPriority: u64 = 1 << 20;
    pub const TransactionLongevity: u64 = u64::MAX;
    pub const ProtocolVersion: u8 = 0;
    pub const ProgramHash: Felt252Wrapper = SN_OS_PROGRAM_HASH;
    pub const L1GasPrices: GasPrices = 
        GasPrices { eth_l1_gas_price: unsafe { 
            NonZeroU128::new_unchecked(10) }, strk_l1_gas_price: unsafe { NonZeroU128::new_unchecked(10) }, 
            eth_l1_data_gas_price: unsafe { NonZeroU128::new_unchecked(10) }, 
            strk_l1_data_gas_price: unsafe { NonZeroU128::new_unchecked(10) } 
        };
}


/// Configure the Starknet pallet in pallets/starknet.
impl pallet_starknet::Config for Runtime {
    type SystemHash = StarknetHasher;
    type TimestampProvider = Timestamp;
    type UnsignedPriority = UnsignedPriority;
    type TransactionLongevity = TransactionLongevity;
    #[cfg(not(feature = "disable-transaction-fee"))]
    type DisableTransactionFee = ConstBool<false>;
    #[cfg(feature = "disable-transaction-fee")]
    type DisableTransactionFee = ConstBool<true>;
    type DisableNonceValidation = ConstBool<false>;
    type ProtocolVersion = ProtocolVersion;
    type ProgramHash = ProgramHash;
    type L1GasPrices = L1GasPrices;
}