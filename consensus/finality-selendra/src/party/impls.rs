use std::{marker::PhantomData, sync::Arc};

use sc_client_api::Backend;
use sc_network::NetworkService;
use sc_network_common::ExHashT;
use selendra_primitives::BlockNumber;
use sp_consensus::SyncOracle;
use sp_runtime::traits::{Block as BlockT, Header as HeaderT};

use crate::{
	party::traits::{ChainState, SyncState},
	ClientForSelendra,
};

pub struct ChainStateImpl<B, BE, CFA>
where
	B: BlockT,
	BE: Backend<B>,
	CFA: ClientForSelendra<B, BE>,
{
	pub client: Arc<CFA>,
	pub _phantom: PhantomData<(B, BE)>,
}

impl<B, BE, CFA> ChainState for ChainStateImpl<B, BE, CFA>
where
	B: BlockT,
	B::Header: HeaderT<Number = BlockNumber>,
	BE: Backend<B>,
	CFA: ClientForSelendra<B, BE>,
{
	fn best_block_number(&self) -> BlockNumber {
		self.client.info().best_number
	}
	fn finalized_number(&self) -> BlockNumber {
		self.client.info().finalized_number
	}
}

impl<B: BlockT, H: ExHashT> SyncState for Arc<NetworkService<B, H>> {
	fn is_major_syncing(&self) -> bool {
		NetworkService::is_major_syncing(self)
	}
}
