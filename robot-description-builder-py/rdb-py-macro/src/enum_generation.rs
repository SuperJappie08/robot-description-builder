use crate::parse::Generics;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Ident, Token, TypePath};

use itertools::Itertools;

#[derive(Debug)]
pub struct Variant {
	pub variant_ident: Ident,
	pub ty: TokenStream,
	pub variant_snip: TokenStream,
}

pub fn generate_variants(
	mother_type: &Ident,
	generics: Punctuated<Generics, Token![,]>,
) -> Vec<Variant> {
	let mut gen_iter = generics.into_iter();

	let first = gen_iter.next().unwrap();
	gen_iter
		.fold(
			first.options.into_iter().map(|val| vec![val]).collect_vec(),
			|main: Vec<Vec<TypePath>>, iter| {
				main.into_iter()
					.cartesian_product(iter.options)
					.map(|tup| {
						let mut n = tup.0.clone();
						n.push(tup.1);
						n
					})
					.collect_vec()
			},
		)
		.into_iter()
		.map(|generic_types| generate_variant(mother_type, generic_types))
		.collect()
}

fn generate_variant(mother_type: &Ident, generic_types: Vec<TypePath>) -> Variant {
	let identifier = Ident::new(
		generic_types
			.iter()
			.cloned()
			.map(|t| t.path.get_ident().unwrap().clone())
			.join("")
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
