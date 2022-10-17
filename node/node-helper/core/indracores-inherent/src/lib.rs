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

//! The indracore inherent data provider
//!
//! Indracore backing and approval is an off-chain process, but the indracore needs to progress on chain as well. To
//! make it progress on chain a block producer needs to forward information about the state of a indracore to the
//! runtime. This information is forwarded through an inherent to the runtime. Here we provide the
//! [`IndracoreInherentDataProvider`] that requests the relevant data from the provisioner subsystem and creates the
//! the inherent data that the runtime will use to create an inherent.

#![deny(unused_crate_dependencies, unused_results)]

use futures::{select, FutureExt};
use selendra_node_subsystem::{
	errors::SubsystemError, messages::ProvisionerMessage, overseer::Handle,
};
use selendra_primitives::v2::{Block, Hash, InherentData as IndracoresInherentData};
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use std::time;

pub(crate) const LOG_TARGET: &str = "indracore::indracores-inherent";

/// How long to wait for the provisioner, before giving up.
const PROVISIONER_TIMEOUT: time::Duration = core::time::Duration::from_millis(2500);

/// Provides the indracores inherent data.
pub struct IndracoresInherentDataProvider {
	inherent_data: IndracoresInherentData,
}

impl IndracoresInherentDataProvider {
	/// Create a [`Self`] directly from some [`IndracoresInherentData`].
	pub fn from_data(inherent_data: IndracoresInherentData) -> Self {
		Self { inherent_data }
	}

	/// Create a new instance of the [`IndracoresInherentDataProvider`].
	pub async fn create<C: HeaderBackend<Block>>(
		client: &C,
		mut overseer: Handle,
		parent: Hash,
	) -> Result<Self, Error> {
		let pid = async {
			let (sender, receiver) = futures::channel::oneshot::channel();
			overseer.wait_for_activation(parent, sender).await;
			receiver
				.await
				.map_err(|_| Error::ClosedChannelAwaitingActivation)?
				.map_err(|e| Error::Subsystem(e))?;

			let (sender, receiver) = futures::channel::oneshot::channel();
			overseer
				.send_msg(
					ProvisionerMessage::RequestInherentData(parent, sender),
					std::any::type_name::<Self>(),
				)
				.await;

			receiver.await.map_err(|_| Error::ClosedChannelAwaitingInherentData)
		};

		let mut timeout = futures_timer::Delay::new(PROVISIONER_TIMEOUT).fuse();

		let parent_header = match client.header(BlockId::Hash(parent)) {
			Ok(Some(h)) => h,
			Ok(None) => return Err(Error::ParentHeaderNotFound(parent)),
			Err(err) => return Err(Error::Blockchain(err)),
		};

		let res = select! {
			pid = pid.fuse() => pid,
			_ = timeout => Err(Error::Timeout),
		};

		let inherent_data = match res {
			Ok(pd) => IndracoresInherentData {
				bitfields: pd.bitfields.into_iter().map(Into::into).collect(),
				backed_candidates: pd.backed_candidates,
				disputes: pd.disputes,
				parent_header,
			},
			Err(err) => {
				gum::debug!(
					target: LOG_TARGET,
					?err,
					"Could not get provisioner inherent data; injecting default data",
				);
				IndracoresInherentData {
					bitfields: Vec::new(),
					backed_candidates: Vec::new(),
					disputes: Vec::new(),
					parent_header,
				}
			},
		};

		Ok(Self { inherent_data })
	}
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for IndracoresInherentDataProvider {
	fn provide_inherent_data(
		&self,
		dst_inherent_data: &mut sp_inherents::InherentData,
	) -> Result<(), sp_inherents::Error> {
		dst_inherent_data
			.put_data(selendra_primitives::v2::INDRACORES_INHERENT_IDENTIFIER, &self.inherent_data)
	}

	async fn try_handle_error(
		&self,
		_identifier: &sp_inherents::InherentIdentifier,
		_error: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		// Inherent isn't checked and can not return any error
		None
	}
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Blockchain error")]
	Blockchain(#[from] sp_blockchain::Error),
	#[error("Timeout: provisioner did not return inherent data after {:?}", PROVISIONER_TIMEOUT)]
	Timeout,
	#[error("Could not find the parent header in the blockchain: {:?}", _0)]
	ParentHeaderNotFound(Hash),
	#[error("Closed channel from overseer when awaiting activation")]
	ClosedChannelAwaitingActivation,
	#[error("Closed channel from provisioner when awaiting inherent data")]
	ClosedChannelAwaitingInherentData,
	#[error("Subsystem failed")]
	Subsystem(#[from] SubsystemError),
}
