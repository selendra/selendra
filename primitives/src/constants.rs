/// Time and blocks.
pub mod time {
	use crate::{BlockNumber, Moment};

	pub const MILLISECS_PER_BLOCK: Moment = 6000;
	pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

	// These time units are defined in number of blocks.
	pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const HOURS: BlockNumber = MINUTES * 60;
	pub const DAYS: BlockNumber = HOURS * 24;
	pub const WEEKS: BlockNumber = DAYS * 7;

}

pub mod currency {
	pub const TOKEN_DECIMALS: u32 = 18;
	pub const TOKEN: u128 = 10u128.pow(TOKEN_DECIMALS);
}