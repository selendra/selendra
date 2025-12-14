//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use client_runtime_api::fake_runtime::RuntimeApi;
use finality_aleph::{
    build_network, get_selendra_block_import, run_validator_node, AlephConfig, BlockImporter,
    BuildNetworkOutput, ChannelProvider, FavouriteSelectChainProvider, Justification,
    JustificationTranslator, MillisecsPerBlock, RateLimiterConfig, RedirectingBlockImport,
    SessionPeriod, SubstrateChainStatus, SyncOracle, ValidatorAddressCache,
};
use log::warn;
use pallet_aleph_runtime_api::AlephSessionApi;
use primitives::{Block, DEFAULT_BACKUP_FOLDER, MAX_BLOCK_SIZE};
use sc_basic_authorship::ProposerFactory;
use sc_client_api::HeaderBackend;
use sc_consensus::{ImportQueue, Link, BasicQueue};
use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_consensus_slots::BackoffAuthoringBlocksStrategy;
use sc_service::{
    error::Error as ServiceError, Configuration, KeystoreContainer, TFullClient, TaskManager,
};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_api::ProvideRuntimeApi;
use sp_arithmetic::traits::BaseArithmetic;
use sp_consensus::DisableProofRecording;
use sp_consensus_aura::{sr25519::AuthorityPair as AuraPair, Slot};
use sp_core::U256;

use selendra_runtime::TransactionConverter;
use crate::{
    aleph_cli::AlephCli,
    executor::selendra_executor,
    rpc::{create_full as create_full_rpc, FullDeps as RpcFullDeps},
	eth::{
		db_config_dir, new_frontier_partial, spawn_frontier_tasks, BackendType, EthConfiguration,
		FrontierBackend, FrontierBlockImport, FrontierPartialComponents,
	},
};



pub type SelendraExecutor = selendra_executor::Executor;
pub type FullClient = sc_service::TFullClient<Block, RuntimeApi, SelendraExecutor>;
pub type FullBackend = sc_service::TFullBackend<Block>;
type FullChainApi = sc_transaction_pool::FullChainApi<FullClient, Block>;
type FullPool = sc_transaction_pool::BasicPool<FullChainApi, Block>;
type FullImportQueue = sc_consensus::DefaultImportQueue<Block>;
type FullProposerFactory = ProposerFactory<FullPool, FullClient, DisableProofRecording>;

pub struct ServiceComponents {
	pub client: Arc<FullClient>,
	pub backend: Arc<FullBackend>,
	pub task_manager: TaskManager,
	pub select_chain_provider: FavouriteSelectChainProvider<Block>,
	pub import_queue: FullImportQueue,
	pub transaction_pool: Arc<FullPool>,
	pub keystore_container: KeystoreContainer,
	pub justification_channel_provider: ChannelProvider<Justification>,
	pub telemetry: Option<Telemetry>,
	pub frontier_backend: Arc<FrontierBackend>,
	pub storage_override : Arc<fc_storage::StorageOverrideHandler<Block, FullClient, FullBackend>>,
}
struct LimitNonfinalized(u32);

impl<N: BaseArithmetic> BackoffAuthoringBlocksStrategy<N> for LimitNonfinalized {
	fn should_backoff(
		&self,
		chain_head_number: N,
		_chain_head_slot: Slot,
		finalized_number: N,
		_slow_now: Slot,
		_logging_target: &str,
	) -> bool {
		let nonfinalized_blocks: u32 = chain_head_number
            .saturating_sub(finalized_number)
            .unique_saturated_into();
		match nonfinalized_blocks >= self.0 {
			true => {
				warn!("We have {} nonfinalized blocks, with the limit being {}, delaying block production.", nonfinalized_blocks, self.0);
				true
			}
			false => false,
		}
	}
}

