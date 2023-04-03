use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct LimitData {
	pub lower: Option<f32>,
	pub upper: Option<f32>,
	pub effort: f32,
	pub velocity: f32,
}

impl crate::to_rdf::to_urdf::ToURDF for LimitData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer
			.create_element("limit")
			.with_attribute(Attribute {
				key: QName(b"effort"),
				value: self.effort.to_string().as_bytes().into(),
			})
			.with_attribute(Attribute {
				key: QName(b"velocity"),
				value: self.velocity.to_string().as_bytes().into(),
			});

		if let Some(lower) = self.lower {
			element = element.with_attribute(Attribute {
				key: QName(b"lower"),
				value: lower.to_string().as_bytes().into(),
			})
		}

		if let Some(upper) = self.upper {
			element = element.with_attribute(Attribute {
				key: QName(b"upper"),
				value: upper.to_string().as_bytes().into(),
			})
		}

		element.write_empty()?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::LimitData;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::io::Seek;

		use super::LimitData;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		#[test]
		fn only_required() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 30.,
					velocity: 1000.,
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="30" velocity="1000"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 40e6,
					velocity: 123.456,
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="40000000" velocity="123.456"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 0.003,
					velocity: 0.005,
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="0.003" velocity="0.005"/>"#)
				);
			}
		}

		#[test]
		fn also_lower() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 30.,
					velocity: 1000.,
					lower: Some(-100.),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="30" velocity="1000" lower="-100"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 40e6,
					velocity: 123.456,
					lower: Some(-0.009),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="40000000" velocity="123.456" lower="-0.009"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 0.003,
					velocity: 0.005,
					lower: Some(1e4),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="0.003" velocity="0.005" lower="10000"/>"#)
				);
			}
		}

		#[test]
		fn also_upper() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 30.,
					velocity: 1000.,
					upper: Some(100.),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="30" velocity="1000" upper="100"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 40e6,
					velocity: 123.456,
					upper: Some(0.009),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="40000000" velocity="123.456" upper="0.009"/>"#)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 0.003,
					velocity: 0.005,
					upper: Some(-1e4),
					..Default::default()
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(r#"<limit effort="0.003" velocity="0.005" upper="-10000"/>"#)
				);
			}
		}

		#[test]
		fn lower_upper() {
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 30.,
					velocity: 1000.,
					lower: Some(-200.),
					upper: Some(100.),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(
						r#"<limit effort="30" velocity="1000" lower="-200" upper="100"/>"#
					)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 40e6,
					velocity: 123.456,
					lower: Some(-0.0004),
					upper: Some(0.009),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(
						r#"<limit effort="40000000" velocity="123.456" lower="-0.0004" upper="0.009"/>"#
					)
				);
			}
			{
				let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
				assert!(LimitData {
					effort: 0.003,
					velocity: 0.005,
					lower: Some(2e5),
					upper: Some(-1e4),
				}
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

				writer.inner().rewind().unwrap();

				assert_eq!(
					std::io::read_to_string(writer.inner()).unwrap(),
					String::from(
						r#"<limit effort="0.003" velocity="0.005" lower="200000" upper="-10000"/>"#
					)
				);
			}
		}
	}
}
