// // This file is part of Selendra.

// // Copyright (C) 2021-2022 Selendra.
// // SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// // This program is free software: you can redistribute it and/or modify
// // it under the terms of the GNU General Public License as published by
// // the Free Software Foundation, either version 3 of the License, or
// // (at your option) any later version.

// // This program is distributed in the hope that it will be useful,
// // but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// // GNU General Public License for more details.

// // You should have received a copy of the GNU General Public License
// // along with this program. If not, see <https://www.gnu.org/licenses/>.

// //! The Dev runtime. This can be compiled with `#[no_std]`, ready for Wasm.

// #![cfg_attr(not(feature = "std"), no_std)]
// // `construct_runtime!` does a lot of recursion and requires us to increase the limit.
// #![recursion_limit = "512"]

// // Make the WASM binary available.
// #[cfg(feature = "std")]
// include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

// /// The version infromation used to identify this runtime when compiled
// /// natively.
// #[cfg(feature = "std")]
// pub fn native_version() -> NativeVersion {
// 	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
// }

// mod authority;
pub mod constants;
// mod voter_bags;
// mod weights;

// // runtime config
// mod config;

// pub use authority::AuthorityConfigImpl;
// use codec::{DecodeLimit, Encode};

// use sp_api::impl_runtime_apis;
// use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
// use sp_core::{crypto::KeyTypeId, OpaqueMetadata, H160};
// #[cfg(any(feature = "std", test))]
// pub use sp_runtime::BuildStorage;
// use sp_runtime::{
// 	create_runtime_str, generic, impl_opaque_keys,
// 	traits::{
// 		AccountIdConversion, BadOrigin, BlakeTwo256, Block as BlockT, Bounded, Convert, NumberFor,
// 		SaturatedConversion, StaticLookup, Verify,
// 	},
// 	transaction_validity::{TransactionSource, TransactionValidity},
// 	ApplyExtrinsicResult, DispatchResult, FixedPointNumber, Perbill, Percent, Permill,
// };
// use sp_std::prelude::*;
// #[cfg(feature = "std")]
// use sp_version::NativeVersion;
// use sp_version::RuntimeVersion;

// use pallet_grandpa::{
// 	fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
// };
// use pallet_session::historical::{self as pallet_session_historical};
// #[cfg(feature = "std")]
// pub use pallet_staking::StakerStatus;
// pub use pallet_timestamp::Call as TimestampCall;
// use pallet_transaction_payment::{FeeDetails, RuntimeDispatchInfo};

// use frame_support::{
// 	construct_runtime, log, parameter_types,
// 	traits::{Contains, InstanceFilter, KeyOwnerProofSystem, LockIdentifier},
// 	weights::{constants::RocksDbWeight, Weight},
// 	PalletId, RuntimeDebug,
// };
// use frame_system::EnsureRoot;

// use module_asset_registry::AssetIdMaps;
// use module_cdp_engine::CollateralCurrencyIds;
// use module_evm::{runner::RunnerExtended, CallInfo, CreateInfo};
// use module_evm_accounts::EvmAddressMapping;
// use module_support::{AssetIdMapping, DispatchableTask};

// use orml_traits::{
// 	create_median_value_data_provider, parameter_type_with_key, DataFeeder, DataProviderExtended,
// 	GetByKey,
// };

// pub use primitives::{
// 	currency::AssetIds,
// 	evm::{AccessListItem, BlockLimits, EstimateResourcesRequest, EthereumTransactionMessage},
// 	unchecked_extrinsic::SelendraUncheckedExtrinsic,
// 	AccountId, AccountIndex, Amount, AuthoritysOriginId, Balance, BlockNumber, CurrencyId,
// 	DataProviderId, Hash, Moment, Nonce, ReserveIdentifier, Signature, TokenSymbol,
// };
// pub use runtime_common::{
// 	cent, dollar, microcent, millicent, AllPrecompiles, BlockHashCount, ExchangeRate,
// 	ExistentialDepositsTimesOneHundred, GasToWeight, Price, ProxyType, Rate, Ratio,
// 	RuntimeBlockLength, RuntimeBlockWeights, TimeStampedPrice, DAI, DOT, KSM, KUSD, LSEL, RENBTC,
// 	SEL,
// };

