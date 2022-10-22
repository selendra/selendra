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

#[cfg(test)]
mod test_fees {
	use crate::*;
	use frame_support::weights::{GetDispatchInfo, WeightToFee as WeightToFeeT};
	use keyring::Sr25519Keyring::{Alice, Charlie};
	use pallet_transaction_payment::Multiplier;
	use runtime_common::MinimumMultiplier;
	use separator::Separatable;
	use sp_runtime::{assert_eq_error_rate, FixedPointNumber, MultiAddress, MultiSignature};

	#[test]
	fn payout_weight_portion() {
		use pallet_staking::WeightInfo;
		let payout_weight =
			<Runtime as pallet_staking::Config>::WeightInfo::payout_stakers_alive_staked(
				MaxNominatorRewardedPerValidator::get(),
			) as f64;
		let block_weight = BlockWeights::get().max_block as f64;

		println!(
			"a full payout takes {:.2} of the block weight [{} / {}]",
			payout_weight / block_weight,
			payout_weight,
			block_weight
		);
		assert!(payout_weight * 2f64 < block_weight);
	}

	#[test]
	fn block_cost() {
		let max_block_weight = BlockWeights::get().max_block;
		let raw_fee = WeightToFee::weight_to_fee(&max_block_weight);

		let fee_with_multiplier = |m: Multiplier| {
			println!(
				"Full Block weight == {} // multiplier: {:?} // WeightToFee(full_block) == {} plank",
				max_block_weight,
				m,
				m.saturating_mul_int(raw_fee).separated_string(),
			);
		};
		fee_with_multiplier(MinimumMultiplier::get());
		fee_with_multiplier(Multiplier::from_rational(1, 2));
		fee_with_multiplier(Multiplier::from_u32(1));
		fee_with_multiplier(Multiplier::from_u32(2));
	}

	#[test]
	fn transfer_cost_min_multiplier() {
		let min_multiplier = MinimumMultiplier::get();
		let call = pallet_balances::Call::<Runtime>::transfer_keep_alive {
			dest: Charlie.to_account_id().into(),
			value: Default::default(),
		};
		let info = call.get_dispatch_info();
		println!("call = {:?} / info = {:?}", call, info);
		// convert to outer call.
		let call = Call::Balances(call);
		let extra: SignedExtra = (
			frame_system::CheckNonZeroSender::<Runtime>::new(),
			frame_system::CheckSpecVersion::<Runtime>::new(),
			frame_system::CheckTxVersion::<Runtime>::new(),
			frame_system::CheckGenesis::<Runtime>::new(),
			frame_system::CheckMortality::<Runtime>::from(generic::Era::immortal()),
			frame_system::CheckNonce::<Runtime>::from(1),
			frame_system::CheckWeight::<Runtime>::new(),
			pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(0),
		);
		let uxt = UncheckedExtrinsic {
			function: call,
			signature: Some((
				MultiAddress::Id(Alice.to_account_id()),
				MultiSignature::Sr25519(Alice.sign(b"foo")),
				extra,
			)),
		};
		let len = uxt.encoded_size();

		let mut ext = sp_io::TestExternalities::new_empty();
		let mut test_with_multiplier = |m: Multiplier| {
			ext.execute_with(|| {
				pallet_transaction_payment::NextFeeMultiplier::<Runtime>::put(m);
				let fee = TransactionPayment::query_fee_details(uxt.clone(), len as u32);
				println!(
					"multiplier = {:?} // fee details = {:?} // final fee = {:?}",
					pallet_transaction_payment::NextFeeMultiplier::<Runtime>::get(),
					fee,
					fee.final_fee().separated_string(),
				);
			});
		};

		test_with_multiplier(min_multiplier);
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1u128));
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1_0u128));
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1_00u128));
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1_000u128));
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1_000_000u128));
		test_with_multiplier(Multiplier::saturating_from_rational(1u128, 1_000_000_000u128));
	}

	#[test]
	fn full_block_council_election_cost() {
		// the number of voters needed to consume almost a full block in council election, and how
		// much it is going to cost.
		use pallet_elections_phragmen::WeightInfo;

		// Loser candidate lose a lot of money; sybil attack by candidates is even more expensive,
		// and we don't care about it here. For now, we assume no extra candidates, and only
		// superfluous voters.
		let candidates = DesiredMembers::get() + DesiredRunnersUp::get();
		let mut voters = 1u32;
		let weight_with = |v| {
			<Runtime as pallet_elections_phragmen::Config>::WeightInfo::election_phragmen(
				candidates,
				v,
				v * 16,
			)
		};

		while weight_with(voters) <= BlockWeights::get().max_block {
			voters += 1;
		}

		let cost = voters as Balance * (VotingBondBase::get() + 16 * VotingBondFactor::get());
		let cost_dollars = cost / DOLLARS;
		println!(
			"can support {} voters in a single block for council elections; total bond {}",
			voters, cost_dollars,
		);
		assert!(cost_dollars > 150_00); // DOLLAR ~ new SEL ~ 10e12
	}

	#[test]
	fn nominator_limit() {
		use pallet_election_provider_multi_phase::WeightInfo;
		// starting point of the nominators.
		let target_voters: u32 = 50_000;

		// assuming we want around 5k candidates and 1k active validators. (March 31, 2021)
		let all_targets: u32 = 5_000;
		let desired: u32 = 1_000;
		let weight_with = |active| {
			<Runtime as pallet_election_provider_multi_phase::Config>::WeightInfo::submit_unsigned(
				active,
				all_targets,
				active,
				desired,
			)
		};

		let mut active = target_voters;
		while weight_with(active) <= OffchainSolutionWeightLimit::get() || active == target_voters {
			active += 1;
		}

		println!("can support {} nominators to yield a weight of {}", active, weight_with(active));
		assert!(active > target_voters, "we need to reevaluate the weight of the election system");
	}

	#[test]
	fn signed_deposit_is_sensible() {
		// ensure this number does not change, or that it is checked after each change.
		// a 1 MB solution should take (40 + 10) SELs of deposit.
		let deposit = SignedDepositBase::get() + (SignedDepositByte::get() * 1024 * 1024);
		assert_eq_error_rate!(deposit, 50 * DOLLARS, DOLLARS);
	}
}

