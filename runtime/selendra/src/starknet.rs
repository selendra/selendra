//! Configuration of the pallets used in the runtime.
//! The pallets used in the runtime are configured here.
//! This file is used to generate the `construct_runtime!` macro.
use crate::{Runtime, Timestamp};

use std::{num::NonZeroU128, sync::Arc, ops::Deref};
use lazy_static::lazy_static;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Perbill, Permill};

pub use frame_support::traits::{
    ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, OnTimestampSet, Randomness, StorageInfo,
};
pub use frame_support::weights::constants::{
    BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
};
pub use frame_support::weights::{IdentityFee, Weight};
pub use frame_support::{construct_runtime, parameter_types, StorageValue};
pub use frame_system::Call as SystemCall;

use blockifier::blockifier::block::GasPrices;
use blockifier::versioned_constants::VersionedConstants;

use mp_program_hash::SN_OS_PROGRAM_HASH;
use mp_felt::Felt252Wrapper;

/// Import the StarkNet pallet.
pub use pallet_starknet;
pub use pallet_timestamp::Call as TimestampCall;

#[cfg(not(all(debug_assertions, feature = "dev")))]
lazy_static! {
    static ref EXECUTION_CONSTANTS: Arc<VersionedConstants> = serde_json::from_str(selendra_primitives::EXECUTION_CONSTANTS_STR).unwrap();
}

#[cfg(all(debug_assertions, feature = "dev"))]
lazy_static! {
    static ref EXECUTION_CONSTANTS: Arc<VersionedConstants> = Arc::new(
        std::env::var("EXECUTION_CONSTANTS_PATH")
            .map(|path| {
                VersionedConstants::try_from(Path::new(path.as_str()))
                    .expect("Failed to load execution constants from path")
            })
            .unwrap_or_else(|e| {
                match e {
                    VarError::NotPresent => serde_json::from_str(EXECUTION_CONSTANTS_STR).unwrap(),
                    VarError::NotUnicode(_) => panic!("Failed to load execution constants variable"),
                }
            })
    );
}

fn get_execution_constants() -> Arc<VersionedConstants> {
    EXECUTION_CONSTANTS.deref().clone()
}

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
    pub ExecutionConstants: Arc<VersionedConstants> = get_execution_constants();
}


/// Configure the Starknet pallet in pallets/starknet.
impl pallet_starknet::Config for Runtime {
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
    type ExecutionConstants = ExecutionConstants;
}
