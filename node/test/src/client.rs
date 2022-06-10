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
// along with Selendra.  If not, see <http://www.gnu.org/licenses/>.

//! Utilities to build a `TestClient` for `node-runtime`.

use sc_service::client;
use sp_runtime::BuildStorage;
/// Re-export test-client utilities.
pub use substrate_test_client::*;

/// Call executor for `node-runtime` `TestClient`.
pub type ExecutorDispatch =
	sc_executor::NativeElseWasmExecutor<selendra_node_executor::ExecutorDispatch>;

/// Default backend type.
pub type Backend = sc_client_db::Backend<selendra_primitives::Block>;

/// Test client type.
pub type Client = client::Client<
	Backend,
	client::LocalCallExecutor<selendra_primitives::Block, Backend, ExecutorDispatch>,
	selendra_primitives::Block,
	selendra_runtime::RuntimeApi,
>;

/// Transaction for node-runtime.
pub type Transaction = sc_client_api::backend::TransactionFor<Backend, selendra_primitives::Block>;

/// Genesis configuration parameters for `TestClient`.
#[derive(Default)]
pub struct GenesisParameters;

impl substrate_test_client::GenesisInit for GenesisParameters {
	fn genesis_storage(&self) -> Storage {
		crate::genesis::config(None).build_storage().unwrap()
	}
}

/// A `test-runtime` extensions to `TestClientBuilder`.
pub trait TestClientBuilderExt: Sized {
	/// Create test client builder.
	fn new() -> Self;

	/// Build the test client.
	fn build(self) -> Client;
}

impl TestClientBuilderExt
	for substrate_test_client::TestClientBuilder<
		selendra_primitives::Block,
		client::LocalCallExecutor<selendra_primitives::Block, Backend, ExecutorDispatch>,
		Backend,
		GenesisParameters,
	>
{
	fn new() -> Self {
		Self::default()
	}

	fn build(self) -> Client {
		self.build_with_native_executor(None).0
	}
}
