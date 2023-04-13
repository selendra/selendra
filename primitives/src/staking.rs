// Copyright 2023 Smallworld Selendra
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

use sp_runtime::Perbill;

use crate::{Balance, TOKEN};

pub const MIN_VALIDATOR_BOND: u128 = 25_000 * TOKEN;
pub const MIN_NOMINATOR_BOND: u128 = 100 * TOKEN;
pub const MAX_NOMINATORS_REWARDED_PER_VALIDATOR: u32 = 1024;
pub const YEARLY_INFLATION: Balance = 10_000_000 * TOKEN;
pub const VALIDATOR_REWARD: Perbill = Perbill::from_percent(90);

pub fn era_payout(miliseconds_per_era: u64) -> (Balance, Balance) {
	// Milliseconds per year for the Julian year (365.25 days).
	const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;

	let portion = Perbill::from_rational(miliseconds_per_era, MILLISECONDS_PER_YEAR);
	let total_payout = portion * YEARLY_INFLATION;
	let validators_payout = VALIDATOR_REWARD * total_payout;
	let rest = total_payout - validators_payout;

	(validators_payout, rest)
}

/// Macro for making a default implementation of non-self methods from given class.
///
/// As an input it expects list of tuples of form
///
/// `(method_name(arg1: type1, arg2: type2, ...), class_name, return_type)`
///
/// where
///   * `method_name`is a wrapee method,
///   * `arg1: type1, arg2: type,...`is a list of arguments and will be passed as is, can be empty
///   * `class_name`is a class that has non-self `method-name`,ie symbol `class_name::method_name` exists,
///   * `return_type` is type returned from `method_name`
/// Example
/// ```ignore
/// wrap_methods!(
///     (bond(), SubstrateStakingWeights, Weight),
///     (bond_extra(), SubstrateStakingWeights, Weight)
/// );
/// ```
#[macro_export]
macro_rules! wrap_methods {
    ($(($wrapped_method:ident( $($arg_name:ident: $argument_type:ty), *), $wrapped_class:ty, $return_type:ty)), *) => {
        $(
            fn $wrapped_method($($arg_name: $argument_type), *) -> $return_type {
                <$wrapped_class>::$wrapped_method($($arg_name), *)
            }
        )*
    };
}
