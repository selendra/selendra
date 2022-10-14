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

//! Client side declaration and registration of the indracore Prometheus metrics.
//! All of the metrics have a correspondent runtime metric definition.

use crate::runtime::RuntimeMetricsProvider;
use primitives::v2::metric_definitions::{
	INDRACORE_CREATE_INHERENT_BITFIELDS_SIGNATURE_CHECKS,
	INDRACORE_INHERENT_DATA_BITFIELDS_PROCESSED, INDRACORE_INHERENT_DATA_CANDIDATES_PROCESSED,
	INDRACORE_INHERENT_DATA_DISPUTE_SETS_INCLUDED, INDRACORE_INHERENT_DATA_DISPUTE_SETS_PROCESSED,
	INDRACORE_INHERENT_DATA_WEIGHT,
};

/// Register the indracore runtime metrics.
pub fn register_metrics(runtime_metrics_provider: &RuntimeMetricsProvider) {
	runtime_metrics_provider.register_counter(INDRACORE_INHERENT_DATA_DISPUTE_SETS_INCLUDED);
	runtime_metrics_provider.register_counter(INDRACORE_INHERENT_DATA_BITFIELDS_PROCESSED);

	runtime_metrics_provider.register_countervec(INDRACORE_INHERENT_DATA_WEIGHT);
	runtime_metrics_provider.register_countervec(INDRACORE_INHERENT_DATA_DISPUTE_SETS_PROCESSED);
	runtime_metrics_provider.register_countervec(INDRACORE_INHERENT_DATA_CANDIDATES_PROCESSED);
	runtime_metrics_provider
		.register_countervec(INDRACORE_CREATE_INHERENT_BITFIELDS_SIGNATURE_CHECKS);
}
