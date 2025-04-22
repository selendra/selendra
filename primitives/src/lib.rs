// Copyright 2022 Smallworld Selendra
// This file is part of Selendra.

// Selendra is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Selendra is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Selendra.  If not, see <http://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

//! Core Selendra types.

pub mod common;
pub mod constants;
pub mod impls;

pub use common::*;
pub use constants::*;

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::KeyTypeId;
pub use sp_runtime::{
	generic,
	traits::{BlakeTwo256, ConstU32, Header as HeaderT},
	BoundedVec, ConsensusEngineId, OpaqueExtrinsic as UncheckedExtrinsic, Perbill,
};
use sp_runtime::{
	impl_opaque_keys,
	traits::{IdentifyAccount, Verify},
	MultiSignature, Perquintill,
};
pub use sp_staking::{EraIndex, SessionIndex};
use sp_std::vec::Vec;

/// The block number type used by Selendra.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

// /// Alias to type for a signature for a transaction on the relay chain. This allows one of several
// /// kinds of underlying crypto to be used, so isn't a fixed size when encoded.
// pub type Signature = MultiSignature;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
// pub type Signature = fp_account::EthereumSignature;
pub type Signature = MultiSignature;

/// Alias to the public key used for this chain, actually a `MultiSigner`. Like the signature, this
/// also isn't a fixed size when encoded, as different cryptos have different size public keys.
pub type AccountPublic = <Signature as Verify>::Signer;

/// Alias to the opaque account ID type for this chain, actually a `AccountId32`. This is always
/// 32 bytes.
pub type AccountId = <AccountPublic as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

/// Identifier for a chain. 32-bit should be plenty.
pub type ChainId = u32;

/// The hashing algorithm we use for everything.
pub type Hashing = BlakeTwo256;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Nonce = u32;

/// The balance of an account.
/// 128-bits (or 38 significant decimal figures) will allow for 10 m currency (`10^7`) at a resolution
/// to all for one second's worth of an annualised 50% reward be paid to a unit holder (`10^11` unit
/// denomination), or `10^18` total atomic units, to grow at 50%/year for 51 years (`10^9` multiplier)
/// for an eventual total of `10^27` units (27 significant decimal figures).
/// We round denomination to `10^12` (12 SDF), and leave the other redundancy at the upper end so
/// that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow.
pub type Balance = u128;

/// Session Count type
pub type SessionCount = u32;

/// Block Count type
pub type BlockCount = u32;

/// Header type.
pub type Header = generic::Header<BlockNumber, Hashing>;
/// Block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Block ID.
pub type BlockId = generic::BlockId<Block>;

/// Block Hash type
pub type BlockHash = <Header as HeaderT>::Hash;

pub type DigestItem = generic::DigestItem;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"alp0");

// Same as GRANDPA_ENGINE_ID because as of right now substrate sends only
// grandpa justifications over the network.
// TODO: change this once https://github.com/paritytech/substrate/issues/8172 will be resolved.
pub const ALEPH_ENGINE_ID: ConsensusEngineId = *b"FRNK";

/// A hash of extrinsic.
pub type TransactionHash = Hash;

pub type AuthoritySignature = app::Signature;

sp_application_crypto::with_pair! {
	pub type AuthorityPair = app::Pair;
}

impl_opaque_keys! {
	pub struct AlephNodeSessionKeys {
		pub aura: AuraId,
		pub aleph: AuthorityId,
	}
}

mod app {
	use sp_application_crypto::{app_crypto, ed25519};
	app_crypto!(ed25519, crate::KEY_TYPE);
}

pub type AuthorityId = app::Public;

/// How many blocks is in single session
#[cfg(feature = "short_session")]
pub const DEFAULT_SESSION_PERIOD: u32 = 30;

/// How many sessions is in single era
#[cfg(feature = "short_session")]
pub const DEFAULT_SESSIONS_PER_ERA: SessionIndex = 3;

