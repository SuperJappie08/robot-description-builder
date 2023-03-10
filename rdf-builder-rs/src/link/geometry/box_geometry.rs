use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{link::geometry::GeometryInterface, to_rdf::to_urdf::ToURDF};

#[derive(Debug, PartialEq, Clone)]
pub struct BoxGeometry {
	/// TODO: Figure out correct field names
	/// TODO: Figure out if pub necesary for ToRDF things
	pub side1: f32,
	pub side2: f32,
	pub side3: f32,
}

impl BoxGeometry {
	/// TODO: REPLACE PARAMETER NAMES AND MAYBE NOT PUBLIC
	pub fn new(side1: f32, side2: f32, side3: f32) -> Self {
		Self {
			side1,
			side2,
			side3,
		}
	}
}

impl ToURDF for BoxGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| {
			writer
				.create_element("box")
				.with_attribute(Attribute {
					key: QName(b"size"),
					value: format!("{} {} {}", self.side1, self.side2, self.side3)
						.as_bytes()
						.into(),
				})
				.write_empty()?;
			Ok(())
		})?;
		Ok(())
	}
}

impl GeometryInterface for BoxGeometry {
	fn volume(&self) -> f32 {
		self.side1 * self.side2 * self.side3
	}

	fn surface_area(&self) -> f32 {
		2f32 * (self.side1 * self.side2 + self.side1 * self.side3 + self.side2 * self.side3)
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}
}

impl From<BoxGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: BoxGeometry) -> Self {
		Box::new(value)
	}
}