// use crate::config::{
// 	consensus::EpochDuration,
// 	dex::TradingPathLimit,
// 	evm::{ConvertEthereumTx, PayerSignatureVerification, StorageDepositPerByte, TxFeePerGas},
// 	funan::{MaxSwapSlippageCompareToOracle, SelendraSwap},
// 	utility::PreimageByteDeposit,
// };
// #[cfg(test)]
// use config::evm::NewContractExtraBytes;
// pub use constants::{accounts::*, currency::*, fee::*, time::*};

// /// This runtime version.
// #[sp_version::runtime_version]
// pub const VERSION: RuntimeVersion = RuntimeVersion {
// 	spec_name: create_runtime_str!("selendra"),
// 	impl_name: create_runtime_str!("selendra"),
// 	authoring_version: 1,
// 	spec_version: 1004,
// 	impl_version: 0,
// 	apis: RUNTIME_API_VERSIONS,
// 	transaction_version: 3,
// 	state_version: 0,
// };

// /// The BABE epoch configuration at genesis.
// pub const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
// 	sp_consensus_babe::BabeEpochConfiguration {
// 		c: PRIMARY_PROBABILITY,
// 		allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
// 	};

// impl_opaque_keys! {
// 	pub struct SessionKeys {
// 		pub babe: Babe,
// 		pub grandpa: Grandpa,
// 		pub im_online: ImOnline,
// 		pub authority_discovery: AuthorityDiscovery,
// 	}
// }

// pub struct BaseCallFilter;
// impl Contains<Call> for BaseCallFilter {
// 	fn contains(call: &Call) -> bool {
// 		let is_core_call = matches!(call, Call::System(_) | Call::Timestamp(_));
// 		if is_core_call {
// 			// always allow core call
// 			return true
// 		}

// 		let is_paused =
// 			module_transaction_pause::PausedTransactionFilter::<Runtime>::contains(call);
// 		if is_paused {
// 			// no paused call
// 			return false
// 		}
// 		true
// 	}
// }

// parameter_types! {
// 	pub const Version: RuntimeVersion = VERSION;
// 	pub const SS58Prefix: u8 = 204;
// }

// impl frame_system::Config for Runtime {
// 	type BaseCallFilter = BaseCallFilter;
// 	type BlockWeights = RuntimeBlockWeights;
// 	type BlockLength = RuntimeBlockLength;
// 	type BlockHashCount = BlockHashCount;
// 	type DbWeight = RocksDbWeight;
// 	type Origin = Origin;
// 	type Call = Call;
// 	type Event = Event;
// 	type Index = Nonce;
// 	type BlockNumber = BlockNumber;
// 	type Hash = Hash;
// 	type Hashing = BlakeTwo256;
// 	type AccountId = AccountId;
// 	type AccountData = pallet_balances::AccountData<Balance>;
// 	type Lookup = (Indices, EvmAccounts);
// 	type Header = generic::Header<BlockNumber, BlakeTwo256>;
// 	type Version = Version;
// 	type PalletInfo = PalletInfo;
// 	type OnSetCode = ();
// 	type OnNewAccount = ();
// 	type OnKilledAccount =
// 		(module_evm::CallKillAccount<Runtime>, module_evm_accounts::CallKillAccount<Runtime>);
// 	type SS58Prefix = SS58Prefix;
// 	type MaxConsumers = frame_support::traits::ConstU32<16>;
// 	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
// }

// parameter_types! {
// 	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
// }

// impl pallet_timestamp::Config for Runtime {
// 	type Moment = Moment;
// 	type OnTimestampSet = Babe;
// 	type MinimumPeriod = MinimumPeriod;
// 	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
// }

// parameter_types! {
// 	pub const MaxReserves: u32 = ReserveIdentifier::Count as u32;
// 	pub const MaxLocks: u32 = 50;
// }

// impl pallet_balances::Config for Runtime {
// 	type Balance = Balance;
// 	type DustRemoval = Treasury;
// 	type Event = Event;
// 	type ExistentialDeposit = NativeTokenExistentialDeposit;
// 	type AccountStore = System;
// 	type MaxLocks = MaxLocks;
// 	type MaxReserves = MaxReserves;
// 	type ReserveIdentifier = ReserveIdentifier;
// 	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
// }

// impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
// where
// 	Call: From<LocalCall>,
// {
// 	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
// 		call: Call,
// 		public: <Signature as sp_runtime::traits::Verify>::Signer,
// 		account: AccountId,
// 		nonce: Nonce,
// 	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
// 		// take the biggest period possible.
// 		let period =
// 			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
// 		let current_block = System::block_number()
// 			.saturated_into::<u64>()
// 			// The `System::block_number` is initialized with `n+1`,
// 			// so the actual block number is `n`.
// 			.saturating_sub(1);
// 		let tip = 0;
// 		let extra: SignedExtra = (
// 			frame_system::CheckNonZeroSender::<Runtime>::new(),
// 			frame_system::CheckSpecVersion::<Runtime>::new(),
// 			frame_system::CheckTxVersion::<Runtime>::new(),
// 			frame_system::CheckGenesis::<Runtime>::new(),
// 			frame_system::CheckMortality::<Runtime>::from(generic::Era::mortal(
// 				period,
// 				current_block,
// 			)),
// 			runtime_common::CheckNonce::<Runtime>::from(nonce),
// 			frame_system::CheckWeight::<Runtime>::new(),
// 			module_evm::SetEvmOrigin::<Runtime>::new(),
// 			module_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
// 		);
// 		let raw_payload = SignedPayload::new(call, extra)
// 			.map_err(|e| {
// 				log::warn!("Unable to create signed payload: {:?}", e);
// 			})
// 			.ok()?;
// 		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
// 		let address = Indices::unlookup(account);
// 		let (call, extra, _) = raw_payload.deconstruct();
// 		Some((call, (address, signature, extra)))
// 	}
// }

// impl frame_system::offchain::SigningTypes for Runtime {
// 	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
// 	type Signature = Signature;
// }

// impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
// where
// 	Call: From<C>,
// {
// 	type OverarchingCall = Call;
// 	type Extrinsic = UncheckedExtrinsic;
// }

// construct_runtime!(
// 	pub enum Runtime where
// 		Block = Block,
// 		NodeBlock = primitives::Block,
// 		UncheckedExtrinsic = UncheckedExtrinsic
// 	{
// 		// Core
// 		System: frame_system = 0,
// 		Timestamp: pallet_timestamp = 1,
// 		Scheduler: pallet_scheduler = 2,
// 		Preimage: pallet_preimage = 4,

// 		// Tokens & Related
// 		Balances: pallet_balances = 10,
// 		Tokens: orml_tokens exclude_parts { Call } = 11,
// 		Currencies: module_currencies = 12,
// 		TransactionPayment: module_transaction_payment = 14,

// 		// Treasury
// 		Treasury: pallet_treasury = 20,
// 		Bounties: pallet_bounties = 21,
// 		Tips: pallet_tips = 22,

// 		// Utility
// 		Utility: pallet_utility = 30,
// 		Multisig: pallet_multisig = 31,
// 		Recovery: pallet_recovery = 32,
// 		Proxy: pallet_proxy = 33,
// 		Indices: pallet_indices = 36,
// 		Identity: pallet_identity = 37,
// 		Vesting: pallet_vesting = 38,

// 		// Consensus
// 		// Authorship must be before session in order to note author in the correct session and era
// 		// for im-online and staking.
// 		Authorship: pallet_authorship = 40,
// 		Babe: pallet_babe = 41,
// 		Staking: pallet_staking = 42,
// 		Offences: pallet_offences = 43,
// 		Historical: pallet_session_historical::{Pallet} = 44,
// 		Session: pallet_session = 45,
// 		Grandpa: pallet_grandpa = 46,
// 		ImOnline: pallet_im_online = 47,
// 		AuthorityDiscovery: pallet_authority_discovery = 48,
// 		// placed behind indices to maintain it.
// 		ElectionProviderMultiPhase: pallet_election_provider_multi_phase = 49,
// 		VoterList: pallet_bags_list = 50,
// 		NominationPools: pallet_nomination_pools = 51,

// 		// Governance
// 		Authority: orml_authority = 60,
// 		Council: pallet_collective::<Instance1> = 61,
// 		CouncilMembership: pallet_membership::<Instance1> = 62,
// 		TechnicalCommittee: pallet_collective::<Instance4> = 65,
// 		TechnicalMembership: pallet_membership::<Instance4> = 66,
// 		PhragmenElection: pallet_elections_phragmen = 67,
// 		Democracy: pallet_democracy = 68,

