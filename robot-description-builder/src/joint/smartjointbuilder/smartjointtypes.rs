macro_rules! impl_jointtype_traits {
	($obj:ty, $val:literal) => {
		impl crate::joint::smartjointbuilder::smartjointtypes::SmartJointTypeTrait<$val> for $obj {}
		impl crate::joint::smartjointbuilder::smartjointtypes::JointTypeTrait for $obj {
			const IS_CONTINOUS: bool = $val;
		}
	};
}

mod continuous_joint_type;
mod fixed_joint_type;
mod floating_joint_type;
mod planar_joint_type;
mod prismatic_joint_type;
mod revolute_joint_type;

pub trait SmartJointTypeTrait<const IS_CONTINOUS: bool>: JointTypeTrait {}

pub trait JointTypeTrait: Copy + Default + Into<JointType> {
	const IS_CONTINOUS: bool;

	#[inline]
	fn as_type(&self) -> JointType {
		(*self).into()
	}
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct NoType;

pub use continuous_joint_type::ContinuousType;
pub use fixed_joint_type::FixedType;
pub use floating_joint_type::FloatingType;
pub use planar_joint_type::PlanarType;
pub use prismatic_joint_type::PrismaticType;
pub use revolute_joint_type::RevoluteType;

use crate::JointType;
