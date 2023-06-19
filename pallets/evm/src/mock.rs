#![cfg(test)]

use super::*;

use sp_std::cell::RefCell;

use frame_support::{
	construct_runtime, ord_parameter_types, parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, Everything, FindAuthor, ReservableCurrency},
	weights::WeightToFee as WeightToFeeT,
	ConsensusEngineId,
};
use frame_system::EnsureSignedBy;
use pallet_transaction_payment::CurrencyAdapter;
use pallets_support::mock::MockAddressMapping;
use selendra_primitives::{define_combined_task, evm::ReserveIdentifier, AccountId};
use sp_core::{H160, H256};
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, BlockNumberProvider, IdentityLookup},
	AccountId32,
};
use std::{collections::BTreeMap, str::FromStr};

type Balance = u128;

pub mod evm_mod {
	pub use super::super::*;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = crate::CallKillAccount<Runtime>;
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Runtime {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = ReserveIdentifier;
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1000>;
	type WeightInfo = ();
}

define_combined_task! {
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	pub enum ScheduledTasks {
		EvmTask(EvmTask<Runtime>),
	}
}

pub struct MockBlockNumberProvider;

impl BlockNumberProvider for MockBlockNumberProvider {
	type BlockNumber = u32;

	fn current_block_number() -> Self::BlockNumber {
		Zero::zero()
	}
}

parameter_types! {
	pub MinimumWeightRemainInBlock: Weight = Weight::zero();
}

impl pallet_idle_scheduler::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Task = ScheduledTasks;
	type MinimumWeightRemainInBlock = MinimumWeightRemainInBlock;
	type BlockNumberProvider = MockBlockNumberProvider;
	type DisableBlockThreshold = ConstU32<6>;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
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
		Some(<Runtime as Config>::AddressMapping::get_account_id(
			&H160::from_str("1234500000000000000000000000000000000000").unwrap(),
		))
	}
}

parameter_types! {
	pub NetworkContractSource: H160 = alice();
}

ord_parameter_types! {
	pub const CouncilAccount: AccountId32 = AccountId32::from([1u8; 32]);
	pub const TreasuryAccount: AccountId32 = AccountId32::from([2u8; 32]);
	pub const NetworkContractAccount: AccountId32 = AccountId32::from([0u8; 32]);
	pub const StorageDepositPerByte: Balance = 10;
}

pub const NEW_CONTRACT_EXTRA_BYTES: u32 = 100;
pub const DEVELOPER_DEPOSIT: u128 = 1000;
pub const PUBLICATION_FEE: u128 = 200;

thread_local! {
	pub static TIP_UNBALANCED_AMOUNT: RefCell<u128> = RefCell::new(0);
	pub static FEE_UNBALANCED_AMOUNT: RefCell<u128> = RefCell::new(0);
}

pub struct DealWithFees;
impl OnUnbalanced<pallet_balances::NegativeImbalance<Runtime>> for DealWithFees {
	fn on_unbalanceds<B>(
		mut fees_then_tips: impl Iterator<Item = pallet_balances::NegativeImbalance<Runtime>>,
	) {
		if let Some(fees) = fees_then_tips.next() {
			FEE_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += fees.peek());
			if let Some(tips) = fees_then_tips.next() {
				TIP_UNBALANCED_AMOUNT.with(|a| *a.borrow_mut() += tips.peek());
			}
		}
	}
}

impl Config for Runtime {
	type AddressMapping = MockAddressMapping;
	type Currency = Balances;
	type TransferAll = TransferAllEvm;
	type NewContractExtraBytes = ConstU32<NEW_CONTRACT_EXTRA_BYTES>;
	type StorageDepositPerByte = StorageDepositPerByte;
	type TxFeePerGas = ConstU128<20_000_000>;

	type RuntimeEvent = RuntimeEvent;
	type PrecompilesType = ();
	type PrecompilesValue = ();
	type GasToWeight = GasToWeight;

	type OnTransactionPayment = DealWithFees;

	type NetworkContractOrigin = EnsureSignedBy<NetworkContractAccount, AccountId32>;
	type NetworkContractSource = NetworkContractSource;
	type DeveloperDeposit = ConstU128<DEVELOPER_DEPOSIT>;
	type PublicationFee = ConstU128<PUBLICATION_FEE>;
	type TreasuryAccount = TreasuryAccount;
	type FreePublicationOrigin = EnsureSignedBy<CouncilAccount, AccountId32>;

	type Runner = crate::runner::stack::Runner<Self>;
	type FindAuthor = AuthorGiven;
	type Task = ScheduledTasks;
	type IdleScheduler = IdleScheduler;
	type WeightToFee = WeightToFee;
	type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

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

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
	type OperationalFeeMultiplier = OperationalFeeMultiplier;
	type WeightToFee = WeightToFee;
	type LengthToFee = TransactionByteFee;
	type FeeMultiplierUpdate = ();
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Timestamp: pallet_timestamp,
		EVM: evm_mod,
		Balances: pallet_balances,
		IdleScheduler: pallet_idle_scheduler,
		Utility: pallet_utility,
		TransactionPayment: pallet_transaction_payment,
	}
);

pub const INITIAL_BALANCE: Balance = 1_000_000_000_000_000_000_000;

pub fn contract_a() -> H160 {
	H160::from_str("2000000000000000000000000000000000000001").unwrap()
}

pub fn contract_b() -> H160 {
	H160::from_str("2000000000000000000000000000000000000002").unwrap()
}

pub fn alice() -> H160 {
	H160::from_str("1000000000000000000000000000000000000001").unwrap()
}

pub fn bob() -> H160 {
	H160::from_str("1000000000000000000000000000000000000002").unwrap()
}

pub fn charlie() -> H160 {
	H160::from_str("1000000000000000000000000000000000000003").unwrap()
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let mut accounts = BTreeMap::new();

	accounts.insert(contract_a(), GenesisAccount { nonce: 1, ..Default::default() });
	accounts.insert(contract_b(), GenesisAccount { nonce: 1, ..Default::default() });

	accounts.insert(
		alice(),
		GenesisAccount { nonce: 1, balance: INITIAL_BALANCE, ..Default::default() },
	);
	accounts
		.insert(bob(), GenesisAccount { nonce: 1, balance: INITIAL_BALANCE, ..Default::default() });

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(TreasuryAccount::get(), INITIAL_BALANCE)],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	evm_mod::GenesisConfig::<Runtime> { chain_id: 1, accounts }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn balance(address: H160) -> Balance {
	let account_id = <Runtime as Config>::AddressMapping::get_account_id(&address);
	Balances::free_balance(account_id)
}

pub fn eth_balance(address: H160) -> U256 {
	EVM::account_basic(&address).balance
}

pub fn reserved_balance(address: H160) -> Balance {
	let account_id = <Runtime as Config>::AddressMapping::get_account_id(&address);
	Balances::reserved_balance(account_id)
}

#[cfg(not(feature = "with-ethereum-compatibility"))]
pub fn publish_free(contract: H160) {
	let _ = EVM::publish_free(RuntimeOrigin::signed(CouncilAccount::get()), contract);
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
