use log::debug;
use selendra_bft::{default_config, Config, LocalIO, Terminator};
pub use selendra_primitives::CURRENT_FINALITY_VERSION as VERSION;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block;

use crate::{
	abft::{common::unit_creation_delay_fn, NetworkWrapper, SpawnHandleT},
	crypto::Signature,
	data_io::{OrderedDataInterpreter, SelendraData},
	network::data::Network,
	oneshot,
	party::{
		backup::ABFTBackup,
		manager::{SubtaskCommon, Task},
	},
	CurrentNetworkData, Hasher, Keychain, NodeIndex, SessionId, SignatureSet, UnitCreationDelay,
};

pub fn run_member<
	B: Block,
	C: HeaderBackend<B> + Send + 'static,
	ADN: Network<CurrentNetworkData<B>> + 'static,
>(
	subtask_common: SubtaskCommon,
	multikeychain: Keychain,
	config: Config,
	network: NetworkWrapper<
		selendra_bft::NetworkData<Hasher, SelendraData<B>, Signature, SignatureSet<Signature>>,
		ADN,
	>,
	data_provider: impl selendra_bft::DataProvider<SelendraData<B>> + Send + 'static,
	ordered_data_interpreter: OrderedDataInterpreter<B, C>,
	backup: ABFTBackup,
) -> Task {
	let SubtaskCommon { spawn_handle, session_id } = subtask_common;
	let (stop, exit) = oneshot::channel();
	let member_terminator = Terminator::create_root(exit, "member");
	let local_io = LocalIO::new(data_provider, ordered_data_interpreter, backup.0, backup.1);

	let task = {
		let spawn_handle = spawn_handle.clone();
		async move {
			debug!(target: "selendra-party", "Running the member task for {:?}", session_id);
			selendra_bft::run_session(
				config,
				local_io,
				network,
				multikeychain,
				spawn_handle,
				member_terminator,
			)
			.await;
			debug!(target: "selendra-party", "Member task stopped for {:?}", session_id);
		}
	};

	let handle = spawn_handle.spawn_essential("selendra/consensus_session_member", task);
	Task::new(handle, stop)
}

pub fn create_selendra_config(
	n_members: usize,
	node_id: NodeIndex,
	session_id: SessionId,
	unit_creation_delay: UnitCreationDelay,
) -> Config {
	let mut config = default_config(n_members.into(), node_id.into(), session_id.0 as u64);
	config.delay_config.unit_creation_delay = unit_creation_delay_fn(unit_creation_delay);
	config
}
