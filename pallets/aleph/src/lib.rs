#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod impls;
pub mod traits;

use frame_support::{
    sp_runtime::BoundToRuntimeAppPublic,
    traits::{OneSessionHandler, StorageVersion},
};
pub use pallet::*;
use primitives::{
    crypto::{AuthorityVerifier, SignatureSet},
    Balance, SessionIndex, Version, VersionChange, DEFAULT_FINALITY_VERSION,
    LEGACY_FINALITY_VERSION, TOKEN,
};
use sp_runtime::Perbill;
use sp_std::prelude::*;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);
pub(crate) const LOG_TARGET: &str = "pallet-aleph";

#[frame_support::pallet]
#[pallet_doc("../README.md")]
pub mod pallet {
    use frame_support::{
        dispatch::{DispatchResult, DispatchResultWithPostInfo, Pays},
        pallet_prelude::{TransactionSource, TransactionValidityError, ValueQuery, *},
        sp_runtime::RuntimeAppPublic,
    };
    use frame_system::{
        ensure_none, ensure_root,
        pallet_prelude::{BlockNumberFor, OriginFor},
    };
    use pallet_session::SessionManager;
    use primitives::{Score, ScoreNonce, SessionInfoProvider, TotalIssuanceProvider};
    use sp_runtime::traits::{Hash, ValidateUnsigned};
    use sp_std::collections::btree_map::BTreeMap;
    #[cfg(feature = "std")]
    use sp_std::marker::PhantomData;

    use super::*;
    use crate::traits::NextSessionAuthorityProvider;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + frame_system::offchain::SendTransactionTypes<Call<Self>>
    {
        type AuthorityId: Member + Parameter + RuntimeAppPublic + MaybeSerializeDeserialize;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type SessionInfoProvider: SessionInfoProvider<BlockNumberFor<Self>>;
        type SessionManager: SessionManager<<Self as frame_system::Config>::AccountId>;
        type NextSessionAuthorityProvider: NextSessionAuthorityProvider<Self>;
        type TotalIssuanceProvider: TotalIssuanceProvider;
        #[pallet::constant]
        type ScoreSubmissionPeriod: Get<u32>;
    }

