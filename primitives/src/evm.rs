use parity_scale_codec::{Encode, Decode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_core::{Hasher, H256};
use sp_runtime::traits::UniqueSaturatedInto;
use core::marker::PhantomData;
use frame_support::traits::{
    Currency, Imbalance, OnUnbalanced,
};
use pallet_evm::{OnChargeEVMTransaction, HashedAddressMapping, AddressMapping};
use ethereum_types::U256;
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

/// Wrapper around the `EVMCurrencyAdapter` from the `pallet-evm`.
///
/// While it provides most of the functionality we need,
/// it doesn't allow the tip to be deposited into an arbitrary account.
/// This adapter allows us to do that.
///
/// Two separate `OnUnbalanced` handlers are used:
/// - `FeeHandler` for the fee
/// - `TipHandler` for the tip
pub struct EVMCurrencyAdapterWrapper<C, FeeHandler, TipHandler>(
    core::marker::PhantomData<(C, FeeHandler, TipHandler)>,
);

impl<T, C, FeeHandler, TipHandler> OnChargeEVMTransaction<T>
    for EVMCurrencyAdapterWrapper<C, FeeHandler, TipHandler>
where
    T: pallet_evm::Config,
    C: Currency<<T as frame_system::Config>::AccountId>,
    C::PositiveImbalance: Imbalance<
        <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
        Opposite = C::NegativeImbalance,
    >,
    C::NegativeImbalance: Imbalance<
        <C as Currency<<T as frame_system::Config>::AccountId>>::Balance,
        Opposite = C::PositiveImbalance,
    >,
    FeeHandler: OnUnbalanced<C::NegativeImbalance>,
    TipHandler: OnUnbalanced<C::NegativeImbalance>,
    U256: UniqueSaturatedInto<<C as Currency<<T as frame_system::Config>::AccountId>>::Balance>,
{
    // Kept type as Option to satisfy bound of Default
    type LiquidityInfo = Option<C::NegativeImbalance>;

    fn withdraw_fee(who: &EvmAddress, fee: U256) -> Result<Self::LiquidityInfo, pallet_evm::Error<T>> {
        pallet_evm::EVMCurrencyAdapter::<C, FeeHandler>::withdraw_fee(who, fee)
    }

    fn correct_and_deposit_fee(
        who: &EvmAddress,
        corrected_fee: U256,
        base_fee: U256,
        already_withdrawn: Self::LiquidityInfo,
    ) -> Self::LiquidityInfo {
        <pallet_evm::EVMCurrencyAdapter::<C, FeeHandler> as OnChargeEVMTransaction<T>>::correct_and_deposit_fee(
            who,
            corrected_fee,
            base_fee,
            already_withdrawn,
        )
    }

    fn pay_priority_fee(tip: Self::LiquidityInfo) {
        if let Some(tip) = tip {
            TipHandler::on_unbalanceds(Some(tip).into_iter());
        }
    }
}
