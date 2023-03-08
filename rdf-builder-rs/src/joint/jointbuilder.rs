use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{Joint, JointType},
	link::Link,
	transform_data::TransformData,
};

pub trait BuildJoint {
	fn build(
		self,
		tree: Weak<RwLock<KinematicTreeData>>,
		parent_link: Weak<RwLock<Link>>,
		child_link: Arc<RwLock<Link>>,
	) -> Joint;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct JointBuilder {
	name: String,
	joint_type: JointType, // TODO: FINISH ME
}

impl JointBuilder {
	pub(crate) fn new(name: String, joint_type: JointType) -> Self {
		Self { name, joint_type }
	}

	/// For now return a Specific Joint maybe go dyn JointInterface
	pub fn build(
		self,
		tree: Weak<RwLock<KinematicTreeData>>,
		parent_link: Weak<RwLock<Link>>,
		child_link: Arc<RwLock<Link>>,
	) -> Joint {
		Joint {
			name: self.name,
			tree: tree,
			parent_link: parent_link,
			child_link: child_link,
			joint_type: self.joint_type,
			origin: TransformData::default(), // FIXME: Mmmh Data Where?
		}
	}
}

impl BuildJoint for JointBuilder {
	fn build(
		self,
		tree: Weak<RwLock<KinematicTreeData>>,
		parent_link: Weak<RwLock<Link>>,
		child_link: Arc<RwLock<Link>>,
	) -> Joint {
		self.build(tree, parent_link, child_link)
	}
}
