use crate::{
	parameter_type_with_key, parameter_types, weights, AggregatedDataProvider, BlockNumber,
	ConstU32, Currencies, CurrencyId, DEXPalletId, Dex, Event, FixedPointNumber,
	GetLiquidCurrencyId, GetNativeCurrencyId, GetStableCurrencyId, Incentives, Price,
	RebasedStableAsset, Runtime, Timestamp, Vec, DAYS, LSEL, SEL,
};
use module_asset_registry::EvmErc20InfoMapping;
use module_support::{ExchangeRate, ExchangeRateProvider};
use runtime_common::{EnsureRootOrHalfCouncil, EnsureRootOrTwoThirdsCouncil};
use sp_std::vec;

parameter_types! {
	pub StableCurrencyFixedPrice: Price = Price::saturating_from_rational(1, 1);
}

parameter_type_with_key! {
	pub PricingPegged: |_currency_id: CurrencyId| -> Option<CurrencyId> {
		None
	};
}

pub struct LiquidNativeExchangeProvider;
impl ExchangeRateProvider for LiquidNativeExchangeProvider {
	fn get_exchange_rate() -> ExchangeRate {
		ExchangeRate::saturating_from_rational(1, 10)
	}
}

impl module_prices::Config for Runtime {
	type Event = Event;
	type Source = AggregatedDataProvider;
	type GetStableCurrencyId = GetStableCurrencyId;
	type StableCurrencyFixedPrice = StableCurrencyFixedPrice;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type GetLiquidCurrencyId = GetLiquidCurrencyId;
	type LiquidNativeExchangeRateProvider = LiquidNativeExchangeProvider;
	type LockOrigin = EnsureRootOrTwoThirdsCouncil;
	type DEX = Dex;
	type Currency = Currencies;
	type Erc20InfoMapping = EvmErc20InfoMapping<Runtime>;
	type PricingPegged = PricingPegged;
	type WeightInfo = weights::module_prices::WeightInfo<Runtime>;
}

impl module_dex_oracle::Config for Runtime {
	type DEX = Dex;
	type Time = Timestamp;
	type UpdateOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = weights::module_dex_oracle::WeightInfo<Runtime>;
}

parameter_types! {
	pub const GetExchangeFee: (u32, u32) = (1, 1000);	// 0.1%
	pub const ExtendedProvisioningBlocks: BlockNumber = 2 * DAYS;
	pub const TradingPathLimit: u32 = 4;
	pub AlternativeSwapPathJointList: Vec<Vec<CurrencyId>> = vec![
		vec![SEL],
		vec![LSEL],
	];
}

impl module_dex::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetExchangeFee = GetExchangeFee;
	type TradingPathLimit = TradingPathLimit;
	type PalletId = DEXPalletId;
	type Erc20InfoMapping = EvmErc20InfoMapping<Runtime>;
	type DEXIncentives = Incentives;
	type WeightInfo = weights::module_dex::WeightInfo<Runtime>;
	type ListingOrigin = EnsureRootOrHalfCouncil;
	type ExtendedProvisioningBlocks = ExtendedProvisioningBlocks;
	type OnLiquidityPoolUpdated = ();
}

impl module_aggregated_dex::Config for Runtime {
	type DEX = Dex;
	type StableAsset = RebasedStableAsset;
	type GovernanceOrigin = EnsureRootOrHalfCouncil;
	type DexSwapJointList = AlternativeSwapPathJointList;
	type SwapPathLimit = ConstU32<3>;
	type WeightInfo = ();
}
