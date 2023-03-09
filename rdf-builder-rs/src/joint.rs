mod fixedjoint;
mod jointbuilder;
mod smartjointbuilder;

use std::{
	fmt::Debug,
	sync::{Arc, Weak},
};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData, link::Link,
	transform_data::TransformData, ArcLock, WeakLock,
};

pub use jointbuilder::{BuildJoint, JointBuilder};
pub use smartjointbuilder::{OffsetMode, SmartJointBuilder};

pub use fixedjoint::FixedJoint;

pub trait JointInterface: Debug {
	fn get_name(&self) -> String;

	/// Adds the `Joint` to a kinematic tree
	fn add_to_tree(&mut self, new_parent_tree: &ArcLock<KinematicTreeData>) {
		{
			let mut new_ptree = new_parent_tree.write().unwrap(); // FIXME: Probablly shouldn't unwrap
			new_ptree.try_add_link(self.get_child_link()).unwrap();
			// TODO: Add materials, and other stuff
		}
		self.get_child_link()
			.write()
			.unwrap() // FIXME: Probablly shouldn't unwrap
			.add_to_tree(new_parent_tree);
		self.set_tree(Arc::downgrade(new_parent_tree));
	}

	fn get_parent_link(&self) -> ArcLock<Link>;
	fn get_child_link(&self) -> ArcLock<Link>;

	/// Set the paren tree
	fn set_tree(&mut self, tree: WeakLock<KinematicTreeData>);

	/// TODO: Semi TMP
	fn get_transform_data(&self) -> &TransformData;

	fn rebuild(&self) -> JointBuilder;
}

#[derive(Debug)]
pub struct Joint {
	/// The name of the `Joint`
	pub name: String,
	/// A Reference to the parent Kinematic Tree
	pub(crate) tree: WeakLock<KinematicTreeData>,
	/// A Reference to the parent `Link`
	pub(crate) parent_link: WeakLock<Link>,
	pub child_link: ArcLock<Link>, //temp pub TODO: THIS PROBABLY ISN'T THE NICEST WAY TO DO THIS.
	/// The information specific to the JointType: TODO: DECIDE IF THIS SHOULD BE PUBLIC
	pub(crate) joint_type: JointType,
	origin: TransformData,
}

impl Joint {
	// pub fn new(name: String, joint_type: JointType) -> JointBuilder {
	// JointBuilder::new(name, joint_type)
	// }
}

impl JointInterface for Joint {
	fn get_name(&self) -> String {
		self.name.clone()
	}

	/// Returns a reference to the parent `Link`
	///
	/// TODO: ADD EXAMPLE
	///
	/// For now pub crate, this should maybe go to joint trait
	fn get_parent_link(&self) -> ArcLock<Link> {
		// If this panics, the Joint is not initialized propperly.
		self.parent_link.upgrade().unwrap()
	}

	/// For now pub crate, this should maybe go to joint trait
	/// Is this even necessary?
	fn get_child_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.child_link)
	}

	fn set_tree(&mut self, tree: WeakLock<KinematicTreeData>) {
		self.tree = tree;
	}

	fn get_transform_data(&self) -> &TransformData {
		&self.origin
	}

	/// Make a `JointBuilder` to build a 'Clone' of the `Joint`
	fn rebuild(&self) -> JointBuilder {
		JointBuilder::new(self.name.clone(), self.joint_type.clone())
	}
}

impl PartialEq for Joint {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& Weak::ptr_eq(&self.parent_link, &other.parent_link)
			&& Arc::ptr_eq(&self.child_link, &other.child_link)
			&& self.joint_type == other.joint_type
	}
}

/// TODO: Might add data of specif joint type to Struct Spaces.
#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub enum JointType {
	/// TODO: TEMP DEFAULT
	#[default]
	Fixed, // — this is not really a joint because it cannot move. All degrees of freedom are locked. This type of joint does not require the <axis>, <calibration>, <dynamics>, <limits> or <safety_controller>.
	Revolute, // — a hinge joint that rotates along the axis and has a limited range specified by the upper and lower limits.
	Continuous, // — a continuous hinge joint that rotates around the axis and has no upper and lower limits.
	Prismatic, // — a sliding joint that slides along the axis, and has a limited range specified by the upper and lower limits.
	Floating,  // — this joint allows motion for all 6 degrees of freedom.
	Planar,    // — this joint allows motion in a plane perpendicular to the axis.
}
