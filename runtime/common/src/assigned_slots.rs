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

//! This pallet allows to assign permanent (long-lived) or temporary
//! (short-lived) indracore slots to indras, leveraging the existing
//! indracore slot lease mechanism. Temporary slots are given turns
//! in a fair (though best-effort) manner.
//! The dispatchables must be called from the configured origin
//! (typically `Sudo` or a governance origin).
//! This pallet should not be used on a production relay chain,
//! only on a test relay chain (e.g. Rococo).

use crate::{
	slots::{self, Pallet as Slots, WeightInfo},
	traits::{LeaseError, Leaser, Registrar},
	MAXIMUM_BLOCK_WEIGHT,
};
use frame_support::{pallet_prelude::*, traits::Currency};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use primitives::v2::Id as IndraId;
use runtime_indracores::{configuration, paras as indras};
use scale_info::TypeInfo;
use sp_runtime::traits::{One, Saturating, Zero};
use sp_std::prelude::*;

/// Lease period an assigned slot should start from (current, or next one).
#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum SlotLeasePeriodStart {
	Current,
	Next,
}

/// Information about a temporary indracore slot.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub struct IndracoreTemporarySlot<AccountId, LeasePeriod> {
	/// Manager account of the indra.
	pub manager: AccountId,
	/// Lease period the indracore slot should ideally start from,
	/// As slot are allocated in a best-effort manner, this could be later,
	/// but not earlier than the specified period.
	pub period_begin: LeasePeriod,
	/// Number of lease period the slot lease will last.
	/// This is set to the value configured in `TemporarySlotLeasePeriodLength`.
	pub period_count: LeasePeriod,
	/// Last lease period this slot had a turn in (incl. current).
	/// This is set to the beginning period of a slot.
	pub last_lease: Option<LeasePeriod>,
	/// Number of leases this temporary slot had (incl. current).
	pub lease_count: u32,
}

