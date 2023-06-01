use std::fmt::{Display, Error as FmtError, Formatter};

use selendra_primitives::BlockNumber;
use sp_runtime::traits::{Block, Header};

use crate::{
	justification::SelendraJustification,
	sync::{
		substrate::{
			chain_status::{Error as ChainStatusError, SubstrateChainStatus},
			BlockId, Justification, JustificationTranslator,
		},
		BlockStatus, ChainStatus,
	},
};

#[derive(Debug)]
pub enum Error<B: Block> {
	ChainStatus(ChainStatusError<B>),
	NoBlock,
}

impl<B: Block> Display for Error<B> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
		use Error::*;
		match self {
			ChainStatus(e) => {
				write!(f, "error retrieving block status: {}", e)
			},
			NoBlock => write!(f, "block not present"),
		}
	}
}

impl<B: Block> From<ChainStatusError<B>> for Error<B> {
	fn from(value: ChainStatusError<B>) -> Self {
		Error::ChainStatus(value)
	}
}

impl<B> JustificationTranslator<B::Header> for SubstrateChainStatus<B>
where
	B: Block,
	B::Header: Header<Number = BlockNumber>,
{
	type Error = Error<B>;

	fn translate(
		&self,
		selendra_justification: SelendraJustification,
		block_id: BlockId<B::Header>,
	) -> Result<Justification<B::Header>, Self::Error> {
		use BlockStatus::*;
		match self.status_of(block_id)? {
			Justified(Justification { header, .. }) | Present(header) =>
				Ok(Justification::selendra_justification(header, selendra_justification)),
			Unknown => Err(Error::NoBlock),
		}
	}
}