/// Gather ABFT score every SCORE_SUBMISSION_PERIOD rounds
#[cfg(feature = "short_session")]
pub const SCORE_SUBMISSION_PERIOD: u32 = 15;
// --------------- Test build end  ---------------------

// --------------- Production build ---------------------
/// How many blocks is in single session
#[cfg(not(feature = "short_session"))]
pub const DEFAULT_SESSION_PERIOD: u32 = 900;

/// How many sessions is in single era
#[cfg(not(feature = "short_session"))]
pub const DEFAULT_SESSIONS_PER_ERA: SessionIndex = 96;

/// Gather ABFT score every SCORE_SUBMISSION_PERIOD rounds
#[cfg(not(feature = "short_session"))]
pub const SCORE_SUBMISSION_PERIOD: u32 = 300;

/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
pub const ADDRESSES_ENCODING: u8 = 42;

/// ABFT unit creation delay (in ms)
pub const DEFAULT_UNIT_CREATION_DELAY: u64 = 200;

/// Committee Size for new chains
pub const DEFAULT_COMMITTEE_SIZE: u32 = 4;

pub const DEFAULT_CLEAN_SESSION_COUNTER_DELAY: SessionCount = 960;
pub const DEFAULT_BAN_PERIOD: EraIndex = 10;

/// Version returned when no version has been set.
pub const DEFAULT_FINALITY_VERSION: Version = 0;

/// Current version of abft.
pub const CURRENT_FINALITY_VERSION: u16 = LEGACY_FINALITY_VERSION + 1;

/// Legacy version of abft.
pub const LEGACY_FINALITY_VERSION: u16 = 4;

/// Percentage of validator performance that is treated as 100% performance
pub const LENIENT_THRESHOLD: Perquintill = Perquintill::from_percent(90);

/// Number of non-finalized blocks that halts block production
pub const DEFAULT_MAX_NON_FINALIZED_BLOCKS: u32 = 20;

/// A relative folder where to store ABFT backups
pub const DEFAULT_BACKUP_FOLDER: &str = "backup-stash";

/// Default number of heap pages. That gives a limit of 256MB for a runtime instance, since each page is 64KB
pub const HEAP_PAGES: u64 = 4096;

/// Hold set of validators that produce blocks and set of validators that participate in finality
/// during session.
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub struct SessionCommittee<T> {
	pub finalizers: Vec<T>,
	pub producers: Vec<T>,
}

/// Openness of the process of the elections
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum ElectionOpenness {
	Permissioned,
	Permissionless,
}

/// Represent desirable size of a committee in a session
#[derive(Decode, Encode, TypeInfo, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommitteeSeats {
	/// Size of reserved validators in a session
	pub reserved_seats: u32,
	/// Size of non reserved validators in a session
	pub non_reserved_seats: u32,
	/// Size of non reserved validators participating in the finality in a session.
	/// A subset of the non reserved validators.
	pub non_reserved_finality_seats: u32,
}

impl CommitteeSeats {
	pub fn size(&self) -> u32 {
		self.reserved_seats.saturating_add(self.non_reserved_seats)
	}
}

impl Default for CommitteeSeats {
	fn default() -> Self {
		CommitteeSeats {
			reserved_seats: DEFAULT_COMMITTEE_SIZE,
			non_reserved_seats: 0,
			non_reserved_finality_seats: 0,
		}
	}
}

pub trait FinalityCommitteeManager<T> {
	/// `committee` is the set elected for finality committee for the next session
	fn on_next_session_finality_committee(committee: Vec<T>);
}

pub trait AbftScoresProvider {
	fn scores_for_session(session_id: SessionIndex) -> Option<Score>;
	fn clear_scores();
	fn clear_nonce();
}

