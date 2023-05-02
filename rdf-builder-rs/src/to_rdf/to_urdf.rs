use std::io::Cursor;

use quick_xml::{
	events::{BytesDecl, Event},
	Writer,
};

use crate::cluster_objects::KinematicInterface;

use crate::to_rdf::XMLMode;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// FIXME: FIX CONFIG, MAYBE MAKE AN INTERNAL CONFIG TYPE
pub struct URDFConfig {
	pub material_references: URDFMaterialReferences,
	pub direct_material_ref: URDFMaterialMode,
	pub urdf_target: URDFTarget,
	pub xml_mode: XMLMode,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum URDFMaterialReferences {
	#[default]
	AllNamedMaterialOnTop,
	OnlyMultiUseMaterials,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum URDFMaterialMode {
	#[default]
	FullMaterial,
	Referenced,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum URDFTarget {
	/// for RobotHW
	#[default]
	Standard,
	/// Not fully implemented yet
	Gazebo,
}

pub trait ToURDF {
	/// Represents the element as in the URDF format.
	fn to_urdf(
		&self,
		writer: &mut Writer<Cursor<Vec<u8>>>,
		urdf_config: &URDFConfig,
	) -> Result<(), quick_xml::Error>;
}

pub fn to_urdf(
	tree: impl KinematicInterface + ToURDF,
	urdf_config: URDFConfig,
) -> Result<Writer<Cursor<Vec<u8>>>, quick_xml::Error> {
	let mut writer = match urdf_config.xml_mode {
		XMLMode::NoIndent => Writer::new(Cursor::new(Vec::new())),
		XMLMode::Indent(c, depth) => {
			Writer::new_with_indent(Cursor::new(Vec::new()), c as u8, depth)
		}
	};
	writer.write_bom()?;
	writer.write_event(&Event::Decl(BytesDecl::new("1.0", None, None)))?;
	tree.to_urdf(&mut writer, &urdf_config)?;
	Ok(writer)
}
