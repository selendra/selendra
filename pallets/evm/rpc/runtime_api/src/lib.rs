#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::all)]

use primitives::evm::{
	AccessListItem, BlockLimits, CallInfo, CreateInfo, EstimateResourcesRequest,
};
use sp_core::H160;
use sp_runtime::{
	codec::Codec,
	traits::{MaybeDisplay, MaybeFromStr},
};
use sp_std::vec::Vec;

sp_api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait EVMRuntimeRPCApi<Balance> where
		Balance: Codec + MaybeDisplay + MaybeFromStr,
	{
		fn call(
			from: H160,
			to: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			estimate: bool,
		) -> Result<CallInfo, sp_runtime::DispatchError>;

		fn create(
			from: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			estimate: bool,
		) -> Result<CreateInfo, sp_runtime::DispatchError>;

		fn get_estimate_resources_request(data: Vec<u8>) -> Result<EstimateResourcesRequest, sp_runtime::DispatchError>;

		fn block_limits() -> BlockLimits;
	}
}
