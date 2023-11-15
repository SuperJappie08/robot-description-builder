use std::sync::Weak;

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, identifiers::GroupID, joint::Joint,
	utils::WeakLock,
};

#[derive(Debug, Clone)]
pub struct MimicData {
	pub joint: WeakLock<Joint>,
	pub multiplier: Option<f32>,
	pub offset: Option<f32>,
}

#[cfg(feature = "urdf")]
impl crate::to_rdf::to_urdf::ToURDF for MimicData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("mimic").with_attribute(Attribute {
			key: QName(b"joint"),
			value: self
				.joint
				.upgrade()
				.unwrap() // FIXME: Is unwrap Ok here?
				.read()
				.unwrap() // FIXME: Is unwrap Ok here?
				.name()
				.display()
				.as_bytes()
				.into(),
		});

		if let Some(multiplier) = self.multiplier {
			element = element.with_attribute(Attribute {
				key: QName(b"multiplier"),
				value: multiplier.to_string().as_bytes().into(),
			})
		}

		if let Some(offset) = self.offset {
			element = element.with_attribute(Attribute {
				key: QName(b"offset"),
				value: offset.to_string().as_bytes().into(),
			})
		}

		element.write_empty()?;
		Ok(())
	}
}

impl PartialEq for MimicData {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.joint, &other.joint)
			&& self.multiplier == other.multiplier
			&& self.offset == other.offset
	}
}

impl From<MimicData> for MimicBuilderData {
	fn from(value: MimicData) -> Self {
		Self {
			joint_name: value
				.joint
				.upgrade()
				.unwrap() // FIXME: Is unwrap Ok?
				.try_read()
				.unwrap() // FIXME: Is unwrap Ok?
				.name()
				.clone(),
			multiplier: value.multiplier,
			offset: value.offset,
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub struct MimicBuilderData {
	pub joint_name: String,
	pub multiplier: Option<f32>,
	pub offset: Option<f32>,
}

impl MimicBuilderData {
	pub(crate) fn to_mimic_data(&self, tree: &Weak<KinematicDataTree>) -> MimicData {
		MimicData {
			joint: Weak::clone(
				tree.upgrade()
					.unwrap() // This unwrap is Ok
					.joints
					.try_read()
					.unwrap() // FIXME: Is this unwrap OK?
					.get(&self.joint_name)
					.unwrap(), // FIXME: Is this unwrap OK?
			),
			multiplier: self.multiplier,
			offset: self.offset,
		}
	}
}

#[cfg(test)]
mod tests {
	// use crate::joint::joint_data::MimicData;
	use test_log::test;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use std::io::Seek;

		use super::test;

		use crate::{
			cluster_objects::KinematicInterface,
			joint::{jointbuilder::BuildJoint, smartjointbuilder::SmartJointBuilder},
			link::Link,
			to_rdf::to_urdf::{ToURDF, URDFConfig},
		};

		fn test_to_urdf_mimic(
			joint_builder: impl BuildJoint,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let tree = Link::builder("root").build_tree();
			tree.get_root_link()
				.try_write()
				.unwrap()
				.try_attach_child(
					SmartJointBuilder::new_fixed("joint-s"),
					Link::builder("child_link").build_tree(),
				)
				.unwrap();

			tree.get_root_link()
				.try_write()
				.unwrap()
				.try_attach_child(joint_builder, Link::builder("child_link_2"))
				.unwrap();

			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

			assert!(tree
				.get_joint("joint-t")
				.unwrap()
				.try_read()
				.unwrap()
				.mimic
				.as_ref()
				.unwrap()
				.to_urdf(&mut writer, urdf_config)
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			);
		}

		#[test]
		fn only_joint() {
			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t").with_mimic("joint-s"),
				String::from(r#"<mimic joint="joint-s"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn multiplier() {
			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_multiplier(20.),
				String::from(r#"<mimic joint="joint-s" multiplier="20"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_multiplier(0.00001),
				String::from(r#"<mimic joint="joint-s" multiplier="0.00001"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_multiplier(90000.3),
				String::from(r#"<mimic joint="joint-s" multiplier="90000.3"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn offset() {
			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_offset(20.),
				String::from(r#"<mimic joint="joint-s" offset="20"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_offset(0.00001),
				String::from(r#"<mimic joint="joint-s" offset="0.00001"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_offset(9000000.),
				String::from(r#"<mimic joint="joint-s" offset="9000000"/>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn multiplier_offset() {
			test_to_urdf_mimic(
				SmartJointBuilder::new_continuous("joint-t")
					.with_mimic("joint-s")
					.set_mimic_offset(20.)
					.set_mimic_multiplier(-18.),
				String::from(r#"<mimic joint="joint-s" multiplier="-18" offset="20"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new("joint-t")
					.continuous()
					.with_mimic("joint-s")
					.set_mimic_multiplier(100000.)
					.set_mimic_offset(0.00001),
				String::from(r#"<mimic joint="joint-s" multiplier="100000" offset="0.00001"/>"#),
				&URDFConfig::default(),
			);

			test_to_urdf_mimic(
				SmartJointBuilder::new("joint-t")
					.continuous()
					.with_mimic("joint-s")
					.set_mimic_multiplier(0.00000123)
					.set_mimic_offset(9000000.),
				String::from(
					r#"<mimic joint="joint-s" multiplier="0.00000123" offset="9000000"/>"#,
				),
				&URDFConfig::default(),
			);
		}
	}
}
