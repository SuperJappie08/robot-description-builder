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
	ArcLock, JointInterface, WeakLock,
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
			parent_link,
			child_link,
			TransformData {
				translation: self.offset.and_then(|mode| match mode {
					crate::OffsetMode::Offset(x, y, z) => Some((x, y, z)),
					crate::OffsetMode::FigureItOut => todo!("Not implemented yet"),
				}), // FIXME:
				rotation: self.rotation,
			},
		))
	}
}
