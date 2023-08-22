use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, joint::Joint, utils::WeakLock,
};

/// A element to contain a reference to the parent element of a `Link`.
#[derive(Debug)]
pub enum LinkParent {
	/// Variant for when the Parent of a [`Link`](super::Link) is [`Joint`].
	Joint(WeakLock<Joint>),
	/// Variant for when this [`Link`](super::Link) element is the root of the [kinematic tree](crate::KinematicTree).
	// TODO: Change link of kinematictree to a submodule
	KinematicTree(Weak<KinematicDataTree>),
}

impl LinkParent {
	/// Validate if the reference to the parent is still valid.
	// TODO: Example?
	pub fn is_valid_reference(&self) -> bool {
		match self {
			LinkParent::Joint(joint) => joint.upgrade().is_some(),
			LinkParent::KinematicTree(tree) => tree.upgrade().is_some(),
		}
	}
}

impl Clone for LinkParent {
	fn clone(&self) -> Self {
		match self {
			Self::Joint(joint) => Self::Joint(Weak::clone(joint)),
			Self::KinematicTree(tree) => Self::KinematicTree(Weak::clone(tree)),
		}
	}
}

impl From<Weak<KinematicDataTree>> for LinkParent {
	fn from(value: Weak<KinematicDataTree>) -> Self {
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
