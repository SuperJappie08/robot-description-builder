use std::{
	collections::HashMap,
	sync::{Arc, PoisonError, RwLockWriteGuard},
};

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
use crate::{
	cluster_objects::{
		kinematic_data_errors::AddTransmissionError, kinematic_data_tree::KinematicDataTree,
		KinematicInterface,
	},
	joint::Joint,
	link::Link,
	material_mod::{Material, MaterialData},
	transmission::Transmission,
	ArcLock, WeakLock,
};

#[derive(Debug)]
pub struct Robot {
	/// The name of the robot
	pub name: String, //TODO: Temp Pub
	data: Arc<KinematicDataTree>,
}

impl Robot {
	pub(crate) fn new<Name: Into<String>>(name: Name, data: Arc<KinematicDataTree>) -> Self {
		Self {
			name: name.into(),
			data,
		}
	}

	/// Gets a refence to the name of the `Robot`
	///
	/// # Example
	/// ```rust
	/// # use rdf_builder_rs::{Robot, KinematicInterface, linkbuilding::{LinkBuilder, BuildLink}};
	/// let robot: Robot = LinkBuilder::new("my-link")
	///     .build_tree()
	///     .to_robot("my-robot-called-rob");
	///
	/// assert_eq!(robot.get_name(), "my-robot-called-rob")
	/// ```
	pub fn get_name(&self) -> &String {
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
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>> {
		self.data
			.joints
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
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
			.map(|data| (name.to_string(), data).into())
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
		transmission: ArcLock<Transmission>,
	) -> Result<(), AddTransmissionError> {
		self.data.try_add_transmission(transmission)
	}

	fn purge_links(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, WeakLock<Link>>>>> {
		self.data.purge_links()
	}

	fn purge_joints(
		&self,
	) -> Result<(), PoisonError<RwLockWriteGuard<HashMap<String, WeakLock<Joint>>>>> {
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
			value: self.name.as_bytes().into(),
		});
		element.write_inner_content(|writer| self.data.to_urdf(writer, urdf_config))?;
		Ok(())
	}
}

impl From<Robot> for Box<dyn KinematicInterface> {
	fn from(value: Robot) -> Self {
		Box::new(value)
	}
}
