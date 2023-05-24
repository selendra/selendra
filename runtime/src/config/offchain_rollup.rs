use crate::{
	deposit, Balances, OriginCaller, Runtime, RuntimeEvent, Scheduler, Treasury, MILLI_CENT,
	MINUTES, TOKEN, pallet_anchor, pallet_oracle, PhatOracle
};

use sp_std::prelude::*;

use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU32},
};

use selendra_primitives::{AccountId, Balance, BlockNumber};

parameter_types! {
    pub const QueuePrefix: &'static [u8] = b"_queue/";
    pub const QueueCapacity: u32 = 128;
}

impl pallet_anchor::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnResponse = PhatOracle;
    type QueuePrefix = QueuePrefix;
    type QueueCapacity = QueueCapacity;
}
impl pallet_oracle::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
}