/// Configurable parameters for ban validator mechanism
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalityBanConfig {
	/// Number representing how many rounds a parent of a head of an abft round is allowed to be behind the head.
	pub minimal_expected_performance: u16,
	/// How many bad sessions force validator to be removed from the committee
	pub underperformed_session_count_threshold: SessionCount,
	/// how many eras a validator is banned for
	pub ban_period: EraIndex,
	/// underperformed session counter is cleared every subsequent `clean_session_counter_delay` sessions
	pub clean_session_counter_delay: SessionCount,
}

pub const DEFAULT_FINALITY_BAN_MINIMAL_EXPECTED_PERFORMANCE: u16 = 11;
// The value of the following param effectively turns off bans and rewords.
pub const DEFAULT_FINALITY_BAN_SESSION_COUNT_THRESHOLD: SessionCount = SessionCount::MAX;

impl Default for FinalityBanConfig {
	fn default() -> Self {
		FinalityBanConfig {
			minimal_expected_performance: DEFAULT_FINALITY_BAN_MINIMAL_EXPECTED_PERFORMANCE,
			underperformed_session_count_threshold: DEFAULT_FINALITY_BAN_SESSION_COUNT_THRESHOLD,
			ban_period: DEFAULT_BAN_PERIOD,
			clean_session_counter_delay: DEFAULT_CLEAN_SESSION_COUNTER_DELAY,
		}
	}
}

/// Configurable parameters for ban validator mechanism related to block production
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionBanConfig {
	/// performance ratio threshold in a session
	/// calculated as ratio of number of blocks produced to expected number of blocks for a single validator
	pub minimal_expected_performance: Perbill,
	/// how many bad uptime sessions force validator to be removed from the committee
	pub underperformed_session_count_threshold: SessionCount,
	/// underperformed session counter is cleared every subsequent `clean_session_counter_delay` sessions
	pub clean_session_counter_delay: SessionCount,
	/// how many eras a validator is banned for
	pub ban_period: EraIndex,
}

pub const DEFAULT_BAN_MINIMAL_EXPECTED_PERFORMANCE: Perbill = Perbill::from_percent(0);
pub const DEFAULT_BAN_SESSION_COUNT_THRESHOLD: SessionCount = 3;
pub const DEFAULT_BAN_REASON_LENGTH: u32 = 300;
pub const DEFAULT_MAX_WINNERS: u32 = u32::MAX;

impl Default for ProductionBanConfig {
	fn default() -> Self {
		ProductionBanConfig {
			minimal_expected_performance: DEFAULT_BAN_MINIMAL_EXPECTED_PERFORMANCE,
			underperformed_session_count_threshold: DEFAULT_BAN_SESSION_COUNT_THRESHOLD,
			clean_session_counter_delay: DEFAULT_CLEAN_SESSION_COUNTER_DELAY,
			ban_period: DEFAULT_BAN_PERIOD,
		}
	}
}

/// Represent any possible reason a validator can be removed from the committee due to
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug)]
pub enum BanReason {
	/// Validator has been removed from the committee due to insufficient production in a given number of sessions
	InsufficientProduction(u32),

	/// Validator has been removed from the committee due to insufficient abft performance in a given number of sessions
	InsufficientFinalization(u32),

	/// Any arbitrary reason
	OtherReason(BoundedVec<u8, ConstU32<DEFAULT_BAN_REASON_LENGTH>>),
}

/// Details of why and for how long a validator is removed from the committee
#[derive(PartialEq, Eq, Clone, Encode, Decode, TypeInfo, Debug)]
pub struct BanInfo {
	/// reason for banning a validator
	pub reason: BanReason,
	/// index of the first era when a ban starts
	pub start: EraIndex,
}

/// Represent committee, ie set of nodes that produce and finalize blocks in the session
#[derive(Eq, Clone, PartialEq, Decode, Encode, TypeInfo)]
pub struct EraValidators<AccountId> {
	/// Validators that are chosen to be in committee every single session.
	pub reserved: Vec<AccountId>,
	/// Validators that can be banned out from the committee, under the circumstances
	pub non_reserved: Vec<AccountId>,
}

