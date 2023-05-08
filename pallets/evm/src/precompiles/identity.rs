use super::LinearCostPrecompile;
use crate::runner::state::PrecompileFailure;
use pallet_evm_utility::evm::ExitSucceed;
use sp_std::vec::Vec;

/// The identity precompile.
pub struct Identity;

impl LinearCostPrecompile for Identity {
	const BASE: u64 = 15;
	const WORD: u64 = 3;

	fn execute(input: &[u8], _: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
		Ok((ExitSucceed::Returned, input.to_vec()))
	}
}