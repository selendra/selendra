#![cfg_attr(not(feature = "std"), no_std)]
//! # Indra Contract Offchain Rollup

pub mod anchor;
pub mod oracle;
pub mod types;

#[cfg(test)]
mod mock;
