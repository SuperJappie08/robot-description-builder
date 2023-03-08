use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{
		smartjointbuilder::{
			smartparams::{
				NoAxis, NoCalibration, NoDynamics, NoLimit, NoMimic, NoSafetyController,
			},
			SmartJointBuilder,
		},
		BuildJoint, Joint,
	},
	link::Link,
	transform_data::TransformData,
};

#[derive(Debug, Default, Clone)]
pub struct FixedType;

impl BuildJoint
	for SmartJointBuilder<
		FixedType,
		NoAxis,
		NoCalibration,
		NoLimit,
		NoDynamics,
		NoMimic,
		NoSafetyController,
	>
{
	fn build(
		self,
		tree: Weak<RwLock<KinematicTreeData>>,
		parent_link: Weak<RwLock<Link>>,
		child_link: Arc<RwLock<Link>>,
	) -> Joint {
		Joint {
			name: self.name,
			tree,
			parent_link,
			child_link,
			joint_type: crate::JointType::Fixed,
			origin: TransformData {
				translation: self.offset.and_then(|_| todo!("Not implemented yet")), // FIXME:
				rotation: self.rotation,
			},
		}
	}
}
