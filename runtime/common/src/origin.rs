use super::{AccountId, EitherOfDiverse, EnsureRoot};

pub type CouncilInstance = pallet_collective::Instance1;
pub type FinancialCouncilInstance = pallet_collective::Instance2;
pub type HomaCouncilInstance = pallet_collective::Instance3;
pub type TechnicalCommitteeInstance = pallet_collective::Instance4;

pub type CouncilMembershipInstance = pallet_membership::Instance1;
pub type FinancialCouncilMembershipInstance = pallet_membership::Instance2;
pub type HomaCouncilMembershipInstance = pallet_membership::Instance3;
pub type TechnicalMembershipInstance = pallet_membership::Instance4;
pub type OperatorMembershipInstanceSelendra = pallet_membership::Instance5;

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

// Financial Council
pub type EnsureRootOrAllFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsFinancialCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, FinancialCouncilInstance, 3, 4>,
>;

pub type EnsureRootOrAllHomaCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, HomaCouncilInstance, 1, 1>,
>;

pub type EnsureRootOrHalfHomaCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, HomaCouncilInstance, 1, 2>,
>;

pub type EnsureRootOrOneThirdsHomaCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, HomaCouncilInstance, 1, 3>,
>;

pub type EnsureRootOrTwoThirdsHomaCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, HomaCouncilInstance, 2, 3>,
>;

pub type EnsureRootOrThreeFourthsHomaCouncil = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, HomaCouncilInstance, 3, 4>,
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
