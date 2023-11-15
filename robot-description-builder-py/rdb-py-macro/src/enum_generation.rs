use crate::parse::GenericsEnumInput;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, TypePath};

use itertools::Itertools;

#[derive(Debug)]
pub struct Variant {
	pub variant_ident: Ident,
	pub ty: TokenStream,
	pub variant_snip: TokenStream,
}

pub fn generate_variants(input: &GenericsEnumInput) -> Vec<Variant> {
	let mut gen_iter = input.as_vec_of_vecs().into_iter();
	let counts = input.as_counts();

	let first = gen_iter.next().unwrap();
	gen_iter
		.fold(vec![first], |main: Vec<Vec<TypePath>>, iter| {
			main.into_iter()
				.cartesian_product(iter)
				.map(|tup| {
					let mut n = tup.0.clone();
					n.push(tup.1);
					n
				})
				.collect_vec()
		})
		.into_iter()
		.map(|generic_types| generate_variant(&input.mother_type, generic_types, &counts))
		.collect()
}

fn generate_variant(
	mother_type: &Ident,
	generic_types: Vec<TypePath>,
	counts: &[usize],
) -> Variant {
	let identifier = Ident::new(
		generic_types
			.to_vec()
			.iter()
			.zip_eq(counts.iter().copied())
			.filter(|(_, count)| *count > 1)
			.map(|(t, _)| t)
			.map(|t| t.path.get_ident().unwrap().clone())
			.join("")
			.replace("With", "")
			.replace("SafetyController", "SC")
			.replace("Calibration", "Calib")
			.replace("Dynamics", "Dynam")
			.as_str(),
		generic_types
			.first()
			.unwrap()
			.path
			.segments
			.last()
			.unwrap()
			.ident
			.span(),
	);

	let generics = quote! {
		#(#generic_types ,)*
	};

	let ty = quote! {
		#mother_type<#generics>
	};

	Variant {
		variant_ident: identifier.clone(),
		ty: ty.clone(),
		variant_snip: quote! {
			#identifier ( #ty ),
		},
	}
}