impl<AccountId> Default for EraValidators<AccountId> {
	fn default() -> Self {
		Self { reserved: Vec::new(), non_reserved: Vec::new() }
	}
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
pub enum ApiError {
	DecodeKey,
}

#[derive(Encode, Decode, TypeInfo, PartialEq, Eq, Debug)]
pub enum SessionValidatorError {
	SessionNotWithinRange { lower_limit: SessionIndex, upper_limit: SessionIndex },
	Other(Vec<u8>),
}

/// All the data needed to verify block finality justifications.
#[derive(Clone, Debug, TypeInfo, Encode, Decode, PartialEq, Eq)]
pub struct SessionAuthorityData {
	authorities: Vec<AuthorityId>,
	emergency_finalizer: Option<AuthorityId>,
}

impl SessionAuthorityData {
	pub fn new(authorities: Vec<AuthorityId>, emergency_finalizer: Option<AuthorityId>) -> Self {
		SessionAuthorityData { authorities, emergency_finalizer }
	}

	pub fn authorities(&self) -> &Vec<AuthorityId> {
		&self.authorities
	}

	pub fn emergency_finalizer(&self) -> &Option<AuthorityId> {
		&self.emergency_finalizer
	}
}

pub type Version = u32;

#[derive(Clone, Debug, Decode, Encode, PartialEq, Eq, TypeInfo)]
pub struct VersionChange {
	pub version_incoming: Version,
	pub session: SessionIndex,
}

pub trait BanHandler {
	type AccountId;
	/// returns whether the account can be banned
	fn can_ban(who: &Self::AccountId) -> bool;
}

pub trait ValidatorProvider {
	type AccountId;
	/// returns validators for the current era.
	fn current_era_validators() -> EraValidators<Self::AccountId>;
	/// returns committee seats for the current era.
	fn current_era_committee_size() -> CommitteeSeats;
}

#[derive(Decode, Encode, TypeInfo, Clone, Serialize, Deserialize)]
pub struct SessionValidators<T> {
	pub producers: Vec<T>,
	pub finalizers: Vec<T>,
	pub non_committee: Vec<T>,
}

impl<T> Default for SessionValidators<T> {
	fn default() -> Self {
		Self { producers: Vec::new(), finalizers: Vec::new(), non_committee: Vec::new() }
	}
}

/// Information provider from `pallet_session`. Loose pallet coupling via traits.
pub trait SessionInfoProvider<T> {
	fn current_session() -> SessionIndex;
	fn next_session_block_number(current_block: T) -> Option<T>;
}

pub trait BannedValidators {
	type AccountId;
	/// returns currently banned validators
	fn banned() -> Vec<Self::AccountId>;
}

pub trait EraManager {
	/// new era has been planned
	fn on_new_era(era: EraIndex);
}

/// Provides the current total issuance.
pub trait TotalIssuanceProvider {
	/// Get the current total issuance.
	fn get() -> Balance;
}

pub mod staking {
	use crate::constants::currency::TOKEN;

	pub const MIN_VALIDATOR_BOND: u128 = 10_000 * TOKEN;
	pub const MIN_NOMINATOR_BOND: u128 = 100 * TOKEN;
	pub const MAX_NOMINATORS_REWARDED_PER_VALIDATOR: u32 = 1024;

