use std::{
	fmt::Debug,
	hash::{Hash, Hasher},
};

use codec::{Decode, Encode};
use sp_runtime::traits::Block as BlockT;

mod chain_info;
mod data_interpreter;
mod data_provider;
mod data_store;
mod proposal;
mod status_provider;

pub use chain_info::ChainInfoProvider;
pub use data_interpreter::OrderedDataInterpreter;
pub use data_provider::{ChainTracker, DataProvider};
pub use data_store::{DataStore, DataStoreConfig};
pub use proposal::UnvalidatedSelendraProposal;

// Maximum number of blocks above the last finalized allowed in an SelendraBFT proposal.
pub const MAX_DATA_BRANCH_LEN: usize = 7;

/// The data ordered by the Selendra consensus.
#[derive(Clone, Debug, Encode, Decode)]
pub struct SelendraData<B: BlockT> {
	pub head_proposal: UnvalidatedSelendraProposal<B>,
}

// Need to be implemented manually, as deriving does not work (`BlockT` is not `Hash`).
impl<B: BlockT> Hash for SelendraData<B> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.head_proposal.hash(state);
	}
}

// Clippy does not allow deriving PartialEq when implementing Hash manually
impl<B: BlockT> PartialEq for SelendraData<B> {
	fn eq(&self, other: &Self) -> bool {
		self.head_proposal.eq(&other.head_proposal)
	}
}

impl<B: BlockT> Eq for SelendraData<B> {}

/// A trait allowing to check the data contained in an SelendraBFT network message, for the purpose of
/// data availability checks.
pub trait SelendraNetworkMessage<B: BlockT>: Clone + Debug {
	fn included_data(&self) -> Vec<SelendraData<B>>;
}

#[derive(Clone, Debug)]
pub struct ChainInfoCacheConfig {
	pub block_cache_capacity: usize,
}

impl Default for ChainInfoCacheConfig {
	fn default() -> ChainInfoCacheConfig {
		ChainInfoCacheConfig { block_cache_capacity: 2000 }
	}
}
