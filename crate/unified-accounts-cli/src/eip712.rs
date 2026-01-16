//! EIP-712 signature generation for Selendra Unified Accounts
//!
//! This module implements the EIP-712 typed data signing scheme used by the
//! unified accounts pallet for verifying ownership of EVM addresses.

use codec::Encode;
use libsecp256k1::{Message, SecretKey};
use sp_core::{H160, H256, U256};
use sp_core_hashing::keccak_256;

use crate::{AccountId, EvmSignature};

/// EIP-712 helper for generating signatures
pub struct Eip712;

impl Eip712 {
    /// Domain separator type hash
    const DOMAIN_TYPE_HASH: &'static [u8] = b"EIP712Domain(string name,string version,uint256 chainId,bytes32 salt)";
    /// Domain name used by Selendra
    const DOMAIN_NAME: &'static [u8] = b"Selendra EVM Claim";
    /// Domain version
    const DOMAIN_VERSION: &'static [u8] = b"1";
    /// Claim type hash
    const CLAIM_TYPE_HASH: &'static [u8] = b"Claim(bytes substrateAddress)";

    /// Sign an EIP-712 typed data message for claiming an EVM address.
    ///
    /// # Arguments
    /// * `account_id` - The Substrate account that will be mapped
    /// * `secret` - The secp256k1 secret key of the EVM account
    /// * `chain_id` - The EVM chain ID
    /// * `genesis_hash` - The genesis block hash (used as salt in domain separator)
    ///
    /// # Returns
    /// A 65-byte signature with recovery id
    pub fn sign(
        account_id: &AccountId,
        secret: &SecretKey,
        chain_id: u64,
        genesis_hash: H256,
    ) -> EvmSignature {
        let payload = Self::build_signing_payload(account_id, chain_id, genesis_hash);
        Self::eth_sign_prehash(&payload, secret)
    }

    /// Build the EIP-712 signing payload.
    ///
    /// The payload is: keccak256("\x19\x01" ‖ domainSeparator ‖ structHash)
    pub fn build_signing_payload(
        account_id: &AccountId,
        chain_id: u64,
        genesis_hash: H256,
    ) -> [u8; 32] {
        let domain_separator = Self::build_domain_separator(chain_id, genesis_hash);
        let struct_hash = Self::build_struct_hash(account_id);

        let mut payload = Vec::with_capacity(66);
        payload.extend_from_slice(b"\x19\x01");
        payload.extend_from_slice(&domain_separator);
        payload.extend_from_slice(&struct_hash);
        
        keccak_256(&payload)
    }

    /// Build the EIP-712 domain separator (public for debugging).
    pub fn debug_domain_separator(chain_id: u64, genesis_hash: H256) -> [u8; 32] {
        Self::build_domain_separator(chain_id, genesis_hash)
    }

    /// Build the struct hash (public for debugging).
    pub fn debug_struct_hash(account_id: &AccountId) -> [u8; 32] {
        Self::build_struct_hash(account_id)
    }

    /// Build the EIP-712 domain separator.
    ///
    /// domainSeparator = keccak256(
    ///     DOMAIN_TYPE_HASH ‖
    ///     keccak256(name) ‖
    ///     keccak256(version) ‖
    ///     chainId ‖
    ///     salt
    /// )
    fn build_domain_separator(chain_id: u64, genesis_hash: H256) -> [u8; 32] {
        let mut domain = Vec::with_capacity(160);
        domain.extend_from_slice(&keccak_256(Self::DOMAIN_TYPE_HASH));
        domain.extend_from_slice(&keccak_256(Self::DOMAIN_NAME));
        domain.extend_from_slice(&keccak_256(Self::DOMAIN_VERSION));
        
        // Chain ID as uint256 (big endian)
        let mut chain_id_bytes = [0u8; 32];
        U256::from(chain_id).to_big_endian(&mut chain_id_bytes);
        domain.extend_from_slice(&chain_id_bytes);
        
        // Genesis hash as salt
        domain.extend_from_slice(genesis_hash.as_bytes());
        
        keccak_256(&domain)
    }

