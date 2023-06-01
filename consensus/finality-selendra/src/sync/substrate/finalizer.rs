use sc_client_api::{Backend, Finalizer as SubstrateFinalizer, HeaderBackend, LockImportRun};
use selendra_primitives::BlockNumber;
use sp_blockchain::Error as ClientError;
use sp_runtime::traits::{Block as BlockT, Header as SubstrateHeader};

use crate::{
	finalization::{BlockFinalizer, SelendraFinalizer},
	sync::{
		substrate::{InnerJustification, Justification},
		Finalizer,
	},
};

impl<B, BE, C> Finalizer<Justification<B::Header>> for SelendraFinalizer<B, BE, C>
where
	B: BlockT,
	B::Header: SubstrateHeader<Number = BlockNumber>,
	BE: Backend<B>,
	C: HeaderBackend<B> + LockImportRun<B, BE> + SubstrateFinalizer<B, BE>,
{
	type Error = ClientError;

	fn finalize(&self, justification: Justification<B::Header>) -> Result<(), Self::Error> {
		match justification.inner_justification {
			InnerJustification::SelendraJustification(selendra_justification) => self
				.finalize_block(
					(justification.header.hash(), *justification.header.number()).into(),
					selendra_justification.into(),
				),
			_ => Err(Self::Error::BadJustification(
				"Trying fo finalize the genesis block using virtual sync justification."
					.to_string(),
			)),
		}
	}
}
