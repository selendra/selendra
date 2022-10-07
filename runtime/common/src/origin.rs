// This file is part of Selendra.

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
