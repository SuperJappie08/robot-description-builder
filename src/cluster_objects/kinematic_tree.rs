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

	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>> {
		Rc::clone(&self.0)
	}

	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>> {
		Rc::clone(&self.0.borrow().links)
	}

	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>> {
		Rc::clone(&self.0.borrow().joints)
	}

	fn get_newest_link(&self) -> Weak<RefCell<Link>> {
		Weak::clone(&self.0.borrow().newest_link)
	}
}
