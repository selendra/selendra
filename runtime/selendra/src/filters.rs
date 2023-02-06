// Copyright 2021-2022 Selendra.
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
// along with Selendra.  If not, see <http://www.gnu.org/licenses/>

/// Filers
use super::{paras_registrar, Call, RuntimeDebug};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{Contains, InstanceFilter};

pub struct BaseFilter;
impl Contains<Call> for BaseFilter {
	fn contains(call: &Call) -> bool {
		match call {
			RuntimeCall::Democracy(_) |
			RuntimeCall::NominationPools(_) |
			RuntimeCall::Council(_) |
			RuntimeCall::TechnicalCommittee(_) |
			RuntimeCall::TechnicalMembership(_) |
			RuntimeCall::Treasury(_) |
			RuntimeCall::PhragmenElection(_) |
			RuntimeCall::System(_) |
			RuntimeCall::Scheduler(_) |
			RuntimeCall::Preimage(_) |
			RuntimeCall::Indices(_) |
			RuntimeCall::Babe(_) |
			RuntimeCall::Timestamp(_) |
			RuntimeCall::Balances(_) |
			RuntimeCall::Authorship(_) |
			RuntimeCall::Staking(_) |
			RuntimeCall::Session(_) |
			RuntimeCall::Grandpa(_) |
			RuntimeCall::ImOnline(_) |
			RuntimeCall::Utility(_) |
			RuntimeCall::Vesting(_) |
			RuntimeCall::Identity(_) |
			RuntimeCall::Proxy(_) |
			RuntimeCall::Multisig(_) |
			RuntimeCall::Bounties(_) |
			RuntimeCall::Tips(_) |
			RuntimeCall::ElectionProviderMultiPhase(_) |
			RuntimeCall::Recovery(_) |
			RuntimeCall::CouncilMembership(_) |
			RuntimeCall::Sudo(_) |
			RuntimeCall::Configuration(_) |
			RuntimeCall::ParasShared(_) |
			RuntimeCall::ParaInclusion(_) |
			RuntimeCall::Paras(_) |
			RuntimeCall::Initializer(_) |
			RuntimeCall::ParaInherent(_) |
			RuntimeCall::ParasDisputes(_) |
			RuntimeCall::Dmp(_) |
			RuntimeCall::Ump(_) |
			RuntimeCall::Hrmp(_) |
			RuntimeCall::Slots(_) |
			RuntimeCall::Registrar(_) |
			RuntimeCall::XcmPallet(_) |
			RuntimeCall::ParasSudoWrapper(_) |
			RuntimeCall::VoterList(_) => true,
		}
	}
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	Any = 0,
	NonTransfer = 1,
	Governance = 2,
	Staking = 3,
	// Skip 4 as it is now removed (was SudoBalances)
	IdentityJudgement = 5,
	CancelProxy = 6,
}

#[cfg(test)]
mod proxy_type_tests {
	use super::*;

	#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Encode, Decode, RuntimeDebug)]
	pub enum OldProxyType {
		Any,
		NonTransfer,
		Governance,
		Staking,
		SudoBalances,
		IdentityJudgement,
	}

	#[test]
	fn proxy_type_decodes_correctly() {
		for (i, j) in vec![
			(OldProxyType::Any, ProxyType::Any),
			(OldProxyType::NonTransfer, ProxyType::NonTransfer),
			(OldProxyType::Governance, ProxyType::Governance),
			(OldProxyType::Staking, ProxyType::Staking),
			(OldProxyType::IdentityJudgement, ProxyType::IdentityJudgement),
		]
		.into_iter()
		{
			assert_eq!(i.encode(), j.encode());
		}
		assert!(ProxyType::decode(&mut &OldProxyType::SudoBalances.encode()[..]).is_err());
	}
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}
impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => matches!(
				c,
				RuntimeCall::System(..) |
				RuntimeCall::Scheduler(..) |
				RuntimeCall::Babe(..) |
				RuntimeCall::Timestamp(..) |
				RuntimeCall::Indices(pallet_indices::Call::claim{..}) |
				RuntimeCall::Indices(pallet_indices::Call::free{..}) |
				RuntimeCall::Indices(pallet_indices::Call::freeze{..}) |
				// Specifically omitting Indices `transfer`, `force_transfer`
				// Specifically omitting the entire Balances pallet
				RuntimeCall::Authorship(..) |
				RuntimeCall::Staking(..) |
				RuntimeCall::Session(..) |
				RuntimeCall::Grandpa(..) |
				RuntimeCall::ImOnline(..) |
				RuntimeCall::Democracy(..) |
				RuntimeCall::Council(..) |
				RuntimeCall::TechnicalCommittee(..) |
				RuntimeCall::PhragmenElection(..) |
				RuntimeCall::TechnicalMembership(..) |
				RuntimeCall::Treasury(..) |
				RuntimeCall::Bounties(..) |
				RuntimeCall::Tips(..) |
				RuntimeCall::Vesting(pallet_vesting::Call::vest{..}) |
				RuntimeCall::Vesting(pallet_vesting::Call::vest_other{..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::as_recovered {..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::vouch_recovery {..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::claim_recovery {..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::close_recovery {..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::remove_recovery {..}) |
				RuntimeCall::Recovery(pallet_recovery::Call::cancel_recovered {..}) |
				RuntimeCall::Registrar(paras_registrar::Call::register {..}) |
				RuntimeCall::Registrar(paras_registrar::Call::deregister {..}) |
				// Specifically omitting Registrar `swap`
				RuntimeCall::Registrar(paras_registrar::Call::reserve {..}) |
				RuntimeCall::Slots(..) |
				// Specifically omitting Vesting `vested_transfer`, and `force_vested_transfer`
				RuntimeCall::Utility(..) |
				RuntimeCall::Identity(..) |
				RuntimeCall::Proxy(..) |
				RuntimeCall::Multisig(..) |
				RuntimeCall::VoterList(..) |
				RuntimeCall::CouncilMembership(_)
			),
			ProxyType::Governance => matches!(
				c,
				RuntimeCall::Democracy(..) |
					RuntimeCall::Council(..) | RuntimeCall::TechnicalCommittee(..) |
					RuntimeCall::PhragmenElection(..) |
					RuntimeCall::Treasury(..) | RuntimeCall::Bounties(..) |
					RuntimeCall::Tips(..) | RuntimeCall::Utility(..) |
					RuntimeCall::Sudo(..)
			),
			ProxyType::Staking => {
				matches!(c, RuntimeCall::Staking(..) | RuntimeCall::Session(..) | RuntimeCall::Utility(..))
			},
			ProxyType::IdentityJudgement => matches!(
				c,
				RuntimeCall::Identity(pallet_identity::RuntimeCall::provide_judgement { .. }) | RuntimeCall::Utility(..)
			),
			ProxyType::CancelProxy => {
				matches!(c, RuntimeCall::Proxy(pallet_proxy::RuntimeCall::reject_announcement { .. }))
			},
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}
