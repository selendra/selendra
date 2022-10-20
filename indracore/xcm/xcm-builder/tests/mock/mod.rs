// Copyright (C) 2021-2022 Selendra.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use frame_support::{
	construct_runtime, parameter_types,
	traits::{Everything, Nothing},
	weights::Weight,
};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};
use sp_std::cell::RefCell;

use selendra_indracore::primitives::Id as IndraId;
use selendra_runtime_indracores::{configuration, origin, shared};
use xcm::latest::{opaque, prelude::*};
use xcm_executor::XcmExecutor;

use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
	ChildIndracoreAsNative, ChildIndracoreConvertsVia, ChildSystemIndracoreAsSuperuser,
	CurrencyAdapter as XcmCurrencyAdapter, FixedRateOfFungible, FixedWeightBounds,
	IsChildSystemIndracore, IsConcrete, LocationInverter, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit,
};

pub type AccountId = AccountId32;
pub type Balance = u128;

thread_local! {
	pub static SENT_XCM: RefCell<Vec<(MultiLocation, opaque::Xcm)>> = RefCell::new(Vec::new());
}
pub fn sent_xcm() -> Vec<(MultiLocation, opaque::Xcm)> {
	SENT_XCM.with(|q| (*q.borrow()).clone())
}
pub struct TestSendXcm;
impl SendXcm for TestSendXcm {
	fn send_xcm(dest: impl Into<MultiLocation>, msg: opaque::Xcm) -> SendResult {
		SENT_XCM.with(|q| q.borrow_mut().push((dest.into(), msg)));
		Ok(())
	}
}

// copied from Selendra constants
pub const UNITS: Balance = 1_000_000_000_000;
pub const CENTS: Balance = UNITS / 30_000;

parameter_types! {
	pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Runtime {
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = ::sp_runtime::traits::BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type BlockWeights = ();
	type BlockLength = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type DbWeight = ();
	type BaseCallFilter = Everything;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub ExistentialDeposit: Balance = 1 * CENTS;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = MaxLocks;
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
}

impl shared::Config for Runtime {}

impl configuration::Config for Runtime {
	type WeightInfo = configuration::TestWeightInfo;
}

// aims to closely emulate the Selendra XcmConfig
parameter_types! {
	pub const SelLocation: MultiLocation = MultiLocation::here();
	pub const SelendraNetwork: NetworkId = NetworkId::Selendra;
	pub Ancestry: MultiLocation = Here.into();
	pub CheckAccount: AccountId = XcmPallet::check_account();
}

pub type SovereignAccountOf =
	(ChildIndracoreConvertsVia<IndraId, AccountId>, AccountId32Aliases<SelendraNetwork, AccountId>);

pub type LocalCurrencyAdapter = XcmCurrencyAdapter<
	Balances,
	IsConcrete<SelLocation>,
	SovereignAccountOf,
	AccountId,
	CheckAccount,
>;

pub type LocalAssetTransactor = (LocalCurrencyAdapter,);

type LocalOriginConverter = (
	SovereignSignedViaLocation<SovereignAccountOf, Origin>,
	ChildIndracoreAsNative<origin::Origin, Origin>,
	SignedAccountId32AsNative<SelendraNetwork, Origin>,
	ChildSystemIndracoreAsSuperuser<IndraId, Origin>,
);

parameter_types! {
	pub const BaseXcmWeight: Weight = 1_000_000_000;
	pub SelPerSecond: (AssetId, u128) = (SelLocation::get().into(), 1);
}

pub type Barrier = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	// Unused/Untested
	AllowUnpaidExecutionFrom<IsChildSystemIndracore<IndraId>>,
);

parameter_types! {
	pub const SelendraForSelendraEvm: (MultiAssetFilter, MultiLocation) =
		(MultiAssetFilter::Wild(WildMultiAsset::AllOf { id: Concrete(MultiLocation::here()), fun: WildFungible }), X1(Indracore(1000)).into());
	pub const MaxInstructions: u32 = 100;
}
pub type TrustedTeleporters = (xcm_builder::Case<SelendraForSelendraEvm>,);

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
	type XcmSender = TestSendXcm;
	type AssetTransactor = LocalAssetTransactor;
	type OriginConverter = LocalOriginConverter;
	type IsReserve = ();
	type IsTeleporter = TrustedTeleporters;
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barrier;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type Trader = FixedRateOfFungible<SelPerSecond, ()>;
	type ResponseHandler = XcmPallet;
	type AssetTrap = XcmPallet;
	type AssetClaims = XcmPallet;
	type SubscriptionService = XcmPallet;
}

pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, SelendraNetwork>;

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type LocationInverter = LocationInverter<Ancestry>;
	type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = TestSendXcm;
	// Anyone can execute XCM messages locally...
	type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Everything;
	type XcmReserveTransferFilter = Everything;
	type Weigher = FixedWeightBounds<BaseXcmWeight, Call, MaxInstructions>;
	type Call = Call;
	type Origin = Origin;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl origin::Config for Runtime {}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		IndrasOrigin: origin::{Pallet, Origin},
		XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin},
	}
);

pub fn selendra_like_with_balances(
	balances: Vec<(AccountId, Balance)>,
) -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
