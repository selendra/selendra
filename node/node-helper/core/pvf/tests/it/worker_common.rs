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

use crate::PUPPET_EXE;
use selendra_node_core_pvf::testing::{spawn_with_program_path, SpawnErr};
use std::time::Duration;

// Test spawning a program that immediately exits with a failure code.
#[tokio::test]
async fn spawn_immediate_exit() {
	let result =
		spawn_with_program_path("integration-test", PUPPET_EXE, &["exit"], Duration::from_secs(2))
			.await;
	assert!(matches!(result, Err(SpawnErr::AcceptTimeout)));
}

#[tokio::test]
async fn spawn_timeout() {
	let result =
		spawn_with_program_path("integration-test", PUPPET_EXE, &["sleep"], Duration::from_secs(2))
			.await;
	assert!(matches!(result, Err(SpawnErr::AcceptTimeout)));
}

#[tokio::test]
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
