use super::{GeometryInterface, GeometryShapeContainer};
use crate::transform::Mirror;
use std::f32::consts::{FRAC_PI_3, PI};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

/// A Represenation for a Sphere Geometry.
///
/// This Sphere is centered at the origin. (URDF)
// The fields are public for the Python wrapper. It doesn't change much for the Rust side, since most of the time these will be `Box<dyn GeometryInterface + Sync + Send>`.
#[derive(Debug, PartialEq, Clone)]
pub struct SphereGeometry {
	/// The radius of the Sphere.
	pub radius: f32,
}

impl SphereGeometry {
	/// Creates a new `SphereGeometry` with the specified `radius`.
	pub fn new(radius: f32) -> Self {
		Self { radius }
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

	fn bounding_box(&self) -> (f32, f32, f32) {
		let diameter = 2. * self.radius;
		(diameter, diameter, diameter)
	}

	fn shape_container(&self) -> GeometryShapeContainer {
		self.clone().into()
	}
}

impl Mirror for SphereGeometry {
	fn mirrored(&self, _mirror_matrix: &nalgebra::Matrix3<f32>) -> Self {
		self.clone()
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for SphereGeometry {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("geometry");
		element.write_inner_content(|writer| -> quick_xml::Result<()> {
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

impl From<SphereGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: SphereGeometry) -> Self {
		Box::new(value)
	}
}

#[cfg(test)]
mod tests {
	use std::f32::consts::{FRAC_PI_3, PI};
	#[cfg(feature = "xml")]
	use std::io::Seek;
	use test_log::test;

	use crate::link::geometry::{
		geometry_shape_data::GeometryShapeContainer, sphere_geometry::SphereGeometry,
		GeometryInterface,
	};
	#[cfg(feature = "urdf")]
	use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

	#[test]
	fn volume() {
		assert_eq!(SphereGeometry::new(1.0).volume(), FRAC_PI_3 * 4.);
		assert_eq!(SphereGeometry::new(2.0).volume(), FRAC_PI_3 * 32.);
		assert_eq!(
			SphereGeometry::new(9.0).volume(),
			4. * FRAC_PI_3 * 9. * 9. * 9.
		);
		assert_eq!(
			SphereGeometry::new(75.35).volume(),
			(std::f64::consts::FRAC_PI_3 * 1711235.4215) as f32
		);
	}

	#[test]
	fn surface_area() {
		assert_eq!(SphereGeometry::new(1.0).surface_area(), PI * 4.);
		assert_eq!(SphereGeometry::new(2.0).surface_area(), PI * 16.);
		assert_eq!(SphereGeometry::new(9.0).surface_area(), PI * 324.);
		assert_eq!(SphereGeometry::new(75.35).surface_area(), PI * 22710.49);
	}

	#[test]
	fn boxed_clone() {
		assert_eq!(
			SphereGeometry::new(1.0).boxed_clone(),
			SphereGeometry::new(1.0).into()
		);
		assert_eq!(
			SphereGeometry::new(2.0).boxed_clone(),
			SphereGeometry::new(2.0).into()
		);
		assert_eq!(
			SphereGeometry::new(9.0).boxed_clone(),
			SphereGeometry::new(9.0).into()
		);
		assert_eq!(
			SphereGeometry::new(75.35).boxed_clone(),
			SphereGeometry::new(75.35).into()
		);
	}

	#[test]
	fn bounding_box() {
		assert_eq!(SphereGeometry::new(1.0).bounding_box(), (2., 2., 2.));
		assert_eq!(SphereGeometry::new(2.0).bounding_box(), (4., 4., 4.));
		assert_eq!(SphereGeometry::new(9.0).bounding_box(), (18., 18., 18.));
		assert_eq!(
			SphereGeometry::new(75.35).bounding_box(),
			(150.7, 150.7, 150.7)
		);
	}

	#[test]
	fn get_shape() {
		assert_eq!(
			SphereGeometry::new(1.0).shape_container(),
			GeometryShapeContainer::Sphere(SphereGeometry::new(1.0))
		);
		assert_eq!(
			SphereGeometry::new(2.0).shape_container(),
			GeometryShapeContainer::Sphere(SphereGeometry::new(2.0))
		);
		assert_eq!(
			SphereGeometry::new(9.0).shape_container(),
			GeometryShapeContainer::Sphere(SphereGeometry::new(9.0))
		);
		assert_eq!(
			SphereGeometry::new(75.35).shape_container(),
			GeometryShapeContainer::Sphere(SphereGeometry::new(75.35))
		);
	}

	#[cfg(feature = "urdf")]
	#[test]
	fn to_urdf() {
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(1.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="1"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(2.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="2"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(9.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="9"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(75.35)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="75.35"/></geometry>"#)
			);
		}
	}
}
