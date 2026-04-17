use parity_scale_codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{Hasher, H256};
use core::marker::PhantomData;
use pallet_evm::{HashedAddressMapping, AddressMapping};
use crate::AccountId;

pub type EvmAddress = sp_core::H160;

/// Mapping between Native and EVM Addresses
pub trait UnifiedAddressMapper<AccountId> {
    /// Gets the account id associated with given evm address, if mapped else None.
    fn to_account_id(evm_address: &EvmAddress) -> Option<AccountId>;

    /// Gets the account id associated with given evm address.
    /// If no mapping exists, then return the default evm address.
    /// Returns `UnifiedAddress` enum which wraps the inner account id
    fn to_account_id_or_default(evm_address: &EvmAddress) -> UnifiedAddress<AccountId> {
        Self::to_account_id(evm_address).map_or_else(
            // fallback to default account_id
            || UnifiedAddress::Default(Self::to_default_account_id(evm_address)),
            |a| UnifiedAddress::Mapped(a),
        )
    }
    /// Gets the default account id which is associated with given evm address.
    fn to_default_account_id(evm_address: &EvmAddress) -> AccountId;

    /// Gets the evm address associated with given account id, if mapped else None.
    fn to_h160(account_id: &AccountId) -> Option<EvmAddress>;

    /// Gets the evm address associated with given account id.
    /// If no mapping exists, then return the default account id.
    /// Returns `UnifiedAddress` enum which wraps the inner evm address
    fn to_h160_or_default(account_id: &AccountId) -> UnifiedAddress<EvmAddress> {
        Self::to_h160(account_id).map_or_else(
            // fallback to default account_id
            || UnifiedAddress::Default(Self::to_default_h160(account_id)),
            |a| UnifiedAddress::Mapped(a),
        )
    }

    /// Gets the default evm address which is associated with given account id.
    fn to_default_h160(account_id: &AccountId) -> EvmAddress;
}

/// Mappings derieved from hashing the original address
pub struct HashedDefaultMappings<H>(PhantomData<H>);
impl<H: Hasher<Out = H256>> UnifiedAddressMapper<AccountId> for HashedDefaultMappings<H> {
    fn to_default_account_id(evm_address: &EvmAddress) -> AccountId {
        HashedAddressMapping::<H>::into_account_id(evm_address.clone())
    }

    fn to_default_h160(account_id: &AccountId) -> EvmAddress {
        let payload = (b"evm:", account_id);
        EvmAddress::from_slice(&payload.using_encoded(H::hash)[0..20])
    }

    fn to_account_id(_: &EvmAddress) -> Option<AccountId> {
        None
    }

    fn to_h160(_: &AccountId) -> Option<EvmAddress> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen, TypeInfo)]
pub enum UnifiedAddress<Address> {
    /// The address fetched from the mappings and the account
    /// is unified
    #[codec(index = 0)]
    Mapped(Address),
    /// The default address associated with account as there
    /// is no mapping found and accounts are not unified
    #[codec(index = 1)]
    Default(Address),
}

impl<Address> UnifiedAddress<Address> {
    /// Get the underlying address
    pub fn into_address(self) -> Address {
        match self {
            Self::Default(a) => a,
            Self::Mapped(a) => a,
        }
    }
}
