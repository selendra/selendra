//! Transaction submission module for Unified Accounts CLI
//!
//! This module handles the construction and submission of extrinsics
//! for the unified accounts pallet.

use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use sp_core::{sr25519, H160, H256};
use subxt::{
    blocks::ExtrinsicEvents,
    dynamic::Value,
    tx::{PairSigner, TxPayload},
    OnlineClient,
};

use crate::{AccountId, EvmSignature, SelendraConfig};

/// Transaction info returned after submission
#[derive(Debug, Clone)]
pub struct TxInfo {
    pub block_hash: H256,
    pub tx_hash: H256,
}

impl<T: subxt::Config> From<ExtrinsicEvents<T>> for TxInfo {
    fn from(events: ExtrinsicEvents<T>) -> Self {
        Self {
            block_hash: H256::from_slice(events.block_hash().as_ref()),
            tx_hash: H256::from_slice(events.extrinsic_hash().as_ref()),
        }
    }
}

/// Submit claim_evm_address extrinsic
pub async fn submit_claim_evm_address(
    client: &OnlineClient<SelendraConfig>,
    signer: sr25519::Pair,
    evm_address: H160,
    signature: EvmSignature,
    finalized: bool,
) -> Result<TxInfo> {
    // Build the extrinsic using dynamic API
    // Pallet index: 87 (UnifiedAccounts)
    // Call index: 0 (claim_evm_address)
    let tx = subxt::dynamic::tx(
        "UnifiedAccounts",
        "claim_evm_address",
        vec![
            // evm_address: H160
            Value::from_bytes(evm_address.as_bytes()),
            // signature: [u8; 65]
            Value::from_bytes(&signature),
        ],
    );

    submit_tx(client, signer, tx, finalized).await
}

/// Submit claim_default_evm_address extrinsic
pub async fn submit_claim_default_evm_address(
    client: &OnlineClient<SelendraConfig>,
    signer: sr25519::Pair,
    finalized: bool,
) -> Result<TxInfo> {
    // Build the extrinsic using dynamic API
    // Pallet index: 87 (UnifiedAccounts)
    // Call index: 1 (claim_default_evm_address)
    let tx = subxt::dynamic::tx(
        "UnifiedAccounts",
        "claim_default_evm_address",
        Vec::<Value<()>>::new(),
    );

    submit_tx(client, signer, tx, finalized).await
}

/// Submit a transaction and wait for the appropriate status
async fn submit_tx<T: TxPayload>(
    client: &OnlineClient<SelendraConfig>,
    signer: sr25519::Pair,
    tx: T,
    finalized: bool,
) -> Result<TxInfo> {
    let pair_signer: PairSigner<SelendraConfig, sr25519::Pair> = PairSigner::new(signer);

    let tx_progress = client
        .tx()
        .sign_and_submit_then_watch_default(&tx, &pair_signer)
        .await
        .context("Failed to submit transaction")?;

    info!("Transaction submitted, waiting for confirmation...");
    debug!("Tx hash: 0x{}", hex::encode(tx_progress.extrinsic_hash().as_ref()));

    let events = if finalized {
        tx_progress
            .wait_for_finalized_success()
            .await
            .context("Transaction failed or not finalized")?
    } else {
        tx_progress
            .wait_for_in_block()
            .await
            .context("Transaction not included in block")?
            .wait_for_success()
            .await
            .context("Transaction failed")?
    };

    Ok(TxInfo::from(events))
}

/// Query module for reading storage
pub mod query_mapping {
    use super::*;

    /// Get EVM address for a native account
    pub async fn get_evm_address(
        client: &OnlineClient<SelendraConfig>,
        account_id: &AccountId,
    ) -> Result<Option<H160>> {
        let account_bytes: &[u8] = account_id.as_ref();
        let storage_address = subxt::dynamic::storage(
            "UnifiedAccounts",
            "NativeToEvm",
            vec![Value::from_bytes(account_bytes)],
        );

        let result = client
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_address)
            .await
            .context("Failed to query storage")?;

        match result {
            Some(value) => {
                // Get the encoded bytes and decode
                let encoded = value.encoded();
                if encoded.len() >= 20 {
                    let mut bytes = [0u8; 20];
                    bytes.copy_from_slice(&encoded[..20]);
                    Ok(Some(H160::from_slice(&bytes)))
                } else {
                    Err(anyhow!("Invalid EVM address data"))
                }
            }
            None => Ok(None),
        }
    }

    /// Get native account for an EVM address
    pub async fn get_native_account(
        client: &OnlineClient<SelendraConfig>,
        evm_address: &H160,
    ) -> Result<Option<AccountId>> {
        let storage_address = subxt::dynamic::storage(
            "UnifiedAccounts",
            "EvmToNative",
            vec![Value::from_bytes(evm_address.as_bytes())],
        );

        let result = client
            .storage()
            .at_latest()
            .await?
            .fetch(&storage_address)
            .await
            .context("Failed to query storage")?;

        match result {
            Some(value) => {
                // Get the encoded bytes and decode
                let encoded = value.encoded();
                if encoded.len() >= 32 {
                    let mut bytes = [0u8; 32];
                    bytes.copy_from_slice(&encoded[..32]);
                    Ok(Some(AccountId::from(bytes)))
                } else {
                    Err(anyhow!("Invalid account ID data"))
                }
            }
            None => Ok(None),
        }
    }

    /// Get the chain ID from the pallet constant
    pub async fn get_chain_id(
        client: &OnlineClient<SelendraConfig>,
    ) -> Result<u64> {
        // Access runtime constants via metadata
        let constant_address = subxt::dynamic::constant("UnifiedAccounts", "ChainId");
        
        let value = client
            .constants()
            .at(&constant_address)
            .context("Failed to get ChainId constant")?;

        // Decode the value from encoded bytes - it's a u64 SCALE encoded
        let encoded = value.encoded();
        if encoded.len() < 8 {
            return Err(anyhow!("ChainId constant too short: {} bytes", encoded.len()));
        }
        
        // u64 is SCALE encoded as little-endian 8 bytes
        let chain_id = u64::from_le_bytes([
            encoded[0], encoded[1], encoded[2], encoded[3],
            encoded[4], encoded[5], encoded[6], encoded[7],
        ]);
        
        Ok(chain_id)
    }
}
