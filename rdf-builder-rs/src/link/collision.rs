use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{
	link::geometry::GeometryInterface, to_rdf::to_urdf::ToURDF, transform_data::TransformData,
};

#[derive(Debug)]
pub struct Collision {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	origin: Option<TransformData>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl Collision {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		origin: Option<TransformData>,
		geometry: Box<dyn GeometryInterface + Sync + Send>,
	) -> Self {
		Self {
			name,
			origin,
			geometry,
		}
	}
}

impl ToURDF for Collision {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("collision");
		if let Some(name) = self.name.clone() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.clone().as_bytes().into(),
			});
		}

		element.write_inner_content(|writer| {
			if let Some(origin) = self.origin.clone() {
				origin.to_urdf(writer, urdf_config)?
			}

			self.geometry.to_urdf(writer, urdf_config)?;
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Collision {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& (&self.geometry == &other.geometry)
	}
}

impl Clone for Collision {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin.clone(),
			geometry: self.geometry.boxed_clone(),
		}
	}
}
