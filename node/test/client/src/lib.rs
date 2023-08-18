// Copyright 2022 Smallworld Selendra
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

//! A Selendra test client.
//!
//! This test client is using the Selendra test runtime.

mod block_builder;

use selendra_primitives::Block;
use sp_runtime::BuildStorage;
use std::sync::Arc;

pub use block_builder::*;
pub use selendra_test_runtime as runtime;
pub use selendra_test_service::{
	construct_extrinsic, construct_transfer_extrinsic, Client, FullBackend,
};
pub use substrate_test_client::*;

/// Test client executor.
pub type Executor = client::LocalCallExecutor<
	Block,
	FullBackend,
	WasmExecutor<(sp_io::SubstrateHostFunctions, frame_benchmarking::benchmarking::HostFunctions)>,
>;

/// Test client builder for Selendra.
pub type TestClientBuilder =
	substrate_test_client::TestClientBuilder<Block, Executor, FullBackend, GenesisParameters>;

/// `LongestChain` type for the test runtime/client.
pub type LongestChain = sc_consensus::LongestChain<FullBackend, Block>;

/// Parameters of test-client builder with test-runtime.
#[derive(Default)]
pub struct GenesisParameters;

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		selendra_test_service::chain_spec::selendra_local_testnet_genesis()
			.build_storage()
			.expect("Builds test runtime genesis storage")
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Build the test client.
	fn build(self) -> Client {
		self.build_with_longest_chain().0
	}

	/// Build the test client and longest chain selector.
	fn build_with_longest_chain(self) -> (Client, LongestChain);
}

impl TestClientBuilderExt for TestClientBuilder {
	fn build_with_longest_chain(self) -> (Client, LongestChain) {
		let executor = WasmExecutor::builder().build();
		let executor = client::LocalCallExecutor::new(
			self.backend().clone(),
			executor.clone(),
			Default::default(),
			ExecutionExtensions::new(
				Default::default(),
				None,
				sc_offchain::OffchainDb::factory_from_backend(&*self.backend()),
				Arc::new(executor),
			),
		)
		.unwrap();

		self.build_with_executor(executor)
	}
}

/// A `TestClientBuilder` with default backend and executor.
pub trait DefaultTestClientBuilderExt: Sized {
	/// Create new `TestClientBuilder`
	fn new() -> Self;
}

impl DefaultTestClientBuilderExt for TestClientBuilder {
	fn new() -> Self {
		Self::with_default_backend()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use sp_consensus::BlockOrigin;

	#[test]
	fn ensure_test_client_can_build_and_import_block() {
		let mut client = TestClientBuilder::new().build();

		let block_builder = client.init_selendra_block_builder();
		let block = block_builder.build().expect("Finalizes the block").block;

		futures::executor::block_on(client.import(BlockOrigin::Own, block))
			.expect("Imports the block");
	}

	#[test]
	fn ensure_test_client_can_push_extrinsic() {
		let mut client = TestClientBuilder::new().build();

		let transfer = construct_transfer_extrinsic(
			&client,
			sp_keyring::Sr25519Keyring::Alice,
			sp_keyring::Sr25519Keyring::Bob,
			1000,
		);
		let mut block_builder = client.init_selendra_block_builder();
		block_builder.push_selendra_extrinsic(transfer).expect("Pushes extrinsic");

		let block = block_builder.build().expect("Finalizes the block").block;

		futures::executor::block_on(client.import(BlockOrigin::Own, block))
			.expect("Imports the block");
	}
}
