use futures::channel::mpsc;
use jsonrpsee::{
	core::{error::Error as JsonRpseeError, RpcResult},
	proc_macros::rpc,
	types::error::{CallError, ErrorObject},
};
use serde::Serialize;

/// System RPC errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Justification argument is malformatted.
	#[error("{0}")]
	MalformattedJustificationArg(String),
	/// Provided block range couldn't be resolved to a list of blocks.
	#[error("Node is not fully functional: {}", .0)]
	FailedJustificationSend(String),
}

// Base code for all system errors.
const BASE_ERROR: i32 = 2000;
// Justification argument is malformatted.
const MALFORMATTED_JUSTIFICATION_ARG_ERROR: i32 = BASE_ERROR + 1;
// SelendraNodeApiServer is failed to send JustificationNotification.
const FAILED_JUSTIFICATION_SEND_ERROR: i32 = BASE_ERROR + 2;

impl From<Error> for JsonRpseeError {
	fn from(e: Error) -> Self {
		match e {
			Error::FailedJustificationSend(e) => CallError::Custom(ErrorObject::owned(
				FAILED_JUSTIFICATION_SEND_ERROR,
				e,
				None::<()>,
			)),
			Error::MalformattedJustificationArg(e) => CallError::Custom(ErrorObject::owned(
				MALFORMATTED_JUSTIFICATION_ARG_ERROR,
				e,
				None::<()>,
			)),
		}
		.into()
	}
}

/// Selendra Node RPC API
#[rpc(client, server)]
pub trait SelendraNodeApi<Hash, Number> {
	/// Finalize the block with given hash and number using attached signature. Returns the empty string or an error.
	#[method(name = "selendraNode_emergencyFinalize")]
	fn selendra_node_emergency_finalize(
		&self,
		justification: Vec<u8>,
		hash: Hash,
		number: Number,
	) -> RpcResult<()>;
}

use finality_selendra::{JustificationNotification, SelendraJustification};
use sp_api::BlockT;
use sp_runtime::traits::NumberFor;

/// Selendra Node API implementation
pub struct SelendraNode<B>
where
	B: BlockT,
	B::Hash: Serialize + for<'de> serde::Deserialize<'de>,
	NumberFor<B>: Serialize + for<'de> serde::Deserialize<'de>,
{
	import_justification_tx: mpsc::UnboundedSender<JustificationNotification<B>>,
}

impl<B> SelendraNode<B>
where
	B: BlockT,
	B::Hash: Serialize + for<'de> serde::Deserialize<'de>,
	NumberFor<B>: Serialize + for<'de> serde::Deserialize<'de>,
{
	pub fn new(
		import_justification_tx: mpsc::UnboundedSender<JustificationNotification<B>>,
	) -> Self {
		SelendraNode { import_justification_tx }
	}
}

impl<B> SelendraNodeApiServer<B::Hash, NumberFor<B>> for SelendraNode<B>
where
	B: BlockT,
	B::Hash: Serialize + for<'de> serde::Deserialize<'de>,
	NumberFor<B>: Serialize + for<'de> serde::Deserialize<'de>,
{
	fn selendra_node_emergency_finalize(
		&self,
		justification: Vec<u8>,
		hash: B::Hash,
		number: NumberFor<B>,
	) -> RpcResult<()> {
		let justification: SelendraJustification =
			SelendraJustification::EmergencySignature(justification.try_into().map_err(|_| {
				Error::MalformattedJustificationArg(
					"Provided justification cannot be converted into correct type".into(),
				)
			})?);
		self.import_justification_tx
			.unbounded_send(JustificationNotification { justification, hash, number })
			.map_err(|_| {
				Error::FailedJustificationSend(
					"SelendraNodeApiServer failed to send JustifictionNotification via its channel"
						.into(),
				)
				.into()
			})
	}
}
