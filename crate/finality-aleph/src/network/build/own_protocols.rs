use sc_network::config::FullNetworkConfiguration;
use sc_network::NetworkBackend;
use sc_network_common::ExHashT;
use sp_runtime::traits::Block;

use crate::{
    network::{
        session::MAX_MESSAGE_SIZE as MAX_AUTHENTICATION_MESSAGE_SIZE, substrate::ProtocolNetwork,
    },
    sync::MAX_MESSAGE_SIZE as MAX_BLOCK_SYNC_MESSAGE_SIZE,
    BlockHash,
};

/// Name of the network protocol used by Aleph Zero to disseminate validator
/// authentications.
const AUTHENTICATION_PROTOCOL_NAME: &str = "/auth/0";

/// Name of the network protocol used by Aleph Zero to synchronize the block state.
const BLOCK_SYNC_PROTOCOL_NAME: &str = "/sync/0";

/// Struct containing networks for our two protocols.
pub struct Networks {
    /// Authentication network.
    pub authentication_network: ProtocolNetwork,
    /// Block sync network.
    pub block_sync_network: ProtocolNetwork,
}

impl Networks {
    fn add_protocol<B: Block, N: NetworkBackend<B, B::Hash>>(
        genesis_hash: &BlockHash,
        protocol_name: &str,
        max_message_size: u64,
        net_config: &mut FullNetworkConfiguration<B, B::Hash, N>,
    ) -> ProtocolNetwork
    where
        B::Hash: ExHashT,
    {
        let peer_store_handle = net_config.peer_store_handle();
        let metrics = sc_network::NotificationMetrics::new(None);
        let (config, notifications) = N::notification_config(
            // full protocol name
            format!("/{genesis_hash}{protocol_name}").into(),
            // no fallback names
            vec![],
            max_message_size,
            // we do not use custom handshake
            None,
            sc_network::config::SetConfig::default(),
            metrics,
            peer_store_handle,
        );
        net_config.add_notification_protocol(config);
        ProtocolNetwork::new(notifications)
    }

    /// Create the full configuration and networks per protocol.
    pub fn new<B: Block, N: NetworkBackend<B, B::Hash>>(net_config: &mut FullNetworkConfiguration<B, B::Hash, N>, genesis_hash: &BlockHash) -> Self
    where
        B::Hash: ExHashT,
    {
        let authentication_network = Self::add_protocol(
            genesis_hash,
            AUTHENTICATION_PROTOCOL_NAME,
            MAX_AUTHENTICATION_MESSAGE_SIZE,
            net_config,
        );
        let block_sync_network = Self::add_protocol(
            genesis_hash,
            BLOCK_SYNC_PROTOCOL_NAME,
            MAX_BLOCK_SYNC_MESSAGE_SIZE,
            net_config,
        );

        Self {
            authentication_network,
            block_sync_network,
        }
    }
}
