// This file is part of Selendra.

// Copyright (C) 2021-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A list of the different weight modules for our runtime.
#![allow(clippy::unnecessary_cast)]

pub mod module_asset_registry;
pub mod module_auction_manager;
pub mod module_cdp_engine;
pub mod module_cdp_treasury;
pub mod module_currencies;
pub mod module_dex;
pub mod module_dex_oracle;
pub mod module_emergency_shutdown;
pub mod module_evm;
pub mod module_evm_accounts;
pub mod module_funan;
pub mod module_incentives;
pub mod module_nft;
pub mod module_prices;
pub mod module_stable_asset;
pub mod module_transaction_pause;
pub mod module_transaction_payment;

pub mod orml_auction;
pub mod orml_authority;
pub mod orml_oracle;
pub mod orml_tokens;

pub mod frame_system;

pub mod pallet_bags_list;
pub mod pallet_balances;
pub mod pallet_bounties;
pub mod pallet_collective;
pub mod pallet_democracy;
pub mod pallet_election_provider_multi_phase;
pub mod pallet_elections_phragmen;
pub mod pallet_identity;
pub mod pallet_im_online;
pub mod pallet_indices;
pub mod pallet_membership;
pub mod pallet_multisig;
pub mod pallet_nomination_pools;
pub mod pallet_preimage;
pub mod pallet_proxy;
pub mod pallet_scheduler;
pub mod pallet_session;
pub mod pallet_staking;
pub mod pallet_timestamp;
pub mod pallet_tips;
pub mod pallet_treasury;
pub mod pallet_utility;
pub mod pallet_vesting;
