#![cfg_attr(not(feature = "std"), no_std)]

//! WebContract Pallets
//!
//! This is the central crate of WebContract tightly-coupled pallets.

#[cfg(target_arch = "wasm32")]
extern crate webpki_wasm as webpki;

#[cfg(not(feature = "std"))]
extern crate alloc;

// Re-export
use utils::{attestation, balance_convert, constants};

pub mod migrations;
pub mod utils;

pub mod compute;
pub mod mq;
pub mod web_contract;
pub mod puppets;
pub mod registry;
pub mod stake_pool;

use compute::{base_pool, computation, pool_proxy, stake_pool_v2, vault, wrapped_balances};

use frame_support::traits::LockableCurrency;
/// The unified config of the compute pallets
pub trait WebContractConfig: frame_system::Config {
	type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}
/// The unified type Balance of pallets from the runtime T.
type BalanceOf<T> = <<T as WebContractConfig>::Currency as frame_support::traits::Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
/// The unified type ImBalance of pallets from the runtime T.
type NegativeImbalanceOf<T> =
	<<T as WebContractConfig>::Currency as frame_support::traits::Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

// Alias
pub use compute::{
	base_pool as pallet_base_pool, computation as pallet_computation,
	stake_pool_v2 as pallet_stake_pool_v2, vault as pallet_vault,
	wrapped_balances as pallet_wrapped_balances,
};
pub use mq as pallet_mq;
pub use web_contract as pallet_web;
pub use registry as pallet_registry;
pub use stake_pool as pallet_stake_pool;
pub use tokenomic as pallet_tokenomic;
pub mod tokenomic;

#[cfg(feature = "native")]
use sp_core::hashing;

#[cfg(not(feature = "native"))]
use sp_io::hashing;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;
