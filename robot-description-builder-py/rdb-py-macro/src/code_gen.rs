use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, TypePath};

pub fn generate_smartjointbuilder_impls(
	enum_name: &Ident,
	base_type: &TypePath,
	variants_data: &[(Ident, TokenStream)],
	counts: [usize; 7],
	err_type: &Ident,
) -> Vec<TokenStream> {
	let mut impls = Vec::new();

	impls.push(impl_default(enum_name, &variants_data));

	impls.push(standard_impls(enum_name, variants_data, err_type));

	// Axis
	if counts[1] > 1 {
		impls.push(impl_axis(enum_name, variants_data));
	}

	// Calibration
	if counts[2] > 1 {
		impls.push(impl_calibration(enum_name, variants_data));
	}

	// Dynamics
	if counts[3] > 1 {
		impls.push(impl_dynamics(enum_name, variants_data));
	}

	// Limit
	if counts[4] > 1 {
		impls.push(impl_limit(enum_name, base_type, variants_data, err_type));
	}

	impls
}

fn impl_limit(
	enum_name: &Ident,
	_base_type: &TypePath,
	variants_data: &[(Ident, TokenStream)],
	err_type: &Ident,
) -> TokenStream {
	let fn_with_limit = gen_with_limit(variants_data, err_type);
	let fn_set_effort = gen_set_effort(variants_data, err_type);
	let fn_get_effort = gen_get_effort(variants_data);

	let fn_set_velocity = gen_set_velocity(variants_data, err_type);
	let fn_get_velocity = gen_get_velocity(variants_data);

	// FIXME: Prevent the generation of function when continous.
    // TODO: Acual limit, which migth lead to compiler errors... 😠

	quote! {
		/// Methods to modify limit
		impl #enum_name {
			#fn_with_limit

			#fn_set_effort
			#fn_get_effort

			#fn_set_velocity
            #fn_get_velocity
		}
	}
}

