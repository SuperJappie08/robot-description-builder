use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
use crate::{
	cluster_objects::{
		kinematic_data_errors::AddTransmissionError, kinematic_tree_data::KinematicTreeData,
		KinematicInterface,
	},
	joint::Joint,
	link::Link,
	material::Material,
	transmission::Transmission,
	ArcLock, WeakLock,
};

#[derive(Debug)]
pub struct Robot {
	/// The name of the robot
	pub name: String, //TODO: Temp Pub
	data: ArcLock<KinematicTreeData>,
}

impl Robot {
	pub(crate) fn new<Name: Into<String>>(name: Name, data: ArcLock<KinematicTreeData>) -> Self {
		Self {
			name: name.into(),
			data,
		}
	}

	/// Gets a refence to the name of the `Robot`
	pub fn get_name(&self) -> &String {
		&self.name
	}
}

impl KinematicInterface for Robot {
	fn get_root_link(&self) -> ArcLock<Link> {
		Arc::clone(&self.data.read().unwrap().root_link) // FIXME: Unwrapping might not be ok
	}

	fn get_newest_link(&self) -> ArcLock<Link> {
		self.data.read().unwrap().newest_link.read().unwrap().upgrade().unwrap() // FIXME: Unwrapping might not be ok
	}

	fn get_kinematic_data(&self) -> ArcLock<KinematicTreeData> {
		Arc::clone(&self.data)
	}

	fn get_links(&self) -> ArcLock<HashMap<String, WeakLock<Link>>> {
		Arc::clone(&self.data.read().unwrap().links) // FIXME: Unwrapping might not be ok
	}

	fn get_joints(&self) -> ArcLock<HashMap<String, WeakLock<Joint>>> {
		Arc::clone(&self.data.read().unwrap().joints) // FIXME: Unwrapping might not be ok
	}

	fn get_materials(&self) -> ArcLock<HashMap<String, ArcLock<Material>>> {
		Arc::clone(&self.data.read().unwrap().material_index) // FIXME: Unwrapping might not be ok
	}

	fn get_transmissions(&self) -> ArcLock<HashMap<String, ArcLock<Transmission>>> {
		Arc::clone(&self.data.read().unwrap().transmissions) // FIXME: Unwrapping might not be ok
	}

	fn get_link(&self, name: &str) -> Option<ArcLock<Link>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.links
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_link| weak_link.upgrade())
	}

	fn get_joint(&self, name: &str) -> Option<ArcLock<Joint>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.joints
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.and_then(|weak_joint| weak_joint.upgrade())
	}

	fn get_material(&self, name: &str) -> Option<ArcLock<Material>> {
		self.data
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.material_index
			.read()
			.unwrap() // FIXME: Unwrapping might not be ok
			.get(name)
			.map(Arc::clone)
	}

	fn get_transmission(&self, name: &str) -> Option<ArcLock<Transmission>> {
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
		transmission: ArcLock<Transmission>,
	) -> Result<(), AddTransmissionError> {
		self.data
			.write()
			.unwrap() // FIXME: Unwrapping might not be ok
			.try_add_transmission(transmission)
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
		let data = Arc::clone(&self.data);
		element.write_inner_content(|writer| {
			data.try_read().unwrap().to_urdf(writer, urdf_config) // FIXME: Is unwrapping OK?
		})?;
		Ok(())
	}
}

impl From<Robot> for Box<dyn KinematicInterface> {
	fn from(value: Robot) -> Self {
		Box::new(value)
	}
}