type BalanceOf<T> = <<<T as Config>::Leaser as Leaser<<T as frame_system::Config>::BlockNumber>>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;
type LeasePeriodOf<T> =
	<<T as Config>::Leaser as Leaser<<T as frame_system::Config>::BlockNumber>>::LeasePeriod;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: configuration::Config + indras::Config + slots::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Origin for assigning slots.
		type AssignSlotOrigin: EnsureOrigin<<Self as frame_system::Config>::Origin>;

		/// The type representing the leasing system.
		type Leaser: Leaser<
			Self::BlockNumber,
			AccountId = Self::AccountId,
			LeasePeriod = Self::BlockNumber,
		>;

		/// The number of lease periods a permanent indracore slot lasts.
		#[pallet::constant]
		type PermanentSlotLeasePeriodLength: Get<u32>;

		/// The number of lease periods a temporary indracore slot lasts.
		#[pallet::constant]
		type TemporarySlotLeasePeriodLength: Get<u32>;

		/// The max number of permanent slots that can be assigned.
		#[pallet::constant]
		type MaxPermanentSlots: Get<u32>;

		/// The max number of temporary slots that can be assigned.
		#[pallet::constant]
		type MaxTemporarySlots: Get<u32>;

		/// The max number of temporary slots to be scheduled per lease periods.
		#[pallet::constant]
		type MaxTemporarySlotPerLeasePeriod: Get<u32>;
	}

	/// Assigned permanent slots, with their start lease period, and duration.
	#[pallet::storage]
	#[pallet::getter(fn permanent_slots)]
	pub type PermanentSlots<T: Config> =
		StorageMap<_, Twox64Concat, IndraId, (LeasePeriodOf<T>, LeasePeriodOf<T>), OptionQuery>;

	/// Number of assigned (and active) permanent slots.
	#[pallet::storage]
	#[pallet::getter(fn permanent_slot_count)]
	pub type PermanentSlotCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Assigned temporary slots.
	#[pallet::storage]
	#[pallet::getter(fn temporary_slots)]
	pub type TemporarySlots<T: Config> = StorageMap<
		_,
		Twox64Concat,
		IndraId,
		IndracoreTemporarySlot<T::AccountId, LeasePeriodOf<T>>,
		OptionQuery,
	>;

	/// Number of assigned temporary slots.
	#[pallet::storage]
	#[pallet::getter(fn temporary_slot_count)]
	pub type TemporarySlotCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Number of active temporary slots in current slot lease period.
	#[pallet::storage]
	#[pallet::getter(fn active_temporary_slot_count)]
	pub type ActiveTemporarySlotCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A indra was assigned a permanent indracore slot
		PermanentSlotAssigned(IndraId),
		/// A indra was assigned a temporary indracore slot
		TemporarySlotAssigned(IndraId),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The specified indracore or indrabase is not registered.
		IndraDoesntExist,
		/// Not a indrabase.
		NotIndrabase,
		/// Cannot upgrade indrabase.
		CannotUpgrade,
		/// Cannot downgrade indracore.
		CannotDowngrade,
		/// Permanent or Temporary slot already assigned.
		SlotAlreadyAssigned,
		/// Permanent or Temporary slot has not been assigned.
		SlotNotAssigned,
		/// An ongoing lease already exists.
		OngoingLeaseExists,
		// Maximum number of permanent slots exceeded
		MaxPermanentSlotsExceeded,
		// Maximum number of temporary slots exceeded
		MaxTemporarySlotsExceeded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			if let Some((lease_period, first_block)) = Self::lease_period_index(n) {
				// If we're beginning a new lease period then handle that.
				if first_block {
					return Self::manage_lease_period_start(lease_period)
				}
			}

			// We didn't return early above, so we didn't do anything.
			0
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// TODO: Benchmark this
		/// Assign a permanent indracore slot and immediately create a lease for it.
		#[pallet::weight(((MAXIMUM_BLOCK_WEIGHT / 10) as Weight, DispatchClass::Operational))]
		pub fn assign_perm_indracore_slot(origin: OriginFor<T>, id: IndraId) -> DispatchResult {
			T::AssignSlotOrigin::ensure_origin(origin)?;

			let manager = T::Registrar::manager_of(id).ok_or(Error::<T>::IndraDoesntExist)?;

			ensure!(T::Registrar::is_indrabase(id), Error::<T>::NotIndrabase,);

			ensure!(
				!Self::has_permanent_slot(id) && !Self::has_temporary_slot(id),
				Error::<T>::SlotAlreadyAssigned
			);

			let current_lease_period: T::BlockNumber = Self::current_lease_period_index();
			ensure!(
				!T::Leaser::already_leased(
					id,
					current_lease_period,
					// Check current lease & next one
					current_lease_period.saturating_add(
						T::BlockNumber::from(2u32)
							.saturating_mul(T::PermanentSlotLeasePeriodLength::get().into())
					)
				),
				Error::<T>::OngoingLeaseExists
			);

			ensure!(
				PermanentSlotCount::<T>::get() < T::MaxPermanentSlots::get(),
				Error::<T>::MaxPermanentSlotsExceeded
			);

			// Permanent slot assignment fails if a lease cannot be created
			Self::configure_slot_lease(
				id,
				manager,
				current_lease_period,
				T::PermanentSlotLeasePeriodLength::get().into(),
			)
			.map_err(|_| Error::<T>::CannotUpgrade)?;

			PermanentSlots::<T>::insert(
				id,
				(
					current_lease_period,
					LeasePeriodOf::<T>::from(T::PermanentSlotLeasePeriodLength::get()),
				),
			);
			<PermanentSlotCount<T>>::mutate(|count| count.saturating_inc());

			Self::deposit_event(Event::<T>::PermanentSlotAssigned(id));
			Ok(())
		}

		// TODO: Benchmark this
		/// Assign a temporary indracore slot. The function tries to create a lease for it
		/// immediately if `SlotLeasePeriodStart::Current` is specified, and if the number
		/// of currently active temporary slots is below `MaxTemporarySlotPerLeasePeriod`.
		#[pallet::weight(((MAXIMUM_BLOCK_WEIGHT / 10) as Weight, DispatchClass::Operational))]
		pub fn assign_temp_indracore_slot(
			origin: OriginFor<T>,
			id: IndraId,
			lease_period_start: SlotLeasePeriodStart,
		) -> DispatchResult {
			T::AssignSlotOrigin::ensure_origin(origin)?;

			let manager = T::Registrar::manager_of(id).ok_or(Error::<T>::IndraDoesntExist)?;

			ensure!(T::Registrar::is_indrabase(id), Error::<T>::NotIndrabase);

			ensure!(
				!Self::has_permanent_slot(id) && !Self::has_temporary_slot(id),
				Error::<T>::SlotAlreadyAssigned
			);

			let current_lease_period: T::BlockNumber = Self::current_lease_period_index();
			ensure!(
				!T::Leaser::already_leased(
					id,
					current_lease_period,
					// Check current lease & next one
					current_lease_period.saturating_add(
						T::BlockNumber::from(2u32)
							.saturating_mul(T::TemporarySlotLeasePeriodLength::get().into())
					)
				),
				Error::<T>::OngoingLeaseExists
			);

			ensure!(
				TemporarySlotCount::<T>::get() < T::MaxTemporarySlots::get(),
				Error::<T>::MaxTemporarySlotsExceeded
			);

			let mut temp_slot = IndracoreTemporarySlot {
				manager: manager.clone(),
				period_begin: match lease_period_start {
					SlotLeasePeriodStart::Current => current_lease_period,
					SlotLeasePeriodStart::Next => current_lease_period + One::one(),
				},
				period_count: T::TemporarySlotLeasePeriodLength::get().into(),
				last_lease: None,
				lease_count: 0,
			};

			if lease_period_start == SlotLeasePeriodStart::Current &&
				Self::active_temporary_slot_count() < T::MaxTemporarySlotPerLeasePeriod::get()
			{
				// Try to allocate slot directly
				match Self::configure_slot_lease(
					id,
					manager,
					temp_slot.period_begin,
					temp_slot.period_count,
				) {
					Ok(_) => {
						ActiveTemporarySlotCount::<T>::mutate(|count| count.saturating_inc());
						temp_slot.last_lease = Some(temp_slot.period_begin);
						temp_slot.lease_count += 1;
					},
					Err(err) => {
						// Treat failed lease creation as warning .. slot will be allocated a lease
						// in a subsequent lease period by the `allocate_temporary_slot_leases` function.
						log::warn!(target: "assigned_slots",
							"Failed to allocate a temp slot for indra {:?} at period {:?}: {:?}",
							id, current_lease_period, err
						);
					},
				}
			}

			TemporarySlots::<T>::insert(id, temp_slot);
			<TemporarySlotCount<T>>::mutate(|count| count.saturating_inc());

			Self::deposit_event(Event::<T>::TemporarySlotAssigned(id));

			Ok(())
		}

		// TODO: Benchmark this
		/// Unassign a permanent or temporary indracore slot
		#[pallet::weight(((MAXIMUM_BLOCK_WEIGHT / 10) as Weight, DispatchClass::Operational))]
		pub fn unassign_indracore_slot(origin: OriginFor<T>, id: IndraId) -> DispatchResult {
			T::AssignSlotOrigin::ensure_origin(origin.clone())?;

			ensure!(
				Self::has_permanent_slot(id) || Self::has_temporary_slot(id),
				Error::<T>::SlotNotAssigned
			);

			// Check & cache indra status before we clear the lease
			let is_indracore = Self::is_indracore(id);

			// Remove perm or temp slot
			Self::clear_slot_leases(origin.clone(), id)?;

			if PermanentSlots::<T>::contains_key(id) {
				PermanentSlots::<T>::remove(id);
				<PermanentSlotCount<T>>::mutate(|count| *count = count.saturating_sub(One::one()));
			} else if TemporarySlots::<T>::contains_key(id) {
				TemporarySlots::<T>::remove(id);
				<TemporarySlotCount<T>>::mutate(|count| *count = count.saturating_sub(One::one()));
				if is_indracore {
					<ActiveTemporarySlotCount<T>>::mutate(|active_count| {
						*active_count = active_count.saturating_sub(One::one())
					});
				}
			}

			// Force downgrade to indrabase (if needed) before end of lease period
			if is_indracore {
				if let Err(err) = runtime_indracores::schedule_indracore_downgrade::<T>(id) {
					// Treat failed downgrade as warning .. slot lease has been cleared,
					// so the indracore will be downgraded anyway by the slots pallet
					// at the end of the lease period .
					log::warn!(target: "assigned_slots",
						"Failed to downgrade indracore {:?} at period {:?}: {:?}",
						id, Self::current_lease_period_index(), err
					);
				}
			}

			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Allocate temporary slot leases up to `MaxTemporarySlotPerLeasePeriod` per lease period.
	/// Beyond the already active temporary slot leases, this function will activate more leases
	/// in the following order of preference:
	/// - Assigned slots that didn't have a turn yet, though their `period_begin` has passed.
	/// - Assigned slots that already had one (or more) turn(s): they will be considered for the
	/// current slot lease if they weren't active in the preceding one, and will be ranked by
	/// total number of lease (lower first), and then when they last a turn (older ones first).
	/// If any remaining ex-aequo, we just take the indra ID in ascending order as discriminator.
	///
	/// Assigned slots with a `period_begin` bigger than current lease period are not considered (yet).
	///
	/// The function will call out to `Leaser::lease_out` to create the appropriate slot leases.
	fn allocate_temporary_slot_leases(lease_period_index: LeasePeriodOf<T>) -> DispatchResult {
		let mut active_temp_slots = 0u32;
		let mut pending_temp_slots = Vec::new();
		TemporarySlots::<T>::iter().for_each(|(indra, slot)| {
				match slot.last_lease {
					Some(last_lease)
						if last_lease <= lease_period_index &&
							lease_period_index <
								(last_lease.saturating_add(slot.period_count)) =>
					{
						// Active slot lease
						active_temp_slots += 1;
					}
					Some(last_lease)
						// Slot w/ past lease, only consider it every other slot lease period (times period_count)
						if last_lease.saturating_add(slot.period_count.saturating_mul(2u32.into())) <= lease_period_index => {
							pending_temp_slots.push((indra, slot));
					},
					None if slot.period_begin <= lease_period_index => {
						// Slot hasn't had a lease yet
						pending_temp_slots.insert(0, (indra, slot));
					},
					_ => {
						// Slot not being considered for this lease period (will be for a subsequent one)
					},
				}
		});

		let mut newly_created_lease = 0u32;
		if active_temp_slots < T::MaxTemporarySlotPerLeasePeriod::get() &&
			!pending_temp_slots.is_empty()
		{
			// Sort by lease_count, favoring slots that had no or less turns first
			// (then by last_lease index, and then Indra ID)
			pending_temp_slots.sort_by(|a, b| {
				a.1.lease_count
					.cmp(&b.1.lease_count)
					.then_with(|| a.1.last_lease.cmp(&b.1.last_lease))
					.then_with(|| a.0.cmp(&b.0))
			});

			let slots_to_be_upgraded = pending_temp_slots.iter().take(
				(T::MaxTemporarySlotPerLeasePeriod::get().saturating_sub(active_temp_slots))
					as usize,
			);

			for (id, temp_slot) in slots_to_be_upgraded {
				TemporarySlots::<T>::try_mutate::<_, _, Error<T>, _>(id, |s| {
					// Configure temp slot lease
					Self::configure_slot_lease(
						*id,
						temp_slot.manager.clone(),
						lease_period_index,
						temp_slot.period_count,
					)
					.map_err(|_| Error::<T>::CannotUpgrade)?;

					// Update temp slot lease info in storage
					*s = Some(IndracoreTemporarySlot {
						manager: temp_slot.manager.clone(),
						period_begin: temp_slot.period_begin,
						period_count: temp_slot.period_count,
						last_lease: Some(lease_period_index),
						lease_count: temp_slot.lease_count + 1,
					});

					newly_created_lease += 1;

					Ok(())
				})?;
			}
		}

		ActiveTemporarySlotCount::<T>::set(active_temp_slots + newly_created_lease);

		Ok(())
	}

	/// Clear out all slot leases for both permanent & temporary slots.
	/// The function merely calls out to `Slots::clear_all_leases`.
	fn clear_slot_leases(origin: OriginFor<T>, id: IndraId) -> DispatchResult {
		Slots::<T>::clear_all_leases(origin, id)
	}

	/// Create a indracore slot lease based on given indrams.
	/// The function merely calls out to `Leaser::lease_out`.
	fn configure_slot_lease(
		indra: IndraId,
		manager: T::AccountId,
		lease_period: LeasePeriodOf<T>,
		lease_duration: LeasePeriodOf<T>,
	) -> Result<(), LeaseError> {
		T::Leaser::lease_out(indra, &manager, BalanceOf::<T>::zero(), lease_period, lease_duration)
	}

	/// Returns whether a indra has been assigned a permanent slot.
	fn has_permanent_slot(id: IndraId) -> bool {
		PermanentSlots::<T>::contains_key(id)
	}

	/// Returns whether a indra has been assigned temporary slot.
	fn has_temporary_slot(id: IndraId) -> bool {
		TemporarySlots::<T>::contains_key(id)
	}

	/// Returns whether a indra is currently a indracore.
	fn is_indracore(id: IndraId) -> bool {
		T::Registrar::is_indracore(id)
	}

	/// Returns current lease period index.
	fn current_lease_period_index() -> LeasePeriodOf<T> {
		T::Leaser::lease_period_index(frame_system::Pallet::<T>::block_number())
			.and_then(|x| Some(x.0))
			.unwrap()
	}

	/// Returns lease period index for block
	fn lease_period_index(block: BlockNumberFor<T>) -> Option<(LeasePeriodOf<T>, bool)> {
		T::Leaser::lease_period_index(block)
	}

	/// Handles start of a lease period.
	fn manage_lease_period_start(lease_period_index: LeasePeriodOf<T>) -> Weight {
		// Note: leases that have ended in previous lease period, should have been cleaned in slots pallet.
		if let Err(err) = Self::allocate_temporary_slot_leases(lease_period_index) {
			log::error!(target: "assigned_slots",
				"Allocating slots failed for lease period {:?}, with: {:?}",
				lease_period_index, err
			);
		}
		<T as slots::Config>::WeightInfo::force_lease() *
			(T::MaxTemporarySlotPerLeasePeriod::get() as u64)
	}
}

