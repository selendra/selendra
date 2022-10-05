use crate::{
	microcent, parameter_types, weights, Balance, Balances, Call, Currencies, CurrencyId, Event,
	GetNativeCurrencyId, MaxSwapSlippageCompareToOracle, NativeTokenExistentialDeposit, Percent,
	Runtime, SelendraSwap, TradingPathLimit, TransactionPaymentPalletId, TreasuryAccount,
	WeightToFee, KUSD, LSEL, SEL,
};
use runtime_common::{
	impls::DealWithFees, EnsureRootOrHalfCouncil, EnsureRootOrThreeFourthsCouncil,
	MaxTipsOfPriority, OperationalFeeMultiplier, SlowAdjustingFeeUpdate, TipPerWeightStep,
};
use sp_std::prelude::*;

parameter_types! {
	pub TransactionByteFee: Balance = 50 * microcent(SEL);
	pub DefaultFeeTokens: Vec<CurrencyId> = vec![KUSD, LSEL];
	pub const CustomFeeSurplus: Percent = Percent::from_percent(50);
	pub const AlternativeFeeSurplus: Percent = Percent::from_percent(25);
}

impl module_transaction_payment::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type NativeCurrencyId = GetNativeCurrencyId;
	type WeightToFee = WeightToFee;
	type DefaultFeeTokens = DefaultFeeTokens;
	type CustomFeeSurplus = CustomFeeSurplus;
	type AlternativeFeeSurplus = AlternativeFeeSurplus;
	type AlternativeFeeSwapDeposit = NativeTokenExistentialDeposit;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type TransactionByteFee = TransactionByteFee;
	type OnTransactionPayment = DealWithFees<Runtime>;
	type TipPerWeightStep = TipPerWeightStep;
	type MaxTipsOfPriority = MaxTipsOfPriority;
	type TreasuryAccount = TreasuryAccount;
	type Swap = SelendraSwap;
	type TradingPathLimit = TradingPathLimit;
	type PriceSource = module_prices::RealTimePriceProvider<Runtime>;
	type MaxSwapSlippageCompareToOracle = MaxSwapSlippageCompareToOracle;
	type PalletId = TransactionPaymentPalletId;
	type UpdateOrigin = EnsureRootOrHalfCouncil;
	type WeightInfo = weights::module_transaction_payment::WeightInfo<Runtime>;
}

