use anyhow::{anyhow, bail, Result};
use pink::types::{AccountId, ExecutionMode, TransactionArguments};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use parity_scale_codec::Decode;
use indranet_mq::SignedMessageChannel;
use indranet_scheduler::RequestScheduler;
use runtime::BlockNumber;
use sidevm::{
    service::{Command as SidevmCommand, CommandSender, ExitReason},
    OcallAborted, VmId,
};

use super::pink::Cluster;
use crate::{
    hex,
    secret_channel::{KeyPair, SecretMessageChannel, SecretReceiver},
    system::{TransactionError, TransactionResult},
    types::BlockInfo,
    ChainStorage, H256,
};
use indratory_api::irpc as pb;
use tracing::{error, info, Instrument};

pub struct ExecuteEnv<'a, 'b> {
    pub block: &'a mut BlockInfo<'b>,
    pub contract_cluster: &'a mut Cluster,
    pub log_handler: Option<CommandSender>,
}

pub struct TransactionContext<'a, 'b> {
    pub block: &'a mut BlockInfo<'b>,
    pub mq: &'a SignedMessageChannel,
    pub secret_mq: SecretMessageChannel<'a, SignedMessageChannel>,
    pub log_handler: Option<CommandSender>,
}

pub struct QueryContext {
    pub block_number: BlockNumber,
    pub now_ms: u64,
    pub sidevm_handle: Option<SidevmHandle>,
    pub log_handler: Option<CommandSender>,
    pub query_scheduler: RequestScheduler<AccountId>,
    pub weight: u32,
    pub worker_pubkey: [u8; 32],
    pub chain_storage: ChainStorage,
    pub req_id: u64,
}

pub(crate) struct RawData(Vec<u8>);

impl Decode for RawData {
    fn decode<I: parity_scale_codec::Input>(
        input: &mut I,
    ) -> Result<Self, parity_scale_codec::Error> {
        // The remaining_len is not guaranteed to be correct by the trait Input definition. We only
        // decode the RawData with <&[u8] as Input>, which obviously impl the correct remaining_len.
        let mut remaining_len = input
            .remaining_len()?
            .ok_or("Can not decode RawData without length")?;
        let mut decoded = Vec::with_capacity(remaining_len);
        let mut buf = [0u8; 256];
        loop {
            let chunk = remaining_len.min(buf.len());
            input.read(&mut buf[..chunk])?;
            decoded.extend_from_slice(&buf[..chunk]);
            remaining_len -= chunk;
            if remaining_len == 0 {
                break;
            }
        }
        Ok(RawData(decoded))
    }
}

#[derive(Clone)]
pub enum SidevmHandle {
    Running(CommandSender),
    Stopped(ExitReason),
}

impl Serialize for SidevmHandle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SidevmHandle::Running(_) => ExitReason::Restore.serialize(serializer),
            SidevmHandle::Stopped(r) => r.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SidevmHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let reason = ExitReason::deserialize(deserializer)?;
        Ok(SidevmHandle::Stopped(reason))
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct SidevmInfo {
    code: Vec<u8>,
    code_hash: H256,
    start_time: String,
    auto_restart: bool,
    handle: Arc<Mutex<SidevmHandle>>,
}

