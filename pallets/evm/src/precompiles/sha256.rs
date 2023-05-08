use super::LinearCostPrecompile;
use crate::runner::state::PrecompileFailure;
use pallet_evm_utility::evm::ExitSucceed;
use sp_std::vec::Vec;

/// The sha256 precompile.
pub struct Sha256;

impl LinearCostPrecompile for Sha256 {
	const BASE: u64 = 60;
	const WORD: u64 = 12;

	fn execute(input: &[u8], _cost: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
		let ret = sp_io::hashing::sha2_256(input);
		Ok((ExitSucceed::Returned, ret.to_vec()))
	}
}