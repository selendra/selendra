use crate::{
	deposit, dollar, parameter_types, weights, AccountId, AccountIdConversion, Auction,
	AuctionManager, Balance, Balances, BlockNumber, CDPEnginePalletId, CDPTreasuryPalletId,
	CdpEngine, CdpTreasury, CollateralCurrencyIds, Currencies, Dex, EmergencyShutdown, Event,
	ExchangeRate, ExistentialDeposits, ExistentialDepositsTimesOneHundred, FixedPointNumber,
	FunanTreasuryPalletId, GetNativeCurrencyId, GetStableCurrencyId, LoansPalletId,
	NativeTokenExistentialDeposit, Prices, Rate, Ratio, RebasedStableAsset, Runtime, Timestamp,
	HOURS, KUSD, MINUTES,
};
use runtime_common::{EnsureRootOrHalfCouncil, EnsureRootOrHalfFinancialCouncil};

parameter_types! {
	pub MinimumIncrementSize: Rate = Rate::saturating_from_rational(2, 100);
	pub const AuctionTimeToClose: BlockNumber = 15 * MINUTES;
	pub const AuctionDurationSoftCap: BlockNumber = 2 * HOURS;
}

impl module_auction_manager::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type Auction = Auction;
	type MinimumIncrementSize = MinimumIncrementSize;
	type AuctionTimeToClose = AuctionTimeToClose;
	type AuctionDurationSoftCap = AuctionDurationSoftCap;
	type GetStableCurrencyId = GetStableCurrencyId;
	type CDPTreasury = CdpTreasury;
	type PriceSource = module_prices::PriorityLockedPriceProvider<Runtime>;
	type UnsignedPriority = runtime_common::AuctionManagerUnsignedPriority;
	type EmergencyShutdown = EmergencyShutdown;
	type WeightInfo = weights::module_auction_manager::WeightInfo<Runtime>;
}

impl module_loans::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type RiskManager = CdpEngine;
	type CDPTreasury = CdpTreasury;
	type PalletId = LoansPalletId;
	type OnUpdateLoan = module_incentives::OnUpdateLoan<Runtime>;
}

parameter_types! {
	pub DepositPerAuthorization: Balance = deposit(1, 64);
}

impl module_funan::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type DepositPerAuthorization = DepositPerAuthorization;
	type CollateralCurrencyIds = CollateralCurrencyIds<Runtime>;
	type WeightInfo = weights::module_funan::WeightInfo<Runtime>;
}

parameter_types! {
	pub FunanTreasuryAccount: AccountId = FunanTreasuryPalletId::get().into_account_truncating();
	pub const MaxAuctionsCount: u32 = 50;
}

pub type SelendraSwap = module_aggregated_dex::AggregatedSwap<Runtime>;

impl module_cdp_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type AuctionManagerHandler = AuctionManager;
	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
	type DEX = Dex;
	type Swap = SelendraSwap;
	type MaxAuctionsCount = MaxAuctionsCount;
	type PalletId = CDPTreasuryPalletId;
	type TreasuryAccount = FunanTreasuryAccount;
	type WeightInfo = weights::module_cdp_treasury::WeightInfo<Runtime>;
	type StableAsset = RebasedStableAsset;
}

parameter_types! {
	pub const MaxLiquidationContracts: u32 = 10;
	pub DefaultLiquidationRatio: Ratio = Ratio::saturating_from_rational(150, 100);
	pub DefaultDebitExchangeRate: ExchangeRate = ExchangeRate::saturating_from_rational(1, 10);
	pub DefaultLiquidationPenalty: Rate = Rate::saturating_from_rational(8, 100);
	pub MinimumDebitValue: Balance = 25 * dollar(KUSD);
	pub MaxSwapSlippageCompareToOracle: Ratio = Ratio::saturating_from_rational(10, 100);
	pub MaxLiquidationContractSlippage: Ratio = Ratio::saturating_from_rational(15, 100);

}

impl module_cdp_engine::Config for Runtime {
	type Event = Event;
	type PriceSource = module_prices::PriorityLockedPriceProvider<Runtime>;
	type DefaultLiquidationRatio = DefaultLiquidationRatio;
	type DefaultDebitExchangeRate = DefaultDebitExchangeRate;
	type DefaultLiquidationPenalty = DefaultLiquidationPenalty;
	type MinimumDebitValue = MinimumDebitValue;
	type MinimumCollateralAmount = ExistentialDepositsTimesOneHundred<
		GetNativeCurrencyId,
		NativeTokenExistentialDeposit,
		ExistentialDeposits,
	>;
	type GetStableCurrencyId = GetStableCurrencyId;
	type CDPTreasury = CdpTreasury;
	type UpdateOrigin = EnsureRootOrHalfFinancialCouncil;
	type MaxSwapSlippageCompareToOracle = MaxSwapSlippageCompareToOracle;
	type UnsignedPriority = runtime_common::CdpEngineUnsignedPriority;
	type EmergencyShutdown = EmergencyShutdown;
	type UnixTime = Timestamp;
	type Currency = Currencies;
	type DEX = Dex;
	type LiquidationContractsUpdateOrigin = EnsureRootOrHalfCouncil;
	type MaxLiquidationContractSlippage = MaxLiquidationContractSlippage;
	type MaxLiquidationContracts = MaxLiquidationContracts;
	type LiquidationEvmBridge = module_evm_bridge::LiquidationEvmBridge<Runtime>;
	type PalletId = CDPEnginePalletId;
	type EvmAddressMapping = module_evm_accounts::EvmAddressMapping<Runtime>;
	type Swap = SelendraSwap;
	type WeightInfo = weights::module_cdp_engine::WeightInfo<Runtime>;
}

impl module_emergency_shutdown::Config for Runtime {
	type Event = Event;
	type CollateralCurrencyIds = CollateralCurrencyIds<Runtime>;
	type PriceSource = Prices;
	type CDPTreasury = CdpTreasury;
	type AuctionManagerHandler = AuctionManager;
	type ShutdownOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = weights::module_emergency_shutdown::WeightInfo<Runtime>;
}
