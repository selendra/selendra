/// Time and blocks.
pub mod time {
	use crate::{BlockNumber, Moment};

	pub const MILLISECS_PER_BLOCK: Moment = 1000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;
}

pub mod currency {
	use crate::Balance;

	pub const TOKEN_DECIMALS: u32 = 18;
	pub const TOKEN: Balance = 10u128.pow(TOKEN_DECIMALS);

	pub const MILLI_SEL: Balance = TOKEN / 1000;
	pub const MICRO_SEL: Balance = MILLI_SEL / 1000;
	pub const NANO_SEL: Balance = MICRO_SEL / 1000;
	pub const PICO_SEL: Balance = NANO_SEL / 1000;
}
