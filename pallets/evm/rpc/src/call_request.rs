use primitives::evm::AccessListItem;
use serde::{Deserialize, Serialize};
use sp_core::{Bytes, H160, U256};
use sp_rpc::number::NumberOrHex;

/// Call request
#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct CallRequest {
	/// From
	pub from: Option<H160>,
	/// To
	pub to: Option<H160>,
	/// Gas Limit
	pub gas_limit: Option<u64>,
	/// Storage Limit
	pub storage_limit: Option<u32>,
	/// Value
	pub value: Option<NumberOrHex>,
	/// Data
	pub data: Option<Bytes>,
	/// AccessList
	pub access_list: Option<Vec<AccessListItem>>,
}

/// EstimateResources response
#[derive(Debug, Eq, PartialEq, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EstimateResourcesResponse {
	/// Used gas
	pub gas: u64,
	/// Used storage
	pub storage: i32,
	/// Adjusted weight fee
	pub weight_fee: U256,
}
