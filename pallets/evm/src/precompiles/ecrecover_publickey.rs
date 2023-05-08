use super::LinearCostPrecompile;
use crate::runner::state::PrecompileFailure;
use pallet_evm_utility::evm::{ExitError, ExitSucceed};
use sp_std::{cmp::min, vec::Vec};

/// The ecrecover precompile.
pub struct ECRecoverPublicKey;

impl LinearCostPrecompile for ECRecoverPublicKey {
	const BASE: u64 = 3000;
	const WORD: u64 = 0;

	fn execute(i: &[u8], _: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), PrecompileFailure> {
		let mut input = [0u8; 128];
		input[..min(i.len(), 128)].copy_from_slice(&i[..min(i.len(), 128)]);

		let mut msg = [0u8; 32];
		let mut sig = [0u8; 65];

		msg[0..32].copy_from_slice(&input[0..32]);
		sig[0..32].copy_from_slice(&input[64..96]);
		sig[32..64].copy_from_slice(&input[96..128]);
		sig[64] = input[63];

		let pubkey = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &msg).map_err(|_| PrecompileFailure::Error {
			exit_status: ExitError::Other("Public key recover failed".into()),
		})?;

		Ok((ExitSucceed::Returned, pubkey.to_vec()))
	}
}

#[test]
fn works() {
	let input = hex_literal::hex! {"
		18c547e4f7b0f325ad1e56f57e26c745b09a3e503d86e00e5255ff7f715d3d1c
		000000000000000000000000000000000000000000000000000000000000001c
		73b1693892219d736caba55bdb67216e485557ea6b6af75f37096c9aa6a5a75f
		eeb940b1d03b21e36b0e47e79769f095fe2ab855bd91e3a38756b7d75a9c4549
	"};

	let expected = hex_literal::hex!("3a514176466fa815ed481ffad09110a2d344f6c9b78c1d14afc351c3a51be33d8072e77939dc03ba44790779b7a1025baf3003f6732430e20cd9b76d953391b3");

	let (exit, output) = ECRecoverPublicKey::execute(&input, 0).unwrap();
	assert_eq!(exit, ExitSucceed::Returned);
	assert_eq!(output, expected);
}