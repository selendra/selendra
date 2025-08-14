use anyhow::Result;
use clap::{Parser, Subcommand};
use hex;
use libsecp256k1::{Message, SecretKey, sign};
use parity_scale_codec::Encode;
use std::str::FromStr;
use subxt::{
    client::OnlineClient,
    config::{Config, DefaultExtrinsicParams},
    tx::PairSigner,
    utils::AccountId32,
};
use sp_core::{
    crypto::{Ss58Codec, Pair},
    sr25519, H256, H160,
};
use sp_runtime::traits::Zero;
use tokio;

// Define our custom config for Selendra
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SelendraConfig;

impl Config for SelendraConfig {
    type Hash = subxt::utils::H256;
    type AccountId = AccountId32;
    type Address = subxt::utils::MultiAddress<Self::AccountId, ()>;
    type Signature = subxt::utils::MultiSignature;
    type Hasher = subxt::config::substrate::BlakeTwo256;
    type Header = subxt::config::substrate::SubstrateHeader<u32, subxt::config::substrate::BlakeTwo256>;
    type ExtrinsicParams = DefaultExtrinsicParams<Self>;
    type AssetId = ();
}

// Generate the interface from metadata
#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod selendra {}

type EvmAddress = [u8; 20];
type EvmSignature = [u8; 65];

#[derive(Parser)]
#[command(name = "bind_account")]
#[command(about = "Selendra Unified Accounts CLI - Bind Substrate and EVM addresses")]
#[command(version = "1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Claim default EVM address for your account
    ClaimDefault {
        /// Substrate account mnemonic phrase
        #[arg(short, long)]
        mnemonic: String,
        /// Chain RPC endpoint
        #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
        rpc: String,
    },
    /// Claim specific EVM address with signature proof
    ClaimEvm {
        /// Substrate account mnemonic phrase
        #[arg(short, long)]
        mnemonic: String,
        /// Chain RPC endpoint
        #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
        rpc: String,
    },
    /// Generate signature and addresses without submitting
    Generate {
        /// Substrate account mnemonic phrase
        #[arg(short, long)]
        mnemonic: String,
        /// Chain RPC endpoint (for genesis hash)
        #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
        rpc: String,
    },
    /// Test unified accounts with balance checks and transfers
    Test {
        /// Substrate account mnemonic phrase
        #[arg(short, long)]
        mnemonic: String,
        /// Chain RPC endpoint
        #[arg(short, long, default_value = "ws://127.0.0.1:9944")]
        rpc: String,
        /// Target address for transfers (Substrate format)
        #[arg(short, long)]
        target: String,
        /// Amount to transfer (in smallest unit)
        #[arg(short, long, default_value = "1000000000000")]
        amount: String,
    },
}

/// Keccak-256 hash function
fn keccak_256(input: &[u8]) -> [u8; 32] {
    use tiny_keccak::{Hasher, Keccak};
    let mut hasher = Keccak::v256();
    hasher.update(input);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}

/// Convert u64 to big-endian bytes
fn u64_to_be_bytes(value: u64) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let value_bytes = value.to_be_bytes();
    bytes[24..].copy_from_slice(&value_bytes);
    bytes
}

/// EIP-712 domain separator for unified accounts
fn build_domain_separator(chain_id: u64, genesis_hash: [u8; 32]) -> [u8; 32] {
    let domain_type_hash = keccak_256(b"EIP712Domain(string name,string version,uint256 chainId,bytes32 salt)");
    let name_hash = keccak_256(b"Selendra EVM Claim");
    let version_hash = keccak_256(b"1");
    
    let mut domain = domain_type_hash.to_vec();
    domain.extend_from_slice(&name_hash);
    domain.extend_from_slice(&version_hash);
    domain.extend_from_slice(&u64_to_be_bytes(chain_id));
    domain.extend_from_slice(&genesis_hash);
    
    keccak_256(&domain)
}

