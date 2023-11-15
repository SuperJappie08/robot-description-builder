use super::{GeometryInterface, GeometryShapeContainer};
use crate::transform::Mirror;

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

/// A Represenation for a Box Geometry.
///
/// This Box is centered at the origin of its parents frame. (URDF)
// The fields are public for the Python wrapper. It doesn't change much for the Rust side, since most of the time these will be `Box<dyn GeometryInterface + Sync + Send>`.
#[derive(Debug, PartialEq, Clone)]
pub struct BoxGeometry {
	// TODO: Figure out correct field names
	/// The side-length in the X-direction.
	pub side1: f32,
	/// The side-length in the Y-direction.
	pub side2: f32,
	/// The side-length in the Z-direction.
	pub side3: f32,
}

impl BoxGeometry {
	/// Creates a new `BoxGeometry` with the specified side lengths.
	pub fn new(side1: f32, side2: f32, side3: f32) -> Self {
		// TODO: REPLACE PARAMETER NAMES
		Self {
			side1,
			side2,
			side3,
		}
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

	fn bounding_box(&self) -> (f32, f32, f32) {
		(self.side1, self.side2, self.side3)
	}

	fn shape_container(&self) -> GeometryShapeContainer {
		self.clone().into()
	}
}

impl Mirror for BoxGeometry {
	fn mirrored(&self, _mirror_matrix: &nalgebra::Matrix3<f32>) -> Self {
		self.clone()
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
		element.write_inner_content(|writer| -> quick_xml::Result<()> {
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

	use crate::link::geometry::{
		box_geometry::BoxGeometry, geometry_shape_data::GeometryShapeContainer, GeometryInterface,
	};
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

	#[test]
	fn bounding_box() {
		assert_eq!(
			BoxGeometry::new(1.0, 1.0, 1.0).bounding_box(),
			(1.0, 1.0, 1.0)
		);
		assert_eq!(
			BoxGeometry::new(1.0, 2.0, 3.0).bounding_box(),
			(1.0, 2.0, 3.0)
		);
		assert_eq!(
			BoxGeometry::new(9.0, 20.0, 100.0).bounding_box(),
			(9.0, 20.0, 100.0)
		);
		assert_eq!(
			BoxGeometry::new(4.5, 20.0, 100.0).bounding_box(),
			(4.5, 20.0, 100.0)
		);
	}

	#[test]
	fn get_shape() {
		assert_eq!(
			BoxGeometry::new(1.0, 1.0, 1.0).shape_container(),
			GeometryShapeContainer::Box(BoxGeometry::new(1.0, 1.0, 1.0))
		);
		assert_eq!(
			BoxGeometry::new(1.0, 2.0, 3.0).shape_container(),
			GeometryShapeContainer::Box(BoxGeometry::new(1.0, 2.0, 3.0))
		);
		assert_eq!(
			BoxGeometry::new(9.0, 20.0, 100.0).shape_container(),
			GeometryShapeContainer::Box(BoxGeometry::new(9.0, 20.0, 100.0))
		);
		assert_eq!(
			BoxGeometry::new(4.5, 20.0, 100.0).shape_container(),
			GeometryShapeContainer::Box(BoxGeometry::new(4.5, 20.0, 100.0))
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

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><box size="1 1 1"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(1.0, 2.0, 3.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><box size="1 2 3"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(9.0, 20.0, 100.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><box size="9 20 100"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(BoxGeometry::new(4.5, 20.0, 100.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><box size="4.5 20 100"/></geometry>"#)
			);
		}
	}
}
