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
	ArcLock, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct FixedType;

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

		// let joint_builder = if let Some(mode) = value.offset {
		// 	joint_builder.add_origin_offset(match mode {
		// 		OffsetMode::Offset(x, y, z) => (x, y, z),
		// 		// FIXME: This is incoorect because it doesn't take rotations into account
		// 		OffsetMode::FigureItOut(_) => parent_link
		// 			.upgrade()
		// 			.unwrap()
		// 			.try_read()
		// 			.unwrap()
		// 			.get_end_point()
		// 			.expect("No ENDPOINT"), //TODO: FIX this
		// 	})
		// } else {
		// 	joint_builder
		// };
		//
		// let joint_builder = if let Some(rotation) = value.rotation {
		// 	joint_builder.add_origin_rotation(rotation)
		// } else {
		// 	joint_builder
		// };

		joint_builder.with_origin(value.offset);
		joint_builder
	}
}