/// tests for this pallet
#[cfg(test)]
mod tests {
	use super::*;

	use crate::{assigned_slots, mock::TestRegistrar, slots};
	use ::test_helpers::{dummy_head_data, dummy_validation_code};
	use frame_support::{assert_noop, assert_ok, parameter_types};
	use frame_system::EnsureRoot;
	use pallet_balances;
	use primitives::v2::{BlockNumber, Header};
	use runtime_indracores::{
		configuration as indracores_configuration, paras as indracores_indras,
		shared as indracores_shared,
	};
	use sp_core::H256;
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup},
		transaction_validity::TransactionPriority,
		DispatchError::BadOrigin,
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			Configuration: indracores_configuration::{Pallet, Call, Storage, Config<T>},
			IndrasShared: indracores_shared::{Pallet, Call, Storage},
			Indracores: indracores_indras::{Pallet, Call, Storage, Config, Event},
			Slots: slots::{Pallet, Call, Storage, Event<T>},
			AssignedSlots: assigned_slots::{Pallet, Call, Storage, Event<T>},
		}
	);

	impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
	where
		Call: From<C>,
	{
		type Extrinsic = UncheckedExtrinsic;
		type OverarchingCall = Call;
	}

	parameter_types! {
		pub const BlockHashCount: u32 = 250;
	}
	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type BlockWeights = ();
		type BlockLength = ();
		type Origin = Origin;
		type Call = Call;
		type Index = u64;
		type BlockNumber = BlockNumber;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type DbWeight = ();
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	parameter_types! {
		pub const ExistentialDeposit: u64 = 1;
	}

	impl pallet_balances::Config for Test {
		type Balance = u64;
		type Event = Event;
		type DustRemoval = ();
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type WeightInfo = ();
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
	}

	impl indracores_configuration::Config for Test {
		type WeightInfo = indracores_configuration::TestWeightInfo;
	}

	parameter_types! {
		pub const IndrasUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	}

	impl indracores_indras::Config for Test {
		type Event = Event;
		type WeightInfo = indracores_indras::TestWeightInfo;
		type UnsignedPriority = IndrasUnsignedPriority;
		type NextSessionRotation = crate::mock::TestNextSessionRotation;
	}

	impl indracores_shared::Config for Test {}

	parameter_types! {
		pub const LeasePeriod: BlockNumber = 3;
		pub static LeaseOffset: BlockNumber = 0;
		pub const IndraDeposit: u64 = 1;
	}

	impl slots::Config for Test {
		type Event = Event;
		type Currency = Balances;
		type Registrar = TestRegistrar<Test>;
		type LeasePeriod = LeasePeriod;
		type LeaseOffset = LeaseOffset;
		type ForceOrigin = EnsureRoot<Self::AccountId>;
		type WeightInfo = crate::slots::TestWeightInfo;
	}

	parameter_types! {
		pub const PermanentSlotLeasePeriodLength: u32 = 3;
		pub const TemporarySlotLeasePeriodLength: u32 = 2;
		pub const MaxPermanentSlots: u32 = 2;
		pub const MaxTemporarySlots: u32 = 6;
		pub const MaxTemporarySlotPerLeasePeriod: u32 = 2;
	}

	impl assigned_slots::Config for Test {
		type Event = Event;
		type AssignSlotOrigin = EnsureRoot<Self::AccountId>;
		type Leaser = Slots;
		type PermanentSlotLeasePeriodLength = PermanentSlotLeasePeriodLength;
		type TemporarySlotLeasePeriodLength = TemporarySlotLeasePeriodLength;
		type MaxPermanentSlots = MaxPermanentSlots;
		type MaxTemporarySlots = MaxTemporarySlots;
		type MaxTemporarySlotPerLeasePeriod = MaxTemporarySlotPerLeasePeriod;
	}

	// This function basically just builds a genesis storage key/value store according to
	// our desired mock up.
	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: vec![(1, 10), (2, 20), (3, 30), (4, 40), (5, 50), (6, 60)],
		}
		.assimilate_storage(&mut t)
		.unwrap();
		t.into()
	}

	fn run_to_block(n: BlockNumber) {
		while System::block_number() < n {
			let mut block = System::block_number();
			// on_finalize hooks
			AssignedSlots::on_finalize(block);
			Slots::on_finalize(block);
			Indracores::on_finalize(block);
			IndrasShared::on_finalize(block);
			Configuration::on_finalize(block);
			Balances::on_finalize(block);
			System::on_finalize(block);
			// Set next block
			System::set_block_number(block + 1);
			block = System::block_number();
			// on_initialize hooks
			System::on_initialize(block);
			Balances::on_initialize(block);
			Configuration::on_initialize(block);
			IndrasShared::on_initialize(block);
			Indracores::on_initialize(block);
			Slots::on_initialize(block);
			AssignedSlots::on_initialize(block);
		}
	}

	#[test]
	fn basic_setup_works() {
		new_test_ext().execute_with(|| {
			run_to_block(1);
			assert_eq!(AssignedSlots::current_lease_period_index(), 0);
			assert_eq!(Slots::deposit_held(1.into(), &1), 0);

			run_to_block(3);
			assert_eq!(AssignedSlots::current_lease_period_index(), 1);
		});
	}

	#[test]
	fn assign_perm_slot_fails_for_unknown_indra() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::root(), IndraId::from(1_u32),),
				Error::<Test>::IndraDoesntExist
			);
		});
	}

	#[test]
	fn assign_perm_slot_fails_for_invalid_origin() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::signed(1), IndraId::from(1_u32),),
				BadOrigin
			);
		});
	}

	#[test]
	fn assign_perm_slot_fails_when_not_indrabase() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));
			assert_ok!(TestRegistrar::<Test>::make_indracore(IndraId::from(1_u32)));

			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::root(), IndraId::from(1_u32),),
				Error::<Test>::NotIndrabase
			);
		});
	}

	#[test]
	fn assign_perm_slot_fails_when_existing_lease() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			// Register lease in current lease period
			assert_ok!(Slots::lease_out(IndraId::from(1_u32), &1, 1, 1, 1));
			// Try to assign a perm slot in current period fails
			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::root(), IndraId::from(1_u32),),
				Error::<Test>::OngoingLeaseExists
			);

			// Cleanup
			assert_ok!(Slots::clear_all_leases(Origin::root(), 1.into()));

			// Register lease for next lease period
			assert_ok!(Slots::lease_out(IndraId::from(1_u32), &1, 1, 2, 1));
			// Should be detected and also fail
			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::root(), IndraId::from(1_u32),),
				Error::<Test>::OngoingLeaseExists
			);
		});
	}

	#[test]
	fn assign_perm_slot_fails_when_max_perm_slots_exceeded() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_ok!(TestRegistrar::<Test>::register(
				2,
				IndraId::from(2_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_ok!(TestRegistrar::<Test>::register(
				3,
				IndraId::from(3_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_ok!(AssignedSlots::assign_perm_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
			));
			assert_ok!(AssignedSlots::assign_perm_indracore_slot(
				Origin::root(),
				IndraId::from(2_u32),
			));
			assert_eq!(AssignedSlots::permanent_slot_count(), 2);

			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::root(), IndraId::from(3_u32),),
				Error::<Test>::MaxPermanentSlotsExceeded
			);
		});
	}

	#[test]
	fn assign_perm_slot_succeeds_for_indrabase() {
		new_test_ext().execute_with(|| {
			let mut block = 1;
			run_to_block(block);
			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_eq!(AssignedSlots::permanent_slot_count(), 0);
			assert_eq!(AssignedSlots::permanent_slots(IndraId::from(1_u32)), None);

			assert_ok!(AssignedSlots::assign_perm_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
			));

			// Indra is a indracore for PermanentSlotLeasePeriodLength * LeasePeriod blocks
			while block < 9 {
				println!("block #{}", block);

				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);

				assert_eq!(AssignedSlots::permanent_slot_count(), 1);
				assert_eq!(AssignedSlots::has_permanent_slot(IndraId::from(1_u32)), true);
				assert_eq!(AssignedSlots::permanent_slots(IndraId::from(1_u32)), Some((0, 3)));

				assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 2), true);

				block += 1;
				run_to_block(block);
			}

			// Indra lease ended, downgraded back to indrabase
			assert_eq!(TestRegistrar::<Test>::is_indrabase(IndraId::from(1_u32)), true);
			assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 5), false);
		});
	}

	#[test]
	fn assign_temp_slot_fails_for_unknown_indra() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(1_u32),
					SlotLeasePeriodStart::Current
				),
				Error::<Test>::IndraDoesntExist
			);
		});
	}

	#[test]
	fn assign_temp_slot_fails_for_invalid_origin() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::signed(1),
					IndraId::from(1_u32),
					SlotLeasePeriodStart::Current
				),
				BadOrigin
			);
		});
	}

	#[test]
	fn assign_temp_slot_fails_when_not_indrabase() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));
			assert_ok!(TestRegistrar::<Test>::make_indracore(IndraId::from(1_u32)));

			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(1_u32),
					SlotLeasePeriodStart::Current
				),
				Error::<Test>::NotIndrabase
			);
		});
	}

	#[test]
	fn assign_temp_slot_fails_when_existing_lease() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			// Register lease in current lease period
			assert_ok!(Slots::lease_out(IndraId::from(1_u32), &1, 1, 1, 1));
			// Try to assign a perm slot in current period fails
			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(1_u32),
					SlotLeasePeriodStart::Current
				),
				Error::<Test>::OngoingLeaseExists
			);

			// Cleanup
			assert_ok!(Slots::clear_all_leases(Origin::root(), 1.into()));

			// Register lease for next lease period
			assert_ok!(Slots::lease_out(IndraId::from(1_u32), &1, 1, 2, 1));
			// Should be detected and also fail
			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(1_u32),
					SlotLeasePeriodStart::Current
				),
				Error::<Test>::OngoingLeaseExists
			);
		});
	}

	#[test]
	fn assign_temp_slot_fails_when_max_temp_slots_exceeded() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			// Register 6 indras & a temp slot for each
			for n in 0..=5 {
				assert_ok!(TestRegistrar::<Test>::register(
					n,
					IndraId::from(n as u32),
					dummy_head_data(),
					dummy_validation_code()
				));

				assert_ok!(AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(n as u32),
					SlotLeasePeriodStart::Current
				));
			}

			assert_eq!(AssignedSlots::temporary_slot_count(), 6);

			// Attempt to assign one more temp slot
			assert_ok!(TestRegistrar::<Test>::register(
				7,
				IndraId::from(7_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));
			assert_noop!(
				AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(7_u32),
					SlotLeasePeriodStart::Current
				),
				Error::<Test>::MaxTemporarySlotsExceeded
			);
		});
	}

	#[test]
	fn assign_temp_slot_succeeds_for_single_indrabase() {
		new_test_ext().execute_with(|| {
			let mut block = 1;
			run_to_block(block);
			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_eq!(AssignedSlots::temporary_slots(IndraId::from(1_u32)), None);

			assert_ok!(AssignedSlots::assign_temp_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
				SlotLeasePeriodStart::Current
			));
			assert_eq!(AssignedSlots::temporary_slot_count(), 1);
			assert_eq!(AssignedSlots::active_temporary_slot_count(), 1);

			// Block 1-5
			// Indra is a indracore for TemporarySlotLeasePeriodLength * LeasePeriod blocks
			while block < 6 {
				println!("block #{}", block);
				println!("lease period #{}", AssignedSlots::current_lease_period_index());
				println!("lease {:?}", Slots::lease(IndraId::from(1_u32)));

				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);

				assert_eq!(AssignedSlots::has_temporary_slot(IndraId::from(1_u32)), true);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 1);
				assert_eq!(
					AssignedSlots::temporary_slots(IndraId::from(1_u32)),
					Some(IndracoreTemporarySlot {
						manager: 1,
						period_begin: 0,
						period_count: 2, // TemporarySlotLeasePeriodLength
						last_lease: Some(0),
						lease_count: 1
					})
				);

				assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 1), true);

				block += 1;
				run_to_block(block);
			}

			// Block 6
			println!("block #{}", block);
			println!("lease period #{}", AssignedSlots::current_lease_period_index());
			println!("lease {:?}", Slots::lease(IndraId::from(1_u32)));

			// Indra lease ended, downgraded back to indrabase
			assert_eq!(TestRegistrar::<Test>::is_indrabase(IndraId::from(1_u32)), true);
			assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 3), false);
			assert_eq!(AssignedSlots::active_temporary_slot_count(), 0);

			// Block 12
			// Indra should get a turn after TemporarySlotLeasePeriodLength * LeasePeriod blocks
			run_to_block(12);
			println!("block #{}", block);
			println!("lease period #{}", AssignedSlots::current_lease_period_index());
			println!("lease {:?}", Slots::lease(IndraId::from(1_u32)));

			assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);
			assert_eq!(Slots::already_leased(IndraId::from(1_u32), 4, 5), true);
			assert_eq!(AssignedSlots::active_temporary_slot_count(), 1);
		});
	}

	#[test]
	fn assign_temp_slot_succeeds_for_multiple_indrabases() {
		new_test_ext().execute_with(|| {
			// Block 1, Period 0
			run_to_block(1);

			// Register 6 indras & a temp slot for each
			// (3 slots in current lease period, 3 in the next one)
			for n in 0..=5 {
				assert_ok!(TestRegistrar::<Test>::register(
					n,
					IndraId::from(n as u32),
					dummy_head_data(),
					dummy_validation_code()
				));

				assert_ok!(AssignedSlots::assign_temp_indracore_slot(
					Origin::root(),
					IndraId::from(n as u32),
					if (n % 2).is_zero() {
						SlotLeasePeriodStart::Current
					} else {
						SlotLeasePeriodStart::Next
					}
				));
			}

			// Block 1-5, Period 0-1
			for n in 1..=5 {
				if n > 1 {
					run_to_block(n);
				}
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), false);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}

			// Block 6-11, Period 2-3
			for n in 6..=11 {
				run_to_block(n);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), false);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}

			// Block 12-17, Period 4-5
			for n in 12..=17 {
				run_to_block(n);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), true);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}

			// Block 18-23, Period 6-7
			for n in 18..=23 {
				run_to_block(n);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), false);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}

			// Block 24-29, Period 8-9
			for n in 24..=29 {
				run_to_block(n);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), false);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}

			// Block 30-35, Period 10-11
			for n in 30..=35 {
				run_to_block(n);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(0)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(2_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(3_u32)), false);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(4_u32)), true);
				assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(5_u32)), true);
				assert_eq!(AssignedSlots::active_temporary_slot_count(), 2);
			}
		});
	}

	#[test]
	fn unassign_slot_fails_for_unknown_indra() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::unassign_indracore_slot(Origin::root(), IndraId::from(1_u32),),
				Error::<Test>::SlotNotAssigned
			);
		});
	}

	#[test]
	fn unassign_slot_fails_for_invalid_origin() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_noop!(
				AssignedSlots::assign_perm_indracore_slot(Origin::signed(1), IndraId::from(1_u32),),
				BadOrigin
			);
		});
	}

	#[test]
	fn unassign_perm_slot_succeeds() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_ok!(AssignedSlots::assign_perm_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
			));

			assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);

			assert_ok!(AssignedSlots::unassign_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
			));

			assert_eq!(AssignedSlots::permanent_slot_count(), 0);
			assert_eq!(AssignedSlots::has_permanent_slot(IndraId::from(1_u32)), false);
			assert_eq!(AssignedSlots::permanent_slots(IndraId::from(1_u32)), None);

			assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 2), false);
		});
	}

	#[test]
	fn unassign_temp_slot_succeeds() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(TestRegistrar::<Test>::register(
				1,
				IndraId::from(1_u32),
				dummy_head_data(),
				dummy_validation_code(),
			));

			assert_ok!(AssignedSlots::assign_temp_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
				SlotLeasePeriodStart::Current
			));

			assert_eq!(TestRegistrar::<Test>::is_indracore(IndraId::from(1_u32)), true);

			assert_ok!(AssignedSlots::unassign_indracore_slot(
				Origin::root(),
				IndraId::from(1_u32),
			));

			assert_eq!(AssignedSlots::temporary_slot_count(), 0);
			assert_eq!(AssignedSlots::active_temporary_slot_count(), 0);
			assert_eq!(AssignedSlots::has_temporary_slot(IndraId::from(1_u32)), false);
			assert_eq!(AssignedSlots::temporary_slots(IndraId::from(1_u32)), None);

			assert_eq!(Slots::already_leased(IndraId::from(1_u32), 0, 1), false);
		});
	}
}
