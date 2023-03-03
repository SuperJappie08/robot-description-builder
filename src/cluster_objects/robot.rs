use std::{
	cell::RefCell,
	collections::HashMap,
	rc::{Rc, Weak},
};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData, joint::Joint, link::Link, Transmission,
};

use crate::cluster_objects::{kinematic_data_errors::AddTransmissionError, KinematicInterface};

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

	fn get_materials(&self) -> Rc<RefCell<HashMap<String, Rc<RefCell<crate::Material>>>>> {
		Rc::clone(&self.data.borrow().material_index)
	}

	fn get_transmissions(&self) -> Rc<RefCell<HashMap<String, Rc<RefCell<Transmission>>>>> {
		Rc::clone(&self.data.borrow().transmissions)
	}

	fn get_link(&self, name: &str) -> Option<Rc<RefCell<Link>>> {
		self.data
			.borrow()
			.links
			.borrow()
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<Rc<RefCell<Joint>>> {
		self.data
			.borrow()
			.joints
			.borrow()
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Rc<RefCell<crate::Material>>> {
		self.data
			.borrow()
			.material_index
			.borrow()
			.get(name)
			.map(Rc::clone)
	}

	fn get_transmission(&self, name: &str) -> Option<Rc<RefCell<Transmission>>> {
		self.data
			.borrow()
			.transmissions
			.borrow()
			.get(name)
			.map(Rc::clone)
	}

	fn try_add_transmission(
		&self,
		transmission: Rc<RefCell<Transmission>>,
	) -> Result<(), AddTransmissionError> {
		self.data.borrow_mut().try_add_transmission(transmission)
	}
}

impl From<Robot> for Box<dyn KinematicInterface> {
	fn from(value: Robot) -> Self {
		Box::new(value)
	}
}
