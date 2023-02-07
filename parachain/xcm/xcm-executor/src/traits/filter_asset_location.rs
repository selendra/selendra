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

use xcm::latest::{MultiAsset, MultiLocation};

/// Filters assets/location pairs.
///
/// Can be amalgamated into tuples. If any item returns `true`, it short-circuits, else `false` is returned.
pub trait FilterAssetLocation {
	/// A filter to distinguish between asset/location pairs.
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl FilterAssetLocation for Tuple {
	fn filter_asset_location(what: &MultiAsset, origin: &MultiLocation) -> bool {
		for_tuples!( #(
			if Tuple::filter_asset_location(what, origin) { return true }
		)* );
		log::trace!(
			target: "xcm::filter_asset_location",
			"got filtered: what: {:?}, origin: {:?}",
			what,
			origin,
		);
		false
	}
}