// 		// temporary
// 		Sudo: pallet_sudo = 200,
// 	}
// );

// /// Block header type as expected by this runtime.
// pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
// /// Block type as expected by this runtime.
// pub type Block = generic::Block<Header, UncheckedExtrinsic>;
// /// A Block signed with a Justification
// pub type SignedBlock = generic::SignedBlock<Block>;
// /// BlockId type as expected by this runtime.
// pub type BlockId = generic::BlockId<Block>;
// /// The SignedExtension to the basic transaction logic.
// pub type SignedExtra = (
// 	frame_system::CheckNonZeroSender<Runtime>,
// 	frame_system::CheckSpecVersion<Runtime>,
// 	frame_system::CheckTxVersion<Runtime>,
// 	frame_system::CheckGenesis<Runtime>,
// 	frame_system::CheckEra<Runtime>,
// 	runtime_common::CheckNonce<Runtime>,
// 	frame_system::CheckWeight<Runtime>,
// 	module_evm::SetEvmOrigin<Runtime>,
// 	module_transaction_payment::ChargeTransactionPayment<Runtime>,
// );
// /// Unchecked extrinsic type as expected by this runtime.
// pub type UncheckedExtrinsic = SelendraUncheckedExtrinsic<
// 	Call,
// 	SignedExtra,
// 	ConvertEthereumTx,
// 	StorageDepositPerByte,
// 	TxFeePerGas,
// 	PayerSignatureVerification,
// >;
// /// The payload being signed in transactions.
// pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
// /// Extrinsic type that has already been checked.
// pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
// /// Executive: handles dispatch to the various modules.
// pub type Executive = frame_executive::Executive<
// 	Runtime,
// 	Block,
// 	frame_system::ChainContext<Runtime>,
// 	Runtime,
// 	AllPalletsWithSystem,
// 	(),
// >;

// #[cfg(feature = "runtime-benchmarks")]
// #[macro_use]
// extern crate frame_benchmarking;

// #[cfg(feature = "runtime-benchmarks")]
// mod benches {
// 	define_benchmarks!(
// 		[frame_benchmarking, BaselineBench::<Runtime>]
// 		[pallet_babe, Babe]
// 		[pallet_bags_list, VoterList]
// 		[pallet_balances, Balances]
// 		[pallet_bounties, Bounties]
// 		[pallet_collective, Council]
// 		[pallet_democracy, Democracy]
// 		[pallet_election_provider_multi_phase, ElectionProviderMultiPhase]
// 		[pallet_election_provider_support_benchmarking, EPSBench::<Runtime>]
// 		[pallet_elections_phragmen, PhragmenElection]
// 		[pallet_grandpa, Grandpa]
// 		[pallet_identity, Identity]
// 		[pallet_im_online, ImOnline]
// 		[pallet_multisig, Multisig]
// 		[pallet_nomination_pools, NominationPoolsBench::<Runtime>]
// 		[pallet_offences, OffencesBench::<Runtime>]
// 		[pallet_preimage, Preimage]
// 		[pallet_proxy, Proxy]
// 		[pallet_scheduler, Scheduler]
// 		[pallet_session, SessionBench::<Runtime>]
// 		[pallet_staking, Staking]
// 		[frame_system, SystemBench::<Runtime>]
// 		[pallet_timestamp, Timestamp]
// 		[pallet_tips, Tips]
// 		[pallet_treasury, Treasury]
// 		[pallet_utility, Utility]
// 		[pallet_vesting, Vesting]
// 	);
// }

// impl_runtime_apis! {
// 	impl sp_api::Core<Block> for Runtime {
// 		fn version() -> RuntimeVersion {
// 			VERSION
// 		}

// 		fn execute_block(block: Block) {
// 			Executive::execute_block(block)
// 		}

// 		fn initialize_block(header: &<Block as BlockT>::Header) {
// 			Executive::initialize_block(header)
// 		}
// 	}

// 	impl sp_api::Metadata<Block> for Runtime {
// 		fn metadata() -> OpaqueMetadata {
// 			OpaqueMetadata::new(Runtime::metadata().into())
// 		}
// 	}

// 	impl sp_block_builder::BlockBuilder<Block> for Runtime {
// 		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
// 			Executive::apply_extrinsic(extrinsic)
// 		}

// 		fn finalize_block() -> <Block as BlockT>::Header {
// 			Executive::finalize_block()
// 		}

