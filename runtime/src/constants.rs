pub mod currency {
    use selendra_primitives::{Balance, TOKEN};

    pub const MILLI_AZERO: Balance = TOKEN / 1000;
    pub const MICRO_AZERO: Balance = MILLI_AZERO / 1000;
    pub const NANO_AZERO: Balance = MICRO_AZERO / 1000;
    pub const PICO_AZERO: Balance = NANO_AZERO / 1000;
}
