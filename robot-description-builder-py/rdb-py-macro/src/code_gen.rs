use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, TypePath};

use crate::joint_type_info;

pub fn generate_smartjointbuilder_impls(
	enum_name: &Ident,
	base_type: &TypePath,
	variants_data: &[(Ident, TokenStream)],
	counts: [usize; 7],
	err_type: &Ident,
) -> Vec<TokenStream> {
	let mut impls = Vec::new();
	let is_continous = joint_type_info::is_continous(base_type);

	impls.push(impl_default(enum_name, variants_data));

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
		impls.push(impl_limit(enum_name, is_continous, variants_data, err_type));
	}

	// Mimic
	if counts[5] > 1 {
		impls.push(impl_mimic(enum_name, variants_data, err_type));
	}

	// Safety Controller
	if counts[6] > 1 {
		impls.push(impl_safety_controller(
			enum_name,
			is_continous,
			variants_data,
			err_type,
		));
	}

	impls
}

fn impl_safety_controller(
	enum_name: &Ident,
	is_continous: bool,
	variants_data: &[(Ident, TokenStream)],
	err_type: &Ident,
) -> TokenStream {
	let fn_with_safety_controller = gen_with_safety_controller(variants_data, err_type);
	let iter_with_sc = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithSafetyController"));

	let fn_set_k_velocity = gen_set_k_velocity(variants_data);
	let fn_get_k_velocity = gen_get_k_velocity(iter_with_sc.clone());

	let fn_set_k_position = gen_set_k_position(iter_with_sc.clone(), err_type);
	let fn_get_k_position = gen_get_k_position(iter_with_sc.clone());

	let mut fns = quote! {
		#fn_with_safety_controller

		#fn_set_k_velocity
		#fn_get_k_velocity

		#fn_set_k_position
		#fn_get_k_position
	};

	if !is_continous {
		let fn_set_soft_lower = gen_set_soft_lower(iter_with_sc.clone(), err_type);
		let fn_get_soft_lower = gen_get_soft_lower(iter_with_sc.clone());

		let fn_set_soft_upper = gen_set_soft_upper(iter_with_sc.clone(), err_type);
		let fn_get_soft_upper = gen_get_soft_upper(iter_with_sc);

		fns.extend(quote! {
			#fn_set_soft_lower
			#fn_get_soft_lower

			#fn_set_soft_upper
			#fn_get_soft_upper
		})
	}

	quote! {
		/// Methods to modify SafetyController.
		impl #enum_name {
			#fns
		}
	}
}

