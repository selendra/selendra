use crate::{pallet_anchor, pallet_oracle, IndraOracle, Runtime, RuntimeEvent};

use frame_support::parameter_types;

parameter_types! {
	pub const QueuePrefix: &'static [u8] = b"_queue/";
	pub const QueueCapacity: u32 = 128;
}

impl pallet_anchor::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnResponse = IndraOracle;
	type QueuePrefix = QueuePrefix;
	type QueueCapacity = QueueCapacity;
}

impl pallet_oracle::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
}
