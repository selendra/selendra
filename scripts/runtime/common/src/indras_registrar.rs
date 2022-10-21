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

//! Pallet to handle indrabase/indracore registration and related fund management.
//! In essence this is a simple wrapper around `indras`.

use frame_support::{
	dispatch::DispatchResult,
	ensure,
	pallet_prelude::Weight,
	traits::{Currency, Get, ReservableCurrency},
};
use frame_system::{self, ensure_root, ensure_signed};
use primitives::v2::{HeadData, Id as IndraId, ValidationCode, LOWEST_PUBLIC_ID};
use runtime_indracores::{
	configuration, ensure_indracore, indras, indras::IndraGenesisArgs, IndraLifecycle, Origin,
};
use sp_std::{prelude::*, result};

use crate::traits::{OnSwap, Registrar};
pub use pallet::*;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{CheckedSub, Saturating},
	RuntimeDebug,
};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
pub struct IndraInfo<Account, Balance> {
	/// The account that has placed a deposit for registering this indra.
	pub(crate) manager: Account,
	/// The amount reserved by the `manager` account for the registration.
	deposit: Balance,
	/// Whether the indra registration should be locked from being controlled by the manager.
	locked: bool,
}

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait WeightInfo {
	fn reserve() -> Weight;
	fn register() -> Weight;
	fn force_register() -> Weight;
	fn deregister() -> Weight;
	fn swap() -> Weight;
}