fn gen_set_soft_lower<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_sc.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_soft_lower_limit(soft_lower_limit).into()),
		}
	});

	quote! {
		pub fn set_soft_lower_limit(mut self, soft_lower_limit: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_soft_lower<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_sc.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.soft_lower_limit(),
		}
	});

	quote! {
		pub fn soft_lower_limit(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_soft_upper<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_sc.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_soft_upper_limit(soft_upper_limit).into()),
		}
	});

	quote! {
		pub fn set_soft_upper_limit(mut self, soft_upper_limit: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_soft_upper<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_sc.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.soft_upper_limit(),
		}
	});

	quote! {
		pub fn soft_upper_limit(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_k_position<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_sc.map(
		|(variant, _)| quote! { Self::#variant(value) => Ok(value.set_k_position(k_position).into()), },
	);

	quote! {
		pub fn set_k_position(mut self, k_position: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_k_position<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_sc
		.map(|(variant, _)| quote! { Self::#variant(value) => value.k_position(), });

	quote! {
		pub fn k_position(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_k_velocity(variants_data: &[(Ident, TokenStream)]) -> TokenStream {
	let variants = variants_data.iter().map(|(variant, ty)| {
		if ty.to_string().contains("WithSafetyController") {
			quote! { Self::#variant(value) => value.set_k_velocity(k_velocity).into(), }
		} else {
			quote! { Self::#variant(value) => value.with_safety_controller(k_velocity).into(), }
		}
	});

	quote! {
		pub fn set_k_velocity(mut self, k_velocity: f32) -> Self {
			match self {
				#(#variants)*
			}
		}
	}
}

fn gen_get_k_velocity<'a>(
	variants_with_sc: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_sc
		.map(|(variant, _)| quote! { Self::#variant(value) => Some(value.k_velocity()), });

	quote! {
		pub fn get_k_velocity(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_with_safety_controller(
	variants_data: &[(Ident, TokenStream)],
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("NoSafetyController"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => Ok(value.with_safety_controller(k_velocity).into()),
			}
		});

	quote! {
		pub fn with_safety_controller(mut self, k_velocity: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn impl_mimic(
	enum_name: &Ident,
	variants_data: &[(Ident, TokenStream)],
	err_type: &Ident,
) -> TokenStream {
	let fn_with_mimic = gen_with_mimic(variants_data, err_type);

	let with_iter = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithMimic"));

	let fn_set_name = gen_set_mimic_joint(with_iter.clone());
	let fn_get_name = gen_get_mimic_name(with_iter.clone());

	let fn_set_mimic_multiplier = gen_set_mimic_multiplier(with_iter.clone(), err_type);
	let fn_get_mimic_multiplier = gen_get_mimic_multiplier(with_iter.clone());

	let fn_set_mimic_offset = gen_set_mimic_offset(with_iter.clone(), err_type);
	let fn_get_mimic_offset = gen_get_mimic_offset(with_iter);

	quote! {
		/// Methods to modify Mimic.
		impl #enum_name {
			#fn_with_mimic

			#fn_set_name
			#fn_get_name

			#fn_set_mimic_multiplier
			#fn_get_mimic_multiplier

			#fn_set_mimic_offset
			#fn_get_mimic_offset
		}
	}
}

fn gen_set_mimic_offset<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_mimic.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_mimic_offset(offset).into()),
		}
	});

	quote! {
		pub fn set_mimic_offset(mut self, offset: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_mimic_offset<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_mimic.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.mimic_offset(),
		}
	});

	quote! {
		pub fn mimic_offset(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_mimic_multiplier<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_mimic.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_mimic_multiplier(multiplier).into()),
		}
	});

	quote! {
		pub fn set_mimic_multiplier(mut self, multiplier: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn gen_get_mimic_multiplier<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_mimic.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => value.mimic_multiplier(),
		}
	});

	quote! {
		pub fn mimic_multiplier(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_set_mimic_joint<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_mimic
		// The type stays the same however this is more readable.
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => value.set_mimiced_joint_name(mimiced_joint_name).into(),
			}
		});

	quote! {
		pub fn set_mimiced_joint_name(mut self,
			mimiced_joint_name: impl ::std::convert::Into<::std::string::String>,
		) -> Self {
			match self {
				#(#variants)*
				// This cannot fail
				other => other.with_mimic(mimiced_joint_name).unwrap(),
			}
		}
	}
}

fn gen_get_mimic_name<'a>(
	variants_with_mimic: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_mimic.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Some(value.mimiced_joint_name()),
		}
	});

	quote! {
		pub fn mimiced_joint_name(&self) -> Option<&String> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

fn gen_with_mimic(variants_data: &[(Ident, TokenStream)], err_type: &Ident) -> TokenStream {
	let variants = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("NoMimic"))
		.map(|(variant, _)| {
			quote! {
				Self::#variant(value) => Ok(value.with_mimic(mimiced_joint_name).into()),
			}
		});

	quote! {
		pub fn with_mimic(mut self,
			mimiced_joint_name: impl ::std::convert::Into<::std::string::String>,
		) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

fn impl_limit(
	enum_name: &Ident,
	is_continous: bool,
	variants_data: &[(Ident, TokenStream)],
	err_type: &Ident,
) -> TokenStream {
	let fn_with_limit = gen_with_limit(variants_data, err_type);
	let iter_with_limit = variants_data
		.iter()
		.filter(|(_, ty)| ty.to_string().contains("WithLimit"));

	let fn_set_effort = gen_set_effort(iter_with_limit.clone(), err_type);
	let fn_get_effort = gen_get_effort(iter_with_limit.clone());

	let fn_set_velocity = gen_set_velocity(iter_with_limit.clone(), err_type);
	let fn_get_velocity = gen_get_velocity(iter_with_limit.clone());

	let mut fns = quote! {
		#fn_with_limit

		#fn_set_effort
		#fn_get_effort

		#fn_set_velocity
		#fn_get_velocity
	};

	if !is_continous {
		let fn_set_upper = gen_set_upper_limit(iter_with_limit.clone(), err_type);
		let fn_get_upper = gen_get_upper_limit(iter_with_limit.clone());
		let fn_set_lower = gen_set_lower_limit(iter_with_limit.clone(), err_type);
		let fn_get_lower = gen_get_lower_limit(iter_with_limit);

		fns.extend(quote! {
			#fn_set_upper
			#fn_get_upper

			#fn_set_lower
			#fn_get_lower
		})
	};

	quote! {
		/// Methods to modify limit.
		impl #enum_name {
			#fns
		}
	}
}

#[inline]
fn gen_set_upper_limit<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_limit.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_upper_limit(upper_limit).into()),
		}
	});

	quote! {
		pub fn set_upper_limit(mut self, upper_limit: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

#[inline]
fn gen_get_upper_limit<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_limit
		.map(|(variant, _)| quote! { Self::#variant(value) => value.upper_limit(), });

	quote! {
		pub fn upper_limit(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
		}
	}
}

#[inline]
fn gen_set_lower_limit<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_limit.map(|(variant, _)| {
		quote! {
			Self::#variant(value) => Ok(value.set_lower_limit(lower_limit).into()),
		}
	});

	quote! {
		pub fn set_lower_limit(mut self, lower_limit: f32) -> Result<Self, #err_type> {
			match self {
				#(#variants)*
				err => Err(err.into()),
			}
		}
	}
}

#[inline]
fn gen_get_lower_limit<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_limit
		.map(|(variant, _)| quote! { Self::#variant(value) => value.lower_limit(), });

	quote! {
		pub fn lower_limit(&self) -> Option<f32> {
			match self {
				#(#variants)*
				_ => None,
			}
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

fn gen_set_velocity<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_limit.map(
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

fn gen_get_velocity<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_limit
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

fn gen_set_effort<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
	err_type: &Ident,
) -> TokenStream {
	let variants = variants_with_limit.map(|(variant, _)| {
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

fn gen_get_effort<'a>(
	variants_with_limit: impl Iterator<Item = &'a (Ident, TokenStream)>,
) -> TokenStream {
	let variants = variants_with_limit
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
		/// Methods to modify Dynamics.
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
		/// Methods to modify Calibration.
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
		/// Methods to modify axis.
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
