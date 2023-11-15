use super::{GeometryInterface, GeometryShapeContainer};
use std::f32::consts::{PI, TAU};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::transform::Mirror;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

/// A Represenation for a Cylinder Geometry.
///
/// This Cylinder is centered at the origin, with the Z-axis as the rotational-symmetry axis. (URDF)
// The fields are public for the Python wrapper. It doesn't change much for the Rust side, since most of the time these will be `Box<dyn GeometryInterface + Sync + Send>`.
#[derive(Debug, PartialEq, Clone)]
pub struct CylinderGeometry {
	/// The radius of the Cylinder.
	///
	/// Around the Z-axis for URDF.
	pub radius: f32,
	/// The Length of the Cylinder.
	pub length: f32,
}

impl CylinderGeometry {
	/// Creates a new `CylinderGeometry` with the specified `radius` and `length`.
	pub fn new(radius: f32, length: f32) -> Self {
		Self { radius, length }
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

	fn bounding_box(&self) -> (f32, f32, f32) {
		(2. * self.radius, 2. * self.radius, self.length)
	}

	fn shape_container(&self) -> GeometryShapeContainer {
		self.clone().into()
	}
}

impl Mirror for CylinderGeometry {
	fn mirrored(&self, _mirror_matrix: &nalgebra::Matrix3<f32>) -> Self {
		self.clone()
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for CylinderGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| -> quick_xml::Result<()> {
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

impl From<CylinderGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: CylinderGeometry) -> Self {
		Box::new(value)
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::PI;
	#[cfg(feature = "xml")]
	use std::io::Seek;
	use test_log::test;

	use crate::link::geometry::{
		cylinder_geometry::CylinderGeometry, geometry_shape_data::GeometryShapeContainer,
		GeometryInterface,
	};

	#[cfg(feature = "urdf")]
	use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

	#[test]
	fn volume() {
		assert_eq!(CylinderGeometry::new(1.0, 1.0).volume(), PI);
		assert_eq!(CylinderGeometry::new(2.0, 3.0).volume(), PI * 12.);
		assert_eq!(CylinderGeometry::new(9.0, 20.0).volume(), PI * 1620.);
		assert_eq!(CylinderGeometry::new(4.5, 75.35).volume(), PI * 1525.8375);
	}

	#[test]
	fn surface_area() {
		assert_eq!(CylinderGeometry::new(1.0, 1.0).surface_area(), PI * 4.);
		assert_eq!(CylinderGeometry::new(2.0, 3.0).surface_area(), PI * 20.);
		assert_eq!(CylinderGeometry::new(9.0, 20.0).surface_area(), PI * 522.);
		assert_eq!(
			CylinderGeometry::new(4.5, 75.35).surface_area(),
			(std::f64::consts::PI * 718.65) as f32
		);
	}

	#[test]
	fn boxed_clone() {
		assert_eq!(
			CylinderGeometry::new(1.0, 1.0).boxed_clone(),
			CylinderGeometry::new(1.0, 1.0).into()
		);
		assert_eq!(
			CylinderGeometry::new(2.0, 3.0).boxed_clone(),
			CylinderGeometry::new(2.0, 3.0).into()
		);
		assert_eq!(
			CylinderGeometry::new(9.0, 20.0).boxed_clone(),
			CylinderGeometry::new(9.0, 20.0).into()
		);
		assert_eq!(
			CylinderGeometry::new(4.5, 75.35).boxed_clone(),
			CylinderGeometry::new(4.5, 75.35).into()
		);
	}

	#[test]
	fn bounding_box() {
		assert_eq!(
			CylinderGeometry::new(1.0, 1.0).bounding_box(),
			(2.0, 2.0, 1.0)
		);
		assert_eq!(
			CylinderGeometry::new(2.0, 3.0).bounding_box(),
			(4.0, 4.0, 3.0)
		);
		assert_eq!(
			CylinderGeometry::new(9.0, 20.0).bounding_box(),
			(18.0, 18.0, 20.)
		);
		assert_eq!(
			CylinderGeometry::new(4.5, 75.35).bounding_box(),
			(9., 9., 75.35)
		);
	}

	#[test]
	fn get_shape() {
		assert_eq!(
			CylinderGeometry::new(1.0, 1.0).shape_container(),
			GeometryShapeContainer::Cylinder(CylinderGeometry::new(1.0, 1.0))
		);
		assert_eq!(
			CylinderGeometry::new(2.0, 3.0).shape_container(),
			GeometryShapeContainer::Cylinder(CylinderGeometry::new(2.0, 3.0))
		);
		assert_eq!(
			CylinderGeometry::new(9.0, 20.0).shape_container(),
			GeometryShapeContainer::Cylinder(CylinderGeometry::new(9.0, 20.0))
		);
		assert_eq!(
			CylinderGeometry::new(4.5, 75.35).shape_container(),
			GeometryShapeContainer::Cylinder(CylinderGeometry::new(4.5, 75.35))
		);
	}

	#[cfg(feature = "urdf")]
	#[test]
	fn to_urdf() {
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(CylinderGeometry::new(1.0, 1.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><cylinder radius="1" length="1"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(CylinderGeometry::new(2.0, 3.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><cylinder radius="2" length="3"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(CylinderGeometry::new(9.0, 20.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><cylinder radius="9" length="20"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(CylinderGeometry::new(4.5, 75.35)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><cylinder radius="4.5" length="75.35"/></geometry>"#)
			);
		}
	}
}
