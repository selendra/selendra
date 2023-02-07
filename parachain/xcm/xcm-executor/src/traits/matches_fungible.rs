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

use xcm::latest::MultiAsset;

pub trait MatchesFungible<Balance> {
	fn matches_fungible(a: &MultiAsset) -> Option<Balance>;
}

#[impl_trait_for_tuples::impl_for_tuples(30)]
impl<Balance> MatchesFungible<Balance> for Tuple {
	fn matches_fungible(a: &MultiAsset) -> Option<Balance> {
		for_tuples!( #(
			match Tuple::matches_fungible(a) { o @ Some(_) => return o, _ => () }
		)* );
		log::trace!(target: "xcm::matches_fungible", "did not match fungible asset: {:?}", &a);
		None
	}
}
