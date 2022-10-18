/// Copyright (C) 2021-2022 Selendra.
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

///! Indracore configuration for Selendra.
use super::{
	deposit, parameter_types, weights, xcm_config, Babe, Balances, EnsureRoot, Event, Historical,
	IndrasDisputes, Origin, Registrar, Runtime, Slots, TransactionPriority, DAYS, DOLLARS, WEEKS,
};

use primitives::v2::{AccountId, Balance, BlockNumber};
use runtime_common::{indras_registrar, prod_or_fast, slots, EnsureRootOrThreeFourthsCouncil};

use runtime_indracores::{
	configuration as indracores_configuration, disputes as indracores_disputes,
	dmp as indracores_dmp, hrmp as indracores_hrmp, inclusion as indracores_inclusion,
	indras as indracores_indras, indras_inherent as indracores_indras_inherent,
	initializer as indracores_initializer, origin as indracores_origin,
	reward_points as indracores_reward_points, scheduler as indracores_scheduler,
	session_info as indracores_session_info, shared as indracores_shared, ump as indracores_ump,
};

impl indracores_origin::Config for Runtime {}

impl indracores_configuration::Config for Runtime {
	type WeightInfo = weights::runtime_indracores_configuration::WeightInfo<Runtime>;
}

impl indracores_shared::Config for Runtime {}

impl indracores_session_info::Config for Runtime {
	type ValidatorSet = Historical;
}

impl indracores_inclusion::Config for Runtime {
	type Event = Event;
	type DisputesHandler = IndrasDisputes;
	type RewardValidators = indracores_reward_points::RewardValidatorsWithEraPoints<Runtime>;
}

parameter_types! {
	pub const IndrasUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl indracores_indras::Config for Runtime {
	type Event = Event;
	type WeightInfo = weights::runtime_indracores_indras::WeightInfo<Runtime>;
	type UnsignedPriority = IndrasUnsignedPriority;
	type NextSessionRotation = Babe;
}

parameter_types! {
	pub const FirstMessageFactorPercent: u64 = 100;
}

impl indracores_ump::Config for Runtime {
	type Event = Event;
	type UmpSink =
		crate::indracores_ump::XcmSink<xcm_executor::XcmExecutor<xcm_config::XcmConfig>, Runtime>;
	type FirstMessageFactorPercent = FirstMessageFactorPercent;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type WeightInfo = indracores_ump::TestWeightInfo;
}

impl indracores_dmp::Config for Runtime {}

impl indracores_hrmp::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Currency = Balances;
	type WeightInfo = weights::runtime_indracores_hrmp::WeightInfo<Self>;
}

impl indracores_indras_inherent::Config for Runtime {
	type WeightInfo = weights::runtime_indracores_indras_inherent::WeightInfo<Runtime>;
}

impl indracores_scheduler::Config for Runtime {}

impl indracores_initializer::Config for Runtime {
	type Randomness = pallet_babe::RandomnessFromOneEpochAgo<Runtime>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = weights::runtime_indracores_initializer::WeightInfo<Runtime>;
}

impl indracores_disputes::Config for Runtime {
	type Event = Event;
	type RewardValidators = ();
	type PunishValidators = ();
	type WeightInfo = weights::runtime_indracores_disputes::WeightInfo<Runtime>;
}

parameter_types! {
	// Mostly arbitrary deposit price, but should provide an adequate incentive not to spam reserve
	// `IndraId`s.
	pub const IndraDeposit: Balance = 100 * DOLLARS;
	pub const IndraDataByteDeposit: Balance = deposit(0, 1);
}

impl indras_registrar::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type Currency = Balances;
	type OnSwap = Slots;
	type IndraDeposit = IndraDeposit;
	type DataDepositPerByte = IndraDataByteDeposit;
	type WeightInfo = weights::runtime_common_indras_registrar::WeightInfo<Runtime>;
}

parameter_types! {
	// 24 weeks = 6 months per lease period -> 8 lease periods ~ 4 years
	pub LeasePeriod: BlockNumber = prod_or_fast!(24 * WEEKS, 24 * WEEKS, "SEL_LEASE_PERIOD");
	pub LeaseOffset: BlockNumber = prod_or_fast!(64 * DAYS, 0, "SEL_LEASE_OFFSET");
}

impl slots::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type Registrar = Registrar;
	type LeasePeriod = LeasePeriod;
	type LeaseOffset = LeaseOffset;
	type ForceOrigin = EnsureRootOrThreeFourthsCouncil;
	type WeightInfo = weights::runtime_common_slots::WeightInfo<Runtime>;
}
