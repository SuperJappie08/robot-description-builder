use syn::{
	braced,
	parse::{Parse, ParseBuffer},
	punctuated::Punctuated,
	Ident, Token, TypePath,
};
pub struct GenericsEnumInput {
	pub mother_type: Ident,
	pub generics: Punctuated<Generics, Token![,]>,
}

impl Parse for GenericsEnumInput {
	fn parse(input: &ParseBuffer) -> syn::Result<Self> {
		let mother_type = input.parse()?;
		let content;
		braced!(content in input);
		Ok(Self {
			mother_type,
			generics: content.parse_terminated(Generics::parse, Token![,])?,
		})
	}
}

#[derive(Clone)]
pub struct Generics {
	pub options: Punctuated<TypePath, Token![,]>,
}

impl Parse for Generics {
	fn parse(input: &ParseBuffer) -> syn::Result<Self> {
		let content;
		syn::bracketed!(content in input);
		Ok(Self {
			options: content.parse_terminated(TypePath::parse, Token![,])?,
		})
	}
}
