use crate::AccountId;
use frame_support::{parameter_types, traits::LockIdentifier, PalletId};
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;

// Pallet accounts of runtime
parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"sel/trsy");
	pub const DEXPalletId: PalletId = PalletId(*b"sel/dexm");
	pub const PhragmenElectionPalletId: LockIdentifier = *b"phrelect";
	pub const SelTreasuryPalletId: PalletId = PalletId(*b"sel/cdpt");
	pub const IncentivesPalletId: PalletId = PalletId(*b"sel/inct");
	// Treasury reserve
	pub const TreasuryReservePalletId: PalletId = PalletId(*b"sel/reve");
	pub const NftPalletId: PalletId = PalletId(*b"sel/aNFT");
	// Vault all unrleased native token.
	pub UnreleasedNativeVaultAccountId: AccountId = PalletId(*b"sel/urls").into_account_truncating();
	// This Pallet is only used to payment fee pool, it's not added to whitelist by design.
	// because transaction payment pallet will ensure the accounts always have enough ED.
	pub const TransactionPaymentPalletId: PalletId = PalletId(*b"sel/fees");
}

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![
		SelTreasuryPalletId::get().into_account_truncating(),
		DEXPalletId::get().into_account_truncating(),
		IncentivesPalletId::get().into_account_truncating(),
		TreasuryPalletId::get().into_account_truncating(),
		TreasuryReservePalletId::get().into_account_truncating(),
		UnreleasedNativeVaultAccountId::get(),
	]
}
