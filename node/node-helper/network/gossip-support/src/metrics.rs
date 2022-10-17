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

use selendra_node_subsystem_util::{
	metrics,
	metrics::{
		prometheus,
		prometheus::{Gauge, PrometheusError, Registry, U64},
	},
};

/// Dispute Distribution metrics.
#[derive(Clone, Default)]
pub struct Metrics(Option<MetricsInner>);

#[derive(Clone)]
struct MetricsInner {
	/// Tracks authority status for producing relay chain blocks.
	is_authority: Gauge<U64>,
	/// Tracks authority status for indracore approval checking.
	is_indracore_validator: Gauge<U64>,
}

impl Metrics {
	/// Dummy constructor for testing.
	#[cfg(test)]
	pub fn new_dummy() -> Self {
		Self(None)
	}

	/// Set the `relaychain validator` metric.
	pub fn on_is_authority(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_authority.set(1);
		}
	}

	/// Unset the `relaychain validator` metric.
	pub fn on_is_not_authority(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_authority.set(0);
		}
	}

	/// Set the `indracore validator` metric.
	pub fn on_is_indracore_validator(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_indracore_validator.set(1);
		}
	}

	/// Unset the `indracore validator` metric.
	pub fn on_is_not_indracore_validator(&self) {
		if let Some(metrics) = &self.0 {
			metrics.is_indracore_validator.set(0);
		}
	}
}

impl metrics::Metrics for Metrics {
	fn try_register(registry: &Registry) -> Result<Self, PrometheusError> {
		let metrics = MetricsInner {
			is_authority: prometheus::register(
				Gauge::new("selendra_node_is_active_validator", "Tracks if the validator is in the active set. \
				Updates at session boundary.")?,
				registry,
			)?,
			is_indracore_validator: prometheus::register(
				Gauge::new("selendra_node_is_indracore_validator",
				"Tracks if the validator participates in indracore consensus. Indracore validators are a \
				subset of the active set validators that perform approval checking of all indracore candidates in a session.\
				Updates at session boundary.")?,
				registry,
			)?,
		};
		Ok(Metrics(Some(metrics)))
	}
}
