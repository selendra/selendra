// This file is part of Selendra.

// Copyright (C) 2020-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

//! The Dev runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit.
#![recursion_limit = "512"]
#![allow(clippy::unnecessary_mut_passed)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::from_over_into)]
#![allow(clippy::upper_case_acronyms)]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use codec::{Decode, DecodeLimit, Encode};
use pallet_grandpa::{
	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use scale_info::TypeInfo;
use sp_api::impl_runtime_apis;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160};
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{
		AccountIdConversion, AccountIdLookup, BadOrigin, BlakeTwo256, Block as BlockT,
		BlockNumberProvider, Convert, NumberFor, SaturatedConversion, StaticLookup, Zero,
	},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult, DispatchResult, FixedPointNumber, Perbill, Percent, Permill, Perquintill,
};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_system::EnsureRoot;
use module_asset_registry::{AssetIdMaps, EvmErc20InfoMapping};
use module_currencies::BasicCurrencyAdapter;
use module_evm::{runner::RunnerExtended, CallInfo, CreateInfo, EvmChainId, EvmTask};
use module_evm_accounts::EvmAddressMapping;
use module_support::{AssetIdMapping, DispatchableTask, PoolId};
use module_transaction_payment::TargetedFeeAdjustment;

use orml_traits::{
	create_median_value_data_provider, parameter_type_with_key, DataFeeder, DataProviderExtended,
	GetByKey,
};
use pallet_session::historical::{self as pallet_session_historical};
use pallet_transaction_payment::RuntimeDispatchInfo;

use frame_support::{
	construct_runtime, log,
	pallet_prelude::InvalidTransaction,
	parameter_types,
	traits::{
		ConstU16, ConstU32, Contains, EqualPrivilegeOnly, InstanceFilter,
		KeyOwnerProofSystem, LockIdentifier,
	},
	weights::{constants::RocksDbWeight, Weight},
	PalletId, RuntimeDebug,
};

pub use pallet_collective::MemberCount;
pub use pallet_timestamp::Call as TimestampCall;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

pub use constants::{fee::*, time::*};
use primitives::currency::AssetIds;
pub use primitives::{
	define_combined_task,
	evm::{AccessListItem, BlockLimits, EstimateResourcesRequest, EthereumTransactionMessage},
	task::TaskResult,
	unchecked_extrinsic::SelendraUncheckedExtrinsic,
	AccountId, AccountIndex, Address, Amount, AuthoritysOriginId, Balance, BlockNumber, CurrencyId,
	DataProviderId, EraIndex, Hash, Lease, Moment, Multiplier, Nonce, ReserveIdentifier, Share,
	Signature, TokenSymbol, TradingPair,
};
pub use runtime_common::{
	cent, dollar, microcent, millicent, prod_or_fast, AllPrecompiles,
	EnsureRootOrAllGeneralCouncil, EnsureRootOrAllTechnicalCommittee,
	EnsureRootOrHalfFinancialCouncil, EnsureRootOrHalfGeneralCouncil,
	EnsureRootOrOneGeneralCouncil, EnsureRootOrOneThirdsTechnicalCommittee,
	EnsureRootOrThreeFourthsGeneralCouncil, EnsureRootOrTwoThirdsGeneralCouncil,
	EnsureRootOrTwoThirdsTechnicalCommittee, ExchangeRate, ExistentialDepositsTimesOneHundred,
	FinancialCouncilInstance, FinancialCouncilMembershipInstance, GasToWeight,
	GeneralCouncilInstance, GeneralCouncilMembershipInstance, MaxTipsOfPriority,
	OffchainSolutionWeightLimit, OperationalFeeMultiplier, OperatorMembershipInstanceSelendra,
	Price, ProxyType, Rate, Ratio, RuntimeBlockLength, RuntimeBlockWeights, SystemContractsFilter,
	TechnicalCommitteeInstance, TechnicalCommitteeMembershipInstance, TimeStampedPrice,
	TipPerWeightStep, DOT, KMD, RENBTC, SEL, SUSD,
};

pub mod constants;

mod authority;
mod benchmarking;
mod voter_bags;
mod weights;

mod consensus_config;
mod governance_config;

use consensus_config::EpochDuration;

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("selendra"),
	impl_name: create_runtime_str!("selendra"),
	authoring_version: 1,
	spec_version: 101,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// The version infromation used to identify this runtime when compiled
/// natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

/// The BABE epoch configuration at genesis.
pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
	sp_consensus_babe::BabeEpochConfiguration {
		c: PRIMARY_PROBABILITY,
		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
	};

impl_opaque_keys! {
	pub struct SessionKeys {
		pub grandpa: Grandpa,
		pub babe: Babe,
		pub im_online: ImOnline,
		pub authority_discovery: AuthorityDiscovery,
	}
}

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

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	pub const SS58Prefix: u8 = 42;
}

pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
	fn contains(call: &Call) -> bool {
		let is_core_call = matches!(call, Call::System(_) | Call::Timestamp(_));
		if is_core_call {
			// always allow core call
			return true
		}

		let is_paused =
			module_transaction_pause::PausedTransactionFilter::<Runtime>::contains(call);
		if is_paused {
			// no paused call
			return false
		}
		true
	}
}

