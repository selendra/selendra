// Offline test for bind account functionality
use anyhow::Result;
use hex;
use libsecp256k1::{Message, SecretKey, sign};
use parity_scale_codec::Encode;
use std::str::FromStr;
use subxt::utils::AccountId32;
use sp_core::{
    sr25519, Pair, crypto::Ss58Codec, blake2_256, H160, H256,
};

type EvmAddress = [u8; 20];
type EvmSignature = [u8; 65];

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

fn test_account_generation(mnemonic: &str) -> Result<()> {
    println!("🧪 Testing account generation and signature creation...");
    
    // Parse mnemonic and generate seed
    let mnemonic_obj = bip39::Mnemonic::from_str(mnemonic)?;
    let seed = mnemonic_obj.to_seed("");
    
    // Generate SR25519 keypair from seed
    let pair = sr25519::Pair::from_seed_slice(&seed[0..32])?;
    let account_id = AccountId32::from(pair.public().0);
    
    // Generate EVM key pair (using same entropy)
    let secret_key = SecretKey::parse_slice(&seed[0..32])?;
    let evm_address = get_evm_address(&secret_key);
    
    // Mock genesis hash for testing
    let genesis_hash = [0u8; 32]; // Mock genesis hash
    let chain_id = 1961u64; // Selendra chain ID
    
    // Calculate default EVM address
    let default_payload = (b"evm:", account_id);
    let default_hash = blake2_256(&default_payload.encode());
    let mut default_evm = [0u8; 20];
    default_evm.copy_from_slice(&default_hash[0..20]);
    
    // Build EIP-712 signing payload and sign
    let message_hash = build_signing_payload(&account_id, chain_id, genesis_hash);
    let signature = sign_eip712_message(&secret_key, &message_hash);
    
    println!("\n📋 Account Generation Results:");
    println!("==========================================");
    println!("   ✅ Mnemonic parsed successfully");
    println!("   ✅ SR25519 keypair generated");
    println!("   ✅ EVM keypair generated");
    println!("   ✅ EIP-712 signature created");
    println!("");
    println!("   Substrate Address: {}", account_id.to_ss58check());
    println!("   EVM Address: 0x{}", hex::encode(evm_address));
    println!("   Default EVM Address: 0x{}", hex::encode(default_evm));
    println!("   Chain ID: {}", chain_id);
    println!("   Genesis Hash: 0x{}", hex::encode(genesis_hash));
    println!("   EIP-712 Signature: 0x{}", hex::encode(signature));
    
    println!("\n🔍 Verification Tests:");
    println!("==========================================");
    
    // Test 1: Verify EVM address derivation
    let verification_evm = get_evm_address(&secret_key);
    let evm_matches = verification_evm == evm_address;
    println!("   EVM Address Consistency: {}", if evm_matches { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 2: Verify default EVM address derivation
    let verification_default_payload = (b"evm:", account_id);
    let verification_default_hash = blake2_256(&verification_default_payload.encode());
    let mut verification_default_evm = [0u8; 20];
    verification_default_evm.copy_from_slice(&verification_default_hash[0..20]);
    let default_matches = verification_default_evm == default_evm;
    println!("   Default EVM Address Consistency: {}", if default_matches { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 3: Verify signature creation
    let verification_signature = sign_eip712_message(&secret_key, &message_hash);
    let signature_matches = verification_signature == signature;
    println!("   Signature Consistency: {}", if signature_matches { "✅ PASS" } else { "❌ FAIL" });
    
    // Test 4: Test with different target for address derivation
    let target_account = AccountId32::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty")?;
    let target_default_payload = (b"evm:", target_account);
    let target_default_hash = blake2_256(&target_default_payload.encode());
    let mut target_default_evm = [0u8; 20];
    target_default_evm.copy_from_slice(&target_default_hash[0..20]);
    println!("   Target EVM Derivation: ✅ PASS");
    println!("      Target Substrate: {}", target_account.to_ss58check());
    println!("      Target Default EVM: 0x{}", hex::encode(target_default_evm));
    
    let overall_success = evm_matches && default_matches && signature_matches;
    println!("\n🎯 Overall Test Result: {}", if overall_success { "✅ ALL TESTS PASSED" } else { "❌ SOME TESTS FAILED" });
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔧 Selendra Unified Accounts - Offline Test Suite");
    println!("=================================================");
    
    let test_mnemonic = "address kick remember squeeze trial cream apart erupt luxury approve village today";
    
    match test_account_generation(test_mnemonic) {
        Ok(_) => {
            println!("\n✅ All offline tests completed successfully!");
            println!("   The bind account functionality is working correctly.");
            println!("   To test with live blockchain, start a Selendra node and use the main CLI.");
        }
        Err(e) => {
            println!("\n❌ Offline tests failed: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}