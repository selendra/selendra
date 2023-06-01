use core::result::Result;
use std::{marker::PhantomData, sync::Arc, time::Instant};

use log::{debug, warn};
use sc_client_api::{Backend, Finalizer, HeaderBackend, LockImportRun};
use selendra_primitives::BlockNumber;
use sp_blockchain::Error;
use sp_runtime::{
	traits::{Block, Header},
	Justification,
};

use crate::{metrics::Checkpoint, BlockId, BlockIdentifier, IdentifierFor, Metrics};

pub trait BlockFinalizer<BI: BlockIdentifier> {
	fn finalize_block(&self, block: BI, justification: Justification) -> Result<(), Error>;
}

pub struct SelendraFinalizer<B, BE, C>
where
	B: Block,
	BE: Backend<B>,
	C: HeaderBackend<B> + LockImportRun<B, BE> + Finalizer<B, BE>,
{
	client: Arc<C>,
	metrics: Metrics<B::Hash>,
	phantom: PhantomData<BE>,
}

impl<B, BE, C> SelendraFinalizer<B, BE, C>
where
	B: Block,
	BE: Backend<B>,
	C: HeaderBackend<B> + LockImportRun<B, BE> + Finalizer<B, BE>,
{
	pub(crate) fn new(client: Arc<C>, metrics: Metrics<B::Hash>) -> Self {
		SelendraFinalizer { client, metrics, phantom: PhantomData }
	}
}

impl<B, BE, C> BlockFinalizer<IdentifierFor<B>> for SelendraFinalizer<B, BE, C>
where
	B: Block,
	B::Header: Header<Number = BlockNumber>,
	BE: Backend<B>,
	C: HeaderBackend<B> + LockImportRun<B, BE> + Finalizer<B, BE>,
{
	fn finalize_block(
		&self,
		block: IdentifierFor<B>,
		justification: Justification,
	) -> Result<(), Error> {
		let BlockId { number, hash } = block;

		let status = self.client.info();
		if status.finalized_number >= number {
			warn!(target: "selendra-finality", "trying to finalize a block with hash {} and number {}
               that is not greater than already finalized {}", hash, number, status.finalized_number);
		}

		debug!(target: "selendra-finality", "Finalizing block with hash {:?} and number {:?}. Previous best: #{:?}.", hash, number, status.finalized_number);

		let update_res = self.client.lock_import_and_run(|import_op| {
			// NOTE: all other finalization logic should come here, inside the lock
			self.client.apply_finality(import_op, hash, Some(justification), true)
		});

		let status = self.client.info();
		match &update_res {
			Ok(_) => {
				debug!(target: "selendra-finality", "Successfully finalized block with hash {:?} and number {:?}. Current best: #{:?}.", hash, number, status.finalized_number);
				self.metrics.report_block(hash, Instant::now(), Checkpoint::Finalized);
			},
			Err(_) => {
				debug!(target: "selendra-finality", "Failed to finalize block with hash {:?} and number {:?}. Current best: #{:?}.", hash, number, status.finalized_number)
			},
		}

		update_res
	}
}