    /// Build the struct hash for the Claim type.
    ///
    /// structHash = keccak256(CLAIM_TYPE_HASH ‖ keccak256(substrateAddress))
    fn build_struct_hash(account_id: &AccountId) -> [u8; 32] {
        let mut struct_data = Vec::with_capacity(64);
        struct_data.extend_from_slice(&keccak_256(Self::CLAIM_TYPE_HASH));
        struct_data.extend_from_slice(&keccak_256(&account_id.encode()));
        
        keccak_256(&struct_data)
    }

    /// Sign a prehashed message with an Ethereum private key.
    fn eth_sign_prehash(prehash: &[u8; 32], secret: &SecretKey) -> EvmSignature {
        let message = Message::parse(prehash);
        let (sig, recovery_id) = libsecp256k1::sign(&message, secret);
        
        let mut signature = [0u8; 65];
        signature[0..64].copy_from_slice(&sig.serialize());
        signature[64] = recovery_id.serialize();
        
        signature
    }

    /// Verify a signature and recover the signer's EVM address.
    ///
    /// # Arguments
    /// * `account_id` - The Substrate account that claims to own the EVM address
    /// * `signature` - The EIP-712 signature
    /// * `chain_id` - The EVM chain ID
    /// * `genesis_hash` - The genesis block hash
    ///
    /// # Returns
    /// The recovered EVM address if verification succeeds
    pub fn verify(
        account_id: &AccountId,
        signature: &EvmSignature,
        chain_id: u64,
        genesis_hash: H256,
    ) -> Option<H160> {
        let payload = Self::build_signing_payload(account_id, chain_id, genesis_hash);
        
        // Try to recover the public key from signature
        let recovery_id = libsecp256k1::RecoveryId::parse(signature[64]).ok()?;
        let sig = libsecp256k1::Signature::parse_standard_slice(&signature[0..64]).ok()?;
        let message = Message::parse(&payload);
        
        let pubkey = libsecp256k1::recover(&message, &sig, &recovery_id).ok()?;
        let pubkey_bytes = &pubkey.serialize()[1..65];
        let hash = keccak_256(pubkey_bytes);
        Some(H160::from_slice(&hash[12..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libsecp256k1::PublicKey;

    fn test_secret_key() -> SecretKey {
        // A well-known test private key (DO NOT USE IN PRODUCTION)
        let bytes = hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
            .unwrap();
        SecretKey::parse_slice(&bytes).unwrap()
    }

    fn evm_address_from_secret(secret: &SecretKey) -> H160 {
        let public = PublicKey::from_secret_key(secret);
        let public_bytes = &public.serialize()[1..65];
        let hash = keccak_256(public_bytes);
        H160::from_slice(&hash[12..])
    }

    #[test]
    fn test_sign_and_verify() {
        let secret = test_secret_key();
        let account_id = AccountId::from([1u8; 32]);
        let chain_id = 1953;
        let genesis_hash = H256::from([0u8; 32]);

        let signature = Eip712::sign(&account_id, &secret, chain_id, genesis_hash);
        let recovered = Eip712::verify(&account_id, &signature, chain_id, genesis_hash);
        
        let expected_address = evm_address_from_secret(&secret);
        assert_eq!(recovered, Some(expected_address));
    }

    #[test]
    fn test_different_chain_id_fails() {
        let secret = test_secret_key();
        let account_id = AccountId::from([1u8; 32]);
        let chain_id = 1953;
        let genesis_hash = H256::from([0u8; 32]);

        let signature = Eip712::sign(&account_id, &secret, chain_id, genesis_hash);
        
        // Verify with different chain_id should fail
        let recovered = Eip712::verify(&account_id, &signature, chain_id + 1, genesis_hash);
        let expected_address = evm_address_from_secret(&secret);
        assert_ne!(recovered, Some(expected_address));
    }
}