pub struct TestWeightInfo;
impl WeightInfo for TestWeightInfo {
	fn reserve() -> Weight {
		0
	}
	fn register() -> Weight {
		0
	}
	fn force_register() -> Weight {
		0
	}
	fn deregister() -> Weight {
		0
	}
	fn swap() -> Weight {
		0
	}
}

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	#[pallet::disable_frame_system_supertrait_check]
	pub trait Config: configuration::Config + indras::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The aggregated origin type must support the `indracores` origin. We require that we can
		/// infallibly convert between this origin and the system origin, but in reality, they're the
		/// same type, we just can't express that to the Rust type system without writing a `where`
		/// clause everywhere.
		type Origin: From<<Self as frame_system::Config>::Origin>
			+ Into<result::Result<Origin, <Self as Config>::Origin>>;

		/// The system's currency for indrabase payment.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Runtime hook for when a indracore and indrabase swap.
		type OnSwap: crate::traits::OnSwap;

		/// The deposit to be paid to run a indrabase.
		/// This should include the cost for storing the genesis head and validation code.
		#[pallet::constant]
		type IndraDeposit: Get<BalanceOf<Self>>;

		/// The deposit to be paid per byte stored on chain.
		#[pallet::constant]
		type DataDepositPerByte: Get<BalanceOf<Self>>;

		/// Weight Information for the Extrinsics in the Pallet
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Registered { indra_id: IndraId, manager: T::AccountId },
		Deregistered { indra_id: IndraId },
		Reserved { indra_id: IndraId, who: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The ID is not registered.
		NotRegistered,
		/// The ID is already registered.
		AlreadyRegistered,
		/// The caller is not the owner of this Id.
		NotOwner,
		/// Invalid indra code size.
		CodeTooLarge,
		/// Invalid indra head data size.
		HeadDataTooLarge,
		/// Indra is not a Indracore.
		NotIndracore,
		/// Indra is not a Indrabase.
		NotIndrabase,
		/// Cannot deregister indra
		CannotDeregister,
		/// Cannot schedule downgrade of indracore to indrabase
		CannotDowngrade,
		/// Cannot schedule upgrade of indrabase to indracore
		CannotUpgrade,
		/// Indra is locked from manipulation by the manager. Must use indracore or relay chain governance.
		IndraLocked,
		/// The ID given for registration has not been reserved.
		NotReserved,
		/// Registering indracore with empty code is not allowed.
		EmptyCode,
		/// Cannot perform a indracore slot / lifecycle swap. Check that the state of both indras are
		/// correct for the swap to work.
		CannotSwap,
	}

	/// Pending swap operations.
	#[pallet::storage]
	pub(super) type PendingSwap<T> = StorageMap<_, Twox64Concat, IndraId, IndraId>;

	/// Amount held on deposit for each indra and the original depositor.
	///
	/// The given account ID is responsible for registering the code and initial head data, but may only do
	/// so if it isn't yet registered. (After that, it's up to governance to do so.)
	#[pallet::storage]
	pub type Indras<T: Config> =
		StorageMap<_, Twox64Concat, IndraId, IndraInfo<T::AccountId, BalanceOf<T>>>;

	/// The next free `IndraId`.
	#[pallet::storage]
	pub type NextFreeIndraId<T> = StorageValue<_, IndraId, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub next_free_indra_id: IndraId,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			GenesisConfig { next_free_indra_id: LOWEST_PUBLIC_ID }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			NextFreeIndraId::<T>::put(self.next_free_indra_id);
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Register head data and validation code for a reserved Indra Id.
		///
		/// ## Arguments
		/// - `origin`: Must be called by a `Signed` origin.
		/// - `id`: The indra ID. Must be owned/managed by the `origin` signing account.
		/// - `genesis_head`: The genesis head data of the indracore/thread.
		/// - `validation_code`: The initial validation code of the indracore/thread.
		///
		/// ## Deposits/Fees
		/// The origin signed account must reserve a corresponding deposit for the registration. Anything already
		/// reserved previously for this indra ID is accounted for.
		///
		/// ## Events
		/// The `Registered` event is emitted in case of success.
		#[pallet::weight(<T as Config>::WeightInfo::register())]
		pub fn register(
			origin: OriginFor<T>,
			id: IndraId,
			genesis_head: HeadData,
			validation_code: ValidationCode,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_register(who, None, id, genesis_head, validation_code, true)?;
			Ok(())
		}

		/// Force the registration of a Indra Id on the relay chain.
		///
		/// This function must be called by a Root origin.
		///
		/// The deposit taken can be specified for this registration. Any `IndraId`
		/// can be registered, including sub-1000 IDs which are System Indracores.
		#[pallet::weight(<T as Config>::WeightInfo::force_register())]
		pub fn force_register(
			origin: OriginFor<T>,
			who: T::AccountId,
			deposit: BalanceOf<T>,
			id: IndraId,
			genesis_head: HeadData,
			validation_code: ValidationCode,
		) -> DispatchResult {
			ensure_root(origin)?;
			Self::do_register(who, Some(deposit), id, genesis_head, validation_code, false)
		}

		/// Deregister a Indra Id, freeing all data and returning any deposit.
		///
		/// The caller must be Root, the `indra` owner, or the `indra` itself. The indra must be a indrabase.
		#[pallet::weight(<T as Config>::WeightInfo::deregister())]
		pub fn deregister(origin: OriginFor<T>, id: IndraId) -> DispatchResult {
			Self::ensure_root_indra_or_owner(origin, id)?;
			Self::do_deregister(id)
		}

		/// Swap a indracore with another indracore or indrabase.
		///
		/// The origin must be Root, the `indra` owner, or the `indra` itself.
		///
		/// The swap will happen only if there is already an opposite swap pending. If there is not,
		/// the swap will be stored in the pending swaps map, ready for a later confirmatory swap.
		///
		/// The `IndraId`s remain mapped to the same head data and code so external code can rely on
		/// `IndraId` to be a long-term identifier of a notional "indracore". However, their
		/// scheduling info (i.e. whether they're a indrabase or indracore), auction information
		/// and the auction deposit are switched.
		#[pallet::weight(<T as Config>::WeightInfo::swap())]
		pub fn swap(origin: OriginFor<T>, id: IndraId, other: IndraId) -> DispatchResult {
			Self::ensure_root_indra_or_owner(origin, id)?;

			// If `id` and `other` is the same id, we treat this as a "clear" function, and exit
			// early, since swapping the same id would otherwise be a noop.
			if id == other {
				PendingSwap::<T>::remove(id);
				return Ok(())
			}

			// Sanity check that `id` is even a indra.
			let id_lifecycle =
				indras::Pallet::<T>::lifecycle(id).ok_or(Error::<T>::NotRegistered)?;

			if PendingSwap::<T>::get(other) == Some(id) {
				let other_lifecycle =
					indras::Pallet::<T>::lifecycle(other).ok_or(Error::<T>::NotRegistered)?;
				// identify which is a indracore and which is a indrabase
				if id_lifecycle == IndraLifecycle::Indracore &&
					other_lifecycle == IndraLifecycle::Indrabase
				{
					Self::do_thread_and_chain_swap(id, other);
				} else if id_lifecycle == IndraLifecycle::Indrabase &&
					other_lifecycle == IndraLifecycle::Indracore
				{
					Self::do_thread_and_chain_swap(other, id);
				} else if id_lifecycle == IndraLifecycle::Indracore &&
					other_lifecycle == IndraLifecycle::Indracore
				{
					// If both chains are currently indracores, there is nothing funny we
					// need to do for their lifecycle management, just swap the underlying
					// data.
					T::OnSwap::on_swap(id, other);
				} else {
					return Err(Error::<T>::CannotSwap.into())
				}
				PendingSwap::<T>::remove(other);
			} else {
				PendingSwap::<T>::insert(id, other);
			}

			Ok(())
		}

		/// Remove a manager lock from a indra. This will allow the manager of a
		/// previously locked indra to deregister or swap a indra without using governance.
		///
		/// Can only be called by the Root origin.
		#[pallet::weight(T::DbWeight::get().reads_writes(1, 1))]
		pub fn force_remove_lock(origin: OriginFor<T>, indra: IndraId) -> DispatchResult {
			ensure_root(origin)?;
			Self::remove_lock(indra);
			Ok(())
		}

		/// Reserve a Indra Id on the relay chain.
		///
		/// This function will reserve a new Indra Id to be owned/managed by the origin account.
		/// The origin account is able to register head data and validation code using `register` to create
		/// a indrabase. Using the Slots pallet, a indrabase can then be upgraded to get a indracore slot.
		///
		/// ## Arguments
		/// - `origin`: Must be called by a `Signed` origin. Becomes the manager/owner of the new indra ID.
		///
		/// ## Deposits/Fees
		/// The origin must reserve a deposit of `IndraDeposit` for the registration.
		///
		/// ## Events
		/// The `Reserved` event is emitted in case of success, which provides the ID reserved for use.
		#[pallet::weight(<T as Config>::WeightInfo::reserve())]
		pub fn reserve(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let id = NextFreeIndraId::<T>::get().max(LOWEST_PUBLIC_ID);
			Self::do_reserve(who, None, id)?;
			NextFreeIndraId::<T>::set(id + 1);
			Ok(())
		}
	}
}

impl<T: Config> Registrar for Pallet<T> {
	type AccountId = T::AccountId;

