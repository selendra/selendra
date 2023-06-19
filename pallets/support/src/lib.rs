#![cfg_attr(not(feature = "std"), no_std)]

mod evm;
mod migration;

pub use evm::*;
pub use migration::{ensure_storage_version, StorageMigration};
