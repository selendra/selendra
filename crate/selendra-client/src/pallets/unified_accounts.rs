//! Pallet Unified Accounts API
//!
//! This module provides an interface for binding Substrate accounts to EVM accounts.
//! The unified accounts pallet allows users to:
//! - Claim a specific EVM address by providing a signature
//! - Claim a default EVM address derived from their Substrate account

use codec::Encode;
use sp_core::{H160, H256, U256};
use sp_io::hashing::keccak_256;

use crate::{
    connections::TxInfo,
    AccountId, BlockHash,
    ConnectionApi, SignedConnectionApi, TxStatus, AsConnection,
};

/// EVM Address type (20 bytes)
pub type EvmAddress = H160;

/// ECDSA Signature type (65 bytes with recovery id)
pub type EvmSignature = [u8; 65];

/// Pallet unified accounts read-only API.
#[async_trait::async_trait]
pub trait UnifiedAccountsApi {
    /// Get the EVM address mapped to a native account.
    /// Returns None if no mapping exists.
    /// * `account` - the native account to query
    /// * `at` - optional block hash to query state from
    async fn get_evm_address(
        &self,
        account: &AccountId,
        at: Option<BlockHash>,
    ) -> Option<EvmAddress>;

    /// Get the native account mapped to an EVM address.
    /// Returns None if no mapping exists.
    /// * `evm_address` - the EVM address to query
    /// * `at` - optional block hash to query state from
    async fn get_native_account(
        &self,
        evm_address: &EvmAddress,
        at: Option<BlockHash>,
    ) -> Option<AccountId>;

    /// Check if a native account has an EVM address mapping.
    /// * `account` - the native account to check
    /// * `at` - optional block hash to query state from
    async fn is_account_mapped(
        &self,
        account: &AccountId,
        at: Option<BlockHash>,
    ) -> bool {
        self.get_evm_address(account, at).await.is_some()
    }

    /// Check if an EVM address has a native account mapping.
    /// * `evm_address` - the EVM address to check
    /// * `at` - optional block hash to query state from
    async fn is_evm_address_mapped(
        &self,
        evm_address: &EvmAddress,
        at: Option<BlockHash>,
    ) -> bool {
        self.get_native_account(evm_address, at).await.is_some()
    }
}

/// Pallet unified accounts API for signed transactions.
#[async_trait::async_trait]
pub trait UnifiedAccountsUserApi {
    /// Claim a specific EVM address by providing a signature.
    /// 
    /// This creates a double mapping between the caller's Substrate account
    /// and the specified EVM address. The signature proves ownership of the EVM address.
    /// 
    /// # Arguments
    /// * `evm_address` - The EVM address to claim
    /// * `signature` - An EIP-712 signature proving ownership of the EVM address
    /// * `status` - Transaction status to wait for
    /// 
    /// # Warning
    /// - Once mapped, the binding cannot be changed
    /// - Any native balance in the default account for this EVM address will be transferred
    /// - Other assets (XC20, unclaimed rewards, etc.) should be transferred manually first
    async fn claim_evm_address(
        &self,
        evm_address: EvmAddress,
        signature: EvmSignature,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;

    /// Claim the default EVM address derived from the caller's Substrate account.
    /// 
    /// This creates a double mapping between the caller's Substrate account
    /// and their default EVM address (derived deterministically from the account).
    /// 
    /// # Arguments
    /// * `status` - Transaction status to wait for
    /// 
    /// # Warning
    /// Once mapped, the binding cannot be changed.
    async fn claim_default_evm_address(
        &self,
        status: TxStatus,
    ) -> anyhow::Result<TxInfo>;
}

/// Helper functions for EIP-712 signature generation.
pub struct Eip712Helper;

impl Eip712Helper {
    /// Domain separator type hash
    const DOMAIN_TYPE_HASH: &'static str = "EIP712Domain(string name,string version,uint256 chainId,bytes32 salt)";
    /// Domain name
    const DOMAIN_NAME: &'static str = "Selendra EVM Claim";
    /// Domain version
    const DOMAIN_VERSION: &'static str = "1";
    /// Claim type hash
    const CLAIM_TYPE_HASH: &'static str = "Claim(bytes substrateAddress)";

    /// Build the signing payload for claiming an EVM address.
    /// 
    /// This implements the EIP-712 typed data signing scheme used by the pallet.
    /// 
    /// # Arguments
    /// * `account_id` - The Substrate account that will be mapped
    /// * `chain_id` - The EVM chain ID
    /// * `genesis_hash` - The genesis block hash (used as salt)
    pub fn build_signing_payload(
        account_id: &AccountId,
        chain_id: u64,
        genesis_hash: H256,
    ) -> [u8; 32] {
        let domain_separator = Self::build_domain_separator(chain_id, genesis_hash);
        let args_hash = Self::build_args_hash(account_id);

        let mut payload = b"\x19\x01".to_vec();
        payload.extend_from_slice(&domain_separator);
        payload.extend_from_slice(&args_hash);
        keccak_256(&payload)
    }