fn backup_path(aleph_config: &AlephCli, base_path: &Path) -> Option<PathBuf> {
	if aleph_config.no_backup() {
		return None;
	}
	if let Some(path) = aleph_config.backup_path() {
		Some(path)
	} else {
		let path = base_path.join(DEFAULT_BACKUP_FOLDER);
		eprintln!("No backup path provided, using default path: {path:?} for AlephBFT backups. Please do not remove this folder");
		Some(path)
	}
}

pub fn new_partial(
	config: &Configuration,
	eth_config: &EthConfiguration,
) -> Result<ServiceComponents, ServiceError> {
	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = selendra_executor::get_executor(config);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, SelendraExecutor>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
		telemetry
	});

	let client: Arc<TFullClient<_, _, _>> = Arc::new(client);

	let select_chain_provider = FavouriteSelectChainProvider::default();

	let transaction_pool = Arc::new(sc_transaction_pool::BasicPool::new_full(
		sc_transaction_pool::Options::default(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	));
	let justification_translator = JustificationTranslator::new(
		SubstrateChainStatus::new(backend.clone())
			.map_err(|e| ServiceError::Other(format!("failed to set up chain status: {e}")))?,
	);
	let justification_channel_provider = ChannelProvider::new();
	let selendra_block_import = get_selendra_block_import(
		client.clone(),
		justification_channel_provider.get_sender(),
		justification_translator,
		select_chain_provider.select_chain(),
	);

    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;

	let frontier_block_import =
	FrontierBlockImport::new(selendra_block_import.clone(), client.clone());

    let storage_override = Arc::new(fc_storage::StorageOverrideHandler::new(client.clone()));
	let frontier_backend = match eth_config.frontier_backend_type {
		BackendType::KeyValue => FrontierBackend::KeyValue(Arc::new(fc_db::kv::Backend::open(
			Arc::clone(&client),
			&config.database,
			&db_config_dir(config),
		)?)),
		BackendType::Sql => {
			let db_path = db_config_dir(config).join("sql");
			std::fs::create_dir_all(&db_path).expect("failed creating sql db directory");
			let backend = futures::executor::block_on(fc_db::sql::Backend::new(
				fc_db::sql::BackendConfig::Sqlite(fc_db::sql::SqliteBackendConfig {
					path: Path::new("sqlite:///")
						.join(db_path)
						.join("frontier.db3")
						.to_str()
						.unwrap(),
					create_if_missing: true,
					thread_count: eth_config.frontier_sql_backend_thread_count,
					cache_size: eth_config.frontier_sql_backend_cache_size,
				}),
			eth_config.frontier_sql_backend_pool_size,
			std::num::NonZeroU32::new(eth_config.frontier_sql_backend_num_ops_timeout),
			storage_override.clone(),
		))
		.unwrap_or_else(|err| panic!("failed creating sql backend: {:?}", err));
		FrontierBackend::Sql(Arc::new(backend))
	}
};	// DO NOT change Aura parameters without updating the finality-aleph sync accordingly,
	// in particular the code responsible for verifying incoming Headers, as it is supposed
	// to duplicate parts of Aura internal logic
	let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(
		ImportQueueParams {
			block_import: frontier_block_import.clone(),
			justification_import: Some(Box::new(selendra_block_import)),
			client: client.clone(),
			create_inherent_data_providers: move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
                    sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                        *timestamp,
                        slot_duration,
                    );

				Ok((slot, timestamp))
			},
			spawner: &task_manager.spawn_essential_handle(),
			registry: config.prometheus_registry(),
			check_for_equivocation: Default::default(),
			telemetry: telemetry.as_ref().map(|x| x.handle()),
			compatibility_mode: Default::default(),
		},
	)?;

	Ok(ServiceComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain_provider,
		transaction_pool,
		justification_channel_provider,
		telemetry,
		frontier_backend: Arc::new(frontier_backend),
		storage_override,
	})
}

struct SelendraRuntimeVars {
	pub session_period: SessionPeriod,
	pub millisecs_per_block: MillisecsPerBlock,
    pub score_submission_period: u32,
}

