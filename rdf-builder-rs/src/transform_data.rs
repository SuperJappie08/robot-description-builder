use quick_xml::{events::attributes::Attribute, name::QName};

use crate::to_rdf::to_urdf::ToURDF;

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
