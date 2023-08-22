use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLockWriteGuard},
};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use super::{
	kinematic_data_errors::AddTransmissionError, kinematic_data_tree::KinematicDataTree,
	KinematicInterface,
};
use crate::{
	identifiers::GroupID,
	joint::Joint,
	link::Link,
	material::{data::MaterialData, Material},
	transmission::{
		transmission_builder_state::{WithActuator, WithJoints},
		Transmission, TransmissionBuilder,
	},
	utils::{ArcLock, WeakLock},
};

#[derive(Debug)]
pub struct Robot {
	/// The name of the robot.
	name: String,
	data: Arc<KinematicDataTree>,
}

impl Robot {
	pub(crate) fn new(name: impl Into<String>, data: Arc<KinematicDataTree>) -> Self {
		Self {
			name: name.into(),
			data,
		}
	}

	/// Gets a refence to the name of the `Robot`.
	///
	/// # Example
	/// ```rust
	/// # use robot_description_builder::{Robot, KinematicInterface, linkbuilding::LinkBuilder};
	/// let robot: Robot = LinkBuilder::new("my-link")
	///     .build_tree()
	///     .to_robot("my-robot-called-rob");
	///
	/// assert_eq!(robot.name(), "my-robot-called-rob")
	/// ```
	pub fn name(&self) -> &String {
		&self.name
	}
}

impl KinematicInterface for Robot {
	fn get_root_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.data.root_link)
	}

	fn get_newest_link(&self) -> ArcLock<Link> {
		self.data.newest_link.read().unwrap().upgrade().unwrap()
	}

	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>> {
		Arc::clone(&self.data.links)
	}

	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>> {
		Arc::clone(&self.data.joints)
	}

	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<MaterialData>>> {
		Arc::clone(&self.data.material_index)
	}

	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>> {
		Arc::clone(&self.data.transmissions)
	}

	fn get_link(&self, name: &str) -> Option<ArcLock<Link>> {
		self.data
			.links
			.read()
			/* In the future the lock could be saved by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.expect("The RwLock of the Link Index was poisoned. In the future this will be recoverable (mutex_unpoison).")
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>> {
		self.data
			.joints
			.read()
			/* In the future the lock could be saved by overwriting with a newly generated index,
			however waiting for "This is a nightly-only experimental API. (mutex_unpoison #96469)" */
			.expect("The RwLock of the Joint Index was poisoned. In the future this will be recoverable (mutex_unpoison).")
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<Material> {
		self.data
			.material_index
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
			.map(|data| Material::new_named_inited(name, data))
	}

	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>> {
		self.data
			.transmissions
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn try_add_transmission(
		&self,
		transmission: TransmissionBuilder<WithJoints, WithActuator>,
	) -> Result<(), AddTransmissionError> {
		self.data.try_add_transmission(transmission)
	}

	fn purge_links(&self) {
		self.data.purge_links()
	}

	fn purge_joints(&self) {
		self.data.purge_joints()
	}

	fn purge_materials(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<MaterialData>>>>> {
		self.data.purge_materials()
	}

	fn purge_transmissions(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, ArcLock<Transmission>>>>> {
		self.data.purge_transmissions()
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Robot {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("robot").with_attribute(Attribute {
			key: QName(b"name"),
			value: self.name.display().as_bytes().into(),
		});
		element.write_inner_content(|writer| self.data.to_urdf(writer, urdf_config))?;
		Ok(())
	}
}
