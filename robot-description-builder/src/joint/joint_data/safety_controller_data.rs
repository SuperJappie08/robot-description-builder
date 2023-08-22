#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct SafetyControllerData {
	//(optional, defaults to 0)
	//
	// An attribute specifying the lower joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be larger than the lower joint limit (see above). See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	pub soft_lower_limit: Option<f32>,
	// (optional, defaults to 0)
	//
	// An attribute specifying the upper joint boundary where the safety controller starts limiting the position of the joint. This limit needs to be smaller than the upper joint limit (see above). See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	pub soft_upper_limit: Option<f32>,
	//  (optional, defaults to 0)
	//
	// An attribute specifying the relation between position and velocity limits. See See safety limits for more details.
	// TODO: FIX DOCUMENTATION
	pub k_position: Option<f32>,
	// An attribute specifying the relation between effort and velocity limits. See See safety limits for more details.
	pub k_velocity: f32,
}

#[cfg(feature = "urdf")]
impl crate::to_rdf::to_urdf::ToURDF for SafetyControllerData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer
			.create_element("safety_controller")
			.with_attribute(Attribute {
				key: QName(b"k_velocity"),
				value: self.k_velocity.to_string().as_bytes().into(),
			});

		if let Some(k_position) = self.k_position {
			element = element.with_attribute(Attribute {
				key: QName(b"k_position"),
				value: k_position.to_string().as_bytes().into(),
			});
		}

		if let Some(soft_lower_limit) = self.soft_lower_limit {
			element = element.with_attribute(Attribute {
				key: QName(b"soft_lower_limit"),
				value: soft_lower_limit.to_string().as_bytes().into(),
			})
		}

		if let Some(soft_upper_limit) = self.soft_upper_limit {
			element = element.with_attribute(Attribute {
				key: QName(b"soft_upper_limit"),
				value: soft_upper_limit.to_string().as_bytes().into(),
			})
		}

		element.write_empty()?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::joint::joint_data::SafetyControllerData;
	use test_log::test;

	#[cfg(feature = "urdf")]
	mod to_urdf {

		use std::io::Seek;

		use super::{test, SafetyControllerData};
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};

		fn test_to_urdf_safety_contoller(
			safety_controller: SafetyControllerData,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

			assert!(safety_controller.to_urdf(&mut writer, urdf_config).is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			);
		}

		#[test]
		fn only_k_velocity() {
			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 10.,
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="10"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 1000000.,
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="1000000"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 0.00123,
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="0.00123"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn k_position() {
			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 10.,
					k_position: Some(100.),
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="10" k_position="100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 1000000.,
					k_position: Some(-0.0000987),
					..Default::default()
				},
				String::from(
					r#"<safety_controller k_velocity="1000000" k_position="-0.0000987"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 0.00123,
					k_position: Some(988000000.),
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="0.00123" k_position="988000000"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn soft_lower_limit() {
			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 10.,
					soft_lower_limit: Some(-100.),
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="10" soft_lower_limit="-100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 1000000.,
					soft_lower_limit: Some(-0.0000987),
					..Default::default()
				},
				String::from(
					r#"<safety_controller k_velocity="1000000" soft_lower_limit="-0.0000987"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 0.00123,
					soft_lower_limit: Some(988000000.),
					..Default::default()
				},
				String::from(
					r#"<safety_controller k_velocity="0.00123" soft_lower_limit="988000000"/>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn soft_upper_limit() {
			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 10.,
					soft_upper_limit: Some(100.),
					..Default::default()
				},
				String::from(r#"<safety_controller k_velocity="10" soft_upper_limit="100"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 1000000.,
					soft_upper_limit: Some(0.0000987),
					..Default::default()
				},
				String::from(
					r#"<safety_controller k_velocity="1000000" soft_upper_limit="0.0000987"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 0.00123,
					soft_upper_limit: Some(-988000000.),
					..Default::default()
				},
				String::from(
					r#"<safety_controller k_velocity="0.00123" soft_upper_limit="-988000000"/>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn full() {
			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 10.,
					k_position: Some(23.),
					soft_lower_limit: Some(-100.),
					soft_upper_limit: Some(100.),
				},
				String::from(
					r#"<safety_controller k_velocity="10" k_position="23" soft_lower_limit="-100" soft_upper_limit="100"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 1000000.,
					k_position: Some(-999.),
					soft_lower_limit: Some(-0.0000987),
					soft_upper_limit: Some(0.0000987),
				},
				String::from(
					r#"<safety_controller k_velocity="1000000" k_position="-999" soft_lower_limit="-0.0000987" soft_upper_limit="0.0000987"/>"#,
				),
				&URDFConfig::default(),
			);

			test_to_urdf_safety_contoller(
				SafetyControllerData {
					k_velocity: 0.00123,
					k_position: Some(10000000000000.),
					soft_upper_limit: Some(-988000000.),
					soft_lower_limit: Some(988000000.), // TODO: Maybe it should be checked if they are reversed?
				},
				String::from(
					r#"<safety_controller k_velocity="0.00123" k_position="10000000000000" soft_lower_limit="988000000" soft_upper_limit="-988000000"/>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
