use std::{
	collections::HashMap,
	sync::{Arc, RwLock, Weak},
};

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData, joint::Joint, link::Link, Material,
	Transmission,
};

use crate::cluster_objects::{kinematic_data_errors::AddTransmissionError, KinematicInterface};

#[derive(Debug)]
pub struct Robot {
	/// The name of the robot
	pub name: String, //TODO: Temp Pub
	data: Arc<RwLock<KinematicTreeData>>,
}

impl KinematicInterface for Robot {
	fn get_root_link(&self) -> Arc<RwLock<Link>> {
		Arc::clone(&self.data.read().unwrap().root_link) // FIXME: Unwrapping might not be ok
	}

	fn get_newest_link(&self) -> Arc<RwLock<Link>> {
		self.data.read().unwrap().newest_link.upgrade().unwrap() // FIXME: Unwrapping might not be ok
	}

	fn get_kinematic_data(&self) -> Arc<RwLock<KinematicTreeData>> {
		Arc::clone(&self.data)
	}

	fn get_links(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Link>>>>> {
		Arc::clone(&self.data.read().unwrap().links) // FIXME: Unwrapping might not be ok
	}

	fn get_joints(&self) -> Arc<RwLock<HashMap<String, Weak<RwLock<Joint>>>>> {
		Arc::clone(&self.data.read().unwrap().joints) // FIXME: Unwrapping might not be ok
	}

	fn get_materials(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Material>>>>> {
		Arc::clone(&self.data.read().unwrap().material_index) // FIXME: Unwrapping might not be ok
	}

	fn get_transmissions(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<Transmission>>>>> {
		Arc::clone(&self.data.read().unwrap().transmissions) // FIXME: Unwrapping might not be ok
	}

	fn get_link(&self, name: &str) -> Option<Arc<RwLock<Link>>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.links
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<Arc<RwLock<Joint>>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.joints
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Arc<RwLock<Material>>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.material_index
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn get_transmission(&self, name: &str) -> Option<Arc<RwLock<Transmission>>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.transmissions
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn try_add_transmission(
		&self,
		transmission: Arc<RwLock<Transmission>>,
	) -> Result<(), AddTransmissionError> {
		self.data
			.write()
			.unwrap() // FIXME: Unwrapping might not be ok
			.try_add_transmission(transmission)
	}
}

impl From<Robot> for Box<dyn KinematicInterface> {
	fn from(value: Robot) -> Self {
		Box::new(value)
	}
}