	/// Return the manager `AccountId` of a indra if one exists.
	fn manager_of(id: IndraId) -> Option<T::AccountId> {
		Some(Indras::<T>::get(id)?.manager)
	}

	// All indracores. Ordered ascending by IndraId. Indrabases are not included.
	fn indracores() -> Vec<IndraId> {
		indras::Pallet::<T>::indracores()
	}

	// Return if a indra is a indrabase
	fn is_indrabase(id: IndraId) -> bool {
		indras::Pallet::<T>::is_indrabase(id)
	}

	// Return if a indra is a indracore
	fn is_indracore(id: IndraId) -> bool {
		indras::Pallet::<T>::is_indracore(id)
	}

	// Apply a lock to the indracore.
	fn apply_lock(id: IndraId) {
		Indras::<T>::mutate(id, |x| x.as_mut().map(|mut info| info.locked = true));
	}

	// Apply a lock to the indracore.
	fn remove_lock(id: IndraId) {
		Indras::<T>::mutate(id, |x| x.as_mut().map(|mut info| info.locked = false));
	}

	// Register a Indra ID under control of `manager`.
	//
	// Note this is a backend registration API, so verification of IndraId
	// is not done here to prevent.
	fn register(
		manager: T::AccountId,
		id: IndraId,
		genesis_head: HeadData,
		validation_code: ValidationCode,
	) -> DispatchResult {
		Self::do_register(manager, None, id, genesis_head, validation_code, false)
	}

	// Deregister a Indra ID, free any data, and return any deposits.
	fn deregister(id: IndraId) -> DispatchResult {
		Self::do_deregister(id)
	}

	// Upgrade a registered indrabase into a indracore.
	fn make_indracore(id: IndraId) -> DispatchResult {
		// Indra backend should think this is a indrabase...
		ensure!(
			indras::Pallet::<T>::lifecycle(id) == Some(IndraLifecycle::Indrabase),
			Error::<T>::NotIndrabase
		);
		runtime_indracores::schedule_indrabase_upgrade::<T>(id)
			.map_err(|_| Error::<T>::CannotUpgrade)?;
		// Once a indra has upgraded to a indracore, it can no longer be managed by the owner.
		// Intentionally, the flag stays with the indra even after downgrade.
		Self::apply_lock(id);
		Ok(())
	}

	// Downgrade a registered indra into a indrabase.
	fn make_indrabase(id: IndraId) -> DispatchResult {
		// Indra backend should think this is a indracore...
		ensure!(
			indras::Pallet::<T>::lifecycle(id) == Some(IndraLifecycle::Indracore),
			Error::<T>::NotIndracore
		);
		runtime_indracores::schedule_indracore_downgrade::<T>(id)
			.map_err(|_| Error::<T>::CannotDowngrade)?;
		Ok(())
	}

	#[cfg(any(feature = "runtime-benchmarks", test))]
	fn worst_head_data() -> HeadData {
		let max_head_size = configuration::Pallet::<T>::config().max_head_data_size;
		assert!(max_head_size > 0, "max_head_data can't be zero for generating worst head data.");
		vec![0u8; max_head_size as usize].into()
	}

	#[cfg(any(feature = "runtime-benchmarks", test))]
	fn worst_validation_code() -> ValidationCode {
		let max_code_size = configuration::Pallet::<T>::config().max_code_size;
		assert!(max_code_size > 0, "max_code_size can't be zero for generating worst code data.");
		let validation_code = vec![0u8; max_code_size as usize];
		validation_code.into()
	}

	#[cfg(any(feature = "runtime-benchmarks", test))]
	fn execute_pending_transitions() {
		use runtime_indracores::shared;
		shared::Pallet::<T>::set_session_index(shared::Pallet::<T>::scheduled_session());
		indras::Pallet::<T>::test_on_new_session();
	}
}

impl<T: Config> Pallet<T> {
	/// Ensure the origin is one of Root, the `indra` owner, or the `indra` itself.
	/// If the origin is the `indra` owner, the `indra` must be unlocked.
	fn ensure_root_indra_or_owner(
		origin: <T as frame_system::Config>::Origin,
		id: IndraId,
	) -> DispatchResult {
		ensure_signed(origin.clone())
			.map_err(|e| e.into())
			.and_then(|who| -> DispatchResult {
				let indra_info = Indras::<T>::get(id).ok_or(Error::<T>::NotRegistered)?;
				ensure!(!indra_info.locked, Error::<T>::IndraLocked);
				ensure!(indra_info.manager == who, Error::<T>::NotOwner);
				Ok(())
			})
			.or_else(|_| -> DispatchResult {
				// Else check if indra origin...
				let caller_id = ensure_indracore(<T as Config>::Origin::from(origin.clone()))?;
				ensure!(caller_id == id, Error::<T>::NotOwner);
				Ok(())
			})
			.or_else(|_| -> DispatchResult {
				// Check if root...
				ensure_root(origin.clone()).map_err(|e| e.into())
			})
	}

	fn do_reserve(
		who: T::AccountId,
		deposit_override: Option<BalanceOf<T>>,
		id: IndraId,
	) -> DispatchResult {
		ensure!(!Indras::<T>::contains_key(id), Error::<T>::AlreadyRegistered);
		ensure!(indras::Pallet::<T>::lifecycle(id).is_none(), Error::<T>::AlreadyRegistered);

		let deposit = deposit_override.unwrap_or_else(T::IndraDeposit::get);
		<T as Config>::Currency::reserve(&who, deposit)?;
		let info = IndraInfo { manager: who.clone(), deposit, locked: false };

		Indras::<T>::insert(id, info);
		Self::deposit_event(Event::<T>::Reserved { indra_id: id, who });
		Ok(())
	}

