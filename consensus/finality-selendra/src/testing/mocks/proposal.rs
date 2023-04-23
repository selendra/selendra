use sp_runtime::traits::Block as BlockT;
use substrate_test_runtime_client::runtime::{Block, Header};

use crate::data_io::{SelendraData, UnvalidatedSelendraProposal};

pub fn unvalidated_proposal_from_headers(
	headers: Vec<Header>,
) -> UnvalidatedSelendraProposal<Block> {
	let num = headers.last().unwrap().number;
	let hashes = headers.into_iter().map(|header| header.hash()).collect();
	UnvalidatedSelendraProposal::new(hashes, num)
}

pub fn selendra_data_from_blocks(blocks: Vec<Block>) -> SelendraData<Block> {
	let headers = blocks.into_iter().map(|b| b.header().clone()).collect();
	selendra_data_from_headers(headers)
}

pub fn selendra_data_from_headers(headers: Vec<Header>) -> SelendraData<Block> {
	SelendraData { head_proposal: unvalidated_proposal_from_headers(headers) }
}
