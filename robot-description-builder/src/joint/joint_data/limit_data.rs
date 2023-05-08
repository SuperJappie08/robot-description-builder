#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct LimitData {
	pub lower: Option<f32>,
	pub upper: Option<f32>,
	pub effort: f32,
	pub velocity: f32,
}

#[cfg(feature = "urdf")]
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
	use test_log::test;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::io::Seek;

		use super::{test, LimitData};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		fn test_to_urdf_limit(limit_data: LimitData, result: String, urdf_config: &URDFConfig) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(limit_data.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			);
		}

		#[test]
		fn only_required() {
			test_to_urdf_limit(
				LimitData {
					effort: 30.,
					velocity: 1000.,
					..Default::default()
				},
				String::from(r#"<limit effort="30" velocity="1000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 40e6,
					velocity: 123.456,
					..Default::default()
				},
				String::from(r#"<limit effort="40000000" velocity="123.456"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 0.003,
					velocity: 0.005,
					..Default::default()
				},
				String::from(r#"<limit effort="0.003" velocity="0.005"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn also_lower() {
			test_to_urdf_limit(
				LimitData {
					effort: 30.,
					velocity: 1000.,
					lower: Some(-100.),
					..Default::default()
				},
				String::from(r#"<limit effort="30" velocity="1000" lower="-100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 40e6,
					velocity: 123.456,
					lower: Some(-0.009),
					..Default::default()
				},
				String::from(r#"<limit effort="40000000" velocity="123.456" lower="-0.009"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 0.003,
					velocity: 0.005,
					lower: Some(1e4),
					..Default::default()
				},
				String::from(r#"<limit effort="0.003" velocity="0.005" lower="10000"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn also_upper() {
			test_to_urdf_limit(
				LimitData {
					effort: 30.,
					velocity: 1000.,
					upper: Some(100.),
					..Default::default()
				},
				String::from(r#"<limit effort="30" velocity="1000" upper="100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 40e6,
					velocity: 123.456,
					upper: Some(0.009),
					..Default::default()
				},
				String::from(r#"<limit effort="40000000" velocity="123.456" upper="0.009"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 0.003,
					velocity: 0.005,
					upper: Some(-1e4),
					..Default::default()
				},
				String::from(r#"<limit effort="0.003" velocity="0.005" upper="-10000"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn lower_upper() {
			test_to_urdf_limit(
				LimitData {
					effort: 30.,
					velocity: 1000.,
					lower: Some(-200.),
					upper: Some(100.),
				},
				String::from(r#"<limit effort="30" velocity="1000" lower="-200" upper="100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 40e6,
					velocity: 123.456,
					lower: Some(-0.0004),
					upper: Some(0.009),
				},
				String::from(
					r#"<limit effort="40000000" velocity="123.456" lower="-0.0004" upper="0.009"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_limit(
				LimitData {
					effort: 0.003,
					velocity: 0.005,
					lower: Some(2e5),
					upper: Some(-1e4),
				},
				String::from(
					r#"<limit effort="0.003" velocity="0.005" lower="200000" upper="-10000"/>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
