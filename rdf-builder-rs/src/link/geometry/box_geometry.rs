#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use crate::link::geometry::GeometryInterface;
#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

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

#[cfg(feature = "urdf")]
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

	// fn get_type(&self) -> GeometryType {
	// 	GeometryType::Box
	// }

	// fn get_data(&self) -> GeometryData {
	// 	GeometryData::Box(self.clone())
	// }
}

impl From<BoxGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: BoxGeometry) -> Self {
		Box::new(value)
	}
}

#[cfg(test)]
mod tests {
	#[cfg(feature = "xml")]
	use std::io::Seek;
	use test_log::test;

	use crate::link::geometry::{box_geometry::BoxGeometry, GeometryInterface};
	#[cfg(feature = "urdf")]
	use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

	#[test]
	fn volume() {
		assert_eq!(BoxGeometry::new(1.0, 1.0, 1.0).volume(), 1.0);
		assert_eq!(BoxGeometry::new(1.0, 2.0, 3.0).volume(), 6.0);
		assert_eq!(BoxGeometry::new(9.0, 20.0, 100.0).volume(), 18000.0);
		assert_eq!(BoxGeometry::new(4.5, 20.0, 100.0).volume(), 9000.0);
	}

	#[test]
	fn surface_area() {
		assert_eq!(BoxGeometry::new(1.0, 1.0, 1.0).surface_area(), 6.);
		assert_eq!(BoxGeometry::new(1.0, 2.0, 3.0).surface_area(), 22.);
		assert_eq!(BoxGeometry::new(9.0, 20.0, 100.0).surface_area(), 6160.);
		assert_eq!(BoxGeometry::new(4.5, 20.0, 100.0).surface_area(), 5080.);
	}

	#[test]
	fn boxed_clone() {
		assert_eq!(
			BoxGeometry::new(1.0, 1.0, 1.0).boxed_clone(),
			BoxGeometry::new(1.0, 1.0, 1.0).into()
		);
		assert_eq!(
			BoxGeometry::new(1.0, 2.0, 3.0).boxed_clone(),
			BoxGeometry::new(1.0, 2.0, 3.0).into()
		);
		assert_eq!(
			BoxGeometry::new(9.0, 20.0, 100.0).boxed_clone(),
			BoxGeometry::new(9.0, 20.0, 100.0).into()
		);
		assert_eq!(
			BoxGeometry::new(4.5, 20.0, 100.0).boxed_clone(),
			BoxGeometry::new(4.5, 20.0, 100.0).into()
		);
	}

	#[cfg(feature = "urdf")]
	#[test]
	fn to_urdf() {
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(1.0, 1.0, 1.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><box size="1 1 1"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(1.0, 2.0, 3.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><box size="1 2 3"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(9.0, 20.0, 100.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><box size="9 20 100"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(4.5, 20.0, 100.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><box size="4.5 20 100"/></geometry>"#)
			);
		}
	}
}
