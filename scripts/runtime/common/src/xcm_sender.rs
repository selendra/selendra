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

//! XCM sender for relay chain.

use parity_scale_codec::Encode;
use runtime_indracores::{configuration, dmp};
use sp_std::marker::PhantomData;
use xcm::latest::prelude::*;

/// XCM sender for relay chain. It only sends downward message.
pub struct ChildIndracoreRouter<T, W>(PhantomData<(T, W)>);

impl<T: configuration::Config + dmp::Config, W: xcm::WrapVersion> SendXcm
	for ChildIndracoreRouter<T, W>
{
	fn send_xcm(dest: impl Into<MultiLocation>, msg: Xcm<()>) -> SendResult {
		let dest = dest.into();
		match dest {
			MultiLocation { parents: 0, interior: X1(Indracore(id)) } => {
				// Downward message passing.
				let versioned_xcm =
					W::wrap_version(&dest, msg).map_err(|()| SendError::DestinationUnsupported)?;
				let config = <configuration::Pallet<T>>::config();
				<dmp::Pallet<T>>::queue_downward_message(
					&config,
					id.into(),
					versioned_xcm.encode(),
				)
				.map_err(Into::<SendError>::into)?;
				Ok(())
			},
			dest => Err(SendError::CannotReachDestination(dest, msg)),
		}
	}
}
