use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{cluster_objects::kinematic_tree_data::KinematicTreeData, joint::Joint, link::Link};

use super::KinematicInterface;

#[derive(Debug)]
pub struct KinematicTree(Rc<RefCell<KinematicTreeData>>);

impl KinematicTree {
	pub fn new(data: Rc<RefCell<KinematicTreeData>>) -> KinematicTree {
		KinematicTree(data)
	}
}

impl KinematicInterface for KinematicTree {
	fn get_root_link(&self) -> Rc<RefCell<Link>> {
		Rc::clone(&self.0.borrow().root_link)
	}

	fn get_newest_link(&self) -> Rc<RefCell<Link>> {
		self.0.borrow().newest_link.upgrade().unwrap()
	}

	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>> {
		Rc::clone(&self.0)
	}

	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>> {
		Rc::clone(&self.0.borrow().links)
	}

	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>> {
		Rc::clone(&self.0.borrow().joints)
	}

	fn get_link(&self, name: &str) -> Option<Rc<RefCell<Link>>> {
		self.0
			.borrow()
			.links
			.borrow()
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<Rc<RefCell<Joint>>> {
		self.0
			.borrow()
			.joints
			.borrow()
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}
}

impl Into<Box<dyn KinematicInterface>> for KinematicTree {
	fn into(self) -> Box<dyn KinematicInterface> {
		Box::new(self)
	}
}