impl frame_system::Config for Runtime {
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = (AccountIdLookup<AccountId, AccountIndex>, EvmAccounts);
	type Index = Nonce;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = RuntimeBlockWeights;
	type BlockLength = RuntimeBlockLength;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount =
		(module_evm::CallKillAccount<Runtime>, module_evm_accounts::CallKillAccount<Runtime>);
	type DbWeight = RocksDbWeight;
	type BaseCallFilter = BaseCallFilter;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

parameter_types! {
	pub NativeTokenExistentialDeposit: Balance = 10 * cent(SEL);	// 0.1 SEL
	pub const MaxReserves: u32 = ReserveIdentifier::Count as u32;
	// For weight estimation, we assume that the most locks on an individual account will be 50.
	// This number may need to be adjusted in the future if this assumption no longer holds true.
	pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = Treasury;
	type Event = Event;
	type ExistentialDeposit = NativeTokenExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = ReserveIdentifier;
	type WeightInfo = ();
}

parameter_types! {
	/// The portion of the `NORMAL_DISPATCH_RATIO` that we adjust the fees with. Blocks filled less
	/// than this will decrease the weight and more will increase.
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	/// The adjustment variable of the runtime. Higher values will cause `TargetBlockFullness` to
	/// change the fees more rapidly.
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(3, 100_000);
	/// Minimum amount of the multiplier. This value cannot be too low. A test case should ensure
	/// that combined with `AdjustmentVariable`, we can recover from the minimum.
	/// See `multiplier_can_grow_from_zero`.
	pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000u128);
}

impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub MultisigDepositBase: Balance = deposit(1, 88);
	pub MultisigDepositFactor: Balance = deposit(0, 32);
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = MultisigDepositBase;
	type DepositFactor = MultisigDepositFactor;
	type MaxSignatories = ConstU16<100>;
	type WeightInfo = ();
}

parameter_types! {
	pub const Burn: Permill = Permill::from_percent(10);
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);

	pub ProposalBondMinimum: Balance = 10 * dollar(SEL);
	pub ProposalBondMaximum: Balance = 50 * dollar(SEL);
	pub TipReportDepositBase: Balance = deposit(1, 0);
	pub BountyDepositBase: Balance = deposit(1, 0);
	pub CuratorDepositMin: Balance = dollar(SEL);
	pub CuratorDepositMax: Balance = 100 * dollar(SEL);
	pub BountyValueMinimum: Balance = 5 * dollar(SEL);
	pub DataDepositPerByte: Balance = deposit(0, 1);

	pub const SpendPeriod: BlockNumber = 14 * DAYS;
	pub const TipCountdown: BlockNumber = 2 * DAYS;
	pub const BountyDepositPayoutDelay: BlockNumber = 6 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 35 * DAYS;

	pub const MaximumReasonLength: u32 = 8192;
	pub const MaxApprovals: u32 = 100;
	pub const SevenDays: BlockNumber = 7 * DAYS;
	pub const OneDay: BlockNumber = DAYS;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrHalfGeneralCouncil;
	type RejectOrigin = EnsureRootOrHalfGeneralCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = ();
	type MaxApprovals = MaxApprovals;
}

impl pallet_bounties::Config for Runtime {
	type Event = Event;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type BountyValueMinimum = BountyValueMinimum;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = ();
	type ChildBountyManager = ();
}

impl pallet_tips::Config for Runtime {
	type Event = Event;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = PhragmenElection;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinimumCount: u32 = 5;
	pub const ExpiresIn: Moment = 1000 * 60 * 60; // 1 hours
	pub RootOperatorAccountId: AccountId = AccountId::from([0xffu8; 32]);
}

type SelendraDataProvider = orml_oracle::Instance1;
impl orml_oracle::Config<SelendraDataProvider> for Runtime {
	type Event = Event;
	type OnNewData = ();
	type CombineData =
		orml_oracle::DefaultCombineData<Runtime, MinimumCount, ExpiresIn, SelendraDataProvider>;
	type Time = Timestamp;
	type OracleKey = CurrencyId;
	type OracleValue = Price;
	type RootOperatorAccountId = RootOperatorAccountId;
	type Members = OperatorMembershipSelendra;
	type MaxHasDispatchedSize = ConstU32<20>;
	type WeightInfo = ();
}

