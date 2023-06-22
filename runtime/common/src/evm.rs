use frame_support::{
	dispatch::{DispatchClass, Weight},
	traits::{Get},
};
use sp_runtime::traits::Convert;
use sp_std::{marker::PhantomData};

// gas_to_weight_ratio
pub const RATIO: u64 = 9000;

/// Convert weight to gas
pub struct WeightToGas;
impl Convert<Weight, u64> for WeightToGas {
	fn convert(weight: Weight) -> u64 {
		weight
			.ref_time()
			.checked_div(RATIO)
			.expect("Compile-time constant is not zero; qed;")
	}
}

pub struct EvmLimits<T>(PhantomData<T>);
impl<T> EvmLimits<T>
where
	T: frame_system::Config,
{
	pub fn max_gas_limit() -> u64 {
		let weights = T::BlockWeights::get();
		let normal_weight = weights.get(DispatchClass::Normal);
		WeightToGas::convert(normal_weight.max_extrinsic.unwrap_or(weights.max_block))
	}

	pub fn max_storage_limit() -> u32 {
		let length = T::BlockLength::get();
		*length.max.get(DispatchClass::Normal)
	}
}
