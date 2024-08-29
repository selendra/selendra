use crate::{
	block::{
		substrate::{InnerJustification, Justification},
		Finalizer,
	},
	finalization::{AlephFinalizer, BlockFinalizer},
};
use sc_client_api::{Backend, Finalizer as SubstrateFinalizer, HeaderBackend, LockImportRun};
use selendra_primitives::Block;
use sp_blockchain::Error as ClientError;
use sp_runtime::traits::Header as SubstrateHeader;

impl<BE, C> Finalizer<Justification> for AlephFinalizer<Block, BE, C>
where
	BE: Backend<Block>,
	C: HeaderBackend<Block> + LockImportRun<Block, BE> + SubstrateFinalizer<Block, BE>,
{
	type Error = ClientError;

	fn finalize(&self, justification: Justification) -> Result<(), Self::Error> {
		match justification.inner_justification {
			InnerJustification::AlephJustification(aleph_justification) => self.finalize_block(
				(justification.header.hash(), *justification.header.number()).into(),
				aleph_justification.into(),
			),
			_ => Err(Self::Error::BadJustification(
				"Trying fo finalize the genesis block using virtual sync justification."
					.to_string(),
			)),
		}
	}
}