fn gen_with_limit(variants_data: &[(Ident, TokenStream)], err_type: &Ident) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("NoLimit"))
		.map(
			|(variant, _)| quote! { Self::#variant(value) => Ok(value.with_limit(effort, velocity).into()), },
		);

	quote! {
		pub fn with_limit(mut self, effort: f32, velocity: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_set_velocity(variants_data: &[(Ident, TokenStream)], err_type: &Ident) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithLimit"))
		.map(
			|(variant, _)| quote! { Self::#variant(value) => Ok(value.set_velocity(velocity).into()), },
		);

	quote! {
		pub fn set_velocity(mut self, velocity: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_velocity(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithLimit"))
		.map(|(variant, _)| quote! { Self::#variant(value) => Some(value.velocity()), });

	quote! {
		pub fn velocity(&self) -> Option<f32> {
            match self {
                #(#variants)*
                _ => None,
            }
		}
	}
}

fn gen_set_effort(variants_data: &[(Ident, TokenStream)], err_type: &Ident) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithLimit"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => Ok(value.set_effort(effort).into()),
			}
		});

	quote! {
		pub fn set_effort(mut self, effort: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_effort(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithLimit"))
		.map(|(variant, _)| quote! { Self::#variant(value) => Some(value.effort()), });

	quote! {
		pub fn effort(&self) -> Option<f32> {
            match self {
                #(#variants)*
                _ => None,
            }
		}
	}
}

fn impl_dynamics(enum_name: &Ident, variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let fn_set_damping = gen_set_damping(variants_data);
	let fn_get_damping = gen_get_damping(variants_data);
	let fn_set_friction = gen_set_friction(variants_data);
	let fn_get_friction = gen_get_friction(variants_data);

	quote! {
		/// Methods to modify Dynamics
		impl #enum_name {
			#fn_set_damping
			#fn_get_damping

			#fn_set_friction
			#fn_get_friction
		}
	}
}

fn gen_get_friction(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithDynamics"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => value.friction(),
			}
		});

	quote! {
		pub fn friction(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_friction(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, ty)| {
		if ty.to_string().contains("NoDynamics") {
			quote! {
				Self::#variant(value) => value.with_dynamics().set_friction(friction).into(),
			}
		} else {
			quote! {
				Self::#variant(value) => value.set_friction(friction).into(),
			}
		}
	});
	quote! {
		pub fn set_friction(mut self, friction: f32) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_get_damping(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithDynamics"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => value.damping(),
			}
		});

	quote! {
		pub fn damping(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_damping(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, ty)| {
		if ty.to_string().contains("NoDynamics") {
			quote! {
				Self::#variant(value) => value.with_dynamics().set_damping(damping).into(),
			}
		} else {
			quote! {
				Self::#variant(value) => value.set_damping(damping).into(),
			}
		}
	});
	quote! {
		pub fn set_damping(mut self, damping: f32) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn impl_calibration(enum_name: &Ident, variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let fn_set_rising_calibration = gen_set_rising_calibration(variants_data);
	let fn_get_rising = gen_get_rising(variants_data);
	let fn_set_falling_calibration = gen_set_falling_calibration(variants_data);
	let fn_get_falling = gen_get_falling(variants_data);

	quote! {
		/// Methods to modify Calibration
		impl #enum_name {
			#fn_set_rising_calibration
			#fn_get_rising

			#fn_set_falling_calibration
			#fn_get_falling
		}
	}
}

fn gen_set_rising_calibration(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, ty)| {
		// Seperate NoCalibration from others
		if ty.to_string().contains("NoCalibration") {
			quote! {
				Self::#variant(value) => value.with_calibration().set_rising_calibration(rising).into(),
			}
		} else {
			quote! {
				Self::#variant(value) => value.set_rising_calibration(rising).into(),
			}
		}
	});

	quote! {
		/// Sets the rising calibration value. `NoCalibration` will be upgraded.
		pub fn set_rising_calibration(mut self, rising: f32) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_get_rising(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| !ty.to_string().contains("NoCalibration"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => value.rising_calibration(),
			}
		});

	quote! {
		pub fn rising_calibration(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_falling_calibration(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, ty)| {
		// Seperate NoCalibration from others
		if ty.to_string().contains("NoCalibration") {
			quote! {
				Self::#variant(value) => value.with_calibration().set_falling_calibration(falling).into(),
			}
		} else {
			quote! {
				Self::#variant(value) => value.set_falling_calibration(falling).into(),
			}
		}
	});

	quote! {
		/// Sets the falling calibration value. `NoCalibration` will be upgraded.
		pub fn set_falling_calibration(mut self, falling: f32) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_get_falling(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| !ty.to_string().contains("NoCalibration"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => value.falling_calibration(),
			}
		});

	quote! {
		pub fn falling_calibration(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn impl_axis(enum_name: &Ident, variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let fn_with_axis = gen_with_axis(variants_data);
	let fn_get_axis = gen_get_axis(variants_data);

	quote! {
		/// Methods to modify axis
		impl #enum_name {
			#fn_with_axis
			#fn_get_axis
		}
	}
}

fn gen_get_axis(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithAxis"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => Some(value.axis()),
			}
		});

	quote! {
		pub fn axis(&self) -> Option<(f32, f32, f32)> {
			match self {
				#(#variants)*
				_ => None
			}
		}
	}
}

fn gen_with_axis(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.with_axis(axis).into(),
		}
	});

	quote! {
		pub fn with_axis(mut self, axis: (f32, f32, f32)) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn impl_default(enum_name: &Ident, variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let default_variant = &variants_data[0].0;
	quote! {
		impl ::core::default::Default for #enum_name {
			fn default() -> Self {
				Self::#default_variant(::robot_description_builder::SmartJointBuilder::default().into())
			}
		}
	}
}

fn standard_impls(
	enum_name: &Ident,
	variants_data: &[(Ident, TokenStream)],
	_err_type: &Ident,
) -> TokenStream {
	let fn_new = gen_new(variants_data);
	let fn_add_transform = gen_add_transform(variants_data);
	let fn_rename = gen_rename(variants_data);
	let fn_as_simple = gen_as_simple(variants_data);

	// TODO: ? Build ?

	quote! {
		impl #enum_name {
			#fn_new
			// TODO: dynamic transform
			#fn_add_transform
			#fn_rename

			#fn_as_simple
		}
	}
}

fn gen_new(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let default = variants_data[0].0.clone();
	quote! {
		pub fn new(name: impl ::core::convert::Into<String>) -> Self {
			match Self::default() {
				Self::#default(value) => value.rename(name).into(),
				_ => ::core::unreachable!(),
			}
		}
	}
}

fn gen_rename(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.rename(name).into(),
		}
	});

	quote! {
		/// Renames the `SmartJointBuilder`.
		pub fn rename(mut self, name: impl ::core::convert::Into<String>) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_add_transform(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.add_transform(transform).into(),
		}
	});

	quote! {
		pub fn add_transform(mut self, transform: ::robot_description_builder::Transform) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_as_simple(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, _)| {
		quote! {
			Self::#variant(val) => unsafe{ val.as_simple() }
		}
	});

	quote! {
		pub fn as_simple(&self) -> ::robot_description_builder::JointBuilder {
			match self {
				#(#variants ,)*
			}
		}
	}
}
