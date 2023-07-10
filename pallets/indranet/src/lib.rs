#![cfg_attr(not(feature = "std"), no_std)]

//! Indranet Pallets
//!
//! This is the central crate of Indranet tightly-coupled pallets.

#[cfg(target_arch = "wasm32")]
extern crate webpki_wasm as webpki;

#[cfg(not(feature = "std"))]
extern crate alloc;

pub mod mq;
pub mod registry;
pub mod utils;

use utils::{attestation, constants, attestation_legacy};

// Alias
pub use mq as pallet_mq;
pub use registry as pallet_registry;

use sp_io::hashing;
