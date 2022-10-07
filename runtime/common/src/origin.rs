use super::{AccountId, EitherOfDiverse, EnsureRoot};

pub type CouncilInstance = pallet_collective::Instance1;
pub type CouncilMembershipInstance = pallet_membership::Instance1;

pub type TechnicalCommitteeInstance = pallet_collective::Instance2;
pub type TechnicalMembershipInstance = pallet_membership::Instance2;

// General Council
pub type EnsureRootOrAllCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, CouncilInstance, 3, 4>,
>;

pub type EnsureRootOrOneCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureMember<AccountId, CouncilInstance>,
>;

// Technical Committee Council
pub type EnsureRootOrAllTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 1>,
>;

pub type EnsureRootOrHalfTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsTechnicalCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCommitteeInstance, 3, 4>,
>;