	/// Attempt to register a new Indra Id under management of `who` in the
	/// system with the given information.
	fn do_register(
		who: T::AccountId,
		deposit_override: Option<BalanceOf<T>>,
		id: IndraId,
		genesis_head: HeadData,
		validation_code: ValidationCode,
		ensure_reserved: bool,
	) -> DispatchResult {
		let deposited = if let Some(indra_data) = Indras::<T>::get(id) {
			ensure!(indra_data.manager == who, Error::<T>::NotOwner);
			ensure!(!indra_data.locked, Error::<T>::IndraLocked);
			indra_data.deposit
		} else {
			ensure!(!ensure_reserved, Error::<T>::NotReserved);
			Default::default()
		};
		ensure!(indras::Pallet::<T>::lifecycle(id).is_none(), Error::<T>::AlreadyRegistered);
		let (genesis, deposit) =
			Self::validate_onboarding_data(genesis_head, validation_code, false)?;
		let deposit = deposit_override.unwrap_or(deposit);

		if let Some(additional) = deposit.checked_sub(&deposited) {
			<T as Config>::Currency::reserve(&who, additional)?;
		} else if let Some(rebate) = deposited.checked_sub(&deposit) {
			<T as Config>::Currency::unreserve(&who, rebate);
		};
		let info = IndraInfo { manager: who.clone(), deposit, locked: false };

		Indras::<T>::insert(id, info);
		// We check above that indra has no lifecycle, so this should not fail.
		let res = runtime_indracores::schedule_indra_initialize::<T>(id, genesis);
		debug_assert!(res.is_ok());
		Self::deposit_event(Event::<T>::Registered { indra_id: id, manager: who });
		Ok(())
	}

	/// Deregister a Indra Id, freeing all data returning any deposit.
	fn do_deregister(id: IndraId) -> DispatchResult {
		match indras::Pallet::<T>::lifecycle(id) {
			// Indra must be a indrabase, or not exist at all.
			Some(IndraLifecycle::Indrabase) | None => {},
			_ => return Err(Error::<T>::NotIndrabase.into()),
		}
		runtime_indracores::schedule_indra_cleanup::<T>(id)
			.map_err(|_| Error::<T>::CannotDeregister)?;

		if let Some(info) = Indras::<T>::take(&id) {
			<T as Config>::Currency::unreserve(&info.manager, info.deposit);
		}

		PendingSwap::<T>::remove(id);
		Self::deposit_event(Event::<T>::Deregistered { indra_id: id });
		Ok(())
	}

	/// Verifies the onboarding data is valid for a indra.
	///
	/// Returns `IndraGenesisArgs` and the deposit needed for the data.
	fn validate_onboarding_data(
		genesis_head: HeadData,
		validation_code: ValidationCode,
		indracore: bool,
	) -> Result<(IndraGenesisArgs, BalanceOf<T>), sp_runtime::DispatchError> {
		let config = configuration::Pallet::<T>::config();
		ensure!(validation_code.0.len() > 0, Error::<T>::EmptyCode);
		ensure!(validation_code.0.len() <= config.max_code_size as usize, Error::<T>::CodeTooLarge);
		ensure!(
			genesis_head.0.len() <= config.max_head_data_size as usize,
			Error::<T>::HeadDataTooLarge
		);

		let per_byte_fee = T::DataDepositPerByte::get();
		let deposit = T::IndraDeposit::get()
			.saturating_add(per_byte_fee.saturating_mul((genesis_head.0.len() as u32).into()))
			.saturating_add(per_byte_fee.saturating_mul((validation_code.0.len() as u32).into()));

		Ok((IndraGenesisArgs { genesis_head, validation_code, indracore }, deposit))
	}