	/// Macro for making a default implementation of non-self methods from given class.
	///
	/// As an input it expects list of tuples of form
	///
	/// `(method_name(arg1: type1, arg2: type2, ...), class_name, return_type)`
	///
	/// where
	///   * `method_name`is a wrapee method,
	///   * `arg1: type1, arg2: type,...`is a list of arguments and will be passed as is, can be empty
	///   * `class_name`is a class that has non-self `method-name`,ie symbol `class_name::method_name` exists,
	///   * `return_type` is type returned from `method_name`
	/// Example
	/// ```ignore
	/// wrap_methods!(
	///     (bond(), SubstrateStakingWeights, Weight),
	///     (bond_extra(), SubstrateStakingWeights, Weight)
	/// );
	/// ```
	#[macro_export]
	macro_rules! wrap_methods {
        ($(($wrapped_method:ident( $($arg_name:ident: $argument_type:ty), *), $wrapped_class:ty, $return_type:ty)), *) => {
            $(
                fn $wrapped_method($($arg_name: $argument_type), *) -> $return_type {
                    <$wrapped_class>::$wrapped_method($($arg_name), *)
                }
            )*
        };
    }
}

pub type ScoreNonce = u32;

pub type RawScore = Vec<u16>;

#[derive(PartialEq, Decode, Encode, TypeInfo, Debug, Clone)]
pub struct Score {
	pub session_id: SessionIndex,
	pub nonce: ScoreNonce,
	pub points: RawScore,
}

pub mod crypto {
	use core::marker::PhantomData;

	use parity_scale_codec::{Decode, Encode};
	use scale_info::TypeInfo;
	use sp_runtime::RuntimeAppPublic;
	use sp_std::vec::Vec;

	use super::AuthoritySignature;

	#[derive(PartialEq, Decode, Encode, TypeInfo, Debug, Clone)]
	pub struct IndexedSignature<S> {
		pub index: u64,
		pub signature: S,
	}

	#[derive(PartialEq, Decode, Encode, TypeInfo, Debug, Clone)]
	pub struct SignatureSet<S>(pub Vec<IndexedSignature<S>>);

	#[cfg_attr(feature = "std", derive(Hash))]
	#[derive(PartialEq, Eq, Clone, Debug, Decode, Encode, TypeInfo)]
	pub struct Signature(pub AuthoritySignature);

	impl From<AuthoritySignature> for Signature {
		fn from(authority_signature: AuthoritySignature) -> Signature {
			Signature(authority_signature)
		}
	}

	impl From<Signature> for AuthoritySignature {
		fn from(signature: Signature) -> AuthoritySignature {
			signature.0
		}
	}

	/// Holds the public authority keys for a session allowing for verification of messages from that
	/// session.
	#[derive(PartialEq, Clone, Debug, Default, Decode, Encode, TypeInfo)]
	pub struct AuthorityVerifier<AID, S> {
		authorities: Vec<AID>,
		_phantom: PhantomData<S>,
	}

	impl<AID: RuntimeAppPublic<Signature = S>, S> AuthorityVerifier<AID, S> {
		/// Constructs a new authority verifier from a set of public keys.
		pub fn new(authorities: Vec<AID>) -> Self {
			AuthorityVerifier { authorities, _phantom: PhantomData }
		}

		/// Verifies whether the message is correctly signed with the signature assumed to be made by a
		/// node of the given index.
		pub fn verify(&self, msg: &Vec<u8>, sgn: &S, index: u64) -> bool {
			match self.authorities.get(index as usize) {
				Some(authority) => authority.verify(msg, sgn),
				None => false,
			}
		}

		pub fn node_count(&self) -> usize {
			self.authorities.len()
		}

		fn threshold(&self) -> usize {
			2 * self.node_count() / 3 + 1
		}

		/// Verifies whether the given signature set is a correct and complete multisignature of the
		/// message. Completeness requires more than 2/3 of all authorities.
		pub fn is_complete(
			verifier: &AuthorityVerifier<AID, S>,
			msg: &Vec<u8>,
			partial: &SignatureSet<S>,
		) -> bool {
			let signature_count = partial.0.len();
			if signature_count < verifier.threshold() {
				return false;
			}
			partial
				.0
				.iter()
				.all(|i_sgn| verifier.verify(msg, &i_sgn.signature, i_sgn.index))
		}
	}
}
