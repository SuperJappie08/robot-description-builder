use std::{
	cell::RefCell,
	rc::{Rc, Weak},
};

use crate::{cluster_objects::kinematic_tree_data::KinematicTreeData, link::Link};

#[derive(Debug)]
pub struct Joint {
	pub name: String,
	pub(crate) tree: Weak<RefCell<KinematicTreeData>>,
	pub(crate) parent_link: Weak<RefCell<Link>>,
	pub child_link: Rc<RefCell<Link>>, //temp pub TODO: THIS PROBABLY ISN'T THE NICEST WAY TO DO THIS.
	/// The information specific to the JointType: TODO: DECIDE IF THIS SHOULD BE PUBLIC
	pub(crate) joint_type: JointType,
}

impl Joint {
	pub(crate) fn add_to_tree(&mut self, new_parent_tree: &Rc<RefCell<KinematicTreeData>>) {
		{
			let mut new_ptree = new_parent_tree.borrow_mut();
			new_ptree.try_add_link(Rc::clone(&self.child_link)).unwrap();
			// TODO: Add materials, and other stuff
		}
		self.child_link.borrow_mut().add_to_tree(new_parent_tree);
		self.tree = Rc::downgrade(new_parent_tree);
	}

	/// For now pub crate, this should maybe go to joint trait
	pub fn get_parent_link(&self) -> Rc<RefCell<Link>> {
		// If this panics, the joint is not initialized propperly.
		self.parent_link.upgrade().unwrap()
	}

	/// For now pub crate, this should maybe go to joint trait
	/// Is this even necessary?
	pub fn get_child_link(&self) -> Rc<RefCell<Link>> {
		// If this panics, the joint is not initialized propperly.
		Rc::clone(&self.child_link)
	}
}

impl PartialEq for Joint {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.parent_link.upgrade() == other.parent_link.upgrade()
			&& self.child_link == other.child_link
			&& self.joint_type == other.joint_type
	}
}

/// TODO: Might add data of specif joint type to Struct Spaces.
#[derive(Debug, PartialEq, Clone)]
pub enum JointType {
	Fixed, // — this is not really a joint because it cannot move. All degrees of freedom are locked. This type of joint does not require the <axis>, <calibration>, <dynamics>, <limits> or <safety_controller>.
	Revolute, // — a hinge joint that rotates along the axis and has a limited range specified by the upper and lower limits.
	Continuous, // — a continuous hinge joint that rotates around the axis and has no upper and lower limits.
	Prismatic, // — a sliding joint that slides along the axis, and has a limited range specified by the upper and lower limits.
	Floating,  // — this joint allows motion for all 6 degrees of freedom.
	Planar,    // — this joint allows motion in a plane perpendicular to the axis.
}
