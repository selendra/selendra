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

use crate::PUPPET_EXE;
use selendra_node_core_pvf::testing::worker_common::{spawn_with_program_path, SpawnErr};
use std::time::Duration;

#[async_std::test]
async fn spawn_timeout() {
	let result =
		spawn_with_program_path("integration-test", PUPPET_EXE, &["sleep"], Duration::from_secs(2))
			.await;
	assert!(matches!(result, Err(SpawnErr::AcceptTimeout)));
}

#[async_std::test]
async fn should_connect() {
	let _ = spawn_with_program_path(
		"integration-test",
		PUPPET_EXE,
		&["prepare-worker"],
		Duration::from_secs(2),
	)
	.await
	.unwrap();
}
