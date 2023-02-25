use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{cluster_objects::kinematic_tree_data::KinematicTreeData, joint::Joint, link::Link};

use super::KinematicInterface;

#[derive(Debug)]
pub struct Robot {
	pub name: String, //TODO: Temp Pub
	data: Rc<RefCell<KinematicTreeData>>,
}

impl KinematicInterface for Robot {
	fn get_root_link(&self) -> Rc<RefCell<Link>> {
		Rc::clone(&self.data.borrow().root_link)
	}

	fn get_newest_link(&self) -> Rc<RefCell<Link>> {
		self.data.borrow().newest_link.upgrade().unwrap()
	}

	fn get_kinematic_data(&self) -> Rc<RefCell<KinematicTreeData>> {
		Rc::clone(&self.data)
	}

	fn get_links(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Link>>>>> {
		Rc::clone(&self.data.borrow().links)
	}

	fn get_joints(&self) -> Rc<RefCell<HashMap<String, Weak<RefCell<Joint>>>>> {
		Rc::clone(&self.data.borrow().joints)
	}
}