	/// Swap a indracore and indrabase, which involves scheduling an appropriate lifecycle update.
	fn do_thread_and_chain_swap(to_downgrade: IndraId, to_upgrade: IndraId) {
		let res1 = runtime_indracores::schedule_indracore_downgrade::<T>(to_downgrade);
		debug_assert!(res1.is_ok());
		let res2 = runtime_indracores::schedule_indrabase_upgrade::<T>(to_upgrade);
		debug_assert!(res2.is_ok());
		T::OnSwap::on_swap(to_upgrade, to_downgrade);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{indras_registrar, traits::Registrar as RegistrarTrait};
	use frame_support::{
		assert_noop, assert_ok,
		error::BadOrigin,
		parameter_types,
		traits::{GenesisBuild, OnFinalize, OnInitialize},
	};
	use frame_system::limits;
	use pallet_balances::Error as BalancesError;
	use primitives::v2::{Balance, BlockNumber, Header};
	use runtime_indracores::{configuration, origin, shared};
	use sp_core::H256;
	use sp_io::TestExternalities;
	use sp_runtime::{
		traits::{BlakeTwo256, IdentityLookup},
		transaction_validity::TransactionPriority,
		Perbill,
	};
	use sp_std::collections::btree_map::BTreeMap;

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
			Configuration: configuration::{Pallet, Call, Storage, Config<T>},
			Indracores: indras::{Pallet, Call, Storage, Config, Event},
			IndrasShared: shared::{Pallet, Call, Storage},
			Registrar: indras_registrar::{Pallet, Call, Storage, Event<T>},
			IndracoresOrigin: origin::{Pallet, Origin},
		}
	);

	impl<C> frame_system::offchain::SendTransactionTypes<C> for Test
	where
		Call: From<C>,
	{
		type Extrinsic = UncheckedExtrinsic;
		type OverarchingCall = Call;
	}

	const NORMAL_RATIO: Perbill = Perbill::from_percent(75);
	parameter_types! {
		pub const BlockHashCount: u32 = 250;
		pub BlockWeights: limits::BlockWeights =
			frame_system::limits::BlockWeights::simple_max(1024);
		pub BlockLength: limits::BlockLength =
			limits::BlockLength::max_with_normal_ratio(4 * 1024 * 1024, NORMAL_RATIO);
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type Origin = Origin;
		type Call = Call;
		type Index = u64;
		type BlockNumber = BlockNumber;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<u64>;
		type Header = Header;
		type Event = Event;
		type BlockHashCount = BlockHashCount;
		type DbWeight = ();
		type BlockWeights = BlockWeights;
		type BlockLength = BlockLength;
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u128>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	parameter_types! {
		pub const ExistentialDeposit: Balance = 1;
	}

	impl pallet_balances::Config for Test {
		type Balance = u128;
		type DustRemoval = ();
		type Event = Event;
		type ExistentialDeposit = ExistentialDeposit;
		type AccountStore = System;
		type MaxLocks = ();
		type MaxReserves = ();
		type ReserveIdentifier = [u8; 8];
		type WeightInfo = ();
	}

	impl shared::Config for Test {}

	impl origin::Config for Test {}

	parameter_types! {
		pub const IndrasUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
	}

	impl indras::Config for Test {
		type Event = Event;
		type WeightInfo = indras::TestWeightInfo;
		type UnsignedPriority = IndrasUnsignedPriority;
		type NextSessionRotation = crate::mock::TestNextSessionRotation;
	}

	impl configuration::Config for Test {
		type WeightInfo = configuration::TestWeightInfo;
	}

	parameter_types! {
		pub const IndraDeposit: Balance = 10;
		pub const DataDepositPerByte: Balance = 1;
		pub const MaxRetries: u32 = 3;
	}

	impl Config for Test {
		type Event = Event;
		type Origin = Origin;
		type Currency = Balances;
		type OnSwap = MockSwap;
		type IndraDeposit = IndraDeposit;
		type DataDepositPerByte = DataDepositPerByte;
		type WeightInfo = TestWeightInfo;
	}

	pub fn new_test_ext() -> TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		GenesisBuild::<Test>::assimilate_storage(
			&configuration::GenesisConfig {
				config: configuration::HostConfiguration {
					max_code_size: 2 * 1024 * 1024,      // 2 MB
					max_head_data_size: 1 * 1024 * 1024, // 1 MB
					..Default::default()
				},
			},
			&mut t,
		)
		.unwrap();

		pallet_balances::GenesisConfig::<Test> { balances: vec![(1, 10_000_000), (2, 10_000_000)] }
			.assimilate_storage(&mut t)
			.unwrap();

		t.into()
	}

	parameter_types! {
		pub static SwapData: BTreeMap<IndraId, u64> = BTreeMap::new();
	}

	pub struct MockSwap;
	impl OnSwap for MockSwap {
		fn on_swap(one: IndraId, other: IndraId) {
			let mut swap_data = SwapData::get();
			let one_data = swap_data.remove(&one).unwrap_or_default();
			let other_data = swap_data.remove(&other).unwrap_or_default();
			swap_data.insert(one, other_data);
			swap_data.insert(other, one_data);
			SwapData::set(swap_data);
		}
	}

	const BLOCKS_PER_SESSION: u32 = 3;

	fn run_to_block(n: BlockNumber) {
		// NOTE that this function only simulates modules of interest. Depending on new pallet may
		// require adding it here.
		assert!(System::block_number() < n);
		while System::block_number() < n {
			let b = System::block_number();

			if System::block_number() > 1 {
				System::on_finalize(System::block_number());
			}
			// Session change every 3 blocks.
			if (b + 1) % BLOCKS_PER_SESSION == 0 {
				shared::Pallet::<Test>::set_session_index(
					shared::Pallet::<Test>::session_index() + 1,
				);
				Indracores::test_on_new_session();
			}
			System::set_block_number(b + 1);
			System::on_initialize(System::block_number());
		}
	}

	fn run_to_session(n: BlockNumber) {
		let block_number = n * BLOCKS_PER_SESSION;
		run_to_block(block_number);
	}

	fn test_genesis_head(size: usize) -> HeadData {
		HeadData(vec![0u8; size])
	}

	fn test_validation_code(size: usize) -> ValidationCode {
		let validation_code = vec![0u8; size as usize];
		ValidationCode(validation_code)
	}

	fn indra_origin(id: IndraId) -> Origin {
		runtime_indracores::Origin::Indracore(id).into()
	}

	fn max_code_size() -> u32 {
		Configuration::config().max_code_size
	}

	fn max_head_size() -> u32 {
		Configuration::config().max_head_data_size
	}

	#[test]
	fn basic_setup_works() {
		new_test_ext().execute_with(|| {
			assert_eq!(PendingSwap::<Test>::get(&IndraId::from(0u32)), None);
			assert_eq!(Indras::<Test>::get(&IndraId::from(0u32)), None);
		});
	}

	#[test]
	fn end_to_end_scenario_works() {
		new_test_ext().execute_with(|| {
			let indra_id = LOWEST_PUBLIC_ID;
			run_to_block(1);
			// first indra is not yet registered
			assert!(!Indracores::is_indrabase(indra_id));
			// We register the Indra ID
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				test_genesis_head(32),
				test_validation_code(32),
			));
			run_to_session(2);
			// It is now a indrabase.
			assert!(Indracores::is_indrabase(indra_id));
			assert!(!Indracores::is_indracore(indra_id));
			// Some other external process will elevate indrabase to indracore
			assert_ok!(Registrar::make_indracore(indra_id));
			run_to_session(4);
			// It is now a indracore.
			assert!(!Indracores::is_indrabase(indra_id));
			assert!(Indracores::is_indracore(indra_id));
			// Turn it back into a indrabase
			assert_ok!(Registrar::make_indrabase(indra_id));
			run_to_session(6);
			assert!(Indracores::is_indrabase(indra_id));
			assert!(!Indracores::is_indracore(indra_id));
			// Deregister it
			assert_ok!(Registrar::deregister(Origin::root(), indra_id,));
			run_to_session(8);
			// It is nothing
			assert!(!Indracores::is_indrabase(indra_id));
			assert!(!Indracores::is_indracore(indra_id));
		});
	}

	#[test]
	fn register_works() {
		new_test_ext().execute_with(|| {
			run_to_block(1);
			let indra_id = LOWEST_PUBLIC_ID;
			assert!(!Indracores::is_indrabase(indra_id));
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_eq!(Balances::reserved_balance(&1), <Test as Config>::IndraDeposit::get());
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				test_genesis_head(32),
				test_validation_code(32),
			));
			run_to_session(2);
			assert!(Indracores::is_indrabase(indra_id));
			assert_eq!(
				Balances::reserved_balance(&1),
				<Test as Config>::IndraDeposit::get() +
					64 * <Test as Config>::DataDepositPerByte::get()
			);
		});
	}

	#[test]
	fn register_handles_basic_errors() {
		new_test_ext().execute_with(|| {
			let indra_id = LOWEST_PUBLIC_ID;

			assert_noop!(
				Registrar::register(
					Origin::signed(1),
					indra_id,
					test_genesis_head(max_head_size() as usize),
					test_validation_code(max_code_size() as usize),
				),
				Error::<Test>::NotReserved
			);

			// Successfully register indra
			assert_ok!(Registrar::reserve(Origin::signed(1)));

			assert_noop!(
				Registrar::register(
					Origin::signed(2),
					indra_id,
					test_genesis_head(max_head_size() as usize),
					test_validation_code(max_code_size() as usize),
				),
				Error::<Test>::NotOwner
			);

			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				test_genesis_head(max_head_size() as usize),
				test_validation_code(max_code_size() as usize),
			));

			run_to_session(2);

			assert_ok!(Registrar::deregister(Origin::root(), indra_id));

			// Can't do it again
			assert_noop!(
				Registrar::register(
					Origin::signed(1),
					indra_id,
					test_genesis_head(max_head_size() as usize),
					test_validation_code(max_code_size() as usize),
				),
				Error::<Test>::NotReserved
			);

			// Head Size Check
			assert_ok!(Registrar::reserve(Origin::signed(2)));
			assert_noop!(
				Registrar::register(
					Origin::signed(2),
					indra_id + 1,
					test_genesis_head((max_head_size() + 1) as usize),
					test_validation_code(max_code_size() as usize),
				),
				Error::<Test>::HeadDataTooLarge
			);

			// Code Size Check
			assert_noop!(
				Registrar::register(
					Origin::signed(2),
					indra_id + 1,
					test_genesis_head(max_head_size() as usize),
					test_validation_code((max_code_size() + 1) as usize),
				),
				Error::<Test>::CodeTooLarge
			);

			// Needs enough funds for deposit
			assert_noop!(
				Registrar::reserve(Origin::signed(1337)),
				BalancesError::<Test, _>::InsufficientBalance
			);
		});
	}

	#[test]
	fn deregister_works() {
		new_test_ext().execute_with(|| {
			run_to_block(1);
			let indra_id = LOWEST_PUBLIC_ID;
			assert!(!Indracores::is_indrabase(indra_id));
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				test_genesis_head(32),
				test_validation_code(32),
			));
			run_to_session(2);
			assert!(Indracores::is_indrabase(indra_id));
			assert_ok!(Registrar::deregister(Origin::root(), indra_id,));
			run_to_session(4);
			assert!(indras::Pallet::<Test>::lifecycle(indra_id).is_none());
			assert_eq!(Balances::reserved_balance(&1), 0);
		});
	}

	#[test]
	fn deregister_handles_basic_errors() {
		new_test_ext().execute_with(|| {
			run_to_block(1);
			let indra_id = LOWEST_PUBLIC_ID;
			assert!(!Indracores::is_indrabase(indra_id));
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				test_genesis_head(32),
				test_validation_code(32),
			));
			run_to_session(2);
			assert!(Indracores::is_indrabase(indra_id));
			// Owner check
			assert_noop!(Registrar::deregister(Origin::signed(2), indra_id,), BadOrigin);
			assert_ok!(Registrar::make_indracore(indra_id));
			run_to_session(4);
			// Cant directly deregister indracore
			assert_noop!(
				Registrar::deregister(Origin::root(), indra_id,),
				Error::<Test>::NotIndrabase
			);
		});
	}

	#[test]
	fn swap_works() {
		new_test_ext().execute_with(|| {
			// Successfully register first two indracores
			let indra_1 = LOWEST_PUBLIC_ID;
			let indra_2 = LOWEST_PUBLIC_ID + 1;
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_1,
				test_genesis_head(max_head_size() as usize),
				test_validation_code(max_code_size() as usize),
			));
			assert_ok!(Registrar::reserve(Origin::signed(2)));
			assert_ok!(Registrar::register(
				Origin::signed(2),
				indra_2,
				test_genesis_head(max_head_size() as usize),
				test_validation_code(max_code_size() as usize),
			));
			run_to_session(2);

			// Upgrade indra 1 into a indracore
			assert_ok!(Registrar::make_indracore(indra_1));

			// Set some mock swap data.
			let mut swap_data = SwapData::get();
			swap_data.insert(indra_1, 69);
			swap_data.insert(indra_2, 1337);
			SwapData::set(swap_data);

			run_to_session(4);

			// Roles are as we expect
			assert!(Indracores::is_indracore(indra_1));
			assert!(!Indracores::is_indrabase(indra_1));
			assert!(!Indracores::is_indracore(indra_2));
			assert!(Indracores::is_indrabase(indra_2));

			// Both indras initiate a swap
			assert_ok!(Registrar::swap(indra_origin(indra_1), indra_1, indra_2,));
			assert_ok!(Registrar::swap(indra_origin(indra_2), indra_2, indra_1,));

			run_to_session(6);

			// Roles are swapped
			assert!(!Indracores::is_indracore(indra_1));
			assert!(Indracores::is_indrabase(indra_1));
			assert!(Indracores::is_indracore(indra_2));
			assert!(!Indracores::is_indrabase(indra_2));

			// Data is swapped
			assert_eq!(SwapData::get().get(&indra_1).unwrap(), &1337);
			assert_eq!(SwapData::get().get(&indra_2).unwrap(), &69);
		});
	}

	#[test]
	fn indra_lock_works() {
		new_test_ext().execute_with(|| {
			run_to_block(1);

			assert_ok!(Registrar::reserve(Origin::signed(1)));
			let indra_id = LOWEST_PUBLIC_ID;
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_id,
				vec![1; 3].into(),
				vec![1, 2, 3].into(),
			));

			// Owner can call swap
			assert_ok!(Registrar::swap(Origin::signed(1), indra_id, indra_id + 1));

			// 2 session changes to fully onboard.
			run_to_session(2);
			assert_eq!(Indracores::lifecycle(indra_id), Some(IndraLifecycle::Indrabase));

			// Once they begin onboarding, we lock them in.
			assert_ok!(Registrar::make_indracore(indra_id));

			// Owner cannot call swap anymore
			assert_noop!(Registrar::swap(Origin::signed(1), indra_id, indra_id + 2), BadOrigin);
		});
	}

	#[test]
	fn swap_handles_bad_states() {
		new_test_ext().execute_with(|| {
			let indra_1 = LOWEST_PUBLIC_ID;
			let indra_2 = LOWEST_PUBLIC_ID + 1;
			run_to_block(1);
			// indras are not yet registered
			assert!(!Indracores::is_indrabase(indra_1));
			assert!(!Indracores::is_indrabase(indra_2));

			// Cannot even start a swap
			assert_noop!(
				Registrar::swap(Origin::root(), indra_1, indra_2),
				Error::<Test>::NotRegistered
			);

			// We register Indras 1 and 2
			assert_ok!(Registrar::reserve(Origin::signed(1)));
			assert_ok!(Registrar::reserve(Origin::signed(2)));
			assert_ok!(Registrar::register(
				Origin::signed(1),
				indra_1,
				test_genesis_head(32),
				test_validation_code(32),
			));
			assert_ok!(Registrar::register(
				Origin::signed(2),
				indra_2,
				test_genesis_head(32),
				test_validation_code(32),
			));

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			run_to_session(2);

			// They are now a indrabase.
			assert!(Indracores::is_indrabase(indra_1));
			assert!(Indracores::is_indrabase(indra_2));

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			// Some other external process will elevate one indrabase to indracore
			assert_ok!(Registrar::make_indracore(indra_1));

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			run_to_session(3);

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			run_to_session(4);

			// It is now a indracore.
			assert!(Indracores::is_indracore(indra_1));
			assert!(Indracores::is_indrabase(indra_2));

			// Swap works here.
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_ok!(Registrar::swap(Origin::root(), indra_2, indra_1));

			run_to_session(5);

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			run_to_session(6);

			// Swap worked!
			assert!(Indracores::is_indracore(indra_2));
			assert!(Indracores::is_indrabase(indra_1));

			// Something starts to downgrade a indra
			assert_ok!(Registrar::make_indrabase(indra_2));

			run_to_session(7);

			// Cannot swap
			assert_ok!(Registrar::swap(Origin::root(), indra_1, indra_2));
			assert_noop!(
				Registrar::swap(Origin::root(), indra_2, indra_1),
				Error::<Test>::CannotSwap
			);

			run_to_session(8);

			assert!(Indracores::is_indrabase(indra_1));
			assert!(Indracores::is_indrabase(indra_2));
		});
	}
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
	use super::{Pallet as Registrar, *};
	use crate::traits::Registrar as RegistrarT;
	use frame_support::assert_ok;
	use frame_system::RawOrigin;
	use runtime_indracores::{indras, shared, Origin as IndraOrigin};
	use sp_runtime::traits::Bounded;

	use frame_benchmarking::{account, benchmarks, whitelisted_caller};

	fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
		let events = frame_system::Pallet::<T>::events();
		let system_event: <T as frame_system::Config>::Event = generic_event.into();
		// compare to the last event record
		let frame_system::EventRecord { event, .. } = &events[events.len() - 1];
		assert_eq!(event, &system_event);
	}

	fn register_indra<T: Config>(id: u32) -> IndraId {
		let indra = IndraId::from(id);
		let genesis_head = Registrar::<T>::worst_head_data();
		let validation_code = Registrar::<T>::worst_validation_code();
		let caller: T::AccountId = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		assert_ok!(Registrar::<T>::reserve(RawOrigin::Signed(caller.clone()).into()));
		assert_ok!(Registrar::<T>::register(
			RawOrigin::Signed(caller).into(),
			indra,
			genesis_head,
			validation_code
		));
		return indra
	}

	fn indra_origin(id: u32) -> IndraOrigin {
		IndraOrigin::Indracore(id.into())
	}

	// This function moves forward to the next scheduled session for indracore lifecycle upgrades.
	fn next_scheduled_session<T: Config>() {
		shared::Pallet::<T>::set_session_index(shared::Pallet::<T>::scheduled_session());
		indras::Pallet::<T>::test_on_new_session();
	}

	benchmarks! {
		where_clause { where IndraOrigin: Into<<T as frame_system::Config>::Origin> }

		reserve {
			let caller: T::AccountId = whitelisted_caller();
			T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
		}: _(RawOrigin::Signed(caller.clone()))
		verify {
			assert_last_event::<T>(Event::<T>::Reserved { indra_id: LOWEST_PUBLIC_ID, who: caller }.into());
			assert!(Indras::<T>::get(LOWEST_PUBLIC_ID).is_some());
			assert_eq!(indras::Pallet::<T>::lifecycle(LOWEST_PUBLIC_ID), None);
		}

		register {
			let indra = LOWEST_PUBLIC_ID;
			let genesis_head = Registrar::<T>::worst_head_data();
			let validation_code = Registrar::<T>::worst_validation_code();
			let caller: T::AccountId = whitelisted_caller();
			T::Currency::make_free_balance_be(&caller, BalanceOf::<T>::max_value());
			assert_ok!(Registrar::<T>::reserve(RawOrigin::Signed(caller.clone()).into()));
		}: _(RawOrigin::Signed(caller.clone()), indra, genesis_head, validation_code)
		verify {
			assert_last_event::<T>(Event::<T>::Registered{ indra_id: indra, manager: caller }.into());
			assert_eq!(indras::Pallet::<T>::lifecycle(indra), Some(IndraLifecycle::Onboarding));
			next_scheduled_session::<T>();
			assert_eq!(indras::Pallet::<T>::lifecycle(indra), Some(IndraLifecycle::Indrabase));
		}

		force_register {
			let manager: T::AccountId = account("manager", 0, 0);
			let deposit = 0u32.into();
			let indra = IndraId::from(69);
			let genesis_head = Registrar::<T>::worst_head_data();
			let validation_code = Registrar::<T>::worst_validation_code();
		}: _(RawOrigin::Root, manager.clone(), deposit, indra, genesis_head, validation_code)
		verify {
			assert_last_event::<T>(Event::<T>::Registered { indra_id: indra, manager }.into());
			assert_eq!(indras::Pallet::<T>::lifecycle(indra), Some(IndraLifecycle::Onboarding));
			next_scheduled_session::<T>();
			assert_eq!(indras::Pallet::<T>::lifecycle(indra), Some(IndraLifecycle::Indrabase));
		}

		deregister {
			let indra = register_indra::<T>(LOWEST_PUBLIC_ID.into());
			next_scheduled_session::<T>();
			let caller: T::AccountId = whitelisted_caller();
		}: _(RawOrigin::Signed(caller), indra)
		verify {
			assert_last_event::<T>(Event::<T>::Deregistered { indra_id: indra }.into());
		}

		swap {
			let indrabase = register_indra::<T>(LOWEST_PUBLIC_ID.into());
			let indracore = register_indra::<T>((LOWEST_PUBLIC_ID + 1).into());

			let indracore_origin = indra_origin(indracore.into());

			// Actually finish registration process
			next_scheduled_session::<T>();

			// Upgrade the indracore
			Registrar::<T>::make_indracore(indracore)?;
			next_scheduled_session::<T>();

			assert_eq!(indras::Pallet::<T>::lifecycle(indracore), Some(IndraLifecycle::Indracore));
			assert_eq!(indras::Pallet::<T>::lifecycle(indrabase), Some(IndraLifecycle::Indrabase));

			let caller: T::AccountId = whitelisted_caller();
			Registrar::<T>::swap(indracore_origin.into(), indracore, indrabase)?;
		}: _(RawOrigin::Signed(caller.clone()), indrabase, indracore)
		verify {
			next_scheduled_session::<T>();
			// Swapped!
			assert_eq!(indras::Pallet::<T>::lifecycle(indracore), Some(IndraLifecycle::Indrabase));
			assert_eq!(indras::Pallet::<T>::lifecycle(indrabase), Some(IndraLifecycle::Indracore));
		}

		impl_benchmark_test_suite!(
			Registrar,
			crate::integration_tests::new_test_ext(),
			crate::integration_tests::Test,
		);
	}
}
