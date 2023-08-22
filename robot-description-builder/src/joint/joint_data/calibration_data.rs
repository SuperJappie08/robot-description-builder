#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
// TODO: DOC
pub struct CalibrationData {
	pub rising: Option<f32>,
	pub falling: Option<f32>,
}

impl CalibrationData {
	/// A function to check if any of the fields are set.
	///
	/// It doesn't check if the some fields have the default value, since it can be format depended.
	///
	/// ## Example
	///
	/// ```
	/// # use robot_description_builder::joint_data::CalibrationData;
	/// assert!(CalibrationData {
	///     rising: Some(1.),
	///     falling: Some(2.)
	/// }
	/// .contains_some());
	///
	/// assert!(CalibrationData {
	///     rising: Some(1.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(CalibrationData {
	///     falling: Some(2.),
	///     ..Default::default()
	/// }
	/// .contains_some());
	///
	/// assert!(!CalibrationData::default().contains_some())
	/// ```
	pub fn contains_some(&self) -> bool {
		self.rising.is_some() || self.falling.is_some()
	}
}

#[cfg(feature = "urdf")]
impl crate::to_rdf::to_urdf::ToURDF for CalibrationData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		if self.contains_some() {
			let mut element = writer.create_element("calibration");

			if let Some(rising) = self.rising {
				element = element.with_attribute(Attribute {
					key: QName(b"rising"),
					value: rising.to_string().as_bytes().into(),
				});
			}

			if let Some(falling) = self.falling {
				element = element.with_attribute(Attribute {
					key: QName(b"falling"),
					value: falling.to_string().as_bytes().into(),
				});
			}

			element.write_empty()?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::calibration_data::CalibrationData;
	use test_log::test;

	#[test]
	fn contains_some() {
		assert!(CalibrationData {
			rising: Some(1.),
			falling: Some(2.)
		}
		.contains_some());

		assert!(CalibrationData {
			rising: Some(1.),
			..Default::default()
		}
		.contains_some());

		assert!(CalibrationData {
			falling: Some(2.),
			..Default::default()
		}
		.contains_some());

		assert!(!CalibrationData::default().contains_some())
	}

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::io::Seek;

		use super::{test, CalibrationData};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		fn test_to_urdf_calibration(
			calibration_data: CalibrationData,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(calibration_data.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			);
		}

		#[test]
		fn empty() {
			test_to_urdf_calibration(
				CalibrationData::default(),
				String::from(r#""#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn rising() {
			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(1000.),
					..Default::default()
				},
				String::from(r#"<calibration rising="1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(0.02),
					..Default::default()
				},
				String::from(r#"<calibration rising="0.02"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(-9e-6),
					..Default::default()
				},
				String::from(r#"<calibration rising="-0.000009"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn falling() {
			test_to_urdf_calibration(
				CalibrationData {
					falling: Some(-1000.),
					..Default::default()
				},
				String::from(r#"<calibration falling="-1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					falling: Some(-0.02),
					..Default::default()
				},
				String::from(r#"<calibration falling="-0.02"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					falling: Some(9e-6),
					..Default::default()
				},
				String::from(r#"<calibration falling="0.000009"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn rising_falling() {
			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(500.),
					falling: Some(-1000.),
				},
				String::from(r#"<calibration rising="500" falling="-1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(2e9),
					falling: Some(-0.02),
				},
				String::from(r#"<calibration rising="2000000000" falling="-0.02"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_calibration(
				CalibrationData {
					rising: Some(-10000.0),
					falling: Some(9e-6),
				},
				String::from(r#"<calibration rising="-10000" falling="0.000009"/>"#),
				&URDFConfig::default(),
			);
		}
	}
}
