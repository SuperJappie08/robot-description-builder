use robot_description_builder::smart_joint_extension::{
	smartparams::smart_joint_specification::JointTypeTrait,
	types::{ContinuousType, FixedType, FloatingType, PlanarType, PrismaticType, RevoluteType},
};
use syn::TypePath;

pub(crate) fn is_continous(ty: &TypePath) -> bool {
	match ty.path.segments.last().unwrap().ident.to_string().as_str() {
		stringify!(FixedType) => FixedType::IS_CONTINOUS,
		stringify!(FloatingType) => FloatingType::IS_CONTINOUS,
		stringify!(ContinuousType) => ContinuousType::IS_CONTINOUS,
		stringify!(PlanarType) => PlanarType::IS_CONTINOUS,
		stringify!(PrismaticType) => PrismaticType::IS_CONTINOUS,
		stringify!(RevoluteType) => RevoluteType::IS_CONTINOUS,
		_ => unreachable!(),
	}
}
