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

//! Execution part of the pipeline.
//!
//! The validation host [runs the queue][`start`] communicating with it by sending [`ToQueue`]
//! messages. The queue will spawn workers in new processes. Those processes should jump to
//! `selendra_node_core_pvf_worker::execute_worker_entrypoint`.

mod queue;
mod worker_intf;

pub use queue::{start, PendingExecutionRequest, ToQueue};