#[cfg(test)]
mod test {
	use crate::*;

	#[test]
	fn call_size() {
		assert!(
			core::mem::size_of::<Call>() <= 230,
			"size of Call is more than 230 bytes: some calls have too big arguments, use Box to \
			reduce the size of Call.
			If the limit is too strong, maybe consider increase the limit",
		);
	}
}

#[cfg(test)]
mod multiplier_tests {
	use crate::*;
	use frame_support::{dispatch::GetDispatchInfo, traits::OnFinalize};
	use runtime_common::{MinimumMultiplier, TargetBlockFullness};
	use separator::Separatable;
	use sp_runtime::traits::Convert;

	fn run_with_system_weight<F>(w: Weight, mut assertions: F)
	where
		F: FnMut() -> (),
	{
		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();
		t.execute_with(|| {
			System::set_block_consumed_resources(w, 0);
			assertions()
		});
	}

	#[test]
	fn multiplier_can_grow_from_zero() {
		let minimum_multiplier = MinimumMultiplier::get();
		let target = TargetBlockFullness::get() *
			BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap();
		// if the min is too small, then this will not change, and we are doomed forever.
		// the weight is 1/100th bigger than target.
		run_with_system_weight(target * 101 / 100, || {
			let next = SlowAdjustingFeeUpdate::<Runtime>::convert(minimum_multiplier);
			assert!(next > minimum_multiplier, "{:?} !>= {:?}", next, minimum_multiplier);
		})
	}

	#[test]
	#[ignore]
	fn multiplier_growth_simulator() {
		// assume the multiplier is initially set to its minimum. We update it with values twice the
		//target (target is 25%, thus 50%) and we see at which point it reaches 1.
		let mut multiplier = MinimumMultiplier::get();
		let block_weight = BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap();
		let mut blocks = 0;
		let mut fees_paid = 0;

		let call = frame_system::Call::<Runtime>::fill_block {
			ratio: Perbill::from_rational(
				block_weight,
				BlockWeights::get().get(DispatchClass::Normal).max_total.unwrap(),
			),
		};
		println!("calling {:?}", call);
		let info = call.get_dispatch_info();
		// convert to outer call.
		let call = Call::System(call);
		let len = call.using_encoded(|e| e.len()) as u32;

		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();
		// set the minimum
		t.execute_with(|| {
			pallet_transaction_payment::NextFeeMultiplier::<Runtime>::set(MinimumMultiplier::get());
		});

		while multiplier <= Multiplier::from_u32(1) {
			t.execute_with(|| {
				// imagine this tx was called.
				let fee = TransactionPayment::compute_fee(len, &info, 0);
				fees_paid += fee;

				// this will update the multiplier.
				System::set_block_consumed_resources(block_weight, 0);
				TransactionPayment::on_finalize(1);
				let next = TransactionPayment::next_fee_multiplier();

				assert!(next > multiplier, "{:?} !>= {:?}", next, multiplier);
				multiplier = next;

				println!(
					"block = {} / multiplier {:?} / fee = {:?} / fess so far {:?}",
					blocks,
					multiplier,
					fee.separated_string(),
					fees_paid.separated_string()
				);
			});
			blocks += 1;
		}
	}

	#[test]
	#[ignore]
	fn multiplier_cool_down_simulator() {
		// assume the multiplier is initially set to its minimum. We update it with values twice the
		//target (target is 25%, thus 50%) and we see at which point it reaches 1.
		let mut multiplier = Multiplier::from_u32(2);
		let mut blocks = 0;

		let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
			.build_storage::<Runtime>()
			.unwrap()
			.into();
		// set the minimum
		t.execute_with(|| {
			pallet_transaction_payment::NextFeeMultiplier::<Runtime>::set(multiplier);
		});

		while multiplier > Multiplier::from_u32(0) {
			t.execute_with(|| {
				// this will update the multiplier.
				TransactionPayment::on_finalize(1);
				let next = TransactionPayment::next_fee_multiplier();

				assert!(next < multiplier, "{:?} !>= {:?}", next, multiplier);
				multiplier = next;

				println!("block = {} / multiplier {:?}", blocks, multiplier);
			});
			blocks += 1;
		}
	}
}
