#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(any(feature = "urdf", feature = "sdf"))]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransformData {
	pub translation: Option<(f32, f32, f32)>,
	pub rotation: Option<(f32, f32, f32)>,
}

impl TransformData {
	pub fn contains_some(&self) -> bool {
		self.translation.is_some() || self.rotation.is_some()
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for TransformData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("origin");
		if let Some(translation) = self.translation.clone() {
			element = element.with_attribute(Attribute {
				key: QName(b"xyz"),
				value: format!("{} {} {}", translation.0, translation.1, translation.2)
					.as_bytes()
					.into(),
			})
		}

		if let Some(rotation) = self.rotation.clone() {
			element = element.with_attribute(Attribute {
				key: QName(b"rpy"),
				value: format!("{} {} {}", rotation.0, rotation.1, rotation.2)
					.as_bytes()
					.into(),
			});
		}

		element.write_empty()?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::transform_data::TransformData;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::*;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		#[test]
		fn translation_only() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(TransformData {
				translation: Some((1.2, 2.3, 3.4)),
				..Default::default()
			}
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();
			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<origin xyz="1.2 2.3 3.4"/>"#)
			)
		}

		#[test]
		fn rotation_only() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(TransformData {
				rotation: Some((1.2, 2.3, 3.4)),
				..Default::default()
			}
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();
			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<origin rpy="1.2 2.3 3.4"/>"#)
			)
		}

		#[test]
		fn translation_rotatation() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(TransformData {
				translation: Some((1.23, 2.34, 3.45)),
				rotation: Some((4.56, 5.67, 6.78)),
			}
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();
			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<origin xyz="1.23 2.34 3.45" rpy="4.56 5.67 6.78"/>"#)
			)
		}
	}
}
