use frame_support::traits::EitherOfDiverse;
use frame_system::EnsureRoot;
use primitives::AccountId;

pub type CouncilCollective = pallet_collective::Instance1;
pub type TechnicalCollective = pallet_collective::Instance2;

pub type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
>;

/// A 60% super-majority can have the next scheduled referendum be a straight majority-carries vote.
pub type EnsureRootOrMajorityCouncil = EitherOfDiverse<
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
	frame_system::EnsureRoot<AccountId>,
>;

/// ALL council
pub type EnsureRootOrFullCouncil = EitherOfDiverse<
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>,
	frame_system::EnsureRoot<AccountId>,
>;

/// ALL technical
pub type EnsureRootOrFullTechnical = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 1>,
>;

/// Two thirds of the Conncil committee can have an `ExternalMajority/ExternalDefault` vote
/// be tabled immediately and with a shorter voting/enactment period.
pub type EnsureRootOrTwoThirdCouncil = EitherOfDiverse<
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
	frame_system::EnsureRoot<AccountId>,
>;

/// Two thirds of the technical committee can have an `ExternalMajority/ExternalDefault` vote
/// be tabled immediately and with a shorter voting/enactment period.
pub type EnsureRootOrTwoThirdTechnical = EitherOfDiverse<
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
	frame_system::EnsureRoot<AccountId>,
>;