    fn build_domain_separator(chain_id: u64, genesis_hash: H256) -> [u8; 32] {
        let mut domain = keccak_256(Self::DOMAIN_TYPE_HASH.as_bytes()).to_vec();
        domain.extend_from_slice(&keccak_256(Self::DOMAIN_NAME.as_bytes()));
        domain.extend_from_slice(&keccak_256(Self::DOMAIN_VERSION.as_bytes()));
        domain.extend_from_slice(&U256::from(chain_id).to_big_endian());
        domain.extend_from_slice(genesis_hash.as_bytes());
        keccak_256(&domain)
    }

    fn build_args_hash(account_id: &AccountId) -> [u8; 32] {
        let mut args_hash = keccak_256(Self::CLAIM_TYPE_HASH.as_bytes()).to_vec();
        args_hash.extend_from_slice(&keccak_256(&account_id.encode()));
        keccak_256(&args_hash)
    }

    /// Sign a prehash with an Ethereum private key.
    /// 
    /// # Arguments
    /// * `prehash` - The 32-byte message hash to sign
    /// * `secret` - The secp256k1 secret key
    /// 
    /// # Returns
    /// A 65-byte signature with recovery id
    #[cfg(feature = "std")]
    pub fn eth_sign_prehash(prehash: &[u8; 32], secret: &libsecp256k1::SecretKey) -> EvmSignature {
        let (sig, recovery_id) = libsecp256k1::sign(&libsecp256k1::Message::parse(prehash), secret);
        let mut r = [0u8; 65];
        r[0..64].copy_from_slice(&sig.serialize()[..]);
        r[64] = recovery_id.serialize();
        r
    }

    /// Get the Ethereum address from a secret key.
    /// 
    /// # Arguments
    /// * `secret` - The secp256k1 secret key
    /// 
    /// # Returns
    /// The corresponding H160 Ethereum address
    #[cfg(feature = "std")]
    pub fn eth_address(secret: &libsecp256k1::SecretKey) -> EvmAddress {
        EvmAddress::from_slice(
            &keccak_256(
                &libsecp256k1::PublicKey::from_secret_key(secret).serialize()[1..65],
            )[12..],
        )
    }

    /// Compute the default EVM address for a Substrate account.
    /// 
    /// Uses Blake2 hashing truncated to 20 bytes.
    /// 
    /// # Arguments
    /// * `account_id` - The Substrate account
    /// 
    /// # Returns
    /// The default H160 EVM address
    pub fn default_evm_address(account_id: &AccountId) -> EvmAddress {
        use sp_io::hashing::blake2_256;
        let hash = blake2_256(&account_id.encode());
        EvmAddress::from_slice(&hash[0..20])
    }

    /// Compute the default Substrate account for an EVM address.
    /// 
    /// Uses Blake2 hashing with "evm:" prefix.
    /// 
    /// # Arguments
    /// * `evm_address` - The EVM address
    /// 
    /// # Returns
    /// The default AccountId
    pub fn default_account_id(evm_address: &EvmAddress) -> AccountId {
        use sp_io::hashing::blake2_256;
        let mut data = b"evm:".to_vec();
        data.extend_from_slice(evm_address.as_bytes());
        data.resize(32, 0u8);
        AccountId::from(blake2_256(&data))
    }
}

/// Trait for verifying EVM signatures.
pub trait EvmSignatureVerifier {
    /// Verify an EVM signature and recover the signer address.
    /// 
    /// # Arguments
    /// * `account_id` - The Substrate account claiming the EVM address
    /// * `signature` - The EIP-712 signature
    /// * `chain_id` - The EVM chain ID
    /// * `genesis_hash` - The genesis block hash
    /// 
    /// # Returns
    /// The recovered EVM address if verification succeeds
    fn verify_evm_signature(
        account_id: &AccountId,
        signature: &EvmSignature,
        chain_id: u64,
        genesis_hash: H256,
    ) -> Option<EvmAddress> {
        let payload_hash = Eip712Helper::build_signing_payload(account_id, chain_id, genesis_hash);
        
        sp_io::crypto::secp256k1_ecdsa_recover(signature, &payload_hash)
            .map(|pubkey| H160::from(H256::from_slice(&keccak_256(&pubkey))))
            .ok()
    }
}
