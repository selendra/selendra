#![cfg_attr(not(feature = "std"), no_std)]

//! Indranet Pallets
//!
//! This is the central crate of Indranet tightly-coupled pallets.

#[cfg(target_arch = "wasm32")]
extern crate webpki_wasm as webpki;

#[cfg(not(feature = "std"))]
extern crate alloc;

// Re-export
use utils::{attestation, balance_convert, attestation_legacy, fixed_point, constants};

pub mod migrations;
pub mod utils;

pub mod compute;
pub mod mq;
pub mod indra;
pub mod registry;
pub mod stake_pool;

use compute::{base_pool, computation, pool_proxy, stake_pool_v2, vault, wrapped_balances};

use frame_support::traits::LockableCurrency;
/// The unified config of the compute pallets
pub trait IndranetConfig: frame_system::Config {
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}
/// The unified type Balance of pallets from the runtime T.
type BalanceOf<T> = <<T as IndranetConfig>::Currency as frame_support::traits::Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
/// The unified type ImBalance of pallets from the runtime T.
type NegativeImbalanceOf<T> = <<T as IndranetConfig>::Currency as frame_support::traits::Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

// Alias
pub use compute::base_pool as pallet_base_pool;
pub use compute::computation as pallet_computation;
pub use compute::stake_pool_v2 as pallet_stake_pool_v2;
pub use compute::vault as pallet_vault;
pub use compute::wrapped_balances as pallet_wrapped_balances;
pub use mq as pallet_mq;
pub use indra as pallet_indra;
pub use indra_tokenomic as pallet_indra_tokenomic;
pub use registry as pallet_registry;
pub use stake_pool as pallet_stake_pool;
pub mod indra_tokenomic;

#[cfg(feature = "native")]
use sp_core::hashing;

#[cfg(not(feature = "native"))]
use sp_io::hashing;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;
