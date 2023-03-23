use std::f32::consts::{FRAC_PI_3, PI};

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use crate::link::geometry::GeometryInterface;
#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

#[derive(Debug, PartialEq, Clone)]
pub struct SphereGeometry {
	radius: f32,
}

impl SphereGeometry {
	pub fn new(radius: f32) -> Self {
		Self { radius }
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

	// fn get_type(&self) -> GeometryType {
	// GeometryType::Sphere
	// }

	// fn get_data(&self) -> GeometryData {
	// GeometryData::Sphere(self.clone())
	// }
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

	use crate::link::geometry::{sphere_geometry::SphereGeometry, GeometryInterface};
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

	#[cfg(feature = "urdf")]
	#[test]
	fn to_urdf() {
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(1.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="1"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(2.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="2"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(9.0)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="9"/></geometry>"#)
			);
		}
		{
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(SphereGeometry::new(75.35)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<geometry><sphere radius="75.35"/></geometry>"#)
			);
		}
	}
}
