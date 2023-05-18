use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;

#[derive(Encode, Decode, PartialEq, Clone, Debug)]
pub struct PhantomType<T>(core::marker::PhantomData<T>);

impl<T: TypeInfo + 'static> TypeInfo for PhantomType<T> {
	type Identity = PhantomType<T>;

	fn type_info() -> scale_info::Type {
		use scale_info::{build::Fields, type_params, Path, Type};
		Type::builder()
			.path(Path::new("phantom_type", "PhantomType"))
			.type_params(type_params!(T))
			.composite(Fields::unnamed().field(|b| b.ty::<[T; 0]>()))
	}
}
impl<T> MaxEncodedLen for PhantomType<T> {
	fn max_encoded_len() -> usize {
		0
	}
}
