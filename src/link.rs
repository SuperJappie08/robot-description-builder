use std::{
	cell::RefCell,
	rc::{Rc, Weak}, fmt::Debug,
};

use crate::{
	joint::{Joint, JointType},
	Robot,
};

#[derive(Debug)]
pub enum LinkParent {
	Robot(Weak<RefCell<Robot>>),
	Joint(Weak<RefCell<Joint>>),
}

impl Clone for LinkParent {
	fn clone(&self) -> Self {
		match self {
			Self::Robot(robot) => Self::Robot(Weak::clone(robot)),
			Self::Joint(joint) => Self::Joint(Weak::clone(joint)),
		}
	}
}

impl From<Weak<RefCell<Robot>>> for LinkParent {
	fn from(value: Weak<RefCell<Robot>>) -> Self {
		Self::Robot(value)
	}
}

pub trait LinkTrait: Debug {
	/// Returns the parent of the `Link` wrapped in a optional.
	fn get_parent(&self) -> Option<LinkParent>;
	fn set_parent(&mut self, parent: LinkParent);

	/// Returns the name of the `Link`
	fn get_name(&self) -> String; // TODO: This might be temp because I want dynamic names.

	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>>; // TODO: Not final?
	fn attach_child(&mut self, link: Link, joint_type: JointType);
}

#[derive(Debug)]
pub struct Link {
	pub name: String,
	parent: Option<LinkParent>,
	child_joints: Vec<Rc<RefCell<Joint>>>,
}

impl Link {
	pub fn new(name: String, parent: Option<LinkParent>) -> Self {
		Self {
			name,
			parent,
			child_joints: Vec::new(),
		}
	}
}

impl LinkTrait for Link {
	fn get_parent(&self) -> Option<LinkParent> {
		self.parent.clone()
	}

	fn set_parent(&mut self, parent: LinkParent) {
		self.parent = Some(parent);
		// TODO: Add yourself to registry.
	}

	fn get_name(&self) -> String {
		self.name.clone()
	}

	fn get_joints(&self) -> Vec<Rc<RefCell<Joint>>> {
		self.child_joints
			.iter()
			.map(|joint| Rc::clone(joint))
			.collect()
	}

	fn attach_child(&mut self, link: Link, joint_type: JointType) {
		todo!();
		// TODO: NEEDS TO DO SOMETHING WITH JOINT TYPE
		// self.child_joints.push();
	}
}
