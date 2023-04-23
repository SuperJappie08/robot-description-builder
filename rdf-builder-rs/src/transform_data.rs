#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct TransformData {
	pub translation: Option<(f32, f32, f32)>,
	pub rotation: Option<(f32, f32, f32)>,
}

impl TransformData {
	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// # Example
	/// ```rust
	/// # use rdf_builder_rs::TransformData;
	/// assert!(TransformData {
	///     translation: Some((1., 2., 3.)),
	///     rotation: Some((4., 5., 6.))
	/// }
	/// .contains_some());
	///
	/// assert!(TransformData {
	///     translation: Some((1., 2., 3.)),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(TransformData {
	///     rotation: Some((4., 5., 6.)),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!TransformData::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.translation.is_some() || self.rotation.is_some()
	}

	/// TODO: There are multiple ways to mirror. Pick one that works and makes sense, or options
	///  - Mirror full thing and thereby invert joint axis
	///  - Mirror Only first joint
	pub fn mirrored(&self, axis: MirrorAxis) -> Self {
		// I doubt this check makes sense
		if self.contains_some() {
			match axis {
				MirrorAxis::X => todo!(),
				MirrorAxis::Y => todo!(),
				MirrorAxis::Z => todo!(),
			}
		} else {
			// Coping self
			*self
		}
	}
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MirrorAxis {
	X,
	Y,
	Z,
}

#[cfg(feature = "urdf")]
impl ToURDF for TransformData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("origin");
		if let Some(translation) = self.translation {
			element = element.with_attribute(Attribute {
				key: QName(b"xyz"),
				value: format!("{} {} {}", translation.0, translation.1, translation.2)
					.as_bytes()
					.into(),
			})
		}

		if let Some(rotation) = self.rotation {
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

impl From<TransformData> for crate::joint::JointBuilderTransformMode {
	fn from(value: TransformData) -> Self {
		Self::Direct(value)
	}
}

#[cfg(test)]
mod tests {
	use crate::transform_data::TransformData;
	use test_log::test;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, TransformData};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use std::io::Seek;

		fn test_to_urdf_transform(
			transform_data: TransformData,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(transform_data.to_urdf(&mut writer, urdf_config).is_ok());

			writer.inner().rewind().unwrap();
			assert_eq!(std::io::read_to_string(writer.inner()).unwrap(), result)
		}

		#[test]
		fn translation_only() {
			test_to_urdf_transform(
				TransformData {
					translation: Some((1.2, 2.3, 3.4)),
					..Default::default()
				},
				String::from(r#"<origin xyz="1.2 2.3 3.4"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn rotation_only() {
			test_to_urdf_transform(
				TransformData {
					rotation: Some((1.2, 2.3, 3.4)),
					..Default::default()
				},
				String::from(r#"<origin rpy="1.2 2.3 3.4"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn translation_rotatation() {
			test_to_urdf_transform(
				TransformData {
					translation: Some((1.23, 2.34, 3.45)),
					rotation: Some((4.56, 5.67, 6.78)),
				},
				String::from(r#"<origin xyz="1.23 2.34 3.45" rpy="4.56 5.67 6.78"/>"#),
				&URDFConfig::default(),
			);
		}
	}
}
