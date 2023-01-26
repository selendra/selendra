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

use selendra_node_subsystem_util::metrics::{self, prometheus};

#[derive(Clone)]
struct MetricsInner {
	/// Tracks successful/unsuccessful inherent data requests
	inherent_data_requests: prometheus::CounterVec<prometheus::U64>,
	request_inherent_data: prometheus::Histogram,
	/// How much time the `RequestInherentData` processing takes
	request_inherent_data_duration: prometheus::Histogram,
	/// How much time `ProvisionableData` processing takes
	provisionable_data_duration: prometheus::Histogram,
	/// Bitfields array length in `ProvisionerInherentData` (the result for `RequestInherentData`)

	/// The following metrics track how many disputes/votes the runtime will have to process. These will count
	/// all recent statements meaning every dispute from last sessions: 4 hours on Selendra.
	/// The metrics are updated only when the node authors a block, so values vary across nodes.
	inherent_data_dispute_statement_sets: prometheus::Counter<prometheus::U64>,
	inherent_data_dispute_statements: prometheus::CounterVec<prometheus::U64>,
}

/// Provisioner metrics.
#[derive(Default, Clone)]
pub struct Metrics(Option<MetricsInner>);

impl Metrics {
	/// Creates new dummy `Metrics` instance. Used for testing only.
	#[cfg(test)]
	pub fn new_dummy() -> Metrics {
		Metrics(None)
	}

	pub(crate) fn on_inherent_data_request(&self, response: Result<(), ()>) {
		if let Some(metrics) = &self.0 {
			match response {
				Ok(()) => metrics.inherent_data_requests.with_label_values(&["succeeded"]).inc(),
				Err(()) => metrics.inherent_data_requests.with_label_values(&["failed"]).inc(),
			}
		}
	}

	/// Provide a timer for `request_inherent_data` which observes on drop.
	pub(crate) fn time_request_inherent_data(
		&self,
	) -> Option<metrics::prometheus::prometheus::HistogramTimer> {
		self.0
			.as_ref()
			.map(|metrics| metrics.request_inherent_data_duration.start_timer())
	}

	/// Provide a timer for `provisionable_data` which observes on drop.
	pub(crate) fn time_provisionable_data(
		&self,
	) -> Option<metrics::prometheus::prometheus::HistogramTimer> {
		self.0.as_ref().map(|metrics| metrics.provisionable_data_duration.start_timer())
	}

	pub(crate) fn observe_inherent_data_bitfields_count(&self, bitfields_count: usize) {
		self.0.as_ref().map(|metrics| {
			metrics.inherent_data_response_bitfields.observe(bitfields_count as f64)
		});
	}

	pub(crate) fn inc_valid_statements_by(&self, votes: usize) {
		if let Some(metrics) = &self.0 {
			metrics
				.inherent_data_dispute_statements
				.with_label_values(&["valid"])
				.inc_by(votes.try_into().unwrap_or(0));
		}
	}

	pub(crate) fn inc_invalid_statements_by(&self, votes: usize) {
		if let Some(metrics) = &self.0 {
			metrics
				.inherent_data_dispute_statements
				.with_label_values(&["invalid"])
				.inc_by(votes.try_into().unwrap_or(0));
		}
	}

	pub(crate) fn inc_dispute_statement_sets_by(&self, disputes: usize) {
		if let Some(metrics) = &self.0 {
			metrics
				.inherent_data_dispute_statement_sets
				.inc_by(disputes.try_into().unwrap_or(0));
		}
	}
}

impl metrics::Metrics for Metrics {
	fn try_register(registry: &prometheus::Registry) -> Result<Self, prometheus::PrometheusError> {
		let metrics = MetricsInner {
			inherent_data_requests: prometheus::register(
				prometheus::CounterVec::new(
					prometheus::Opts::new(
						"selendra_parachain_inherent_data_requests_total",
						"Number of InherentData requests served by provisioner.",
					),
					&["success"],
				)?,
				registry,
			)?,
			request_inherent_data_duration: prometheus::register(
				prometheus::Histogram::with_opts(prometheus::HistogramOpts::new(
					"selendra_parachain_provisioner_request_inherent_data_time",
					"Time spent within `provisioner::request_inherent_data`",
				))?,
				registry,
			)?,
			provisionable_data_duration: prometheus::register(
				prometheus::Histogram::with_opts(prometheus::HistogramOpts::new(
					"selendra_parachain_provisioner_provisionable_data_time",
					"Time spent within `provisioner::provisionable_data`",
				))?,
				registry,
			)?,
			inherent_data_dispute_statements: prometheus::register(
				prometheus::CounterVec::new(
					prometheus::Opts::new(
						"selendra_parachain_inherent_data_dispute_statements",
						"Number of dispute statements passed to `create_inherent()`.",
					),
					&["validity"],
				)?,
				&registry,
			)?,
			inherent_data_dispute_statement_sets: prometheus::register(
				prometheus::Counter::new(
					"selendra_parachain_inherent_data_dispute_statement_sets",
					"Number of dispute statements sets passed to `create_inherent()`.",
				)?,
				registry,
			)?,
			inherent_data_response_bitfields: prometheus::register(
				prometheus::Histogram::with_opts(
					prometheus::HistogramOpts::new(
						"selendra_parachain_provisioner_inherent_data_response_bitfields_sent",
						"Number of inherent bitfields sent in response to `ProvisionerMessage::RequestInherentData`.",
					).buckets(vec![0.0, 10.0, 25.0, 50.0, 75.0, 100.0, 150.0, 200.0, 250.0, 300.0]),
				)?,
				registry,
			)?,
		};
		Ok(Metrics(Some(metrics)))
	}
}
