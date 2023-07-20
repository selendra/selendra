use std::fmt::Debug;
use anyhow::Result;
use parity_scale_codec::{Decode, Encode, Error as CodecError};
use indranet_types::contract::ContractQueryError;
use thiserror::Error;

extern crate runtime as chain;

// supportive

pub struct BlockInfo<'a> {
    /// The block number.
    pub block_number: chain::BlockNumber,
    /// The timestamp of this block.
    pub now_ms: u64,
    /// The storage snapshot after this block executed.
    pub storage: &'a crate::ChainStorage,
    /// The message queue
    pub send_mq: &'a indranet_mq::MessageSendQueue,
    pub recv_mq: &'a mut indranet_mq::MessageDispatcher,
}

#[derive(Encode, Decode, Debug, Clone)]
pub struct TxRef {
    pub blocknum: chain::BlockNumber,
    pub index: u64,
}

// for contracts
pub type OpaqueQuery = Vec<u8>;
pub type OpaqueReply = Vec<u8>;
pub type OpaqueError = ContractQueryError;

pub fn deopaque_query<T>(mut data: &[u8]) -> Result<T, ContractQueryError>
where
    T: Decode + Debug,
{
    Decode::decode(&mut data).or(Err(ContractQueryError::DecodeError))
}

#[derive(Debug, Error)]
#[error("{:?}", self)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    IoError(#[from] anyhow::Error),
    DecodeError(#[from] CodecError),
    PersistentRuntimeNotFound,
}
