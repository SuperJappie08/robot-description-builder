mod code_gen;
mod conversion_impls;
mod enum_generation;
mod error_gen;
mod joint_type_info;
mod parse;

use enum_generation::generate_variants;
use parse::GenericsEnumInput;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

use itertools::Itertools;

/// TODO: Add field for IsContinous.
#[proc_macro]
pub fn enum_generic_state(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let input = parse_macro_input!(tokens as GenericsEnumInput);

	let mother_type = &input.mother_type;
	let joint_type = &input.base_type;

	let all_generics = input
		.get_all_generic_types()
		.into_iter()
		.unique_by(|ty| format!("{}", ty.path.get_ident().unwrap()));

	let enum_name = {
		Ident::new(
			format!(
				"{}{}",
				joint_type.path.segments.last().unwrap().ident,
				mother_type
			)
			.replace("Type", "")
			.as_str(),
			mother_type.span(),
		)
	};

	let mod_name = {
		Ident::new(
			format!(
				"{}_{}",
				joint_type.path.segments.last().unwrap().ident,
				mother_type
			)
			.to_lowercase()
			.replace("type", "")
			.as_str(),
			mother_type.span(),
		)
	};

	let variants_data = generate_variants(&input);

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

	let impls = code_gen::generate_smartjointbuilder_impls(
		&enum_name,
		&input.base_type,
		&variant_info,
		input.as_counts(),
		&err_type,
	);

	let intos = variant_info.iter().map(|(variant, ty)| {
		conversion_impls::impl_tryfrom_val_enum(&enum_name, variant, ty, &err_type)
	});

	quote! {
		mod #mod_name {
			use super::{#mother_type, #(#all_generics,)*};

			#[derive(::core::fmt::Debug, ::core::clone::Clone)]
			pub(super) enum #enum_name {
				#(#variants)*
			}

			#(#intos)*

			#errors

			#(#impls)*
		}

		use #mod_name::{#enum_name, #err_type};
	}
	.into()
}
