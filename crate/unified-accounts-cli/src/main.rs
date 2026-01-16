//! # Selendra Unified Accounts CLI
//!
//! A command-line tool for binding Substrate accounts to EVM accounts on the Selendra network.
//!
//! ## Features
//!
//! - **Claim EVM Address**: Bind a Substrate account to a specific EVM address you control
//! - **Claim Default EVM Address**: Bind a Substrate account to its default derived EVM address
//! - **Query Mappings**: Check existing account mappings
//! - **Generate Signatures**: Create EIP-712 signatures for claiming
//!
//! ## Usage
//!
//! ```bash
//! # Claim a specific EVM address
//! unified-accounts claim-evm --seed "your seed phrase" --evm-key "your evm private key"
//!
//! # Claim default EVM address
//! unified-accounts claim-default --seed "your seed phrase"
//!
//! # Query mapping for a Substrate account
//! unified-accounts query --account "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
//!
//! # Generate signature only (for offline signing)
//! unified-accounts sign --seed "your seed phrase" --evm-key "your evm private key"
//! ```

use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use codec::Encode;
use hex::FromHex;
use libsecp256k1::{PublicKey, SecretKey};
use log::{debug, info};
use sp_core::{
    crypto::Ss58Codec,
    sr25519, Pair, H160, H256,
};
use sp_core_hashing::{blake2_256, keccak_256};
use subxt::{
    config::polkadot::PolkadotExtrinsicParams,
    ext::sp_runtime::{MultiAddress, MultiSignature},
    Config, OnlineClient, PolkadotConfig,
};

mod eip712;
mod tx;

use eip712::Eip712;
use tx::{submit_claim_default_evm_address, submit_claim_evm_address, query_mapping};

/// Selendra chain configuration for subxt
pub enum SelendraConfig {}

impl Config for SelendraConfig {
    type Hash = <PolkadotConfig as Config>::Hash;
    type AccountId = sp_core::crypto::AccountId32;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Signature = MultiSignature;
    type Hasher = <PolkadotConfig as Config>::Hasher;
    type Header = <PolkadotConfig as Config>::Header;
    type ExtrinsicParams = PolkadotExtrinsicParams<Self>;
}

/// EVM Address type
pub type EvmAddress = H160;

/// ECDSA Signature (65 bytes with recovery id)
pub type EvmSignature = [u8; 65];

/// Account ID type
pub type AccountId = sp_core::crypto::AccountId32;

#[derive(Parser)]
#[command(name = "unified-accounts")]
#[command(author = "Selendra Team")]
#[command(version = "1.0.0")]
#[command(about = "CLI tool for binding Substrate accounts to EVM accounts on Selendra")]
#[command(long_about = r#"
Selendra Unified Accounts CLI

This tool allows you to create mappings between your Substrate (native) account
and EVM (Ethereum-compatible) account on the Selendra network.

IMPORTANT WARNINGS:
- Once a mapping is created, it CANNOT be changed
- When claiming a specific EVM address, native balance from the default account
  will be transferred to your account
- Other assets (XC20 tokens, unclaimed staking rewards, etc.) must be transferred
  manually BEFORE claiming, or they will be LOST FOREVER
- A small storage fee is charged when creating mappings

For more information, visit: https://docs.selendra.org/unified-accounts
"#)]
struct Cli {
    /// WebSocket endpoint of the Selendra node
    #[arg(long, default_value = "wss://rpc-testnet.selendra.org:443")]
    endpoint: String,

    /// Chain ID for EVM (used in EIP-712 signature). 
    /// Set to 1961 for testnet, will be verified against runtime constant.
    #[arg(long, default_value = "1961")]
    chain_id: u64,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Claim a specific EVM address by providing proof of ownership
    ClaimEvm {
        /// Substrate account seed phrase (12 or 24 words) or hex secret
        #[arg(long, env = "SUBSTRATE_SEED")]
        seed: String,

        /// EVM private key (hex format, with or without 0x prefix)
        #[arg(long, env = "EVM_PRIVATE_KEY")]
        evm_key: String,

        /// Wait for transaction to be finalized (default: wait for inclusion)
        #[arg(long, default_value = "false")]
        finalized: bool,
    },

