#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
use crate::transform_data::TransformData;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Default)]
/// TODO: Figure out if things should be private or not?
pub struct InertialData {
	pub origin: Option<TransformData>,
	pub mass: f32,
	pub ixx: f32, // Not the nicesest way of doing this.
	pub ixy: f32,
	pub ixz: f32,
	pub iyy: f32,
	pub iyz: f32,
	pub izz: f32,
}

#[cfg(feature = "urdf")]
impl ToURDF for InertialData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("inertial");
		element.write_inner_content(|writer| {
			if let Some(origin) = &self.origin {
				origin.to_urdf(writer, urdf_config)?;
			}

			writer
				.create_element("mass")
				.with_attribute(Attribute {
					key: QName(b"value"),
					value: format!("{}", self.mass).as_bytes().into(),
				})
				.write_empty()?;

			writer
				.create_element("inertia")
				.with_attributes([
					("ixx", self.ixx.to_string().as_str()),
					("ixy", self.ixy.to_string().as_str()),
					("ixz", self.ixz.to_string().as_str()),
					("iyy", self.iyy.to_string().as_str()),
					("iyz", self.iyz.to_string().as_str()),
					("izz", self.izz.to_string().as_str()),
				])
				.write_empty()?;

			Ok(())
		})?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::link::inertial::InertialData;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::*;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		#[test]
		fn no_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(InertialData {
				mass: 0.12,
				ixx: 1.23,
				ixy: 2.34,
				ixz: 3.45,
				iyy: 4.56,
				iyz: 5.67,
				izz: 6.78,
				..Default::default()
			}
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				r#"<inertial><mass value="0.12"/><inertia ixx="1.23" ixy="2.34" ixz="3.45" iyy="4.56" iyz="5.67" izz="6.78"/></inertial>"#
			)
		}

		#[test]
		fn with_origin() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(InertialData {
				origin: Some(crate::transform_data::TransformData {
					translation: Some((10.1, 20.2, 30.3)),
					..Default::default()
				}),
				mass: 100.,
				ixx: 123.,
				iyy: 456.,
				izz: 789.,
				..Default::default()
			}
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				r#"<inertial><origin xyz="10.1 20.2 30.3"/><mass value="100"/><inertia ixx="123" ixy="0" ixz="0" iyy="456" iyz="0" izz="789"/></inertial>"#
			)
		}
	}
}
