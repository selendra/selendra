use crate::{Call, RuntimeDebug};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::{Contains, InstanceFilter};

pub struct BaseFilter;
impl Contains<Call> for BaseFilter {
	fn contains(call: &Call) -> bool {
		match call {
			Call::Democracy(_) |
			Call::NominationPools(_) |
			Call::Council(_) |
			Call::TechnicalCommittee(_) |
			Call::TechnicalMembership(_) |
			Call::Treasury(_) |
			Call::PhragmenElection(_) |
			Call::System(_) |
			Call::Scheduler(_) |
			Call::Preimage(_) |
			Call::Indices(_) |
			Call::Babe(_) |
			Call::Timestamp(_) |
			Call::Balances(_) |
			Call::Authorship(_) |
			Call::Staking(_) |
			Call::Session(_) |
			Call::Grandpa(_) |
			Call::ImOnline(_) |
			Call::Utility(_) |
			Call::Vesting(_) |
			Call::Identity(_) |
			Call::Proxy(_) |
			Call::Multisig(_) |
			Call::Bounties(_) |
			Call::Tips(_) |
			Call::ElectionProviderMultiPhase(_) |
			Call::Recovery(_) |
			Call::CouncilMembership(_) |
			Call::Sudo(_) |
			Call::VoterList(_) => true,
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
				Call::System(..) |
				Call::Scheduler(..) |
				Call::Babe(..) |
				Call::Timestamp(..) |
				Call::Indices(pallet_indices::Call::claim{..}) |
				Call::Indices(pallet_indices::Call::free{..}) |
				Call::Indices(pallet_indices::Call::freeze{..}) |
				// Specifically omitting Indices `transfer`, `force_transfer`
				// Specifically omitting the entire Balances pallet
				Call::Authorship(..) |
				Call::Staking(..) |
				Call::Session(..) |
				Call::Grandpa(..) |
				Call::ImOnline(..) |
				Call::Democracy(..) |
				Call::Council(..) |
				Call::TechnicalCommittee(..) |
				Call::PhragmenElection(..) |
				Call::TechnicalMembership(..) |
				Call::Treasury(..) |
				Call::Bounties(..) |
				Call::Tips(..) |
				Call::Vesting(pallet_vesting::Call::vest{..}) |
				Call::Vesting(pallet_vesting::Call::vest_other{..}) |
				Call::Recovery(pallet_recovery::Call::as_recovered {..}) |
				Call::Recovery(pallet_recovery::Call::vouch_recovery {..}) |
				Call::Recovery(pallet_recovery::Call::claim_recovery {..}) |
				Call::Recovery(pallet_recovery::Call::close_recovery {..}) |
				Call::Recovery(pallet_recovery::Call::remove_recovery {..}) |
				Call::Recovery(pallet_recovery::Call::cancel_recovered {..}) |
				// Specifically omitting Vesting `vested_transfer`, and `force_vested_transfer`
				Call::Utility(..) |
				Call::Identity(..) |
				Call::Proxy(..) |
				Call::Multisig(..) |
				Call::VoterList(..) |
				Call::CouncilMembership(_)
			),
			ProxyType::Governance => matches!(
				c,
				Call::Democracy(..) |
					Call::Council(..) | Call::TechnicalCommittee(..) |
					Call::PhragmenElection(..) |
					Call::Treasury(..) | Call::Bounties(..) |
					Call::Tips(..) | Call::Utility(..) |
					Call::Sudo(..)
			),
			ProxyType::Staking => {
				matches!(c, Call::Staking(..) | Call::Session(..) | Call::Utility(..))
			},
			ProxyType::IdentityJudgement => matches!(
				c,
				Call::Identity(pallet_identity::Call::provide_judgement { .. }) | Call::Utility(..)
			),
			ProxyType::CancelProxy => {
				matches!(c, Call::Proxy(pallet_proxy::Call::reject_announcement { .. }))
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
