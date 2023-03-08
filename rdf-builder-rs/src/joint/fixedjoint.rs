use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{JointBuilder, JointInterface},
	Link, SmartJointBuilder,
};

#[derive(Debug)]
pub struct FixedJoint {
	name: String,
	tree: Weak<RwLock<KinematicTreeData>>,
	parent_link: Weak<RwLock<Link>>,
	child_link: Arc<RwLock<Link>>,
}

impl JointInterface for FixedJoint {
	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn add_to_tree(&mut self, new_parent_tree: &Arc<RwLock<KinematicTreeData>>) {
		todo!()
	}

	fn get_parent_link(&self) -> Arc<RwLock<Link>> {
		Weak::upgrade(&self.parent_link).unwrap()
	}

	fn get_child_link(&self) -> Arc<RwLock<Link>> {
		Arc::clone(&self.child_link)
	}

	fn rebuild(&self) -> JointBuilder {
		// SmartJointBuilder::new(self.name.clone()).fixed().into();
		todo!()
	}
}
