use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{
		jointbuilder::{BuildJoint, JointBuilder},
		smartjointbuilder::{
			smartparams::{
				NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController,
			},
			SmartJointBuilder,
		},
		Joint, JointType,
	},
	link::Link,
	utils::{ArcLock, WeakLock},
};

// TODO: Maybe flip the JointType Doc order
/// A representation of a fixed joint (`JointType::Fixed`) for the `SmartJointBuilder`.
///
/// See [`JointType::Fixed`] for more details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct FixedType;

impl_jointtype_traits!(FixedType, false);

impl From<FixedType> for JointType {
	fn from(_value: FixedType) -> Self {
		JointType::Fixed
	}
}

impl BuildJoint
	for SmartJointBuilder<
		FixedType,
		NoAxis,
		NoCalibration,
		NoDynamics,
		NoLimit,
		NoMimic,
		NoSafetyController,
	>
{
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_shape_data: crate::link::LinkShapeData,
	) -> ArcLock<Joint> {
		Into::<JointBuilder>::into(self).build(tree, parent_link, child_link, parent_shape_data)
	}
}

impl
	From<
		SmartJointBuilder<
			FixedType,
			NoAxis,
			NoCalibration,
			NoDynamics,
			NoLimit,
			NoMimic,
			NoSafetyController,
		>,
	> for JointBuilder
{
	fn from(
		value: SmartJointBuilder<
			FixedType,
			NoAxis,
			NoCalibration,
			NoDynamics,
			NoLimit,
			NoMimic,
			NoSafetyController,
		>,
	) -> Self {
		let mut joint_builder = JointBuilder::new(value.name, value.joint_type.into());

		joint_builder.with_transform(value.transform.unwrap_or_default());
		joint_builder
	}
}