create_median_value_data_provider!(
	AggregatedDataProvider,
	CurrencyId,
	Price,
	TimeStampedPrice,
	[SelendraOracle]
);
// Aggregated data provider cannot feed.
impl DataFeeder<CurrencyId, Price, AccountId> for AggregatedDataProvider {
	fn feed_value(_: AccountId, _: CurrencyId, _: Price) -> DispatchResult {
		Err("Not supported".into())
	}
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: CurrencyId| -> Balance {
		match currency_id {
			CurrencyId::Token(symbol) => match symbol {
				TokenSymbol::SUSD => 10 * cent(*currency_id),
				TokenSymbol::DOT => cent(*currency_id),
				TokenSymbol::KMD |
				TokenSymbol::KSM |
				TokenSymbol::RENBTC |
				TokenSymbol::SEL |
				TokenSymbol::CASH => Balance::max_value() // unsupported
			},
			CurrencyId::DexShare(dex_share_0, _) => {
				let currency_id_0: CurrencyId = (*dex_share_0).into();

				// initial dex share amount is calculated based on currency_id_0,
				// use the ED of currency_id_0 as the ED of lp token.
				if currency_id_0 == GetNativeCurrencyId::get() {
					NativeTokenExistentialDeposit::get()
				} else if let CurrencyId::Erc20(address) = currency_id_0 {
					// LP token with erc20
					AssetIdMaps::<Runtime>::get_asset_metadata(AssetIds::Erc20(address)).
						map_or(Balance::max_value(), |metatata| metatata.minimal_balance)
				} else {
					Self::get(&currency_id_0)
				}
			},
			CurrencyId::Erc20(_) => Balance::max_value(), // not handled by orml-tokens
			CurrencyId::ForeignAsset(_) => todo!()
		}
	};
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		get_all_module_accounts().contains(a)
	}
}

parameter_types! {
	pub SelendraTreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = CurrencyId;
	type WeightInfo = weights::orml_tokens::WeightInfo<Runtime>;
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = orml_tokens::TransferDust<Runtime, SelendraTreasuryAccount>;
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = ReserveIdentifier;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

parameter_types! {
	pub StableCurrencyFixedPrice: Price = Price::saturating_from_rational(1, 1);
	pub RewardRatePerRelaychainBlock: Rate = Rate::saturating_from_rational(2_492, 100_000_000_000u128);	// 14% annual staking reward rate of Polkadot
}

impl module_prices::Config for Runtime {
	type Event = Event;
	type Source = AggregatedDataProvider;
	type GetStableCurrencyId = GetStableCurrencyId;
	type StableCurrencyFixedPrice = StableCurrencyFixedPrice;
	type LockOrigin = EnsureRootOrTwoThirdsGeneralCouncil;
	type DEX = Dex;
	type Currency = Currencies;
	type Erc20InfoMapping = EvmErc20InfoMapping<Runtime>;
	type WeightInfo = weights::module_prices::WeightInfo<Runtime>;
}

parameter_types! {
	pub const GetNativeCurrencyId: CurrencyId = SEL;
	pub const GetStableCurrencyId: CurrencyId = SUSD;
	pub Erc20HoldingAccount: H160 = primitives::evm::ERC20_HOLDING_ACCOUNT;
}

impl module_currencies::Config for Runtime {
	type Event = Event;
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Runtime, Balances, Amount, BlockNumber>;
	type GetNativeCurrencyId = GetNativeCurrencyId;
	type Erc20HoldingAccount = Erc20HoldingAccount;
	type WeightInfo = weights::module_currencies::WeightInfo<Runtime>;
	type AddressMapping = EvmAddressMapping<Runtime>;
	type EVMBridge = module_evm_bridge::EVMBridge<Runtime>;
	type GasToWeight = GasToWeight;
	type SweepOrigin = EnsureRootOrOneGeneralCouncil;
	type OnDust = module_currencies::TransferDust<Runtime, SelendraTreasuryAccount>;
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(10) * RuntimeBlockWeights::get().max_block;
	// Retry a scheduled item every 25 blocks (5 minute) until the preimage exists.
	pub const NoPreimagePostponement: Option<u32> = Some(5 * MINUTES);
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = ConstU32<10>;
	type WeightInfo = ();
	type OriginPrivilegeCmp = EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
}

parameter_types! {
	pub PreimageBaseDeposit: Balance = deposit(2, 64);
	pub PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = ();
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	// Max size 4MB allowed: 4096 * 1024
	type MaxSize = ConstU32<4194304>;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: Nonce,
	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			runtime_common::CheckNonce::<Runtime>::from(nonce),
			frame_system::CheckWeight::<Runtime>::new(),
			module_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
			module_evm::SetEvmOrigin::<Runtime>::new(),
		);
		let raw_payload = SignedPayload::new(call, extra)
			.map_err(|e| {
				log::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = AccountIdLookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}
impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}

impl module_emergency_shutdown::Config for Runtime {
	type Event = Event;
	type PriceSource = Prices;
	type SelTreasury = SelTreasury;
	type ShutdownOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::module_emergency_shutdown::WeightInfo<Runtime>;
}

parameter_types! {
	pub const GetExchangeFee: (u32, u32) = (3, 1000);	// 0.3%
	pub const ExtendedProvisioningBlocks: BlockNumber = 2 * DAYS;
	pub const TradingPathLimit: u32 = 4;
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
	type ListingOrigin = EnsureRootOrHalfGeneralCouncil;
	type ExtendedProvisioningBlocks = ExtendedProvisioningBlocks;
	type OnLiquidityPoolUpdated = ();
}

impl module_dex_oracle::Config for Runtime {
	type DEX = Dex;
	type Time = Timestamp;
	type UpdateOrigin = EnsureRootOrHalfGeneralCouncil;
	type WeightInfo = weights::module_dex_oracle::WeightInfo<Runtime>;
}

impl module_treasury::Config for Runtime {
	type Currency = Currencies;
	type GetStableCurrencyId = GetStableCurrencyId;
	type PalletId = SelTreasuryPalletId;
}

impl module_transaction_pause::Config for Runtime {
	type Event = Event;
	type UpdateOrigin = EnsureRootOrTwoThirdsGeneralCouncil;
	type WeightInfo = weights::module_transaction_pause::WeightInfo<Runtime>;
}

parameter_types! {
	pub const CustomFeeSurplus: Percent = Percent::from_percent(50);
	pub const AlternativeFeeSurplus: Percent = Percent::from_percent(25);
	pub TransactionByteFee: Balance = 5 * microcent(SEL);
	pub DefaultFeeTokens: Vec<CurrencyId> = vec![SUSD, DOT, KMD];
	pub MaxSwapSlippageCompareToOracle: Ratio = Ratio::saturating_from_rational(15, 100);
}

pub type SlowAdjustingFeeUpdate<R> =
	TargetedFeeAdjustment<R, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;

impl module_transaction_payment::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type NativeCurrencyId = GetNativeCurrencyId;
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type OnTransactionPayment = ();
	type AlternativeFeeSwapDeposit = NativeTokenExistentialDeposit;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type TipPerWeightStep = TipPerWeightStep;
	type MaxTipsOfPriority = MaxTipsOfPriority;
	type WeightToFee = WeightToFee;
	type TransactionByteFee = TransactionByteFee;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type DEX = Dex;
	type MaxSwapSlippageCompareToOracle = MaxSwapSlippageCompareToOracle;
	type TradingPathLimit = TradingPathLimit;
	type PriceSource = module_prices::RealTimePriceProvider<Runtime>;
	type WeightInfo = weights::module_transaction_payment::WeightInfo<Runtime>;
	type PalletId = TransactionPaymentPalletId;
	type TreasuryAccount = SelendraTreasuryAccount;
	type UpdateOrigin = EnsureRootOrHalfGeneralCouncil;
	type CustomFeeSurplus = CustomFeeSurplus;
	type AlternativeFeeSurplus = AlternativeFeeSurplus;
	type DefaultFeeTokens = DefaultFeeTokens;
}

