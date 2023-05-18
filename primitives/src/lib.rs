// Copyright 2023 Smallworld Selendra
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
#![allow(clippy::too_many_arguments, clippy::unnecessary_mut_passed)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use sp_core::crypto::KeyTypeId;
pub use sp_runtime::{
	generic::Header as GenericHeader,
	traits::{BlakeTwo256, ConstU32, Hash as HashT, Header as HeaderT, IdentifyAccount, Verify},
	BoundedVec, ConsensusEngineId, FixedU128, MultiSignature, Perbill,
};
pub use sp_staking::{EraIndex, SessionIndex};
use sp_std::vec::Vec;

pub mod app;
pub mod constants;
// pub mod evm;
// pub mod task;

pub use constants::*;

/// The block number type used by Selendra.
/// 32-bits will allow for 136 years of blocks assuming 1 block per second.
pub type BlockNumber = u32;

/// An instant or duration in time.
pub type Moment = u64;

/// Alias to type for a signature for a transaction on the relay chain. This allows one of several
/// kinds of underlying crypto to be used, so isn't a fixed size when encoded.
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

/// A hash of some data used by the relay chain.
pub type Hash = sp_core::H256;

/// Index of a transaction in the relay chain. 32-bit should be plenty.
pub type Index = u32;

/// Fee multiplier.
pub type Multiplier = FixedU128;

/// The balance of an account.
/// 128-bits (or 38 significant decimal figures) will allow for 10 m currency (`10^7`) at a resolution
/// to all for one second's worth of an annualised 50% reward be paid to a unit holder (`10^11` unit
/// denomination), or `10^18` total atomic units, to grow at 50%/year for 51 years (`10^9` multiplier)
/// for an eventual total of `10^27` units (27 significant decimal figures).
/// We round denomination to `10^12` (12 SDF), and leave the other redundancy at the upper end so
/// that 32 bits may be multiplied with a balance in 128 bits without worrying about overflow.
pub type Balance = u128;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	pub use sp_runtime::{generic, OpaqueExtrinsic as UncheckedExtrinsic};

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
}

sp_application_crypto::with_pair! {
	pub type AuthorityPair = app::Pair;
}

pub type AuthoritySignature = app::Signature;

pub type AuthorityId = app::Public;

pub type SessionCount = u32;

pub type BlockCount = u32;

/// Openness of the process of the elections
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq)]
pub enum ElectionOpenness {
	Permissioned,
	Permissionless,
}

/// Represent desirable size of a committee in a session
#[derive(Decode, Encode, TypeInfo, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CommitteeSeats {
	/// Size of reserved validators in a session
	pub reserved_seats: u32,
	/// Size of non reserved valiadtors in a session
	pub non_reserved_seats: u32,
}

impl CommitteeSeats {
	pub fn size(&self) -> u32 {
		self.reserved_seats.saturating_add(self.non_reserved_seats)
	}
}

impl Default for CommitteeSeats {
	fn default() -> Self {
		CommitteeSeats { reserved_seats: DEFAULT_COMMITTEE_SIZE, non_reserved_seats: 0 }
	}
}

pub trait FinalityCommitteeManager<T> {
	/// `committee` is the set elected for finality committee for the next session
	fn on_next_session_finality_committee(committee: Vec<T>);
}

/// Configurable parameters for ban validator mechanism
#[derive(Decode, Encode, TypeInfo, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct BanConfig {
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

impl Default for BanConfig {
	fn default() -> Self {
		BanConfig {
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
	/// Validator has been removed from the committee due to insufficient uptime in a given number
	/// of sessions
	InsufficientUptime(u32),

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
#[derive(Eq, PartialEq, Decode, Encode, TypeInfo)]
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

#[derive(Encode, Decode, PartialEq, Eq, Debug)]
pub enum ApiError {
	DecodeKey,
}

/// All the data needed to verify block finalization justifications.
#[derive(Clone, Debug, Encode, Decode, PartialEq, Eq)]
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

sp_api::decl_runtime_apis! {
	pub trait SelendraSessionApi
	{
		fn next_session_authorities() -> Result<Vec<AuthorityId>, ApiError>;
		fn authorities() -> Vec<AuthorityId>;
		fn next_session_authority_data() -> Result<SessionAuthorityData, ApiError>;
		fn authority_data() -> SessionAuthorityData;
		fn session_period() -> u32;
		fn millisecs_per_block() -> u64;
		fn finality_version() -> Version;
		fn next_session_finality_version() -> Version;
	}
}

pub trait BanHandler {
	type AccountId;
	/// returns whether the account can be banned
	fn can_ban(who: &Self::AccountId) -> bool;
}

pub trait ValidatorProvider {
	type AccountId;
	/// returns validators for the current era if present.
	fn current_era_validators() -> Option<EraValidators<Self::AccountId>>;
	/// returns committe seats for the current era if present.
	fn current_era_committee_size() -> Option<CommitteeSeats>;
}

#[derive(Decode, Encode, TypeInfo, Clone)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SessionValidators<T> {
	pub committee: Vec<T>,
	pub non_committee: Vec<T>,
}

impl<T> Default for SessionValidators<T> {
	fn default() -> Self {
		Self { committee: Vec::new(), non_committee: Vec::new() }
	}
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
