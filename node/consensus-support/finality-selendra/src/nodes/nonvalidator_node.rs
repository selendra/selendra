use log::{debug, error};
use sc_client_api::Backend;
use sc_network_common::ExHashT;
use sp_consensus::SelectChain;
use sp_runtime::traits::Block;

use crate::{
	nodes::{setup_justification_handler, JustificationParams},
	session_map::{AuthorityProviderImpl, FinalityNotificatorImpl, SessionMapUpdater},
	SelendraConfig, BlockchainBackend,
};

pub async fn run_nonvalidator_node<B, H, C, BB, BE, SC>(selendra_config: SelendraConfig<B, H, C, SC, BB>)
where
	B: Block,
	H: ExHashT,
	C: crate::ClientForSelendra<B, BE> + Send + Sync + 'static,
	C::Api: selendra_primitives::SelendraSessionApi<B>,
	BE: Backend<B> + 'static,
	BB: BlockchainBackend<B> + Send + 'static,
	SC: SelectChain<B> + 'static,
{
	let SelendraConfig {
		network,
		client,
		blockchain_backend,
		metrics,
		session_period,
		millisecs_per_block,
		justification_rx,
		spawn_handle,
		..
	} = selendra_config;
	let map_updater = SessionMapUpdater::<_, _, B>::new(
		AuthorityProviderImpl::new(client.clone()),
		FinalityNotificatorImpl::new(client.clone()),
		session_period,
	);
	let session_authorities = map_updater.readonly_session_map();
	spawn_handle.spawn("selendra/updater", None, async move {
		debug!(target: "selendra-party", "SessionMapUpdater has started.");
		map_updater.run().await
	});
	let (_, handler_task) = setup_justification_handler(JustificationParams {
		justification_rx,
		network,
		client,
		blockchain_backend,
		metrics,
		session_period,
		millisecs_per_block,
		session_map: session_authorities,
	});

	debug!(target: "selendra-party", "JustificationHandler has started.");
	handler_task.await;
	error!(target: "selendra-party", "JustificationHandler finished.");
}
