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
use crate::{
	evm::precompiles::SelendraPrecompiles, AccountId, Babe, Balance, Balances, BaseFee, Runtime,
	RuntimeCall, RuntimeEvent, Signature, Weight, MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO,
};
use codec::Encode;
pub use selendra_runtime_constants::currency::{MILLICENTS, UNITS};

use frame_support::{
	parameter_types, traits::FindAuthor, weights::constants::WEIGHT_PER_SECOND, ConsensusEngineId,
};

use sp_core::{H160, U256};
use sp_runtime::{
	traits::{BlakeTwo256, Verify},
	transaction_validity::TransactionPriority,
	Permill,
};

pub type Precompiles = SelendraPrecompiles<Runtime>;

parameter_types! {
	// Tells `pallet_base_fee` whether to calculate a new BaseFee `on_finalize` or not.
	pub DefaultBaseFeePerGas: U256 = (MILLICENTS / 1_000_000).into();
	// At the moment, we don't use dynamic fee calculation for Shibuya by default
	pub DefaultElasticity: Permill = Permill::zero();
}

pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		Permill::from_parts(500_000)
	}
	fn upper() -> Permill {
		Permill::from_parts(1_000_000)
	}
}

impl pallet_base_fee::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Threshold = BaseFeeThreshold;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}

/// Current approximation of the gas/s consumption considering
/// EVM execution over compiled WASM (on 4.4Ghz CPU).
/// Given the 500ms Weight, from which 75% only are used for transactions,
/// the total EVM execution gas limit is: GAS_PER_SECOND * 0.500 * 0.75 ~= 15_000_000.
pub const GAS_PER_SECOND: u64 = 40_000_000;

/// Approximate ratio of the amount of Weight per Gas.
/// u64 works for approximations because Weight is a very small unit compared to gas.
pub const WEIGHT_PER_GAS: u64 = WEIGHT_PER_SECOND.saturating_div(GAS_PER_SECOND).ref_time();

pub struct FindAuthorTruncated<F>(sp_std::marker::PhantomData<F>);
impl<F: FindAuthor<u32>> FindAuthor<H160> for FindAuthorTruncated<F> {
	fn find_author<'a, I>(digests: I) -> Option<H160>
	where
		I: 'a + IntoIterator<Item = (ConsensusEngineId, &'a [u8])>,
	{
		if let Some(author_index) = F::find_author(digests) {
			let authority_id = Babe::authorities()[author_index as usize].clone();
			return Some(H160::from_slice(&authority_id.encode()[4..24]))
		}

		None
	}
}

parameter_types! {
	pub ChainId: u64 = 0x7A9;
	/// EVM gas limit
	pub BlockGasLimit: U256 = U256::from(
		NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT.ref_time() / WEIGHT_PER_GAS
	);
	pub PrecompilesValue: Precompiles = SelendraPrecompiles::<_>::new();
	pub WeightPerGas: Weight = Weight::from_ref_time(WEIGHT_PER_GAS);
}

impl pallet_evm::Config for Runtime {
	type FeeCalculator = BaseFee;
	type GasWeightMapping = pallet_evm::FixedGasWeightMapping<Self>;
	type WeightPerGas = WeightPerGas;
	type BlockHashMapping = pallet_ethereum::EthereumBlockHashMapping<Runtime>;
	type CallOrigin = pallet_evm::EnsureAddressRoot<AccountId>;
	type WithdrawOrigin = pallet_evm::EnsureAddressTruncated;
	type AddressMapping = pallet_evm::HashedAddressMapping<BlakeTwo256>;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type Runner = pallet_evm::runner::stack::Runner<Self>;
	type PrecompilesType = Precompiles;
	type PrecompilesValue = PrecompilesValue;
	// Ethereum-compatible chain_id:
	// * Selendra: 1961
	type ChainId = ChainId;
	type OnChargeTransaction = ();
	type BlockGasLimit = BlockGasLimit;
	type FindAuthor = FindAuthorTruncated<Babe>;
}

impl pallet_ethereum::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self>;
}

parameter_types! {
	pub const EcdsaUnsignedPriority: TransactionPriority = TransactionPriority::MAX / 2;
	pub const CallFee: Balance = UNITS / 10;
	pub const CallMagicNumber: u16 = 0x0250;
}

impl pallet_custom_signatures::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Signature = pallet_custom_signatures::ethereum::EthereumSignature;
	type Signer = <Signature as Verify>::Signer;
	type CallMagicNumber = CallMagicNumber;
	type Currency = Balances;
	type CallFee = CallFee;
	type OnChargeTransaction = ();
	type UnsignedPriority = EcdsaUnsignedPriority;
}