    pub type Signature<T> = <<T as Config>::AuthorityId as RuntimeAppPublic>::Signature;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ChangeEmergencyFinalizer(T::AuthorityId),
        ScheduleFinalityVersionChange(VersionChange),
        FinalityVersionChange(VersionChange),
        InflationParametersChange(Balance, u64),
    }

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// Default finality version. Relevant for sessions before the first version change occurs.
    #[pallet::type_value]
    pub(crate) fn DefaultFinalityVersion() -> Version {
        DEFAULT_FINALITY_VERSION
    }

    /// Default SEL Cap. Relevant for eras before we set this value by hand.
    #[pallet::type_value]
    pub fn DefaultSelCap() -> Balance {
        320_000_000 * TOKEN
    }

    /// Default length of the exponential inflation horizon.
    /// Relevant for eras before we set this value by hand.
    #[pallet::type_value]
    pub fn DefaultExponentialInflationHorizon() -> u64 {
        154_283_512_497
    }

    #[pallet::storage]
    pub type SelCap<T: Config> = StorageValue<_, Balance, ValueQuery, DefaultSelCap>;

    #[pallet::storage]
    pub type ExponentialInflationHorizon<T: Config> =
        StorageValue<_, u64, ValueQuery, DefaultExponentialInflationHorizon>;

    #[pallet::storage]
    #[pallet::getter(fn authorities)]
    pub type Authorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn next_authorities)]
    pub(super) type NextAuthorities<T: Config> = StorageValue<_, Vec<T::AuthorityId>, ValueQuery>;

    /// Set of account ids that will be used as authorities in the next session
    #[pallet::storage]
    pub type NextFinalityCommittee<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn emergency_finalizer)]
    pub(super) type EmergencyFinalizer<T: Config> = StorageValue<_, T::AuthorityId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn queued_emergency_finalizer)]
    pub(super) type QueuedEmergencyFinalizer<T: Config> =
        StorageValue<_, T::AuthorityId, OptionQuery>;

    #[pallet::storage]
    type NextEmergencyFinalizer<T: Config> = StorageValue<_, T::AuthorityId, OptionQuery>;

    /// Current finality version.
    #[pallet::storage]
    #[pallet::getter(fn finality_version)]
    pub(super) type FinalityVersion<T: Config> =
        StorageValue<_, Version, ValueQuery, DefaultFinalityVersion>;

    /// Scheduled finality version change.
    #[pallet::storage]
    #[pallet::getter(fn finality_version_change)]
    pub(super) type FinalityScheduledVersionChange<T: Config> =
        StorageValue<_, VersionChange, OptionQuery>;

    // clear this storage on session end
    #[pallet::storage]
    #[pallet::getter(fn abft_scores)]
    pub type AbftScores<T: Config> = StorageMap<_, Twox64Concat, SessionIndex, Score>;

    #[pallet::storage]
    #[pallet::getter(fn last_score_nonce)]
    pub(super) type LastScoreNonce<T: Config> = StorageValue<_, ScoreNonce, ValueQuery>;

    impl<T: Config> Pallet<T> {
        pub(crate) fn initialize_authorities(
            authorities: &[T::AuthorityId],
            next_authorities: &[T::AuthorityId],
        ) {
            if !authorities.is_empty() {
                if !<Authorities<T>>::get().is_empty() {
                    log::error!(target: LOG_TARGET, "Authorities are already initialized!");
                } else {
                    <Authorities<T>>::put(authorities);
                }
            }
            if !next_authorities.is_empty() {
                // Storage NextAuthorities has default value so should never be empty.
                <NextAuthorities<T>>::put(next_authorities);
            }
        }

        fn get_authorities_for_next_session(
            next_authorities: Vec<(&T::AccountId, T::AuthorityId)>,
        ) -> Vec<T::AuthorityId> {
            let mut account_to_authority: BTreeMap<_, _> = next_authorities.into_iter().collect();
            let next_committee_accounts = NextFinalityCommittee::<T>::get();
            let expected_len = next_committee_accounts.len();
            let next_committee_authorities: Vec<_> = next_committee_accounts
                .into_iter()
                .filter_map(|account_id| account_to_authority.remove(&account_id))
                .collect();

            if next_committee_authorities.len() != expected_len {
                log::error!(
                    target: LOG_TARGET,
                    "Not all committee members were converted to keys."
                );
            }

            next_committee_authorities
        }

        pub(crate) fn update_authorities(next_authorities: Vec<(&T::AccountId, T::AuthorityId)>) {
            let next_authorities = Self::get_authorities_for_next_session(next_authorities);

            <Authorities<T>>::put(<NextAuthorities<T>>::get());
            <NextAuthorities<T>>::put(next_authorities);
        }

        pub(crate) fn update_emergency_finalizer() {
            if let Some(emergency_finalizer) = <QueuedEmergencyFinalizer<T>>::get() {
                <EmergencyFinalizer<T>>::put(emergency_finalizer)
            }

            if let Some(emergency_finalizer) = <NextEmergencyFinalizer<T>>::get() {
                <QueuedEmergencyFinalizer<T>>::put(emergency_finalizer)
            }
        }

        pub(crate) fn set_next_emergency_finalizer(emergency_finalizer: T::AuthorityId) {
            <NextEmergencyFinalizer<T>>::put(emergency_finalizer);
        }

        pub(crate) fn current_session() -> u32 {
            T::SessionInfoProvider::current_session()
        }

        // If a scheduled future version change is rescheduled to a different session,
        // it is possible to reschedule it with the same version as initially.
        // To cancel a future version change, reschedule it with the current version.
        // If a scheduled version change has moved into the past, `SessionManager` records it
        // as the current version.
        pub(crate) fn do_schedule_finality_version_change(
            version_change: VersionChange,
        ) -> Result<(), &'static str> {
            let current_session = Self::current_session();

            let session_to_schedule = version_change.session;

            if session_to_schedule < current_session {
                return Err("Cannot schedule finality version changes for sessions in the past!");
            } else if session_to_schedule < current_session + 2 {
                return Err(
                    "Tried to schedule an finality version change less than 2 sessions in advance!",
                );
            }

            // Update the scheduled version change with the supplied version change.
            <FinalityScheduledVersionChange<T>>::put(version_change);

            Ok(())
        }

        pub fn next_session_finality_version() -> Version {
            let next_session = Self::current_session() + 1;
            let scheduled_version_change = Self::finality_version_change();

            if let Some(version_change) = scheduled_version_change {
                if next_session == version_change.session {
                    return version_change.version_incoming;
                }
            }

            Self::finality_version()
        }

        pub fn check_horizon_upper_bound(
            new_horizon: u64,
            current_horizon: u64,
        ) -> Result<(), &'static str> {
            match new_horizon > current_horizon.saturating_mul(2).saturating_add(1) {
                true => {
                    Err("Horizon too large, should be at most twice the current value plus one!")
                }
                false => Ok(()),
            }
        }

        pub fn check_horizon_lower_bound(
            new_horizon: u64,
            current_horizon: u64,
        ) -> Result<(), &'static str> {
            match new_horizon < current_horizon / 2 {
                true => Err("Horizon too small, should be at least half the current value!"),
                false => Ok(()),
            }
        }

        pub fn check_sel_cap_upper_bound(
            new_cap: Balance,
            current_cap: Balance,
            total_issuance: Balance,
        ) -> Result<(), &'static str> {
            let current_gap = current_cap.saturating_sub(total_issuance);
            let new_gap = match new_cap.checked_sub(total_issuance) {
                Some(new_gap) => new_gap,
                None => return Err("SEL Cap cannot be lower than the current total issuance!"),
            };
            match (new_gap > current_gap.saturating_mul(2).saturating_add(1))
                && (new_gap > total_issuance / 128)
            {
                true => Err("Future issuance too large, should be at most the current total issuance divided by 128, or at most twice the current value plus one!"),
                false => Ok(()),
            }
        }

        pub fn check_sel_cap_lower_bound(
            new_cap: Balance,
            current_cap: Balance,
            total_issuance: Balance,
        ) -> Result<(), &'static str> {
            let current_gap = current_cap.saturating_sub(total_issuance);
            let new_gap = match new_cap.checked_sub(total_issuance) {
                Some(new_gap) => new_gap,
                None => return Err("SEL Cap cannot be lower than the current total issuance!"),
            };
            match new_gap < current_gap / 2 {
                true => {
                    Err("Future issuance too small, should be at least half the current value!")
                }
                false => Ok(()),
            }
        }

        fn check_session_id(session_id: SessionIndex) -> Result<(), TransactionValidityError> {
            let current_session_id = Self::current_session();
            if current_session_id < session_id {
                return Err(InvalidTransaction::Future.into());
            }
            if current_session_id > session_id {
                return Err(InvalidTransaction::Stale.into());
            }

            Ok(())
        }

        fn check_nonce(nonce: ScoreNonce) -> Result<(), TransactionValidityError> {
            let last_nonce = Self::last_score_nonce();
            if nonce <= last_nonce {
                return Err(InvalidTransaction::Stale.into());
            }

            Ok(())
        }

        fn check_score(
            score: &Score,
            signature: &SignatureSet<Signature<T>>,
        ) -> Result<(), TransactionValidityError> {
            Self::check_session_id(score.session_id)?;
            Self::check_nonce(score.nonce)?;
            Self::verify_score(score, signature)?;

            Ok(())
        }

        pub fn verify_score(
            score: &Score,
            signature: &SignatureSet<Signature<T>>,
        ) -> Result<(), TransactionValidityError> {
            let msg = T::Hashing::hash_of(&score.encode()).encode();
            let authority_verifier = AuthorityVerifier::new(Self::authorities());
            if !AuthorityVerifier::is_complete(&authority_verifier, &msg, signature) {
                return Err(InvalidTransaction::BadProof.into());
            }
            Ok(())
        }

        pub fn submit_abft_score(
            score: Score,
            signature: SignatureSet<Signature<T>>,
        ) -> Option<()> {
            use frame_system::offchain::SubmitTransaction;

            let call = Call::unsigned_submit_abft_score { score, signature };
            SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()).ok()
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Sets the emergency finalization key. If called in session `N` the key can be used to
        /// finalize blocks from session `N+2` onwards, until it gets overridden.
        #[pallet::call_index(0)]
        #[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
        pub fn set_emergency_finalizer(
            origin: OriginFor<T>,
            emergency_finalizer: T::AuthorityId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            Self::set_next_emergency_finalizer(emergency_finalizer.clone());
            Self::deposit_event(Event::ChangeEmergencyFinalizer(emergency_finalizer));
            Ok(())
        }

        /// Schedules a finality version change for a future session. If such a scheduled future
        /// version is already set, it is replaced with the provided one.
        /// Any rescheduling of a future version change needs to occur at least 2 sessions in
        /// advance of the provided session of the version change.
        /// In order to cancel a scheduled version change, a new version change should be scheduled
        /// with the same version as the current one.
        #[pallet::call_index(1)]
        #[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
        pub fn schedule_finality_version_change(
            origin: OriginFor<T>,
            version_incoming: Version,
            session: SessionIndex,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let version_change = VersionChange {
                version_incoming,
                session,
            };

            if let Err(e) = Self::do_schedule_finality_version_change(version_change.clone()) {
                return Err(DispatchError::Other(e));
            }

            Self::deposit_event(Event::ScheduleFinalityVersionChange(version_change));
            Ok(())
        }

        /// Sets the values of inflation parameters.
        #[pallet::call_index(2)]
        #[pallet::weight((T::BlockWeights::get().max_block, DispatchClass::Operational))]
        pub fn set_inflation_parameters(
            origin: OriginFor<T>,
            sel_cap: Option<Balance>,
            horizon_millisecs: Option<u64>,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let current_sel_cap = SelCap::<T>::get();
            let current_horizon_millisecs = ExponentialInflationHorizon::<T>::get();
            let total_issuance = T::TotalIssuanceProvider::get();

            let sel_cap = sel_cap.unwrap_or(current_sel_cap);
            let horizon_millisecs = horizon_millisecs.unwrap_or(current_horizon_millisecs);

            Self::check_horizon_lower_bound(horizon_millisecs, current_horizon_millisecs)
                .map_err(DispatchError::Other)?;
            Self::check_horizon_upper_bound(horizon_millisecs, current_horizon_millisecs)
                .map_err(DispatchError::Other)?;
            Self::check_sel_cap_upper_bound(sel_cap, current_sel_cap, total_issuance)
                .map_err(DispatchError::Other)?;
            Self::check_sel_cap_lower_bound(sel_cap, current_sel_cap, total_issuance)
                .map_err(DispatchError::Other)?;

            SelCap::<T>::put(sel_cap);
            ExponentialInflationHorizon::<T>::put(horizon_millisecs);

            Self::deposit_event(Event::InflationParametersChange(
                sel_cap,
                horizon_millisecs,
            ));

            Ok(())
        }

        // fix weight, take into account validate_unsigned
        #[pallet::call_index(3)]
        #[pallet::weight(T::BlockWeights::get().max_block * Perbill::from_percent(10))]
        /// Stores abft score
        pub fn unsigned_submit_abft_score(
            origin: OriginFor<T>,
            score: Score,
            _signature: SignatureSet<Signature<T>>, // We don't check signature as it was checked by ValidateUnsigned trait
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            <LastScoreNonce<T>>::put(score.nonce);
            AbftScores::<T>::insert(score.session_id, score);

            Ok(Pays::No.into())
        }
    }

    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            if let Call::unsigned_submit_abft_score { score, signature } = call {
                Self::check_score(score, signature)?;
                ValidTransaction::with_tag_prefix("AbftScore")
                    .priority(score.nonce as u64) // this ensures that later nonces are first in tx queue
                    .longevity(TransactionLongevity::MAX) // consider restricting longevity
                    .propagate(true)
                    .build()
            } else {
                InvalidTransaction::Call.into()
            }
        }
    }

    impl<T: Config> BoundToRuntimeAppPublic for Pallet<T> {
        type Public = T::AuthorityId;
    }

    impl<T: Config> OneSessionHandler<T::AccountId> for Pallet<T> {
        type Key = T::AuthorityId;

        fn on_genesis_session<'a, I>(validators: I)
        where
            I: 'a + Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
            T::AccountId: 'a,
        {
            let (_, authorities): (Vec<_>, Vec<_>) = validators.unzip();
            // it is guaranteed that the first validator set will also be used in the next session
            Self::initialize_authorities(authorities.as_slice(), authorities.as_slice());
        }

        fn on_new_session<'a, I>(changed: bool, _: I, queued_validators: I)
        where
            I: 'a + Iterator<Item = (&'a T::AccountId, T::AuthorityId)>,
            T::AccountId: 'a,
        {
            Self::update_emergency_finalizer();
            if changed {
                Self::update_authorities(queued_validators.collect());
            }
        }

        fn on_disabled(_validator_index: u32) {}
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub finality_version: Version,
        pub _marker: PhantomData<T>,
    }

    impl<T: Config> core::default::Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                finality_version: LEGACY_FINALITY_VERSION as u32,
                _marker: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            <FinalityVersion<T>>::put(self.finality_version);
        }
    }
}
