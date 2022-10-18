//! Autogenerated weights for `runtime_indracores::indras_inherent`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-07, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("selendra-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/selendra
// benchmark
// pallet
// --chain=selendra-dev
// --steps=50
// --repeat=20
// --pallet=runtime_indracores::indras_inherent
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/selendra/src/weights/runtime_indracores_indras_inherent.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_indracores::indras_inherent`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_indracores::indras_inherent::WeightInfo for WeightInfo<T> {
	// Storage: IndraInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: IndrasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: IndraSessionInfo Sessions (r:1 w:0)
	// Storage: IndrasDisputes Disputes (r:1 w:1)
	// Storage: IndrasDisputes Included (r:1 w:1)
	// Storage: IndrasDisputes SpamSlots (r:1 w:1)
	// Storage: IndraScheduler AvailabilityCores (r:1 w:1)
	// Storage: IndrasDisputes Frozen (r:1 w:0)
	// Storage: IndraInclusion PendingAvailability (r:2 w:1)
	// Storage: IndrasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Indras Indracores (r:1 w:0)
	// Storage: IndraInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: IndraSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Indras FutureCodeUpgrades (r:1 w:0)
	// Storage: IndraInherent OnChainVotes (r:1 w:1)
	// Storage: IndraScheduler SessionStartBlock (r:1 w:0)
	// Storage: IndraScheduler indrabaseQueue (r:1 w:1)
	// Storage: IndraScheduler Scheduled (r:1 w:1)
	// Storage: IndraScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Indras Heads (r:0 w:1)
	// Storage: Indras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[10, 200]`.
	fn enter_variable_disputes(v: u32, ) -> Weight {
		(440_736_000 as Weight)
			// Standard Error: 23_000
			.saturating_add((48_510_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(18 as Weight))
	}
	// Storage: IndraInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: IndrasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: IndraScheduler AvailabilityCores (r:1 w:1)
	// Storage: IndrasDisputes Frozen (r:1 w:0)
	// Storage: IndrasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Indras Indracores (r:1 w:0)
	// Storage: IndraInclusion PendingAvailability (r:2 w:1)
	// Storage: IndraInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: IndraSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Indras FutureCodeUpgrades (r:1 w:0)
	// Storage: IndraInherent OnChainVotes (r:1 w:1)
	// Storage: IndrasDisputes Disputes (r:1 w:0)
	// Storage: IndraScheduler SessionStartBlock (r:1 w:0)
	// Storage: IndraScheduler indrabaseQueue (r:1 w:1)
	// Storage: IndraScheduler Scheduled (r:1 w:1)
	// Storage: IndraScheduler ValidatorGroups (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: IndraInclusion AvailabilityBitfields (r:0 w:1)
	// Storage: IndrasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Indras Heads (r:0 w:1)
	// Storage: Indras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_bitfields() -> Weight {
		(414_544_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(17 as Weight))
	}
	// Storage: IndraInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: IndrasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: IndraScheduler AvailabilityCores (r:1 w:1)
	// Storage: IndrasDisputes Frozen (r:1 w:0)
	// Storage: IndrasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Indras Indracores (r:1 w:0)
	// Storage: IndraInclusion PendingAvailability (r:2 w:1)
	// Storage: IndraInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: IndraSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Indras FutureCodeUpgrades (r:1 w:0)
	// Storage: IndraInherent OnChainVotes (r:1 w:1)
	// Storage: IndrasDisputes Disputes (r:2 w:0)
	// Storage: IndraScheduler SessionStartBlock (r:1 w:0)
	// Storage: IndraScheduler indrabaseQueue (r:1 w:1)
	// Storage: IndraScheduler Scheduled (r:1 w:1)
	// Storage: IndraScheduler ValidatorGroups (r:1 w:0)
	// Storage: Indras CurrentCodeHash (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: IndrasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Indras Heads (r:0 w:1)
	// Storage: Indras UpgradeGoAheadSignal (r:0 w:1)
	/// The range of component `v` is `[101, 200]`.
	fn enter_backed_candidates_variable(v: u32, ) -> Weight {
		(1_050_332_000 as Weight)
			// Standard Error: 47_000
			.saturating_add((48_179_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(29 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: IndraInherent Included (r:1 w:1)
	// Storage: System ParentHash (r:1 w:0)
	// Storage: IndrasShared CurrentSessionIndex (r:1 w:0)
	// Storage: Configuration ActiveConfig (r:1 w:0)
	// Storage: Babe AuthorVrfRandomness (r:1 w:0)
	// Storage: IndraScheduler AvailabilityCores (r:1 w:1)
	// Storage: IndrasDisputes Frozen (r:1 w:0)
	// Storage: IndrasShared ActiveValidatorKeys (r:1 w:0)
	// Storage: Indras Indracores (r:1 w:0)
	// Storage: IndraInclusion PendingAvailability (r:2 w:1)
	// Storage: IndraInclusion PendingAvailabilityCommitments (r:1 w:1)
	// Storage: IndraSessionInfo AccountKeys (r:1 w:0)
	// Storage: Staking ActiveEra (r:1 w:0)
	// Storage: Staking ErasRewardPoints (r:1 w:1)
	// Storage: Dmp DownwardMessageQueues (r:1 w:1)
	// Storage: Hrmp HrmpChannelDigests (r:1 w:1)
	// Storage: Indras FutureCodeUpgrades (r:1 w:0)
	// Storage: IndraInherent OnChainVotes (r:1 w:1)
	// Storage: IndrasDisputes Disputes (r:2 w:0)
	// Storage: IndraScheduler SessionStartBlock (r:1 w:0)
	// Storage: IndraScheduler indrabaseQueue (r:1 w:1)
	// Storage: IndraScheduler Scheduled (r:1 w:1)
	// Storage: IndraScheduler ValidatorGroups (r:1 w:0)
	// Storage: Indras CurrentCodeHash (r:1 w:0)
	// Storage: Indras FutureCodeHash (r:1 w:0)
	// Storage: Indras UpgradeRestrictionSignal (r:1 w:0)
	// Storage: Ump RelayDispatchQueueSize (r:1 w:0)
	// Storage: Ump NeedsDispatch (r:1 w:1)
	// Storage: Ump NextDispatchRoundStartWith (r:1 w:1)
	// Storage: IndrasDisputes Included (r:0 w:1)
	// Storage: Hrmp HrmpWatermarks (r:0 w:1)
	// Storage: Indras Heads (r:0 w:1)
	// Storage: Indras UpgradeGoAheadSignal (r:0 w:1)
	fn enter_backed_candidate_code_upgrade() -> Weight {
		(43_823_539_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(31 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
}