// 		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
// 			data.create_extrinsics()
// 		}

// 		fn check_inherents(
// 			block: Block,
// 			data: sp_inherents::InherentData,
// 		) -> sp_inherents::CheckInherentsResult {
// 			data.check_extrinsics(&block)
// 		}
// 	}

// 	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
// 		fn validate_transaction(
// 			source: TransactionSource,
// 			tx: <Block as BlockT>::Extrinsic,
// 			block_hash: <Block as BlockT>::Hash,
// 		) -> TransactionValidity {
// 			Executive::validate_transaction(source, tx, block_hash)
// 		}
// 	}

// 	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
// 		fn offchain_worker(header: &<Block as BlockT>::Header) {
// 			Executive::offchain_worker(header)
// 		}
// 	}

// 	impl fg_primitives::GrandpaApi<Block> for Runtime {
// 		fn grandpa_authorities() -> GrandpaAuthorityList {
// 			Grandpa::grandpa_authorities()
// 		}

// 		fn current_set_id() -> fg_primitives::SetId {
// 			Grandpa::current_set_id()
// 		}

// 		fn submit_report_equivocation_unsigned_extrinsic(
// 			equivocation_proof: fg_primitives::EquivocationProof<
// 				<Block as BlockT>::Hash,
// 				NumberFor<Block>,
// 			>,
// 			key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
// 		) -> Option<()> {
// 			let key_owner_proof = key_owner_proof.decode()?;

// 			Grandpa::submit_unsigned_equivocation_report(
// 				equivocation_proof,
// 				key_owner_proof,
// 			)
// 		}

// 		fn generate_key_ownership_proof(
// 			_set_id: fg_primitives::SetId,
// 			authority_id: GrandpaId,
// 		) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
// 			use codec::Encode;

// 			Historical::prove((fg_primitives::KEY_TYPE, authority_id))
// 				.map(|p| p.encode())
// 				.map(fg_primitives::OpaqueKeyOwnershipProof::new)
// 		}
// 	}

// 	impl sp_consensus_babe::BabeApi<Block> for Runtime {
// 		fn configuration() -> sp_consensus_babe::BabeGenesisConfiguration {
// 			// The choice of `c` parameter (where `1 - c` represents the
// 			// probability of a slot being empty), is done in accordance to the
// 			// slot duration and expected target block time, for safely
// 			// resisting network delays of maximum two seconds.
// 			// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
// 			sp_consensus_babe::BabeGenesisConfiguration {
// 				slot_duration: Babe::slot_duration(),
// 				epoch_length: EpochDuration::get(),
// 				c: BABE_GENESIS_EPOCH_CONFIG.c,
// 				genesis_authorities: Babe::authorities().to_vec(),
// 				randomness: Babe::randomness(),
// 				allowed_slots: BABE_GENESIS_EPOCH_CONFIG.allowed_slots,
// 			}
// 		}

// 		fn current_epoch_start() -> sp_consensus_babe::Slot {
// 			Babe::current_epoch_start()
// 		}

// 		fn current_epoch() -> sp_consensus_babe::Epoch {
// 			Babe::current_epoch()
// 		}

// 		fn next_epoch() -> sp_consensus_babe::Epoch {
// 			Babe::next_epoch()
// 		}

// 		fn generate_key_ownership_proof(
// 			_slot: sp_consensus_babe::Slot,
// 			authority_id: sp_consensus_babe::AuthorityId,
// 		) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
// 			use codec::Encode;

// 			Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
// 				.map(|p| p.encode())
// 				.map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
// 		}

// 		fn submit_report_equivocation_unsigned_extrinsic(
// 			equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
// 			key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
// 		) -> Option<()> {
// 			let key_owner_proof = key_owner_proof.decode()?;

// 			Babe::submit_unsigned_equivocation_report(
// 				equivocation_proof,
// 				key_owner_proof,
// 			)
// 		}
// 	}

// 	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
// 		fn authorities() -> Vec<AuthorityDiscoveryId> {
// 			AuthorityDiscovery::authorities()
// 		}
// 	}

// 	impl sp_session::SessionKeys<Block> for Runtime {
// 		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
// 			SessionKeys::generate(seed)
// 		}

// 		fn decode_session_keys(
// 			encoded: Vec<u8>,
// 		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
// 			SessionKeys::decode_into_raw_public_keys(&encoded)
// 		}
// 	}

