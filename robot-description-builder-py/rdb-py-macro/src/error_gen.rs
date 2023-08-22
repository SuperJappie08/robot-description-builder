use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::conversion_impls::{impl_from_enum2enum, impl_from_val2enum};

// TODO: Maybe make a struct Error(GeneratedEnum)
pub fn generate_state_error(
	enum_name: &Ident,
	variant_data: &[(Ident, TokenStream)],
) -> (Ident, TokenStream) {
	let err_name = Ident::new(format!("{enum_name}StateError").as_str(), enum_name.span());

	let (variants, conversion): (Vec<_>, Vec<_>) = variant_data
		.iter()
		.cloned()
		.map(|(var_name, ty)| {
			let error_txt = format!("{} could not be converted to the target type", ty);
			(
				quote! {
					#[error(#error_txt)]
					#var_name(#ty)
				},
				impl_from_val2enum(&err_name, &var_name, &ty),
			)
		})
		.unzip();

	let enum_convert = impl_from_enum2enum(enum_name, &err_name, variant_data);

	(
		err_name.clone(),
		quote! {
			#[derive(Debug, ::thiserror::Error)]
			pub enum #err_name {
				#(#variants, )*
			}

			#(#conversion)*
			#enum_convert
		},
	)
}
