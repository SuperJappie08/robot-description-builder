use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn impl_from_val2enum(enum_name: &Ident, variant: &Ident, ty: &TokenStream) -> TokenStream {
	quote! {
		impl ::core::convert::From<#ty> for #enum_name {
			fn from(value: #ty) -> Self {
				#enum_name::#variant(value)
			}
		}
	}
}

pub fn impl_from_enum2enum(
	start_enum: &Ident,
	target_enum: &Ident,
	variant_imp: &[(Ident, TokenStream)],
) -> TokenStream {
	let convs = variant_imp.iter().map(|(variant, _)| {
		quote! {
			#start_enum::#variant ( val ) => #target_enum::#variant ( val )
		}
	});

	quote! {
		impl ::core::convert::From<#start_enum> for #target_enum {
			fn from(value: #start_enum) -> Self {
				match value {
					#(#convs,)*
				}
			}
		}
	}
}

// TODO: Rename
pub fn impl_tryfrom_val_enum(
	enum_name: &Ident,
	variant: &Ident,
	ty: &TokenStream,
	err: &Ident,
) -> TokenStream {
	let from_impl = impl_from_val2enum(enum_name, variant, ty);

	quote! {
		#from_impl

		impl ::core::convert::TryFrom<#enum_name> for #ty {
			type Error = #err;

			fn try_from(value: #enum_name) -> Result<Self, Self::Error> {
				match value {
					#enum_name::#variant(value) => Ok(value),
					err => Err(err.into())
				}
			}
		}
	}
}