impl module_evm_accounts::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type AddressMapping = EvmAddressMapping<Runtime>;
	type TransferAll = Currencies;
	type ChainId = EvmChainId<Runtime>;
	type WeightInfo = weights::module_evm_accounts::WeightInfo<Runtime>;
}

impl module_asset_registry::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type EVMBridge = module_evm_bridge::EVMBridge<Runtime>;
	type RegisterOrigin = EnsureRootOrHalfGeneralCouncil;
	type WeightInfo = weights::module_asset_registry::WeightInfo<Runtime>;
}

impl orml_rewards::Config for Runtime {
	type Share = Balance;
	type Balance = Balance;
	type PoolId = PoolId;
	type CurrencyId = CurrencyId;
	type Handler = Incentives;
}

parameter_types! {
	pub const AccumulatePeriod: BlockNumber = MINUTES;
	pub const EarnShareBooster: Permill = Permill::from_percent(30);
}

impl module_incentives::Config for Runtime {
	type Event = Event;
	type RewardsSource = UnreleasedNativeVaultAccountId;
	type StableCurrencyId = GetStableCurrencyId;
	type NativeCurrencyId = GetNativeCurrencyId;
	type EarnShareBooster = EarnShareBooster;
	type AccumulatePeriod = AccumulatePeriod;
	type UpdateOrigin = EnsureRootOrThreeFourthsGeneralCouncil;
	type SelTreasury = SelTreasury;
	type Currency = Currencies;
	type DEX = Dex;
	type EmergencyShutdown = EmergencyShutdown;
	type PalletId = IncentivesPalletId;
	type WeightInfo = weights::module_incentives::WeightInfo<Runtime>;
}

parameter_types! {
	pub CreateClassDeposit: Balance = 50 * dollar(SEL);
	pub CreateTokenDeposit: Balance = 20 * cent(SEL);
}

impl module_nft::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type CreateClassDeposit = CreateClassDeposit;
	type CreateTokenDeposit = CreateTokenDeposit;
	type DataDepositPerByte = DataDepositPerByte;
	type PalletId = NftPalletId;
	type MaxAttributesBytes = ConstU32<2048>;
	type WeightInfo = weights::module_nft::WeightInfo<Runtime>;
}

