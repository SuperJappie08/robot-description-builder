use syn::{
	bracketed,
	parse::{Parse, ParseBuffer},
	punctuated::Punctuated,
	Ident, Token, TypePath,
};
pub struct GenericsEnumInput {
	pub mother_type: Ident,
	pub base_type: TypePath,
	pub axis: Punctuated<TypePath, Token![,]>,
	pub calibration: Punctuated<TypePath, Token![,]>,
	pub dynamics: Punctuated<TypePath, Token![,]>,
	pub limit: Punctuated<TypePath, Token![,]>,
	pub mimic: Punctuated<TypePath, Token![,]>,
	pub safety_controller: Punctuated<TypePath, Token![,]>,
}

impl Parse for GenericsEnumInput {
	fn parse(input: &ParseBuffer) -> syn::Result<Self> {
		let mother_type = input.parse()?;
		let base_type = input.parse()?;

		let content_axis;
		bracketed!(content_axis in input);

		let content_calibration;
		bracketed!(content_calibration in input);

		let content_dynamics;
		bracketed!(content_dynamics in input);

		let content_limit;
		bracketed!(content_limit in input);

		let content_mimic;
		bracketed!(content_mimic in input);

		let content_sc;
		bracketed!(content_sc in input);

		Ok(Self {
			mother_type,
			base_type,
			axis: content_axis.parse_terminated(TypePath::parse, Token![,])?,
			calibration: content_calibration.parse_terminated(TypePath::parse, Token![,])?,
			dynamics: content_dynamics.parse_terminated(TypePath::parse, Token![,])?,
			limit: content_limit.parse_terminated(TypePath::parse, Token![,])?,
			mimic: content_mimic.parse_terminated(TypePath::parse, Token![,])?,
			safety_controller: content_sc.parse_terminated(TypePath::parse, Token![,])?,
		})
	}
}

impl GenericsEnumInput {
	pub fn get_all_generic_types(&self) -> Vec<TypePath> {
		let mut types = vec![self.base_type.clone()];

		types.extend(self.axis.iter().cloned());
		types.extend(self.calibration.iter().cloned());
		types.extend(self.dynamics.iter().cloned());
		types.extend(self.limit.iter().cloned());
		types.extend(self.mimic.iter().cloned());
		types.extend(self.safety_controller.iter().cloned());

		types
	}

	pub fn as_vec_of_vecs(&self) -> Vec<Vec<TypePath>> {
		vec![
			vec![self.base_type.clone()],
			self.axis.iter().cloned().collect(),
			self.calibration.iter().cloned().collect(),
			self.dynamics.iter().cloned().collect(),
			self.limit.iter().cloned().collect(),
			self.mimic.iter().cloned().collect(),
			self.safety_controller.iter().cloned().collect(),
		]
	}

	pub fn as_counts(&self) -> [usize; 7] {
		[
			1,
			self.axis.len(),
			self.calibration.len(),
			self.dynamics.len(),
			self.limit.len(),
			self.mimic.len(),
			self.safety_controller.len(),
		]
	}
}