/// Build EIP-712 message hash for account claim
fn build_signing_payload(account_id: &AccountId32, chain_id: u64, genesis_hash: [u8; 32]) -> [u8; 32] {
    let domain_separator = build_domain_separator(chain_id, genesis_hash);
    
    let claim_type_hash = keccak_256(b"Claim(bytes substrateAddress)");
    let account_hash = keccak_256(&account_id.encode());
    
    let mut args_hash_data = claim_type_hash.to_vec();
    args_hash_data.extend_from_slice(&account_hash);
    let args_hash = keccak_256(&args_hash_data);
    
    let mut payload = b"\x19\x01".to_vec();
    payload.extend_from_slice(&domain_separator);
    payload.extend_from_slice(&args_hash);
    
    keccak_256(&payload)
}

/// Generate EVM address from private key
fn get_evm_address(secret_key: &SecretKey) -> EvmAddress {
    let public_key = libsecp256k1::PublicKey::from_secret_key(secret_key);
    let public_key_bytes = &public_key.serialize()[1..65]; // Remove first byte (0x04)
    let hash = keccak_256(public_key_bytes);
    let mut address = [0u8; 20];
    address.copy_from_slice(&hash[12..]);
    address
}

/// Sign EIP-712 message with secp256k1 private key
fn sign_eip712_message(secret_key: &SecretKey, message_hash: &[u8; 32]) -> EvmSignature {
    let message = Message::parse(message_hash);
    let (signature, recovery_id) = sign(&message, secret_key);
    
    let mut signature_bytes = [0u8; 65];
    signature_bytes[0..64].copy_from_slice(&signature.serialize());
    signature_bytes[64] = recovery_id.serialize();
    
    signature_bytes
}

async fn generate_account_data(mnemonic: &str, rpc_url: &str) -> Result<(sr25519::Pair, AccountId32, SecretKey, EvmAddress, [u8; 32], EvmSignature)> {
    // Connect to get genesis hash
    let api = OnlineClient::<SelendraConfig>::from_url(rpc_url).await?;
    
    // Parse mnemonic and generate seed
    let mnemonic_obj = bip39::Mnemonic::from_str(mnemonic)?;
    let seed = mnemonic_obj.to_seed("");
    
    // Generate SR25519 keypair from seed
    use subxt::ext::sp_core::{sr25519, Pair};
    let pair = sr25519::Pair::from_seed_slice(&seed[0..32])?;
    let account_id = AccountId32::from(pair.public().0);
    
    // Generate EVM key pair (using same entropy)
    let secret_key = SecretKey::parse_slice(&seed[0..32])?;
    let evm_address = get_evm_address(&secret_key);
    
    // Get chain info
    let chain_id = 1961u64; // Selendra chain ID
    let genesis_hash_h256 = api.genesis_hash();
    let genesis_hash: [u8; 32] = genesis_hash_h256.0;
    
    // Build EIP-712 signing payload and sign
    let message_hash = build_signing_payload(&account_id, chain_id, genesis_hash);
    let signature = sign_eip712_message(&secret_key, &message_hash);
    
    Ok((pair, account_id, secret_key, evm_address, genesis_hash, signature))
}

