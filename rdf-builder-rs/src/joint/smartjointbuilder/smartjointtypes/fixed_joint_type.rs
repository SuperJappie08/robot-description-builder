use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{
		fixedjoint::FixedJoint,
		smartjointbuilder::{
			smartparams::{
				NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController,
			},
			SmartJointBuilder,
		},
		BuildJoint,
	},
	link::Link,
	transform_data::TransformData,
	ArcLock, JointInterface, WeakLock, OffsetMode,
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
		tree: WeakLock<KinematicTreeData>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> Box<dyn JointInterface + Sync + Send> {
		Box::new(FixedJoint::new(
			self.name,
			tree,
			Weak::clone(&parent_link),
			child_link,
			// FIXME: This is incoorect because it doesn't take rotations into account
			TransformData {
				translation: self.offset.and_then(|mode| match mode {
					OffsetMode::Offset(x, y, z) => Some((x, y, z)),
					OffsetMode::FigureItOut(_) => parent_link.upgrade().unwrap().try_read().unwrap().get_end_point(),
				}), // FIXME:
				rotation: self.rotation,
			},
		))
	}
}
