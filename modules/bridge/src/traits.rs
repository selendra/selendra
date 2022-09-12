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

//! Traits used by the Substrate/Evm chains bridging pallet.

// Frame, system and frame primitives
use frame_support::weights::Weight;

/// Weight information for pallet extrinsics
///
/// Weights are calculated using runtime benchmarking features.
/// See [`benchmarking`] module for more information.
pub trait WeightInfo {
	fn set_threshold() -> Weight;

	fn set_resource() -> Weight;

	fn remove_resource() -> Weight;

	fn whitelist_chain() -> Weight;

	fn add_relayer() -> Weight;

	fn remove_relayer() -> Weight;

	fn acknowledge_proposal(dispatch_weight: Weight) -> Weight;

	fn reject_proposal() -> Weight;

	fn eval_vote_state(dispatch_weight: Weight) -> Weight;
}
