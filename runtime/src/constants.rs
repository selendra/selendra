pub mod currency {
    use selendra_primitives::{Balance, TOKEN};

    pub const MILLI_CENT: Balance = TOKEN / 1000;
    pub const MICRO_CENT: Balance = MILLI_CENT / 1000;
    pub const NANO_CENT: Balance = MICRO_CENT / 1000;
    pub const PICO_CENT: Balance = NANO_CENT / 1000;
}

pub mod time {
    pub const MILLISECS_PER_BLOCK: u64 = 1000;
    pub const BLOCKS_PER_HOUR: u32 = 60 * 60 * 1000 / (MILLISECS_PER_BLOCK as u32);
}