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

use codec::{Decode, Encode};
use frame_support::{
	dispatch::DispatchResult,
	ord_parameter_types, parameter_types,
	traits::{
		ConstU128, ConstU32, ConstU64, FindAuthor, Imbalance, OnUnbalanced, ReservableCurrency,
	},
	transactional,
	weights::{Weight, WeightToFee as WeightToFeeT},
	ConsensusEngineId, RuntimeDebug,
};
use pallet_evm::{EvmChainId, EvmTask, TransferAll};
use pallet_evm_accounts::EvmAddressMapping;
use pallets_support::{mock::MockAddressMapping, scheduler::DispatchableTask};
use scale_info::TypeInfo;
use selendra_primitives::{
	define_combined_task, evm::task::TaskResult, AccountId, Balance, ReserveIdentifier,
};
use sp_core::{H160, H256};
pub use sp_runtime::AccountId32;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, BlockNumberProvider, Convert, IdentityLookup, Zero},
	SaturatedConversion,
};
use sp_std::cell::RefCell;
use std::str::FromStr;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

impl frame_system::Config for TestRuntime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = selendra_primitives::Index;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<10>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for TestRuntime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = ReserveIdentifier;
}

impl pallet_timestamp::Config for TestRuntime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1000>;
	type WeightInfo = ();
}

pub struct MockBlockNumberProvider;

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u32;

	fn current_block_number() -> Self::BlockNumber {
		Zero::zero()
	}
}

parameter_types! {
	pub MinimumWeightRemainInBlock: Weight = Weight::from_parts(0, 0);
}

define_combined_task! {
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum ScheduledTasks {
		EvmTask(EvmTask<TestRuntime>),
	}
}

impl pallet_idle_scheduler::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type BlockNumberProvider = MockBlockNumberProvider;
	type DisableBlockThreshold = ConstU32<6>;
}

pub struct TransferAllEvm;
impl TransferAll<AccountId> for TransferAllEvm {
	#[transactional]
	fn transfer_all(source: &AccountId, dest: &AccountId) -> DispatchResult {
		// unreserve all reserved currency
		<Balances as ReservableCurrency<_>>::unreserve(source, Balances::reserved_balance(source));

		// transfer all free to dest
		match Balances::transfer(
			Some(source.clone()).into(),
			dest.clone().into(),
			Balances::free_balance(source),
		) {
			Ok(_) => Ok(()),
			Err(e) => Err(e.error),
		}
	}
}

thread_local! {
	pub static TIP_UNBALANCED_AMOUNT: RefCell<u128> = RefCell::new(0);
	pub static FEE_UNBALANCED_AMOUNT: RefCell<u128> = RefCell::new(0);
}

pub struct DealWithFees;
impl OnUnbalanced<pallet_balances::NegativeImbalance<TestRuntime>> for DealWithFees {
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<TestRuntime>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			FEE_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += fees.peek());
			if let Some(tips) = fees_then_tips.next() {
				TIP_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += tips.peek());
			}
		}
	}
}

parameter_types! {
	pub static WeightToFee: u128 = 1;
	pub static TransactionByteFee: u128 = 1;
	pub static OperationalFeeMultiplier: u8 = 5;
}

impl WeightToFeeT for WeightToFee {
	type Balance = u128;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time())
			.saturating_mul(WEIGHT_TO_FEE.with(|v| *v.borrow()))
	}
}

impl WeightToFeeT for TransactionByteFee {
	type Balance = u128;

	fn weight_to_fee(weight: &Weight) -> Self::Balance {
		Self::Balance::saturated_from(weight.ref_time())
			.saturating_mul(TRANSACTION_BYTE_FEE.with(|v| *v.borrow()))
	}
}

pub struct GasToWeight;

impl Convert<u64, Weight> for GasToWeight {
	fn convert(a: u64) -> Weight {
		Weight::from_parts(a, 0)
	}
}

pub struct AuthorGiven;
impl FindAuthor<AccountId32> for AuthorGiven {
	fn find_author<'a, I>(_digests: I) -> Option<AccountId32>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		Some(AccountId32::from_str("1234500000000000000000000000000000000000").unwrap())
	}
}

parameter_types! {
	pub NetworkContractSource: H160 = H160::from_low_u64_be(1);
}

ord_parameter_types! {
	pub const CouncilAccount: AccountId32 = AccountId32::from([1u8; 32]);
	pub const TreasuryAccount: AccountId32 = AccountId32::from([2u8; 32]);
	pub const NetworkContractAccount: AccountId32 = AccountId32::from([0u8; 32]);
	pub const StorageDepositPerByte: Balance = 1000000;
}

impl pallet_evm_accounts::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type AddressMapping = EvmAddressMapping<TestRuntime>;
	type TransferAll = TransferAllEvm;
	type ChainId = EvmChainId<TestRuntime>;
	type WeightInfo = ();
}

impl pallet_evm::Config for TestRuntime {
	type AddressMapping = MockAddressMapping;
	type Currency = Balances;
	type TransferAll = TransferAllEvm;
	type NewContractExtraBytes = ConstU32<100>;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = ConstU128<20_000_000>;

	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type GasToWeight = GasToWeight;
	type OnTransactionPayment = DealWithFees;

	type NetworkContractOrigin = frame_system::EnsureSignedBy<NetworkContractAccount, AccountId32>;
	type NetworkContractSource = NetworkContractSource;
	type DeveloperDeposit = ConstU128<1000>;
	type PublicationFee = ConstU128<200>;
	type TreasuryAccount = TreasuryAccount;
	type FreePublicationOrigin = frame_system::EnsureSignedBy<CouncilAccount, AccountId32>;

	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type FindAuthor = AuthorGiven;
	type Task = ScheduledTasks;
	type IdleScheduler = IdleScheduler;
	type WeightInfo = ();
	type WeightToFee = WeightToFee;
}

frame_support::construct_runtime!(
	pub enum TestRuntime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		TimesStamp: pallet_timestamp,
		IdleScheduler: pallet_idle_scheduler,
		EVM: pallet_evm,
		EvmAccounts: pallet_evm_accounts,
	}
);

pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_io::TestExternalities::new_empty()
}
