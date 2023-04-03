#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// TODO: maybe change visibilty
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
	/// ```--rust
	/// # use rdf_builder_rs::joint::joint_data::CalibrationData;
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
	// TODO: Decide if this is neccesary // use test_log::test;

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

		use super::CalibrationData;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		#[test]
		fn empty() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(CalibrationData::default()
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#""#)
			);
		}

		#[test]
		fn rising() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(1000.),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="1000"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(0.02),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="0.02"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(-9e-6),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="-0.000009"/>"#)
				);
			}
		}

		#[test]
		fn falling() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					falling: Some(-1000.),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration falling="-1000"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					falling: Some(-0.02),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration falling="-0.02"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					falling: Some(9e-6),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration falling="0.000009"/>"#)
				);
			}
		}

		#[test]
		fn rising_falling() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(500.),
					falling: Some(-1000.),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="500" falling="-1000"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(2e9),
					falling: Some(-0.02),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="2000000000" falling="-0.02"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(CalibrationData {
					rising: Some(-10000.0),
					falling: Some(9e-6),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<calibration rising="-10000" falling="0.000009"/>"#)
				);
			}
		}
	}
}