impl orml_nft::Config for Runtime {
	type ClassId = u32;
	type TokenId = u64;
	type ClassData = module_nft::ClassData<Balance>;
	type TokenData = module_nft::TokenData<Balance>;
	type MaxClassMetadata = ConstU32<1024>;
	type MaxTokenMetadata = ConstU32<1024>;
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			// Always allowed Call::Utility no matter type.
			// Only transactions allowed by Proxy.filter can be executed,
			// otherwise `BadOrigin` will be returned in Call::Utility.
			_ if matches!(c, Call::Utility(..)) => true,
			ProxyType::Any => true,
			ProxyType::CancelProxy =>
				matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. })),
			ProxyType::Governance => {
				matches!(
					c,
					Call::Authority(..) |
						Call::Democracy(..) | Call::GeneralCouncil(..) |
						Call::FinancialCouncil(..) |
						Call::TechnicalCommittee(..) |
						Call::Treasury(..) | Call::Bounties(..) |
						Call::Tips(..)
				)
			},
			ProxyType::Swap => {
				matches!(
					c,
					Call::Dex(module_dex::Call::swap_with_exact_supply { .. }) |
						Call::Dex(module_dex::Call::swap_with_exact_target { .. })
				)
			},
			ProxyType::DexLiquidity => {
				matches!(
					c,
					Call::Dex(module_dex::Call::add_liquidity { .. }) |
						Call::Dex(module_dex::Call::remove_liquidity { .. })
				)
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			_ => false,
		}
	}
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub ProxyDepositFactor: Balance = deposit(0, 33);
	pub AnnouncementDepositBase: Balance = deposit(1, 8);
	pub AnnouncementDepositFactor: Balance = deposit(0, 66);
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = ();
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

parameter_types! {
	pub const NewContractExtraBytes: u32 = 10_000;
	pub NetworkContractSource: H160 = H160::from_low_u64_be(0);
	pub DeveloperDeposit: Balance = 1_000 * dollar(SEL);
	pub PublicationFee: Balance = 1_000_000 * dollar(SEL);
	pub PrecompilesValue: AllPrecompiles<Runtime> = AllPrecompiles::<_>::selendra();
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct StorageDepositPerByte;
impl<I: From<Balance>> frame_support::traits::Get<I> for StorageDepositPerByte {
	fn get() -> I {
		// NOTE: SEL decimals is 12, convert to 18.
		// 30 * millicent(SEL) * 10^6
		I::from(300_000_000_000_000)
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct TxFeePerGas;
impl<I: From<Balance>> frame_support::traits::Get<I> for TxFeePerGas {
	fn get() -> I {
		// NOTE: 200 GWei
		// ensure suffix is 0x0000
		I::from(200u128.saturating_mul(10u128.saturating_pow(9)) & !0xffff)
	}
}

impl module_evm::Config for Runtime {
	type AddressMapping = EvmAddressMapping<Runtime>;
	type Currency = Balances;
	type TransferAll = Currencies;
	type NewContractExtraBytes = NewContractExtraBytes;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = TxFeePerGas;
	type Event = Event;
	type PrecompilesType = AllPrecompiles<Self>;
	type PrecompilesValue = PrecompilesValue;
	type GasToWeight = GasToWeight;
	type ChargeTransactionPayment = module_transaction_payment::ChargeTransactionPayment<Runtime>;
	type NetworkContractOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type NetworkContractSource = NetworkContractSource;
	type DeveloperDeposit = DeveloperDeposit;
	type PublicationFee = PublicationFee;
	type TreasuryAccount = SelendraTreasuryAccount;
	type FreePublicationOrigin = EnsureRootOrHalfGeneralCouncil;
	type Runner = module_evm::runner::stack::Runner<Self>;
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type Task = ScheduledTasks;
	type IdleScheduler = IdleScheduler;
	type WeightInfo = weights::module_evm::WeightInfo<Runtime>;
}

impl module_evm_bridge::Config for Runtime {
	type EVM = EVM;
}

define_combined_task! {
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum ScheduledTasks {
		EvmTask(EvmTask<Runtime>),
	}
}

parameter_types!(
	// At least 2% of max block weight should remain before idle tasks are dispatched.
	pub MinimumWeightRemainInBlock: Weight = RuntimeBlockWeights::get().max_block / 50;
);

pub struct MockBlockNumberProvider;

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u32;

	fn current_block_number() -> Self::BlockNumber {
		Zero::zero()
	}
}

impl module_idle_scheduler::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type RelayChainBlockNumberProvider = MockBlockNumberProvider;
	type DisableBlockThreshold = ConstU32<6>;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = primitives::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// Core & Utility
		System: frame_system = 0,
		Timestamp: pallet_timestamp = 1,
		Scheduler: pallet_scheduler = 2,
		Utility: pallet_utility = 3,
		Multisig: pallet_multisig = 4,
		Proxy: pallet_proxy = 5,
		TransactionPause: module_transaction_pause = 6,

		IdleScheduler: module_idle_scheduler = 7,
		Preimage: pallet_preimage = 8,

		// Tokens & Related
		Balances: pallet_balances = 10,
		Tokens: orml_tokens exclude_parts { Call } = 11,
		Currencies: module_currencies = 12,
		TransactionPayment: module_transaction_payment = 14,

		// Treasury
		Treasury: pallet_treasury = 20,
		Bounties: pallet_bounties = 21,
		Tips: pallet_tips = 22,

		// Consensus
		// Authorship must be before session in order to note author in the correct session and era
		// for im-online and staking.
		Authorship: pallet_authorship = 30,
		Babe: pallet_babe = 31,
		Staking: pallet_staking = 32,
		Offences: pallet_offences = 33,
		Historical: pallet_session_historical::{Pallet} = 34,
		Session: pallet_session = 35,
		Grandpa: pallet_grandpa = 36,
		ImOnline: pallet_im_online = 37,
		AuthorityDiscovery: pallet_authority_discovery = 38,
		ElectionProviderMultiPhase: pallet_election_provider_multi_phase = 39,

		// norminator
		VoterList: pallet_bags_list = 40,
		NominationPools: pallet_nomination_pools = 41,

		// Governance
		Authority: orml_authority = 60,
		GeneralCouncil: pallet_collective::<Instance1> = 61,
		GeneralCouncilMembership: pallet_membership::<Instance1> = 62,
		FinancialCouncil: pallet_collective::<Instance2> = 63,
		FinancialCouncilMembership: pallet_membership::<Instance2> = 64,
		PhragmenElection: pallet_elections_phragmen::{Pallet, Call, Storage, Event<T>, Config<T>} = 65,
		TechnicalCommittee: pallet_collective::<Instance4> = 67,
		TechnicalCommitteeMembership: pallet_membership::<Instance4> = 68,
		Democracy: pallet_democracy = 69,

		// Oracle
		//
		// NOTE: OperatorMembership must be placed after Oracle or else will have race condition on initialization
		SelendraOracle: orml_oracle::<Instance1> = 70,
		OperatorMembershipSelendra: pallet_membership::<Instance5> = 71,

		// ORML Core
		Rewards: orml_rewards = 81,
		OrmlNFT: orml_nft exclude_parts { Call } = 82,

		// Selendra Core
		Prices: module_prices = 90,
		Dex: module_dex = 91,
		DexOracle: module_dex_oracle = 92,

		SelTreasury: module_treasury = 103,
		EmergencyShutdown: module_emergency_shutdown = 105,

		// Selendra Other
		Incentives: module_incentives = 120,
		NFT: module_nft = 121,
		AssetRegistry: module_asset_registry = 122,

		// Smart contracts
		EVM: module_evm = 130,
		EVMBridge: module_evm_bridge exclude_parts { Call } = 131,
		EvmAccounts: module_evm_accounts = 132,

		// Temporary
		Sudo: pallet_sudo = 255,
	}
);

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	runtime_common::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	module_transaction_payment::ChargeTransactionPayment<Runtime>,
	module_evm::SetEvmOrigin<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = SelendraUncheckedExtrinsic<
	Call,
	SignedExtra,
	ConvertEthereumTx,
	StorageDepositPerByte,
	TxFeePerGas,
	PayerSignatureVerification,