fn get_selendra_runtime_vars(client: &Arc<FullClient>) -> SelendraRuntimeVars {
	let finalized = client.info().finalized_hash;

	let session_period = SessionPeriod(
		client
			.runtime_api()
			.session_period(finalized)
			.expect("should always be available"),
	);

	let millisecs_per_block = MillisecsPerBlock(
		client
			.runtime_api()
			.millisecs_per_block(finalized)
			.expect("should always be available"),
	);

    let score_submission_period = client
        .runtime_api()
        .score_submission_period(finalized)
        .unwrap_or(u32::MAX);

    SelendraRuntimeVars {
        session_period,
        millisecs_per_block,
        score_submission_period,
    }
}

fn get_validator_address_cache(aleph_config: &AlephCli) -> Option<ValidatorAddressCache> {
    aleph_config
        .collect_validator_network_data()
        .then(ValidatorAddressCache::new)
}

fn get_proposer_factory(
	service_components: &ServiceComponents,
	config: &Configuration,
) -> FullProposerFactory {
	let mut proposer_factory = FullProposerFactory::new(
		service_components.task_manager.spawn_handle(),
		service_components.client.clone(),
		service_components.transaction_pool.clone(),
		config.prometheus_registry().cloned().as_ref(),
		None,
	);
	proposer_factory.set_default_block_size_limit(MAX_BLOCK_SIZE as usize);

	proposer_factory
}

fn get_rate_limit_config(aleph_config: &AlephCli) -> RateLimiterConfig {
    RateLimiterConfig {
        alephbft_network_bit_rate: aleph_config.alephbft_network_bit_rate(),
        substrate_network_bit_rate: aleph_config.substrate_network_bit_rate(),
    }
}

struct NoopLink;

impl Link<Block> for NoopLink {}

