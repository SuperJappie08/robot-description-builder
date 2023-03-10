use std::f32::consts::{PI, TAU};

use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{link::geometry::GeometryInterface, to_rdf::to_urdf::ToURDF};

/// TODO: Figure out if things should be pub
#[derive(Debug, PartialEq, Clone)]
pub struct CylinderGeometry {
	pub radius: f32,
	pub length: f32,
}

impl CylinderGeometry {
	pub fn new(radius: f32, length: f32) -> Self {
		Self { radius, length }
	}
}

impl ToURDF for CylinderGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| {
			writer
				.create_element("cylinder")
				.with_attribute(Attribute {
					key: QName(b"radius"),
					value: self.radius.to_string().as_bytes().into(),
				})
				.with_attribute(Attribute {
					key: QName(b"length"),
					value: self.length.to_string().as_bytes().into(),
				})
				.write_empty()?;
			Ok(())
		})?;
		Ok(())
	}
}

impl GeometryInterface for CylinderGeometry {
	fn volume(&self) -> f32 {
		self.radius * self.radius * PI * self.length
	}

	fn surface_area(&self) -> f32 {
		2f32 * (self.radius * self.radius * PI) + self.length * self.radius * TAU
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}
}

impl From<CylinderGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: CylinderGeometry) -> Self {
		Box::new(value)
	}
}