    /// Claim the default EVM address derived from your Substrate account
    ClaimDefault {
        /// Substrate account seed phrase (12 or 24 words) or hex secret
        #[arg(long, env = "SUBSTRATE_SEED")]
        seed: String,

        /// Wait for transaction to be finalized (default: wait for inclusion)
        #[arg(long, default_value = "false")]
        finalized: bool,
    },

    /// Query account mappings
    Query {
        /// Substrate account address (SS58 format)
        #[arg(long, conflicts_with = "evm_address")]
        account: Option<String>,

        /// EVM address (hex format, with or without 0x prefix)
        #[arg(long, conflicts_with = "account")]
        evm_address: Option<String>,
    },

    /// Generate an EIP-712 signature for claiming an EVM address (offline)
    Sign {
        /// Substrate account seed phrase (12 or 24 words) or hex secret
        #[arg(long, env = "SUBSTRATE_SEED")]
        seed: String,

        /// EVM private key (hex format, with or without 0x prefix)
        #[arg(long, env = "EVM_PRIVATE_KEY")]
        evm_key: String,

        /// Genesis block hash (required for signature generation)
        #[arg(long)]
        genesis_hash: String,
    },

    /// Show information about accounts without submitting transactions
    Info {
        /// Substrate account seed phrase (12 or 24 words) or hex secret
        #[arg(long)]
        seed: Option<String>,

        /// EVM private key (hex format, with or without 0x prefix)
        #[arg(long)]
        evm_key: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = Cli::parse();

    match cli.command {
        Commands::ClaimEvm { seed, evm_key, finalized } => {
            claim_evm_address(&cli.endpoint, &seed, &evm_key, cli.chain_id, finalized).await?;
        }
        Commands::ClaimDefault { seed, finalized } => {
            claim_default_evm_address(&cli.endpoint, &seed, finalized).await?;
        }
        Commands::Query { account, evm_address } => {
            query_accounts(&cli.endpoint, account, evm_address).await?;
        }
        Commands::Sign { seed, evm_key, genesis_hash } => {
            generate_signature(&seed, &evm_key, cli.chain_id, &genesis_hash)?;
        }
        Commands::Info { seed, evm_key } => {
            show_info(seed, evm_key)?;
        }
    }

    Ok(())
}

/// Claim a specific EVM address
async fn claim_evm_address(
    endpoint: &str,
    seed: &str,
    evm_key: &str,
    chain_id: u64,
    finalized: bool,
) -> Result<()> {
    info!("Connecting to {}", endpoint);
    let client = OnlineClient::<SelendraConfig>::from_url(endpoint)
        .await
        .context("Failed to connect to node")?;

    // Parse Substrate keypair
    let substrate_pair = sr25519::Pair::from_string(seed, None)
        .map_err(|e| anyhow!("Invalid substrate seed: {:?}", e))?;
    let account_id = AccountId::from(substrate_pair.public());
    
    info!("Substrate account: {}", account_id.to_ss58check());
    debug!("Account bytes (SCALE encoded): 0x{}", hex::encode(account_id.encode()));

    // Parse EVM private key
    let evm_secret = parse_evm_private_key(evm_key)?;
    let evm_address = evm_address_from_secret(&evm_secret);
    
    info!("EVM address: 0x{}", hex::encode(evm_address.as_bytes()));

    // Get genesis hash for EIP-712 signature
    let genesis_hash = client.genesis_hash();
    info!("Genesis hash: 0x{}", hex::encode(genesis_hash.as_bytes()));
    
    // Try to get the chain ID from the runtime constant
    let runtime_chain_id = query_mapping::get_chain_id(&client).await?;
    
    // Use runtime chain_id if it differs from user provided
    let actual_chain_id = if runtime_chain_id != chain_id {
        println!("⚠️  Warning: Using runtime chain ID {} instead of provided {}", runtime_chain_id, chain_id);
        runtime_chain_id
    } else {
        chain_id
    };
    debug!("Using Chain ID: {}", actual_chain_id);

    // Generate EIP-712 signature
    debug!("Generating EIP-712 signature...");
    let signature = Eip712::sign(&account_id, &evm_secret, actual_chain_id, H256::from_slice(genesis_hash.as_ref()));
    debug!("Generated signature: 0x{}", hex::encode(&signature));
    
    // Verify the signature locally before submitting
    let recovered = Eip712::verify(&account_id, &signature, actual_chain_id, H256::from_slice(genesis_hash.as_ref()));
    match recovered {
        Some(addr) => {
            debug!("Recovered address: 0x{}", hex::encode(addr.as_bytes()));
            if addr != evm_address {
                return Err(anyhow!("Signature verification failed: recovered address doesn't match EVM address"));
            }
            debug!("✓ Local signature verification passed");
        }
        None => {
            return Err(anyhow!("Failed to recover address from signature"));
        }
    }

    // Submit transaction
    info!("Submitting claim_evm_address transaction...");
    let tx_info = submit_claim_evm_address(
        &client,
        substrate_pair,
        evm_address,
        signature,
        finalized,
    ).await?;

    println!("\n✅ Successfully claimed EVM address!");
    println!("   Substrate account: {}", account_id.to_ss58check());
    println!("   EVM address: 0x{}", hex::encode(evm_address.as_bytes()));
    println!("   Block hash: 0x{}", hex::encode(tx_info.block_hash.as_bytes()));
    println!("   Tx hash: 0x{}", hex::encode(tx_info.tx_hash.as_bytes()));

    Ok(())
}

/// Claim the default EVM address
async fn claim_default_evm_address(
    endpoint: &str,
    seed: &str,
    finalized: bool,
) -> Result<()> {
    info!("Connecting to {}", endpoint);
    let client = OnlineClient::<SelendraConfig>::from_url(endpoint)
        .await
        .context("Failed to connect to node")?;

    // Parse Substrate keypair
    let substrate_pair = sr25519::Pair::from_string(seed, None)
        .map_err(|e| anyhow!("Invalid substrate seed: {:?}", e))?;
    let account_id = AccountId::from(substrate_pair.public());
    
    info!("Substrate account: {}", account_id.to_ss58check());

    // Calculate default EVM address
    let default_evm = default_evm_address(&account_id);
    info!("Default EVM address: 0x{}", hex::encode(default_evm.as_bytes()));

    // Submit transaction
    info!("Submitting claim_default_evm_address transaction...");
    let tx_info = submit_claim_default_evm_address(
        &client,
        substrate_pair,
        finalized,
    ).await?;

    println!("\n✅ Successfully claimed default EVM address!");
    println!("   Substrate account: {}", account_id.to_ss58check());
    println!("   Default EVM address: 0x{}", hex::encode(default_evm.as_bytes()));
    println!("   Block hash: 0x{}", hex::encode(tx_info.block_hash.as_bytes()));
    println!("   Tx hash: 0x{}", hex::encode(tx_info.tx_hash.as_bytes()));

    Ok(())
}

/// Query account mappings
async fn query_accounts(
    endpoint: &str,
    account: Option<String>,
    evm_address: Option<String>,
) -> Result<()> {
    info!("Connecting to {}", endpoint);
    let client = OnlineClient::<SelendraConfig>::from_url(endpoint)
        .await
        .context("Failed to connect to node")?;

    if let Some(account_str) = account {
        let account_id = AccountId::from_ss58check(&account_str)
            .map_err(|_| anyhow!("Invalid SS58 account address"))?;
        
        println!("Querying EVM address for Substrate account: {}", account_str);
        
        match query_mapping::get_evm_address(&client, &account_id).await? {
            Some(evm_addr) => {
                println!("✅ Mapped EVM address: 0x{}", hex::encode(evm_addr.as_bytes()));
            }
            None => {
                let default_evm = default_evm_address(&account_id);
                println!("❌ No mapping found");
                println!("   Default EVM address (if claimed): 0x{}", hex::encode(default_evm.as_bytes()));
            }
        }
    } else if let Some(evm_str) = evm_address {
        let evm_addr = parse_evm_address(&evm_str)?;
        
        println!("Querying Substrate account for EVM address: 0x{}", hex::encode(evm_addr.as_bytes()));
        
        match query_mapping::get_native_account(&client, &evm_addr).await? {
            Some(account_id) => {
                println!("✅ Mapped Substrate account: {}", account_id.to_ss58check());
            }
            None => {
                let default_account = default_account_id(&evm_addr);
                println!("❌ No mapping found");
                println!("   Default Substrate account (if claimed): {}", default_account.to_ss58check());
            }
        }
    } else {
        return Err(anyhow!("Please specify either --account or --evm-address"));
    }

    Ok(())
}

/// Generate signature offline
fn generate_signature(
    seed: &str,
    evm_key: &str,
    chain_id: u64,
    genesis_hash: &str,
) -> Result<()> {
    // Parse Substrate account
    let substrate_pair = sr25519::Pair::from_string(seed, None)
        .map_err(|e| anyhow!("Invalid substrate seed: {:?}", e))?;
    let account_id = AccountId::from(substrate_pair.public());

    // Parse EVM private key
    let evm_secret = parse_evm_private_key(evm_key)?;
    let evm_address = evm_address_from_secret(&evm_secret);

    // Parse genesis hash
    let genesis = parse_hash(genesis_hash)?;

    // Generate signature
    let signature = Eip712::sign(&account_id, &evm_secret, chain_id, genesis);

    println!("\n📝 EIP-712 Signature Generated");
    println!("================================");
    println!("Substrate account: {}", account_id.to_ss58check());
    println!("EVM address: 0x{}", hex::encode(evm_address.as_bytes()));
    println!("Chain ID: {}", chain_id);
    println!("Genesis hash: 0x{}", hex::encode(genesis.as_bytes()));
    println!("\nSignature (hex): 0x{}", hex::encode(&signature));
    println!("\nUse this signature with the claim_evm_address extrinsic.");

    Ok(())
}

/// Show account information
fn show_info(seed: Option<String>, evm_key: Option<String>) -> Result<()> {
    println!("\n📋 Account Information");
    println!("======================\n");

    if let Some(ref seed) = seed {
        let substrate_pair = sr25519::Pair::from_string(seed, None)
            .map_err(|e| anyhow!("Invalid substrate seed: {:?}", e))?;
        let account_id = AccountId::from(substrate_pair.public());
        let default_evm = default_evm_address(&account_id);

        println!("Substrate Account:");
        println!("  Address (SS58): {}", account_id.to_ss58check());
        println!("  Public key: 0x{}", hex::encode(substrate_pair.public().0));
        println!("  Default EVM address: 0x{}", hex::encode(default_evm.as_bytes()));
        println!();
    }

    if let Some(ref evm_key) = evm_key {
        let evm_secret = parse_evm_private_key(evm_key)?;
        let evm_address = evm_address_from_secret(&evm_secret);
        let default_substrate = default_account_id(&evm_address);

        println!("EVM Account:");
        println!("  Address: 0x{}", hex::encode(evm_address.as_bytes()));
        println!("  Default Substrate address: {}", default_substrate.to_ss58check());
        println!();
    }

    if seed.is_none() && evm_key.is_none() {
        println!("No account information provided.");
        println!("Use --seed for Substrate account or --evm-key for EVM account.");
    }

    Ok(())
}

// Helper functions

fn parse_evm_private_key(key: &str) -> Result<SecretKey> {
    let key = key.trim_start_matches("0x");
    let bytes = <[u8; 32]>::from_hex(key)
        .map_err(|_| anyhow!("Invalid EVM private key format (expected 32 bytes hex)"))?;
    SecretKey::parse(&bytes)
        .map_err(|e| anyhow!("Invalid secp256k1 private key: {:?}", e))
}

fn parse_evm_address(addr: &str) -> Result<EvmAddress> {
    let addr = addr.trim_start_matches("0x");
    let bytes = <[u8; 20]>::from_hex(addr)
        .map_err(|_| anyhow!("Invalid EVM address format (expected 20 bytes hex)"))?;
    Ok(EvmAddress::from_slice(&bytes))
}

fn parse_hash(hash: &str) -> Result<H256> {
    let hash = hash.trim_start_matches("0x");
    let bytes = <[u8; 32]>::from_hex(hash)
        .map_err(|_| anyhow!("Invalid hash format (expected 32 bytes hex)"))?;
    Ok(H256::from_slice(&bytes))
}

fn evm_address_from_secret(secret: &SecretKey) -> EvmAddress {
    let public = PublicKey::from_secret_key(secret);
    let public_bytes = &public.serialize()[1..65];
    let hash = keccak_256(public_bytes);
    EvmAddress::from_slice(&hash[12..])
}

fn default_evm_address(account_id: &AccountId) -> EvmAddress {
    let hash = blake2_256(&account_id.encode());
    EvmAddress::from_slice(&hash[0..20])
}

fn default_account_id(evm_address: &EvmAddress) -> AccountId {
    let mut data = b"evm:".to_vec();
    data.extend_from_slice(evm_address.as_bytes());
    data.resize(32, 0u8);
    AccountId::from(blake2_256(&data))
}
