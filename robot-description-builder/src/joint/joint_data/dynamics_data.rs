#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct DynamicsData {
	pub damping: Option<f32>,
	pub friction: Option<f32>,
}

impl DynamicsData {
	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// ## Example
	///
	/// ```
	/// # use robot_description_builder::joint_data::DynamicsData;
	/// assert!(DynamicsData {
	///     damping: Some(1.),
	///     friction: Some(2.)
	/// }
	/// .contains_some());
	///
	/// assert!(DynamicsData {
	///     damping: Some(1.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(DynamicsData {
	///     friction: Some(2.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!DynamicsData::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.damping.is_some() || self.friction.is_some()
	}
}

#[cfg(feature = "urdf")]
impl crate::to_rdf::to_urdf::ToURDF for DynamicsData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		if self.contains_some() {
			let mut element = writer.create_element("dynamics");

			if let Some(damping) = self.damping {
				element = element.with_attribute(Attribute {
					key: QName(b"damping"),
					value: damping.to_string().as_bytes().into(),
				})
			}

			if let Some(friction) = self.friction {
				element = element.with_attribute(Attribute {
					key: QName(b"friction"),
					value: friction.to_string().as_bytes().into(),
				})
			}

			element.write_empty()?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::DynamicsData;

	#[test]
	fn contains_some() {
		assert!(DynamicsData {
			damping: Some(1.),
			friction: Some(2.)
		}
		.contains_some());

		assert!(DynamicsData {
			damping: Some(1.),
			..Default::default()
		}
		.contains_some());

		assert!(DynamicsData {
			friction: Some(2.),
			..Default::default()
		}
		.contains_some());

		assert!(!DynamicsData::default().contains_some())
	}

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::io::Seek;

		use super::DynamicsData;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		fn test_to_urdf_dynamics(
			dynamics_data: DynamicsData,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(dynamics_data.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			);
		}

		#[test]
		fn empty() {
			test_to_urdf_dynamics(
				DynamicsData::default(),
				String::from(r#""#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn damping() {
			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(1000.),
					..Default::default()
				},
				String::from(r#"<dynamics damping="1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(0.02),
					..Default::default()
				},
				String::from(r#"<dynamics damping="0.02"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(9e-6),
					..Default::default()
				},
				String::from(r#"<dynamics damping="0.000009"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn friction() {
			test_to_urdf_dynamics(
				DynamicsData {
					friction: Some(1000.),
					..Default::default()
				},
				String::from(r#"<dynamics friction="1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					friction: Some(0.02),
					..Default::default()
				},
				String::from(r#"<dynamics friction="0.02"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					friction: Some(9e-6),
					..Default::default()
				},
				String::from(r#"<dynamics friction="0.000009"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn damping_friction() {
			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(1000.),
					friction: Some(900000.),
				},
				String::from(r#"<dynamics damping="1000" friction="900000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(0.02),
					friction: Some(0.004),
				},
				String::from(r#"<dynamics damping="0.02" friction="0.004"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_dynamics(
				DynamicsData {
					damping: Some(9e-6),
					friction: Some(15e-4),
				},
				String::from(r#"<dynamics damping="0.000009" friction="0.0015"/>"#),
				&URDFConfig::default(),
			);
		}
	}
}