async fn claim_default_evm_address(mnemonic: String, rpc_url: String) -> Result<()> {
    println!("🚀 Claiming default EVM address...");
    
    let (pair, account_id, _secret_key, _evm_address, genesis_hash, _signature) = 
        generate_account_data(&mnemonic, &rpc_url).await?;
    
    println!("📝 Account: {}", account_id.to_ss58check());
    println!("🔗 Genesis: 0x{}", hex::encode(genesis_hash));
    
    // Connect to the chain
    let api = OnlineClient::<SelendraConfig>::from_url(&rpc_url).await?;
    let signer = PairSigner::new(pair);
    
    // Create the transaction
    let default_claim_call = selendra::tx()
        .unified_accounts()
        .claim_default_evm_address();
    
    println!("🔄 Submitting claim_default_evm_address()...");
    
    match api.tx().sign_and_submit_then_watch_default(&default_claim_call, &signer).await {
        Ok(progress) => {
            println!("✅ Transaction submitted successfully!");
            
            // Wait for finalization
            match progress.wait_for_finalized_success().await {
                Ok(events) => {
                    println!("🎉 Transaction finalized!");
                    
                    // Look for AccountClaimed events
                    let account_claimed_events = events.find::<selendra::unified_accounts::events::AccountClaimed>();
                    let mut found_event = false;
                    
                    for event in account_claimed_events {
                        match event {
                            Ok(ev) => {
                                found_event = true;
                                println!("🔗 Default EVM address claimed successfully!");
                                println!("   Substrate Account: {}", AccountId32::from(ev.account_id.0).to_ss58check());
                                println!("   Default EVM Address: 0x{}", hex::encode(ev.evm_address));
                            }
                            Err(e) => println!("❌ Error decoding event: {:?}", e),
                        }
                    }
                    
                    if !found_event {
                        println!("⚠️  No AccountClaimed event found. Account may already be mapped.");
                    }
                }
                Err(e) => {
                    println!("❌ Transaction failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Transaction submission failed: {:?}", e);
            println!("   Common causes: account already mapped, insufficient funds, or node issues");
        }
    }
    
    Ok(())
}

async fn claim_evm_address(mnemonic: String, rpc_url: String) -> Result<()> {
    println!("🚀 Claiming specific EVM address with signature proof...");
    
    let (pair, account_id, _secret_key, evm_address, genesis_hash, signature) = 
        generate_account_data(&mnemonic, &rpc_url).await?;
    
    println!("📝 Account: {}", account_id.to_ss58check());
    println!("🔗 Genesis: 0x{}", hex::encode(genesis_hash));
    println!("🎯 EVM Address: 0x{}", hex::encode(evm_address));
    println!("✍️  Signature: 0x{}", hex::encode(signature));
    
    // Connect to the chain
    let api = OnlineClient::<SelendraConfig>::from_url(&rpc_url).await?;
    let signer = PairSigner::new(pair);
    
    // Create the transaction
    let claim_call = selendra::tx()
        .unified_accounts()
        .claim_evm_address(subxt::utils::H160(evm_address), signature);
    
    println!("🔄 Submitting claim_evm_address()...");
    
    match api.tx().sign_and_submit_then_watch_default(&claim_call, &signer).await {
        Ok(progress) => {
            println!("✅ Transaction submitted successfully!");
            
            // Wait for finalization
            match progress.wait_for_finalized_success().await {
                Ok(events) => {
                    println!("🎉 Transaction finalized!");
                    
                    // Look for AccountClaimed events
                    let account_claimed_events = events.find::<selendra::unified_accounts::events::AccountClaimed>();
                    let mut found_event = false;
                    
                    for event in account_claimed_events {
                        match event {
                            Ok(ev) => {
                                found_event = true;
                                println!("🔗 EVM address claimed successfully!");
                                println!("   Substrate Account: {}", AccountId32::from(ev.account_id.0).to_ss58check());
                                println!("   EVM Address: 0x{}", hex::encode(ev.evm_address));
                            }
                            Err(e) => println!("❌ Error decoding event: {:?}", e),
                        }
                    }
                    
                    if !found_event {
                        println!("⚠️  No AccountClaimed event found. Account may already be mapped.");
                    }
                }
                Err(e) => {
                    println!("❌ Transaction failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Transaction submission failed: {:?}", e);
            println!("   Common causes: invalid signature, account already mapped, insufficient funds");
        }
    }
    
    Ok(())
}

async fn generate_only(mnemonic: String, rpc_url: String) -> Result<()> {
    println!("🔧 Generating unified accounts data...");
    
    let (_pair, account_id, _secret_key, evm_address, genesis_hash, signature) = 
        generate_account_data(&mnemonic, &rpc_url).await?;
    
    // Calculate default EVM address
    let default_payload = (b"evm:", account_id);
    let default_hash = sp_core::blake2_256(&default_payload.encode());
    let mut default_evm = [0u8; 20];
    default_evm.copy_from_slice(&default_hash[0..20]);
    
    println!("\n📋 Generated Account Data:");
    println!("==========================================");
    println!("   Substrate Address: {}", account_id.to_ss58check());
    println!("   EVM Address: 0x{}", hex::encode(evm_address));
    println!("   Default EVM Address: 0x{}", hex::encode(default_evm));
    println!("   Chain ID: 1961");
    println!("   Genesis Hash: 0x{}", hex::encode(genesis_hash));
    println!("   EIP-712 Signature: 0x{}", hex::encode(signature));
    
    println!("\n🔧 Polkadot.js Apps Usage:");
    println!("   1. Fund account: {}", account_id.to_ss58check());
    println!("   2. Go to: https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/extrinsics");
    println!("   3. Use unifiedAccounts.claimDefaultEvmAddress() OR");
    println!("   4. Use unifiedAccounts.claimEvmAddress:");
    println!("      - evmAddress: 0x{}", hex::encode(evm_address));
    println!("      - signature: 0x{}", hex::encode(signature));
    
    Ok(())
}

/// Get Substrate account balance
async fn get_substrate_balance(api: &OnlineClient<SelendraConfig>, account_id: &AccountId32) -> Result<u128> {
    let balance_query = selendra::storage().system().account(account_id);
    let account_info = api.storage().at_latest().await?.fetch(&balance_query).await?;
    
    match account_info {
        Some(info) => Ok(info.data.free.0),
        None => Ok(0),
    }
}

/// Get EVM account balance via pallet-evm
async fn get_evm_balance(api: &OnlineClient<SelendraConfig>, evm_address: &EvmAddress) -> Result<u128> {
    let balance_query = selendra::storage().evm().accounts(subxt::utils::H160(*evm_address));
    let account_info = api.storage().at_latest().await?.fetch(&balance_query).await?;
    
    match account_info {
        Some(info) => {
            // Convert from U256 to u128
            let balance_bytes = info.balance.0;
            let mut result = 0u128;
            for (i, &byte) in balance_bytes.iter().rev().enumerate() {
                if i >= 16 { break; } // Only take first 16 bytes for u128
                result |= (byte as u128) << (i * 8);
            }
            Ok(result)
        },
        None => Ok(0),
    }
}

/// Transfer on Substrate side
async fn substrate_transfer(
    api: &OnlineClient<SelendraConfig>,
    signer: &PairSigner<SelendraConfig, sr25519::Pair>,
    target: &AccountId32,
    amount: u128,
) -> Result<()> {
    let transfer_call = selendra::tx()
        .balances()
        .transfer_allow_death(subxt::utils::MultiAddress::Id(target.clone()), amount);
    
    println!("🔄 Submitting Substrate transfer...");
    
    match api.tx().sign_and_submit_then_watch_default(&transfer_call, signer).await {
        Ok(progress) => {
            println!("✅ Substrate transfer submitted successfully!");
            
            match progress.wait_for_finalized_success().await {
                Ok(_) => println!("🎉 Substrate transfer finalized!"),
                Err(e) => println!("❌ Substrate transfer failed: {:?}", e),
            }
        }
        Err(e) => {
            println!("❌ Substrate transfer submission failed: {:?}", e);
        }
    }
    
    Ok(())
}

/// Transfer on EVM side
async fn evm_transfer(
    api: &OnlineClient<SelendraConfig>,
    signer: &PairSigner<SelendraConfig, sr25519::Pair>,
    from_evm: &EvmAddress,
    target_evm: &EvmAddress,
    amount: u128,
) -> Result<()> {
    // Convert amount to U256 format for EVM
    let mut amount_bytes = [0u8; 32];
    let amount_le_bytes = amount.to_le_bytes();
    amount_bytes[..16].copy_from_slice(&amount_le_bytes);
    let evm_amount = subxt::utils::H256(amount_bytes);
    
    let transfer_call = selendra::tx()
        .evm()
        .call(
            subxt::utils::H160(*from_evm),
            subxt::utils::H160(*target_evm),
            vec![], // No data for simple transfer
            evm_amount,
            600000u64, // Gas limit
            subxt::utils::H256([0; 32]), // Max fee per gas
            None, // Max priority fee per gas
            None, // Nonce
            vec![], // Access list
        );
    
    println!("🔄 Submitting EVM transfer...");
    
    match api.tx().sign_and_submit_then_watch_default(&transfer_call, signer).await {
        Ok(progress) => {
            println!("✅ EVM transfer submitted successfully!");
            
            match progress.wait_for_finalized_success().await {
                Ok(_) => println!("🎉 EVM transfer finalized!"),
                Err(e) => println!("❌ EVM transfer failed: {:?}", e),
            }
        }
        Err(e) => {
            println!("❌ EVM transfer submission failed: {:?}", e);
        }
    }
    
    Ok(())
}

/// Comprehensive test of unified accounts with balance checks and transfers
async fn test_unified_accounts(mnemonic: String, rpc_url: String, target_address: String, amount: String) -> Result<()> {
    println!("🧪 Starting comprehensive unified accounts test...");
    
    // Generate account data
    let (pair, account_id, _secret_key, evm_address, genesis_hash, signature) = 
        generate_account_data(&mnemonic, &rpc_url).await?;
    
    // Parse target address and amount
    let target_account = AccountId32::from_ss58check(&target_address)?;
    let transfer_amount: u128 = amount.parse()?;
    
    // Calculate target EVM address (default derivation)
    let target_default_payload = (b"evm:", target_account);
    let target_default_hash = sp_core::blake2_256(&target_default_payload.encode());
    let mut target_evm = [0u8; 20];
    target_evm.copy_from_slice(&target_default_hash[0..20]);
    
    // Calculate our default EVM address
    let default_payload = (b"evm:", account_id);
    let default_hash = sp_core::blake2_256(&default_payload.encode());
    let mut default_evm = [0u8; 20];
    default_evm.copy_from_slice(&default_hash[0..20]);
    
    println!("\n📋 Test Configuration:");
    println!("===========================================");
    println!("   Source Substrate: {}", account_id.to_ss58check());
    println!("   Source EVM: 0x{}", hex::encode(evm_address));
    println!("   Source Default EVM: 0x{}", hex::encode(default_evm));
    println!("   Target Substrate: {}", target_account.to_ss58check());
    println!("   Target Default EVM: 0x{}", hex::encode(target_evm));
    println!("   Transfer Amount: {} units", transfer_amount);
    println!("   Genesis Hash: 0x{}", hex::encode(genesis_hash));
    
    // Connect to the chain
    let api = OnlineClient::<SelendraConfig>::from_url(&rpc_url).await?;
    let signer = PairSigner::new(pair);
    
    // Step 1: Check initial balances
    println!("\n📊 Step 1: Checking initial balances...");
    println!("===========================================");
    
    let initial_substrate_balance = get_substrate_balance(&api, &account_id).await?;
    let initial_evm_balance = get_evm_balance(&api, &evm_address).await?;
    let initial_default_evm_balance = get_evm_balance(&api, &default_evm).await?;
    let target_initial_substrate = get_substrate_balance(&api, &target_account).await?;
    let target_initial_evm = get_evm_balance(&api, &target_evm).await?;
    
    println!("   Source Substrate Balance: {} units", initial_substrate_balance);
    println!("   Source EVM Balance: {} units", initial_evm_balance);
    println!("   Source Default EVM Balance: {} units", initial_default_evm_balance);
    println!("   Target Substrate Balance: {} units", target_initial_substrate);
    println!("   Target Default EVM Balance: {} units", target_initial_evm);
    
    // Step 2: Claim unified account if not already claimed
    println!("\n🔗 Step 2: Claiming unified account...");
    println!("===========================================");
    
    // Try to claim the EVM address first
    let claim_call = selendra::tx()
        .unified_accounts()
        .claim_evm_address(subxt::utils::H160(evm_address), signature);
    
    match api.tx().sign_and_submit_then_watch_default(&claim_call, &signer).await {
        Ok(progress) => {
            println!("✅ Claim transaction submitted!");
            match progress.wait_for_finalized_success().await {
                Ok(events) => {
                    let account_claimed_events = events.find::<selendra::unified_accounts::events::AccountClaimed>();
                    let mut found_event = false;
                    
                    for event in account_claimed_events {
                        if let Ok(ev) = event {
                            found_event = true;
                            println!("🎉 Account claimed successfully!");
                            println!("   Substrate: {}", AccountId32::from(ev.account_id.0).to_ss58check());
                            println!("   EVM: 0x{}", hex::encode(ev.evm_address));
                        }
                    }
                    
                    if !found_event {
                        println!("ℹ️  Account already mapped or claim failed.");
                    }
                }
                Err(e) => println!("❌ Claim transaction failed: {:?}", e),
            }
        }
        Err(_) => {
            println!("ℹ️  Claim failed (likely already mapped), proceeding with test...");
        }
    }
    
    // Step 3: Substrate transfer
    println!("\n💸 Step 3: Substrate transfer...");
    println!("===========================================");
    
    if initial_substrate_balance >= transfer_amount {
        substrate_transfer(&api, &signer, &target_account, transfer_amount).await?;
    } else {
        println!("⚠️  Insufficient Substrate balance for transfer.");
    }
    
    // Step 4: Check balances after Substrate transfer
    println!("\n📊 Step 4: Balances after Substrate transfer...");
    println!("=============================================");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // Wait for finalization
    
    let post_substrate_balance = get_substrate_balance(&api, &account_id).await?;
    let post_target_substrate = get_substrate_balance(&api, &target_account).await?;
    
    println!("   Source Substrate Balance: {} units (Δ: {})", 
        post_substrate_balance, 
        post_substrate_balance as i128 - initial_substrate_balance as i128);
    println!("   Target Substrate Balance: {} units (Δ: {})", 
        post_target_substrate, 
        post_target_substrate as i128 - target_initial_substrate as i128);
    
    // Step 5: EVM transfer (if we have EVM balance)
    println!("\n💸 Step 5: EVM transfer...");
    println!("===========================================");
    
    let current_evm_balance = get_evm_balance(&api, &evm_address).await?;
    if current_evm_balance >= transfer_amount {
        evm_transfer(&api, &signer, &evm_address, &target_evm, transfer_amount).await?;
    } else {
        println!("⚠️  Insufficient EVM balance for transfer. EVM balance: {}", current_evm_balance);
    }
    
    // Step 6: Final balance check
    println!("\n📊 Step 6: Final balance summary...");
    println!("====================================");
    
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // Wait for finalization
    
    let final_substrate_balance = get_substrate_balance(&api, &account_id).await?;
    let final_evm_balance = get_evm_balance(&api, &evm_address).await?;
    let final_default_evm_balance = get_evm_balance(&api, &default_evm).await?;
    let final_target_substrate = get_substrate_balance(&api, &target_account).await?;
    let final_target_evm = get_evm_balance(&api, &target_evm).await?;
    
    println!("   📈 Balance Changes:");
    println!("   Source Substrate: {} → {} (Δ: {})", 
        initial_substrate_balance, final_substrate_balance,
        final_substrate_balance as i128 - initial_substrate_balance as i128);
    println!("   Source EVM: {} → {} (Δ: {})", 
        initial_evm_balance, final_evm_balance,
        final_evm_balance as i128 - initial_evm_balance as i128);
    println!("   Source Default EVM: {} → {} (Δ: {})", 
        initial_default_evm_balance, final_default_evm_balance,
        final_default_evm_balance as i128 - initial_default_evm_balance as i128);
    println!("   Target Substrate: {} → {} (Δ: {})", 
        target_initial_substrate, final_target_substrate,
        final_target_substrate as i128 - target_initial_substrate as i128);
    println!("   Target Default EVM: {} → {} (Δ: {})", 
        target_initial_evm, final_target_evm,
        final_target_evm as i128 - target_initial_evm as i128);
    
    println!("\n✅ Unified accounts test completed!");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::ClaimDefault { mnemonic, rpc } => {
            claim_default_evm_address(mnemonic, rpc).await?;
        }
        Commands::ClaimEvm { mnemonic, rpc } => {
            claim_evm_address(mnemonic, rpc).await?;
        }
        Commands::Generate { mnemonic, rpc } => {
            generate_only(mnemonic, rpc).await?;
        }
        Commands::Test { mnemonic, rpc, target, amount } => {
            test_unified_accounts(mnemonic, rpc, target, amount).await?;
        }
    }
    
    Ok(())
}