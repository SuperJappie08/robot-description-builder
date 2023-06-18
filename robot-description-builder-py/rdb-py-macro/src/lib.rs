mod conversion_impls;
mod enum_generation;
mod error_gen;
mod parse;

use enum_generation::generate_variants;
use parse::GenericsEnumInput;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

use itertools::Itertools;

#[proc_macro]
pub fn enum_generic_state(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(tokens as GenericsEnumInput);
	let mother_type = input.mother_type;

	let all_generics = input
		.generics
		.clone()
		.into_iter()
		.flat_map(|g| g.options.into_iter())
		.unique_by(|ty| format!("{}", ty.path.get_ident().unwrap()));

	let enum_name = {
		let joint_type = input.generics.first().unwrap().options.first().unwrap();
		Ident::new(
			format!(
				"{}{}",
				joint_type.path.segments.last().unwrap().ident,
				mother_type
			)
			.as_str(),
			mother_type.span(),
		)
	};

	let mod_name = {
		let joint_type = input.generics.first().unwrap().options.first().unwrap();
		Ident::new(
			format!(
				"{}_{}",
				joint_type.path.segments.last().unwrap().ident,
				mother_type
			)
			.to_lowercase()
			.as_str(),
			mother_type.span(),
		)
	};

	let variants_data = generate_variants(&mother_type, input.generics);

	let (variant_info, variants): (Vec<(Ident, TokenStream)>, Vec<TokenStream>) = variants_data
		.into_iter()
		.map(|variant_data| {
			(
				(variant_data.variant_ident, variant_data.ty),
				variant_data.variant_snip,
			)
		})
		.unzip();

	let (err_type, errors) = error_gen::generate_state_error(&enum_name, &variant_info);
	let intos = variant_info.iter().map(|(variant, ty)| {
		conversion_impls::impl_tryfrom_val_enum(&enum_name, variant, ty, &err_type)
	});

	quote! {
		mod #mod_name {
			use super::{#mother_type, #(#all_generics,)*};

			pub(super) enum #enum_name {
				#(#variants)*
			}

			#(#intos)*

			#errors
		}

		use #mod_name::{#enum_name, #err_type};
	}
	.into()
}
