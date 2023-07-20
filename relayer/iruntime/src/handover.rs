use crate::pal_gramine::GraminePlatform;
use anyhow::{Context, Result};
use indratory::RpcService;
use indratory_api::{
	ecall_args::InitArgs, irpc::indratory_api_server::IndratoryApi,
	iruntime_client::new_iruntime_client,
};
use tracing::info;

pub(crate) async fn handover_from(url: &str, args: InitArgs) -> Result<()> {
	let mut this = RpcService::new(GraminePlatform);
	this.lock_indratory(true, false).expect("Failed to lock Indratory").init(args);

	let from_iruntime = new_iruntime_client(url.into());
	info!("Requesting for challenge");
	let challenge = from_iruntime
		.handover_create_challenge(())
		.await
		.context("Failed to create challenge")?;
	info!("Challenge received");
	let response = this
		.handover_accept_challenge(challenge)
		.await
		.context("Failed to accept challenge")?;
	info!("Requesting for key");
	let encrypted_key = from_iruntime
		.handover_start(response)
		.await
		.context("Failed to start handover")?;
	info!("Key received");
	this.handover_receive(encrypted_key)
		.await
		.context("Failed to receive handover result")?;
	Ok(())
}
