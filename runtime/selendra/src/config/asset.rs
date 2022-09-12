use crate::{
	dollar, parameter_types, weights, Amount, Balance, Balances, BlockNumber, CurrencyId,
	DataDepositPerByte, Event, EvmAddressMapping, GasToWeight, GetNativeCurrencyId, NftPalletId,
	RebaseTokens, Runtime, StableAssetPalletId, Tokens, TreasuryAccount, SEL,
};
use module_currencies::BasicCurrencyAdapter;
use runtime_common::{EnsureRootOrHalfCouncil, EnsureRootOrOneCouncil};
use sp_core::H160;

pub struct EnsurePoolAssetId;
impl module_stable_asset::traits::ValidateAssetId<CurrencyId> for EnsurePoolAssetId {
	fn validate(currency_id: CurrencyId) -> bool {
		matches!(currency_id, CurrencyId::StableAssetPoolToken(_))
	}
}

parameter_types! {
	pub const FeePrecision: u128 = 10_000_000_000; // 10 decimals
	pub const APrecision: u128 = 100; // 2 decimals
	pub const SwapExactOverAmount: u128 = 100;
	pub const PoolAssetLimit: u32 = 5;
}

impl module_stable_asset::Config for Runtime {
	type Event = Event;
	type AssetId = CurrencyId;
	type Balance = Balance;
	type Assets = RebaseTokens;
	type PalletId = StableAssetPalletId;
	type AtLeast64BitUnsigned = u128;
	type FeePrecision = FeePrecision;
	type APrecision = APrecision; // 2 decimals
	type PoolAssetLimit = PoolAssetLimit;
	type SwapExactOverAmount = SwapExactOverAmount;
	type WeightInfo = weights::module_stable_asset::WeightInfo<Runtime>;
	type ListingOrigin = EnsureRootOrHalfCouncil;
	type EnsurePoolAssetId = EnsurePoolAssetId;
}

parameter_types! {
	pub Erc20HoldingAccount: H160 = primitives::evm::ERC20_HOLDING_ACCOUNT;
}

impl module_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type Erc20HoldingAccount = Erc20HoldingAccount;
	type AddressMapping = EvmAddressMapping<Runtime>;
	type EVMBridge = module_evm_bridge::EVMBridge<Runtime>;
	type GasToWeight = GasToWeight;
	type SweepOrigin = EnsureRootOrOneCouncil;
	type OnDust = module_currencies::TransferDust<Runtime, TreasuryAccount>;
	type WeightInfo = weights::module_currencies::WeightInfo<Runtime>;
}

impl module_asset_registry::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type EVMBridge = module_evm_bridge::EVMBridge<Runtime>;
	type RegisterOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = weights::module_asset_registry::WeightInfo<Runtime>;
}

parameter_types! {
	pub CreateClassDeposit: Balance = 50 * dollar(SEL);
	pub CreateTokenDeposit: Balance = 10 * dollar(SEL);
	pub const MaxAttributesBytes: u32 = 2048;
}

impl module_nft::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type CreateClassDeposit = CreateClassDeposit;
	type CreateTokenDeposit = CreateTokenDeposit;
	type DataDepositPerByte = DataDepositPerByte;
	type PalletId = NftPalletId;
	type MaxAttributesBytes = MaxAttributesBytes;
	type WeightInfo = weights::module_nft::WeightInfo<Runtime>;
}
