use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
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
	ArcLock, OffsetMode, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct FixedType;

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
		tree: Weak<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let mut joint_builder = JointBuilder::new(self.name, JointType::Fixed);

		if let Some(mode) = self.offset {
			joint_builder.add_origin_offset(match mode {
				OffsetMode::Offset(x, y, z) => (x, y, z),
				// FIXME: This is incoorect because it doesn't take rotations into account
				OffsetMode::FigureItOut(_) => parent_link
					.upgrade()
					.unwrap()
					.try_read()
					.unwrap()
					.get_end_point()
					.expect("No ENDPOINT"), //TODO: FIX this
			});
		}

		if let Some(rotation) = self.rotation {
			joint_builder.add_origin_rotation(rotation);
		}

		joint_builder.build(Weak::clone(&tree), parent_link, child_link)
	}
}
