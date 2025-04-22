use frame_support::{
	parameter_types,
	weights::{constants::WEIGHT_REF_TIME_PER_MILLIS, Weight},
};
use sp_runtime::Perbill;

use crate::BlockNumber;

pub const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(99);

// The whole process for a single block should take 1s, of which 400ms is for creation,
// 200ms for propagation and 400ms for validation. Hence the block weight should be within 400ms.
pub const MAXIMUM_BLOCK_WEIGHT: Weight =
	Weight::from_parts(WEIGHT_REF_TIME_PER_MILLIS.saturating_mul(400), 0);

pub const MAXIMUM_BLOCK_LENGTH: u32 = 5 * 1024 * 1024;

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
		::with_sensible_defaults(MAXIMUM_BLOCK_WEIGHT.set_proof_size(u64::MAX), NORMAL_DISPATCH_RATIO);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(MAXIMUM_BLOCK_LENGTH, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}
