use std::sync::{Arc, RwLock};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{JointBuilder, JointInterface},
	Link,
};

#[derive(Debug)]
pub struct FixedJoint {
	name: String,
}

impl JointInterface for FixedJoint {
	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn add_to_tree(&mut self, new_parent_tree: &Arc<RwLock<KinematicTreeData>>) {
		todo!()
	}

	fn get_parent_link(&self) -> Arc<RwLock<Link>> {
		todo!()
	}

	fn get_child_link(&self) -> Arc<RwLock<Link>> {
		todo!()
	}

	fn rebuild(&self) -> JointBuilder {
		todo!()
	}
}