/// Builds a new service for a full client.
pub async fn new_authority(
	mut config: Configuration,
	aleph_config: AlephCli,
	eth_config: EthConfiguration,
) -> Result<TaskManager, ServiceError> {
	if aleph_config.external_addresses().is_empty() {
		panic!("Cannot run a validator node without external addresses, stopping.");
	}

	let mut service_components = new_partial(&config, &eth_config)?;

	let backup_path = backup_path(&aleph_config, config.base_path.path());

	let backoff_authoring_blocks = Some(LimitNonfinalized(aleph_config.max_nonfinalized_blocks()));
	let prometheus_registry = config.prometheus_registry().cloned();
    let sync_oracle = SyncOracle::new();
	let proposer_factory = get_proposer_factory(&service_components, &config);
	let slot_duration = sc_consensus_aura::slot_duration(&*service_components.client)?;
	let (block_import, block_rx) = RedirectingBlockImport::new(service_components.client.clone());

	let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _>(
		StartAuraParams {
			slot_duration,
			client: service_components.client.clone(),
			select_chain: service_components.select_chain_provider.select_chain(),
			block_import,
			proposer_factory,
			create_inherent_data_providers: move |_, ()| async move {
				let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

				let slot =
                    sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                        *timestamp,
                        slot_duration,
                    );

				Ok((slot, timestamp))
			},
			force_authoring: config.force_authoring,
			backoff_authoring_blocks,
			keystore: service_components.keystore_container.local_keystore(),
			sync_oracle: sync_oracle.clone(),
			justification_sync_link: (),
			block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
			max_block_proposal_slot_portion: None,
			telemetry: service_components.telemetry.as_ref().map(|x| x.handle()),
			compatibility_mode: Default::default(),
		},
	)?;

	let import_queue_handle = BlockImporter::new(service_components.import_queue.service());
    let rate_limiter_config = get_rate_limit_config(&aleph_config);
    let network_config = finality_aleph::SubstrateNetworkConfig {
        substrate_network_bit_rate: rate_limiter_config.substrate_network_bit_rate,
        network_config: config.network.clone(),
    };

	let BuildNetworkOutput {
		network,
		authentication_network,
		block_sync_network,
		sync_service,
		tx_handler_controller,
		system_rpc_tx,
	} = build_network(
        network_config,
		config.protocol_id(),
		service_components.client.clone(),
        sync_oracle.underlying_atomic(),
		service_components.transaction_pool.clone(),
		&service_components.task_manager.spawn_handle(),
		config
            .prometheus_config
            .as_ref()
            .map(|config| config.registry.clone()),
	)?;

	let FrontierPartialComponents { filter_pool, fee_history_cache, fee_history_cache_limit } =
		new_frontier_partial(&eth_config)?;

	// Sinks for pubsub notifications.
	// Everytime a new subscription is created, a new mpsc channel is added to the sink pool.
	// The MappingSyncWorker sends through the channel on block import and the subscription emits a notification to the subscriber on receiving a message through this channel.
	// This way we avoid race conditions when using native substrate block import notification stream.
	let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
		fc_mapping_sync::EthereumBlockNotification<Block>,
	> = Default::default();
	let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);

	let chain_status = SubstrateChainStatus::new(service_components.backend.clone())
		.map_err(|e| ServiceError::Other(format!("failed to set up chain status: {e}")))?;
	let validator_address_cache = get_validator_address_cache(&aleph_config);
	let role = config.role.clone();

	let rpc_builder = {
		let client = service_components.client.clone();
		let pool = service_components.transaction_pool.clone();
		let sync_oracle = sync_oracle.clone();
		let validator_address_cache = validator_address_cache.clone();
		let import_justification_tx =
			service_components.justification_channel_provider.get_sender();
		let chain_status = chain_status.clone();
		let enable_dev_signer = eth_config.enable_dev_signer;
		let max_past_logs = eth_config.max_past_logs;
		let execute_gas_limit_multiplier = eth_config.execute_gas_limit_multiplier;
		let pubsub_notification_sinks = pubsub_notification_sinks.clone();
		let storage_override = service_components.storage_override.clone();
		let fee_history_cache = fee_history_cache.clone();
		let deny_unsafe = sc_rpc::DenyUnsafe::No;
		let block_data_cache = Arc::new(fc_rpc::EthBlockDataCacheTask::new(
			service_components.task_manager.spawn_handle(),
			service_components.storage_override.clone(),
			eth_config.eth_log_block_cache,
			eth_config.eth_statuses_cache,
			prometheus_registry.clone(),
		));
		let network = network.clone();
		let sync_service = sync_service.clone();
		let filter_pool = filter_pool.clone();
		let is_authority = role.is_authority();
		let frontier_backend = service_components.frontier_backend.clone();
		let target_gas_price = eth_config.target_gas_price;
		let task_manager_spawn_handle = service_components.task_manager.spawn_handle();
		let pending_create_inherent_data_providers = move |_, ()| async move {
			let current = sp_timestamp::InherentDataProvider::from_system_time();
			let next_slot = current.timestamp().as_millis() + slot_duration.as_millis();
			let timestamp = sp_timestamp::InherentDataProvider::new(next_slot.into());
			let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
				*timestamp,
				slot_duration,
			);
			let dynamic_fee = fp_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));
			Ok((slot, timestamp, dynamic_fee))
		};

		Box::new(move |subscription_task_executor| {
			
			let eth_deps = crate::rpc::EthDeps {
				client: client.clone(),
			pool: pool.clone(),
			graph: pool.clone(),
			converter: Some(TransactionConverter),
			is_authority,
			enable_dev_signer,
			network: network.clone(),
			sync: sync_service.clone(),
			frontier_backend: match &*frontier_backend {
				fc_db::Backend::KeyValue(b) => b.clone() as Arc<dyn fc_api::Backend<Block>>,
				fc_db::Backend::Sql(b) => b.clone() as Arc<dyn fc_api::Backend<Block>>,
			},
			storage_override: storage_override.clone(),
			block_data_cache: block_data_cache.clone(),
			filter_pool: filter_pool.clone(),
			max_past_logs,
			fee_history_cache: fee_history_cache.clone(),
				fee_history_cache_limit,
				execute_gas_limit_multiplier,
				forced_parent_hashes: None,
				pending_create_inherent_data_providers,
			};
			let deps = RpcFullDeps {
				client: client.clone(),
				pool: pool.clone(),
				deny_unsafe,
				import_justification_tx: import_justification_tx.clone(),
				justification_translator: JustificationTranslator::new(chain_status.clone()),
				sync_oracle: sync_oracle.clone(),
				validator_address_cache: validator_address_cache.clone(),
				eth: eth_deps,
				_phantom: std::marker::PhantomData::<FullChainApi>,
			};

			Ok(create_full_rpc(
				deps,
				subscription_task_executor,
				pubsub_notification_sinks.clone(),
			)?)
		})
	};

	spawn_frontier_tasks(
		&service_components.task_manager,
		service_components.client.clone(),
		service_components.backend.clone(),
		service_components.frontier_backend,
		filter_pool.clone(),
		service_components.storage_override,
		fee_history_cache,
		fee_history_cache_limit,
		sync_service.clone(),
		pubsub_notification_sinks,
	)
	.await;

	service_components.task_manager.spawn_handle().spawn(
		"import-queue",
		None,
		service_components.import_queue.run(&NoopLink),
	);

	sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network,
		sync_service,
		client: service_components.client.clone(),
		keystore: service_components.keystore_container.local_keystore(),
		task_manager: &mut service_components.task_manager,
		transaction_pool: service_components.transaction_pool.clone(),
		rpc_builder: Box::new(rpc_builder),
		backend: service_components.backend,
		system_rpc_tx,
		tx_handler_controller,
		config,
		telemetry: service_components.telemetry.as_mut(),
	})?;

	service_components
		.task_manager
		.spawn_essential_handle()
		.spawn_blocking("aura", None, aura);

    let SelendraRuntimeVars {
        millisecs_per_block,
        session_period,
        score_submission_period,
    } = get_selendra_runtime_vars(&service_components.client);

	let aleph_config = AlephConfig {
		authentication_network,
		block_sync_network,
		client: service_components.client,
		chain_status,
		import_queue_handle,
		select_chain_provider: service_components.select_chain_provider,
		session_period,
		millisecs_per_block,
        score_submission_period,
		spawn_handle: service_components.task_manager.spawn_handle().into(),
		keystore: service_components.keystore_container.local_keystore(),
		justification_channel_provider: service_components.justification_channel_provider,
		block_rx,
		registry: prometheus_registry,
		unit_creation_delay: aleph_config.unit_creation_delay(),
		backup_saving_path: backup_path,
		external_addresses: aleph_config.external_addresses(),
		validator_port: aleph_config.validator_port(),
		rate_limiter_config,
		sync_oracle,
		validator_address_cache,
		transaction_pool: service_components.transaction_pool,
	};

	service_components
        .task_manager
        .spawn_essential_handle()
        .spawn_blocking("aleph", None, run_validator_node(aleph_config));

	Ok(service_components.task_manager)
}

pub fn new_chain_ops(
	config: &mut Configuration,
	eth_config: &EthConfiguration,
) -> Result<
	(Arc<FullClient>, Arc<FullBackend>, BasicQueue<Block>, TaskManager, Arc<FrontierBackend>),
	ServiceError,
> {
	config.keystore = sc_service::config::KeystoreConfig::InMemory;
	let ServiceComponents { client, backend, import_queue, task_manager, frontier_backend, .. } =
		new_partial(config, eth_config)?;
	Ok((client, backend, import_queue, task_manager, frontier_backend))
}

pub async fn build_full(
	config: Configuration,
	aleph_config: AlephCli,
	eth_config: EthConfiguration,
) -> Result<TaskManager, ServiceError> {
	new_authority(config, aleph_config, eth_config).await
}