>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	(),
>;

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate orml_benchmarking;

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[pallet_babe, Babe]
		[module_dex, benchmarking::dex]
		[module_dex_oracle, benchmarking::dex_oracle]
		[module_asset_registry, benchmarking::asset_registry]
		[module_emergency_shutdown, benchmarking::emergency_shutdown]
		[module_evm, benchmarking::evm]
		[module_treasury, benchmarking::treasury]
		[module_transaction_pause, benchmarking::transaction_pause]
		[module_transaction_payment, benchmarking::transaction_payment]
		[module_incentives, benchmarking::incentives]
		[module_prices, benchmarking::prices]
		[module_evm_accounts, benchmarking::evm_accounts]
		[module_currencies, benchmarking::currencies]
		[orml_tokens, benchmarking::tokens]
		[orml_authority, benchmarking::authority]
		[orml_oracle, benchmarking::oracle]
		[module_idle_scheduler, benchmarking::idle_scheduler]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}

		fn current_set_id() -> fg_primitives::SetId {
			Grandpa::current_set_id()
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: fg_primitives::EquivocationProof<
				<Block as BlockT>::Hash,
				NumberFor<Block>,
			>,
			key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Grandpa::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}

		fn generate_key_ownership_proof(
			_set_id: fg_primitives::SetId,
			authority_id: GrandpaId,
		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((fg_primitives::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(fg_primitives::OpaqueKeyOwnershipProof::new)
		}
	}

	impl sp_consensus_babe::BabeApi<Block> for Runtime {
		fn configuration() -> sp_consensus_babe::BabeGenesisConfiguration {
			// The choice of `c` parameter (where `1 - c` represents the
			// probability of a slot being empty), is done in accordance to the
			// slot duration and expected target block time, for safely
			// resisting network delays of maximum two seconds.
			// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
			sp_consensus_babe::BabeGenesisConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: BABE_GENESIS_EPOCH_CONFIG.c,
				genesis_authorities: Babe::authorities().to_vec(),
				randomness: Babe::randomness(),
				allowed_slots: BABE_GENESIS_EPOCH_CONFIG.allowed_slots,
			}
		}

		fn current_epoch_start() -> sp_consensus_babe::Slot {
			Babe::current_epoch_start()
		}

		fn current_epoch() -> sp_consensus_babe::Epoch {
			Babe::current_epoch()
		}

		fn next_epoch() -> sp_consensus_babe::Epoch {
			Babe::next_epoch()
		}

		fn generate_key_ownership_proof(
			_slot: sp_consensus_babe::Slot,
			authority_id: sp_consensus_babe::AuthorityId,
		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
			use codec::Encode;

			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
				.map(|p| p.encode())
				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
		}

		fn submit_report_equivocation_unsigned_extrinsic(
			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
		) -> Option<()> {
			let key_owner_proof = key_owner_proof.decode()?;

			Babe::submit_unsigned_equivocation_report(
				equivocation_proof,
				key_owner_proof,
			)
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}

		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
		fn account_nonce(account: AccountId) -> Nonce {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
	> for Runtime {
		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}

		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> pallet_transaction_payment_rpc_runtime_api::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl orml_oracle_rpc_runtime_api::OracleApi<
		Block,
		DataProviderId,
		CurrencyId,
		TimeStampedPrice,
	> for Runtime {
		fn get_value(provider_id: DataProviderId ,key: CurrencyId) -> Option<TimeStampedPrice> {
			match provider_id {
				DataProviderId::Selendra => SelendraOracle::get_no_op(&key),
				DataProviderId::Aggregated => <AggregatedDataProvider as DataProviderExtended<_, _>>::get_no_op(&key)
			}
		}

		fn get_all_values(provider_id: DataProviderId) -> Vec<(CurrencyId, Option<TimeStampedPrice>)> {
			match provider_id {
				DataProviderId::Selendra => SelendraOracle::get_all_values(),
				DataProviderId::Aggregated => <AggregatedDataProvider as DataProviderExtended<_, _>>::get_all_values()
			}
		}
	}

	impl orml_tokens_rpc_runtime_api::TokensApi<
		Block,
		CurrencyId,
		Balance,
	> for Runtime {
		fn query_existential_deposit(key: CurrencyId) -> Balance {
			if key == GetNativeCurrencyId::get() {
				NativeTokenExistentialDeposit::get()
			} else {
				ExistentialDeposits::get(&key)
			}
		}
	}

	impl module_evm_rpc_runtime_api::EVMRuntimeRPCApi<Block, Balance> for Runtime {
		fn block_limits() -> BlockLimits {
			BlockLimits {
				max_gas_limit: runtime_common::EvmLimits::<Runtime>::max_gas_limit(),
				max_storage_limit: runtime_common::EvmLimits::<Runtime>::max_storage_limit(),
			}
		}

		fn call(
			from: H160,
			to: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			_estimate: bool,
		) -> Result<CallInfo, sp_runtime::DispatchError> {
			<Runtime as module_evm::Config>::Runner::rpc_call(
				from,
				from,
				to,
				data,
				value,
				gas_limit,
				storage_limit,
				access_list.unwrap_or_default().into_iter().map(|v| (v.address, v.storage_keys)).collect(),
				<Runtime as module_evm::Config>::config(),
			)
		}

		fn create(
			from: H160,
			data: Vec<u8>,
			value: Balance,
			gas_limit: u64,
			storage_limit: u32,
			access_list: Option<Vec<AccessListItem>>,
			_estimate: bool,
		) -> Result<CreateInfo, sp_runtime::DispatchError> {
			<Runtime as module_evm::Config>::Runner::rpc_create(
				from,
				data,
				value,
				gas_limit,
				storage_limit,
				access_list.unwrap_or_default().into_iter().map(|v| (v.address, v.storage_keys)).collect(),
				<Runtime as module_evm::Config>::config(),
			)
		}

		fn get_estimate_resources_request(extrinsic: Vec<u8>) -> Result<EstimateResourcesRequest, sp_runtime::DispatchError> {
			let utx = UncheckedExtrinsic::decode_all_with_depth_limit(sp_api::MAX_EXTRINSIC_DEPTH, &mut &*extrinsic)
				.map_err(|_| sp_runtime::DispatchError::Other("Invalid parameter extrinsic, decode failed"))?;

			let request = match utx.0.function {
				Call::EVM(module_evm::Call::call{target, input, value, gas_limit, storage_limit, access_list}) => {
					Some(EstimateResourcesRequest {
						from: None,
						to: Some(target),
						gas_limit: Some(gas_limit),
						storage_limit: Some(storage_limit),
						value: Some(value),
						data: Some(input),
						access_list: Some(access_list)
					})
				}
				Call::EVM(module_evm::Call::create{input, value, gas_limit, storage_limit, access_list}) => {
					Some(EstimateResourcesRequest {
						from: None,
						to: None,
						gas_limit: Some(gas_limit),
						storage_limit: Some(storage_limit),
						value: Some(value),
						data: Some(input),
						access_list: Some(access_list)
					})
				}
				_ => None,
			};

			request.ok_or(sp_runtime::DispatchError::Other("Invalid parameter extrinsic, not evm Call"))
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block_no_check(block: Block) -> Weight {
			Executive::execute_block_no_check(block)
		}
	}

	// benchmarks for selendra modules
	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{list_benchmark as frame_list_benchmark, Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use module_nft::benchmarking::Pallet as NftBench;

			let mut list = Vec::<BenchmarkList>::new();

			frame_list_benchmark!(list, extra, module_nft, NftBench::<Runtime>);
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();

			return (list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark as frame_add_benchmark, TrackedStorageKey};
			use module_nft::benchmarking::Pallet as NftBench;

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				// frame_system::Number::<Runtime>::hashed_key().to_vec(),
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519sel4983ac").to_vec().into(),
				// Total Issuance
				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
				// Caller 0 Account
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da946c154ffd9992e395af90b5b13cc6f295c77033fce8a9045824a6690bbf99c6db269502f0a8d1d2a008542d5690a0749").to_vec().into(),
				// Treasury Account
				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da95ecffd7b6c0f78751baa9d281e0bfa3a6d6f646c70792f74727372790000000000000000000000000000000000000000").to_vec().into(),
			];
			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);

			frame_add_benchmark!(params, batches, module_nft, NftBench::<Runtime>);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this module.".into()) }
			Ok(batches)
		}
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct ConvertEthereumTx;

impl
	Convert<
		(Call, SignedExtra),
		Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction>,
	> for ConvertEthereumTx
{
	fn convert(
		(call, mut extra): (Call, SignedExtra),
	) -> Result<(EthereumTransactionMessage, SignedExtra), InvalidTransaction> {
		if let Call::EVM(module_evm::Call::eth_call {
			action,
			input,
			value,
			gas_limit,
			storage_limit,
			access_list,
			valid_until,
		}) = call
		{
			if System::block_number() > valid_until {
				return Err(InvalidTransaction::Stale)
			}

			let (_, _, _, _, mortality, check_nonce, _, charge, ..) = extra.clone();

			if mortality != frame_system::CheckEra::from(sp_runtime::generic::Era::Immortal) {
				// require immortal
				return Err(InvalidTransaction::BadProof)
			}

			let nonce = check_nonce.nonce;
			let tip = charge.0;

			extra.5.mark_as_ethereum_tx(valid_until);

			Ok((
				EthereumTransactionMessage {
					chain_id: EVM::chain_id(),
					genesis: System::block_hash(0),
					nonce,
					tip,
					gas_limit,
					storage_limit,
					action,
					value,
					input,
					valid_until,
					access_list,
				},
				extra,
			))
		} else {
			Err(InvalidTransaction::BadProof)
		}
	}
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct PayerSignatureVerification;

impl Convert<(Call, SignedExtra), Result<(), InvalidTransaction>> for PayerSignatureVerification {
	fn convert((call, _extra): (Call, SignedExtra)) -> Result<(), InvalidTransaction> {
		if let Call::TransactionPayment(module_transaction_payment::Call::with_fee_paid_by {
			call: _,
			payer_addr: _,
			payer_sig: _,
		}) = call
		{
			// Disabled for now
			return Err(InvalidTransaction::BadProof)
			// let payer_account: [u8; 32] = payer_addr
			// 	.encode()
			// 	.as_slice()
			// 	.try_into()
			// 	.map_err(|_| InvalidTransaction::BadSigner)?;
			// // payer signature is aim at inner call of `with_fee_paid_by` call.
			// let raw_payload = SignedPayload::new(*call, extra).map_err(|_|
			// InvalidTransaction::BadSigner)?; if !raw_payload.using_encoded(|payload|
			// payer_sig.verify(payload, &payer_account.into())) { 	return Err(InvalidTransaction::
			// BadProof); }
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::weights::DispatchClass;
	use frame_system::offchain::CreateSignedTransaction;
	use sp_runtime::traits::Convert;

	fn run_with_system_weight<F>(w: Weight, mut assertions: F)
	where
		F: FnMut(),
	{
		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();
		t.execute_with(|| {
			System::set_block_consumed_resources(w, 0);
			assertions()
		});
	}

	#[test]
	fn validate_transaction_submitter_bounds() {
		fn is_submit_signed_transaction<T>()
		where
			T: CreateSignedTransaction<Call>,
		{
		}

		is_submit_signed_transaction::<Runtime>();
	}

	#[test]
	fn multiplier_can_grow_from_zero() {
		let minimum_multiplier = MinimumMultiplier::get();
		let target = TargetBlockFullness::get() *
			RuntimeBlockWeights::get().get(DispatchClass::Normal).max_total.unwrap();
		// if the min is too small, then this will not change, and we are doomed forever.
		// the weight is 1/100th bigger than target.
		run_with_system_weight(target * 101 / 100, || {
			let next = SlowAdjustingFeeUpdate::<Runtime>::convert(minimum_multiplier);
			assert!(next > minimum_multiplier, "{:?} !>= {:?}", next, minimum_multiplier);
		})
	}

	#[test]
	fn ensure_can_create_contract() {
		// Ensure that the `ExistentialDeposit` for creating the contract >= account
		// `ExistentialDeposit`. Otherwise, the creation of the contract account will fail because
		// it is less than ExistentialDeposit.
		assert!(
			Balance::from(NewContractExtraBytes::get()).saturating_mul(
				<StorageDepositPerByte as frame_support::traits::Get<Balance>>::get() /
					10u128.saturating_pow(6)
			) >= NativeTokenExistentialDeposit::get()
		);
	}

	#[test]
	fn check_call_size() {
		println!("{:?}", core::mem::size_of::<Call>());
		assert!(
			core::mem::size_of::<Call>() <= 280,
			"size of Call is more than 280 bytes: some calls have too big arguments, use Box to \
			reduce the size of Call.
			If the limit is too strong, maybe consider increasing the limit",
		);
	}
}
