// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Transport that serves as a common ground for all connections.

use either::Either;
use futures::{AsyncRead, AsyncWrite};
use libp2p::{
	core::{transport::OptionalTransport, upgrade, StreamMuxer},
	dns, identity,
	identity::Keypair,
	noise, tcp, websocket, PeerId, Transport,
};
use std::time::Duration;

pub use libp2p::bandwidth::BandwidthSinks;

/// Describes network configuration used for building instances of [`libp2p::Transport`].
pub struct NetworkConfig {
	/// Our network identity.
	pub keypair: Keypair,
	/// Indicates whether created [`Transport`] should be only memory-based.
	pub memory_only: bool,
	/// Window size of the muxer.
	pub muxer_window_size: Option<u32>,
	/// Buffer size of the muxer.
	pub muxer_maximum_buffer_size: usize,
}

/// Creates default base layer of network transport, i.e. a transport that allows connectivity for
/// `WS + WSS` (with `DNS`) or `TCP + WS` (when `DNS` is not available). It can be used as basis for
/// building a custom implementation of authenticated and mutliplexed [`libp2p::Transport`] that is
/// required by the [`NetworkWorker`].
pub fn build_basic_transport(
	memory_only: bool,
) -> impl Transport<
	Output = impl AsyncRead + AsyncWrite,
	Dial = impl Send,
	ListenerUpgrade = impl Send,
	Error = impl Send,
> + Send {
	// Build the base layer of the transport.
	if !memory_only {
		// Main transport: DNS(TCP)
		let tcp_config = tcp::Config::new().nodelay(true);
		let tcp_trans = tcp::tokio::Transport::new(tcp_config.clone());
		let dns_init = dns::TokioDnsConfig::system(tcp_trans);

		Either::Left(if let Ok(dns) = dns_init {
			// WS + WSS transport
			//
			// Main transport can't be used for `/wss` addresses because WSS transport needs
			// unresolved addresses (BUT WSS transport itself needs an instance of DNS transport to
			// resolve and dial addresses).
			let tcp_trans = tcp::tokio::Transport::new(tcp_config);
			let dns_for_wss = dns::TokioDnsConfig::system(tcp_trans)
				.expect("same system_conf & resolver to work");
			Either::Left(websocket::WsConfig::new(dns_for_wss).or_transport(dns))
		} else {
			// In case DNS can't be constructed, fallback to TCP + WS (WSS won't work)
			let tcp_trans = tcp::tokio::Transport::new(tcp_config.clone());
			let desktop_trans = websocket::WsConfig::new(tcp_trans)
				.or_transport(tcp::tokio::Transport::new(tcp_config));
			Either::Right(desktop_trans)
		})
	} else {
		Either::Right(OptionalTransport::some(libp2p::core::transport::MemoryTransport::default()))
	}
}

/// Adds authentication and multiplexing to a given implementation of [`libp2p::Transport`].
/// It uses the `noise` protocol for authentication and the `yamux` library for connection multiplexing.
pub fn add_authentication_and_muxing(
	keypair: identity::Keypair,
	yamux_window_size: Option<u32>,
	yamux_maximum_buffer_size: usize,
	transport: impl Transport<
			Output = impl AsyncRead + AsyncWrite + Send + Unpin + 'static,
			Dial = impl Send,
			ListenerUpgrade = impl Send,
			Error = impl Send + 'static,
		> + Send,
) -> impl Transport<
	Output = (PeerId, impl StreamMuxer<Substream = impl Send, Error = impl Send> + Send),
	Dial = impl Send,
	ListenerUpgrade = impl Send,
	Error = impl Send,
> + Send {
	let authentication_config = noise::Config::new(&keypair).expect("Can create noise config. qed");
	let multiplexing_config = {
		let mut yamux_config = libp2p::yamux::Config::default();
		// Enable proper flow-control: window updates are only sent when
		// buffered data has been consumed.
		yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());
		yamux_config.set_max_buffer_size(yamux_maximum_buffer_size);

		if let Some(yamux_window_size) = yamux_window_size {
			yamux_config.set_receive_window_size(yamux_window_size);
		}

		yamux_config
	};

	transport
		.upgrade(upgrade::Version::V1Lazy)
		.authenticate(authentication_config)
		.multiplex(multiplexing_config)
		.timeout(Duration::from_secs(20))
}

/// Builds the transport that serves as a common ground for all connections.
///
/// If `memory_only` is true, then only communication within the same process are allowed. Only
/// addresses with the format `/memory/...` are allowed.
///
/// `yamux_window_size` is the maximum size of the Yamux receive windows. `None` to leave the
/// default (256kiB).
///
/// `yamux_maximum_buffer_size` is the maximum allowed size of the Yamux buffer. This should be
/// set either to the maximum of all the maximum allowed sizes of messages frames of all
/// high-level protocols combined, or to some generously high value if you are sure that a maximum
/// size is enforced on all high-level protocols.
///
/// Returns a multiplexed and authenticated implementation of [`libp2p::Transport``].
pub fn build_transport(
	keypair: identity::Keypair,
	memory_only: bool,
	yamux_window_size: Option<u32>,
	yamux_maximum_buffer_size: usize,
) -> impl Transport<
	Output = (PeerId, impl StreamMuxer<Substream = impl Send, Error = impl Send> + Send),
	Dial = impl Send,
	ListenerUpgrade = impl Send,
	Error = impl Send,
> + Send {
	let basic_transport = build_basic_transport(memory_only);
	add_authentication_and_muxing(
		keypair,
		yamux_window_size,
		yamux_maximum_buffer_size,
		basic_transport,
	)
}
