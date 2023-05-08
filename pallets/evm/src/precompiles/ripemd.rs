
use super::LinearCostPrecompile;
use crate::runner::state::PrecompileFailure;
use pallet_evm_utility::evm::ExitSucceed;
use sha3::Digest;
use sp_std::vec::Vec;

/// The ripemd precompile.
pub struct Ripemd160;

impl LinearCostPrecompile for Ripemd160 {
	const BASE: u64 = 600;
	const WORD: u64 = 120;

	fn execute(input: &[u8], _cost: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
		let mut ret = [0u8; 32];
		ret[12..32].copy_from_slice(&ripemd160::Ripemd160::digest(input));
		Ok((ExitSucceed::Returned, ret.to_vec()))
	}
}