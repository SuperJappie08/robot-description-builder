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

/// A trait to filter the non-continuous and continuous [SmartJoint Types](super::smartjointtypes) with trait bounds.
pub trait SmartJointTypeTrait<const IS_CONTINOUS: bool>: JointTypeTrait {}

/// A trait to designate `SmartJointBuilder` `JointType` types.
pub trait JointTypeTrait: Copy + Default + Into<JointType> {
	/// Designates if the JointType is continous or not.
	///
	/// This must match with the implementation of [`SmartJointTypeTrait`].
	const IS_CONTINOUS: bool;

	/// Return the [`JointType`] associated with this [`SmartJointType`](super::smartjointtypes).
	#[inline]
	fn as_type(&self) -> JointType {
		(*self).into()
	}
}

/// An unspecified `JointType` for `SmartJointBuilder`.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct NoType;

pub use continuous_joint_type::ContinuousType;
pub use fixed_joint_type::FixedType;
pub use floating_joint_type::FloatingType;
pub use planar_joint_type::PlanarType;
pub use prismatic_joint_type::PrismaticType;
pub use revolute_joint_type::RevoluteType;

use crate::JointType;