pub(crate) enum SidevmCode {
    Hash(H256),
    Code(Vec<u8>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contract {
    send_mq: SignedMessageChannel,
    cmd_rcv_mq: SecretReceiver<RawData>,
    #[serde(with = "crate::secret_channel::ecdh_serde")]
    ecdh_key: KeyPair,
    cluster_id: indranet_mq::ContractClusterId,
    address: AccountId,
    sidevm_info: Option<SidevmInfo>,
    weight: u32,
    code_hash: Option<H256>,
    on_block_end: Option<OnBlockEnd>,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
struct OnBlockEnd {
    selector: u32,
    gas_limit: u64,
}

impl Contract {
    pub(crate) fn new(
        send_mq: SignedMessageChannel,
        cmd_rcv_mq: SecretReceiver<RawData>,
        ecdh_key: KeyPair,
        cluster_id: indranet_mq::ContractClusterId,
        address: AccountId,
        code_hash: Option<H256>,
    ) -> Self {
        Contract {
            send_mq,
            cmd_rcv_mq,
            ecdh_key,
            cluster_id,
            address,
            sidevm_info: None,
            weight: 0,
            code_hash,
            on_block_end: None,
        }
    }

    pub(crate) fn address(&self) -> &AccountId {
        &self.address
    }

    pub(crate) fn sidevm_handle(&self) -> Option<SidevmHandle> {
        self.sidevm_info
            .as_ref()
            .map(|info| info.handle.lock().unwrap().clone())
    }

    pub(crate) fn process_next_message(
        &mut self,
        env: &mut ExecuteEnv,
    ) -> Option<TransactionResult> {
        let secret_mq = SecretMessageChannel::new(&self.ecdh_key, &self.send_mq);
        let mut context = TransactionContext {
            block: env.block,
            mq: &self.send_mq,
            secret_mq,
            log_handler: env.log_handler.clone(),
        };

        indranet_mq::select! {
            next_cmd = self.cmd_rcv_mq => match next_cmd {
                Ok((_, cmd, origin)) => {
                    info!("Contract {:?} handling command", self.address());
                    let Ok(command) = Decode::decode(&mut &cmd.0[..]) else {
                        error!("Failed to decode command input");
                        return Some(Err(TransactionError::BadInput));
                    };
                    env.contract_cluster.handle_command(self.address(), origin, command, &mut context)
                }
                Err(_e) => {
                    Err(TransactionError::ChannelError)
                }
            },
        }
    }

    pub(crate) fn on_block_end(&mut self, env: &mut ExecuteEnv) -> TransactionResult {
        let Some(OnBlockEnd { selector, gas_limit }) = self.on_block_end else {
            return Ok(None);
        };

        let input_data = selector.to_be_bytes();
        let tx_args = TransactionArguments {
            origin: self.address.clone(),
            transfer: 0,
            gas_free: false,
            storage_deposit_limit: None,
            gas_limit,
        };
        let mut handle = env.contract_cluster.runtime_mut(env.log_handler.clone());
        _ = handle.call(
            self.address().clone(),
            input_data.to_vec(),
            ExecutionMode::Transaction,
            tx_args,
        );
        Ok(handle.effects)
    }

    pub(crate) fn set_on_block_end_selector(&mut self, selector: u32, gas_limit: u64) {
        self.on_block_end = Some(OnBlockEnd {
            selector,
            gas_limit,
        });
    }

    pub(crate) fn start_sidevm(
        &mut self,
        spawner: &sidevm::service::Spawner,
        code: SidevmCode,
        ensure_waiting_code: bool,
    ) -> Result<()> {
        let handle = self.sidevm_handle();
        if let Some(SidevmHandle::Running(_)) = &handle {
            bail!("Sidevm can only be started once");
        }

        let (code, code_hash) = match code {
            SidevmCode::Hash(hash) => (vec![], hash),
            SidevmCode::Code(code) => {
                let actual_hash = sp_core::blake2_256(&code).into();
                if ensure_waiting_code {
                    if !matches!(
                        &handle,
                        Some(SidevmHandle::Stopped(ExitReason::WaitingForCode))
                    ) {
                        bail!("The sidevm isn't waiting for code");
                    }
                    let expected_hash = self
                        .sidevm_info
                        .as_ref()
                        .ok_or_else(|| anyhow!("No sidevm info"))?
                        .code_hash;
                    if actual_hash != expected_hash {
                        bail!(
                            "Code hash mismatch, expected: {expected_hash:?}, actual: {actual_hash:?}"
                        );
                    }
                }
                (code, actual_hash)
            }
        };

        let handle = if code.is_empty() {
            info!("Sidevm code {code_hash:?} not found, waiting to be uploaded");
            Arc::new(Mutex::new(SidevmHandle::Stopped(
                ExitReason::WaitingForCode,
            )))
        } else {
            do_start_sidevm(spawner, &code, *self.address.as_ref(), self.weight)?
        };

        let start_time = chrono::Utc::now().to_rfc3339();
        self.sidevm_info = Some(SidevmInfo {
            code,
            code_hash,
            start_time,
            handle,
            auto_restart: true,
        });
        Ok(())
    }

    pub(crate) fn restart_sidevm_if_needed(
        &mut self,
        spawner: &sidevm::service::Spawner,
    ) -> Result<()> {
        if let Some(sidevm_info) = &mut self.sidevm_info {
            let guard = sidevm_info.handle.lock().unwrap();
            let handle = if let SidevmHandle::Stopped(reason) = &*guard {
                let need_restart = match reason {
                    ExitReason::Exited(_) => false,
                    ExitReason::Stopped => false,
                    ExitReason::InputClosed => false,
                    ExitReason::Panicked => true,
                    ExitReason::Cancelled => false,
                    // TODO.kevin: Allow to charge new gas? How to charge gas or weather the gas
                    // system works or not is not clear ATM.
                    ExitReason::OcallAborted(OcallAborted::GasExhausted) => false,
                    ExitReason::OcallAborted(OcallAborted::Stifled) => true,
                    ExitReason::Restore => true,
                    ExitReason::WaitingForCode => false,
                };
                if !need_restart {
                    return Ok(());
                }
                sidevm_info.start_time = chrono::Utc::now().to_rfc3339();
                do_start_sidevm(
                    spawner,
                    &sidevm_info.code,
                    *self.address.as_ref(),
                    self.weight,
                )?
            } else {
                return Ok(());
            };
            drop(guard);
            sidevm_info.handle = handle;
        }
        Ok(())
    }

    pub(crate) fn push_message_to_sidevm(&self, message: SidevmCommand) -> Result<()> {
        let handle = self
            .sidevm_info
            .as_ref()
            .ok_or_else(|| anyhow!("Push message to sidevm failed, no sidevm instance"))?
            .handle
            .clone();

        let vmid = sidevm::ShortId(&self.address);
        let span = tracing::info_span!("sidevm:push", %vmid);
        let _enter = span.enter();

        let tx = match &*handle.lock().unwrap() {
            SidevmHandle::Stopped(_) => {
                error!(target: "sidevm", "PM to sidevm failed, instance terminated");
                return Err(anyhow!(
                    "Push message to sidevm failed, instance terminated"
                ));
            }
            SidevmHandle::Running(tx) => tx.clone(),
        };
        let result = tx.try_send(message);
        if let Err(err) = result {
            use tokio::sync::mpsc::error::TrySendError;
            match err {
                TrySendError::Full(_) => {
                    error!(target: "sidevm", "PM to sidevm failed (channel full), the guest program may be stucked");
                }
                TrySendError::Closed(_) => {
                    error!(target: "sidevm", "PM to sidevm failed (channel closed), the VM might be already stopped");
                }
            }
        }
        Ok(())
    }

    pub(crate) fn get_system_message_handler(&self) -> Option<CommandSender> {
        let guard = self.sidevm_info.as_ref()?.handle.lock().unwrap();
        match &*guard {
            SidevmHandle::Stopped(_) => None,
            SidevmHandle::Running(tx) => Some(tx.clone()),
        }
    }

    pub(crate) fn destroy(self, spawner: &sidevm::service::Spawner) {
        if let Some(sidevm_info) = &self.sidevm_info {
            match sidevm_info.handle.lock().unwrap().clone() {
                SidevmHandle::Stopped(_) => {}
                SidevmHandle::Running(tx) => {
                    spawner.spawn(
                        async move {
                            if let Err(err) = tx.send(SidevmCommand::Stop).await {
                                error!("Failed to send stop command to sidevm: {}", err);
                            }
                        }
                        .in_current_span(),
                    );
                }
            }
        }
    }

    pub fn set_weight(&mut self, weight: u32) {
        self.weight = weight;
        info!(
            "Updated weight for contarct {:?} to {}",
            self.address(),
            weight
        );
        if let Some(SidevmHandle::Running(tx)) = self.sidevm_handle() {
            if tx.try_send(SidevmCommand::UpdateWeight(weight)).is_err() {
                error!("Failed to update weight for sidevm, maybe it has crashed");
            }
        }
    }
    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn info(&self) -> pb::ContractInfo {
        pb::ContractInfo {
            id: hex(&self.address),
            weight: self.weight,
            code_hash: self.code_hash.as_ref().map(hex).unwrap_or_default(),
            sidevm: self.sidevm_info.as_ref().map(|info| {
                let handle = info.handle.lock().unwrap().clone();
                let start_time = info.start_time.clone();
                let code_hash = hex(info.code_hash);
                match handle {
                    SidevmHandle::Running(_) => pb::SidevmInfo {
                        state: "running".into(),
                        code_hash,
                        start_time,
                        ..Default::default()
                    },
                    SidevmHandle::Stopped(reason) => pb::SidevmInfo {
                        state: "stopped".into(),
                        code_hash,
                        start_time,
                        stop_reason: format!("{reason}"),
                    },
                }
            }),
        }
    }
}

#[tracing::instrument(name="sidevm", skip_all, fields(id=%sidevm::ShortId(&id)))]
fn do_start_sidevm(
    spawner: &sidevm::service::Spawner,
    code: &[u8],
    id: VmId,
    weight: u32,
) -> Result<Arc<Mutex<SidevmHandle>>> {
    info!(target: "sidevm", "Starting sidevm...");
    let max_memory_pages: u32 = 1024; // 64MB
    let gas_per_breath = 50_000_000_000_u64; // about 20 ms bench
    let (sender, join_handle) = spawner.start(
        code,
        max_memory_pages,
        id,
        gas_per_breath,
        local_cache_ops(),
        weight,
    )?;
    let handle = Arc::new(Mutex::new(SidevmHandle::Running(sender)));
    let cloned_handle = handle.clone();
    spawner.spawn(
        async move {
            let reason = join_handle.await.unwrap_or(ExitReason::Cancelled);
            error!(target: "sidevm", ?reason, "Sidevm process terminated");
            *cloned_handle.lock().unwrap() = SidevmHandle::Stopped(reason);
        }
        .in_current_span(),
    );
    Ok(handle)
}

fn local_cache_ops() -> sidevm::DynCacheOps {
    use ::pink::local_cache as cache;
    type OpResult<T> = Result<T, sidevm::OcallError>;

    struct CacheOps;
    impl sidevm::CacheOps for CacheOps {
        fn get(&self, contract: &[u8], key: &[u8]) -> OpResult<Option<Vec<u8>>> {
            Ok(cache::get(contract, key))
        }

        fn set(&self, contract: &[u8], key: &[u8], value: &[u8]) -> OpResult<()> {
            cache::set(contract, key, value).map_err(|_| sidevm::OcallError::ResourceLimited)
        }

        fn set_expiration(
            &self,
            contract: &[u8],
            key: &[u8],
            expire_after_secs: u64,
        ) -> OpResult<()> {
            cache::set_expiration(contract, key, expire_after_secs);
            Ok(())
        }

        fn remove(&self, contract: &[u8], key: &[u8]) -> OpResult<Option<Vec<u8>>> {
            Ok(cache::remove(contract, key))
        }
    }
    &CacheOps
}

pub use keeper::*;
mod keeper;