// 	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce> for Runtime {
// 		fn account_nonce(account: AccountId) -> Nonce {
// 			System::account_nonce(account)
// 		}
// 	}

// 	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
// 		Block,
// 		Balance,
// 	> for Runtime {
// 		fn query_info(uxt: <Block as BlockT>::Extrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
// 			TransactionPayment::query_info(uxt, len)
// 		}

// 		fn query_fee_details(uxt: <Block as BlockT>::Extrinsic, len: u32) -> FeeDetails<Balance> {
// 			TransactionPayment::query_fee_details(uxt, len)
// 		}
// 	}

// 	#[cfg(feature = "try-runtime")]
// 	impl frame_try_runtime::TryRuntime<Block> for Runtime {
// 		fn on_runtime_upgrade() -> (Weight, Weight) {
// 			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
// 			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
// 			// right here and right now.
// 			let weight = Executive::try_runtime_upgrade().unwrap();
// 			(weight, RuntimeBlockWeights::get().max_block)
// 		}

// 		fn execute_block_no_check(block: Block) -> Weight {
// 			Executive::execute_block_no_check(block)
// 		}
// 	}

// 	#[cfg(feature = "runtime-benchmarks")]
// 	impl frame_benchmarking::Benchmark<Block> for Runtime {
// 		fn benchmark_metadata(extra: bool) -> (
// 			Vec<frame_benchmarking::BenchmarkList>,
// 			Vec<frame_support::traits::StorageInfo>,
// 		) {
// 			use frame_benchmarking::{baseline, Benchmarking, BenchmarkList};
// 			use frame_support::traits::StorageInfoTrait;

// 			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
// 			// issues. To get around that, we separated the Session benchmarks into its own crate,
// 			// which is why we need these two lines below.
// 			use pallet_session_benchmarking::Pallet as SessionBench;
// 			use pallet_offences_benchmarking::Pallet as OffencesBench;
// 			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
// 			use frame_system_benchmarking::Pallet as SystemBench;
// 			use baseline::Pallet as BaselineBench;
// 			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

// 			let mut list = Vec::<BenchmarkList>::new();
// 			list_benchmarks!(list, extra);

// 			let storage_info = AllPalletsWithSystem::storage_info();

// 			(list, storage_info)
// 		}

// 		fn dispatch_benchmark(
// 			config: frame_benchmarking::BenchmarkConfig
// 		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
// 			use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch,  TrackedStorageKey};

// 			// Trying to add benchmarks directly to the Session Pallet caused cyclic dependency
// 			// issues. To get around that, we separated the Session benchmarks into its own crate,
// 			// which is why we need these two lines below.
// 			use pallet_session_benchmarking::Pallet as SessionBench;
// 			use pallet_offences_benchmarking::Pallet as OffencesBench;
// 			use pallet_election_provider_support_benchmarking::Pallet as EPSBench;
// 			use frame_system_benchmarking::Pallet as SystemBench;
// 			use baseline::Pallet as BaselineBench;
// 			use pallet_nomination_pools_benchmarking::Pallet as NominationPoolsBench;

// 			impl pallet_session_benchmarking::Config for Runtime {}
// 			impl pallet_offences_benchmarking::Config for Runtime {}
// 			impl pallet_election_provider_support_benchmarking::Config for Runtime {}
// 			impl frame_system_benchmarking::Config for Runtime {}
// 			impl baseline::Config for Runtime {}
// 			impl pallet_nomination_pools_benchmarking::Config for Runtime {}

// 			let whitelist: Vec<TrackedStorageKey> = vec![
// 				// Block Number
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
// 				// Total Issuance
// 				hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
// 				// Execution Phase
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
// 				// Event Count
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
// 				// System Events
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
// 				// System BlockWeight
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef734abf5cb34d6244378cddbf18e849d96").to_vec().into(),
// 				// Treasury Account
// 				hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da95ecffd7b6c0f78751baa9d281e0bfa3a6d6f646c70792f74727372790000000000000000000000000000000000000000").to_vec().into(),
// 			];

// 			let mut batches = Vec::<BenchmarkBatch>::new();
// 			let params = (&config, &whitelist);
// 			add_benchmarks!(params, batches);
// 			Ok(batches)
// 		}
// 	}
// }
