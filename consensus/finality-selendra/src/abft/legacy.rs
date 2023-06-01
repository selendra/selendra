use legacy_selendra_bft::{default_config, Config, LocalIO, Terminator};
use log::debug;
use network_clique::SpawnHandleT;
pub use selendra_primitives::{BlockNumber, LEGACY_FINALITY_VERSION as VERSION};
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::{Block, Header};

use super::common::{unit_creation_delay_fn, MAX_ROUNDS};
use crate::{
	abft::NetworkWrapper,
	data_io::{OrderedDataInterpreter, SelendraData},
	network::data::Network,
	oneshot,
	party::{
		backup::ABFTBackup,
		manager::{SubtaskCommon, Task},
	},
	Keychain, LegacyNetworkData, NodeIndex, SessionId, UnitCreationDelay,
};

pub fn run_member<B, C, ADN>(
	subtask_common: SubtaskCommon,
	multikeychain: Keychain,
	config: Config,
	network: NetworkWrapper<LegacyNetworkData<B>, ADN>,
	data_provider: impl legacy_selendra_bft::DataProvider<SelendraData<B>> + Send + 'static,
	ordered_data_interpreter: OrderedDataInterpreter<B, C>,
	backup: ABFTBackup,
) -> Task
where
	B: Block,
	B::Header: Header<Number = BlockNumber>,
	C: HeaderBackend<B> + Send + 'static,
	ADN: Network<LegacyNetworkData<B>> + 'static,
{
	let SubtaskCommon { spawn_handle, session_id } = subtask_common;
	let (stop, exit) = oneshot::channel();
	let member_terminator = Terminator::create_root(exit, "member");
	let local_io = LocalIO::new(data_provider, ordered_data_interpreter, backup.0, backup.1);

	let task = {
		let spawn_handle = spawn_handle.clone();
		async move {
			debug!(target: "selendra-party", "Running the member task for {:?}", session_id);
			legacy_selendra_bft::run_session(
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
	config.max_round = MAX_ROUNDS;
	config
}
