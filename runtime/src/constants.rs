// Copyright 2023 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License

use currency::MILLI_CENT;
use selendra_primitives::{Balance, TOKEN};

// Prints debug output of the `contracts` pallet to stdout if the node is started with `-lruntime::contracts=debug`.
pub const CONTRACTS_DEBUG_OUTPUT: bool = true;

// The storage per one byte of contract storage: 4*10^{-5} Selendra per byte.
pub const CONTRACT_DEPOSIT_PER_BYTE: Balance = 4 * (TOKEN / 100_000);

// The storage deposit is roughly 1 TOKEN per 1kB -- this is the legacy value, used for pallet Identity and Multisig.
pub const LEGACY_DEPOSIT_PER_BYTE: Balance = MILLI_CENT;

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
