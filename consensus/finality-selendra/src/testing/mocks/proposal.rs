use sp_runtime::traits::Block as BlockT;

use crate::{
	data_io::{SelendraData, UnvalidatedSelendraProposal},
	testing::mocks::{TBlock, THeader},
};

pub fn unvalidated_proposal_from_headers(
	headers: Vec<THeader>,
) -> UnvalidatedSelendraProposal<TBlock> {
	let num = headers.last().unwrap().number;
	let hashes = headers.into_iter().map(|header| header.hash()).collect();
	UnvalidatedSelendraProposal::new(hashes, num)
}

pub fn selendra_data_from_blocks(blocks: Vec<TBlock>) -> SelendraData<TBlock> {
	let headers = blocks.into_iter().map(|b| b.header().clone()).collect();
	selendra_data_from_headers(headers)
}

pub fn selendra_data_from_headers(headers: Vec<THeader>) -> SelendraData<TBlock> {
	SelendraData { head_proposal: unvalidated_proposal_from_headers(headers) }
}
