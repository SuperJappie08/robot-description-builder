use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData, joint::JointInterface, WeakLock,
};

#[derive(Debug)]
pub enum LinkParent {
	Joint(WeakLock<Box<dyn JointInterface + Sync + Send>>),
	KinematicTree(WeakLock<KinematicTreeData>),
}

impl Clone for LinkParent {
	fn clone(&self) -> Self {
		match self {
			Self::Joint(joint) => Self::Joint(Weak::clone(joint)),
			Self::KinematicTree(tree) => Self::KinematicTree(Weak::clone(tree)),
		}
	}
}

impl From<WeakLock<KinematicTreeData>> for LinkParent {
	fn from(value: WeakLock<KinematicTreeData>) -> Self {
		Self::KinematicTree(value)
	}
}

impl PartialEq for LinkParent {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Joint(l0), Self::Joint(r0)) => l0.ptr_eq(r0),
			(Self::KinematicTree(l0), Self::KinematicTree(r0)) => l0.ptr_eq(r0),
			_ => false,
		}
	}
}
