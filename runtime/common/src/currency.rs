use super::{Balance, CurrencyId, Get, GetByKey, PhantomData, TokenInfo};

// TODO: make those const fn
pub fn dollar(currency_id: CurrencyId) -> Balance {
	10u128.saturating_pow(currency_id.decimals().expect("Not support Non-Token decimals").into())
}

pub fn cent(currency_id: CurrencyId) -> Balance {
	dollar(currency_id) / 100
}

pub fn millicent(currency_id: CurrencyId) -> Balance {
	cent(currency_id) / 1000
}

pub fn microcent(currency_id: CurrencyId) -> Balance {
	millicent(currency_id) / 1000
}

pub struct ExistentialDepositsTimesOneHundred<NativeCurrencyId, NativeED, OtherEDs>(
	PhantomData<(NativeCurrencyId, NativeED, OtherEDs)>,
);

impl<
		NativeCurrencyId: Get<CurrencyId>,
		NativeED: Get<Balance>,
		OtherEDs: GetByKey<CurrencyId, Balance>,
	> GetByKey<CurrencyId, Balance>
	for ExistentialDepositsTimesOneHundred<NativeCurrencyId, NativeED, OtherEDs>
{
	fn get(currency_id: &CurrencyId) -> Balance {
		if *currency_id == NativeCurrencyId::get() {
			NativeED::get().saturating_mul(100u128)
		} else {
			OtherEDs::get(currency_id).saturating_mul(100u128)
		}
	}
}

// The type used for currency conversion.
///
/// This must only be used as long as the balance type is `u128`.
pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
static_assertions::assert_eq_size!(primitives::Balance, u128);
