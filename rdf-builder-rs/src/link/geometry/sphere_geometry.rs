use std::f32::consts::{FRAC_PI_3, PI};

use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{link::geometry::GeometryInterface, to_rdf::to_urdf::ToURDF};

#[derive(Debug, PartialEq, Clone)]
pub struct SphereGeometry {
	radius: f32,
}

impl SphereGeometry {
	pub fn new(radius: f32) -> Self {
		Self { radius }
	}
}

impl ToURDF for SphereGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| {
			writer
				.create_element("sphere")
				.with_attribute(Attribute {
					key: QName(b"radius"),
					value: self.radius.to_string().as_bytes().into(),
				})
				.write_empty()?;
			Ok(())
		})?;
		Ok(())
	}
}

impl GeometryInterface for SphereGeometry {
	fn volume(&self) -> f32 {
		4f32 * FRAC_PI_3 * self.radius * self.radius * self.radius
	}

	fn surface_area(&self) -> f32 {
		4f32 * PI * self.radius * self.radius
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}
}

impl From<SphereGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: SphereGeometry) -> Self {
		Box::new(value)
	}
}
