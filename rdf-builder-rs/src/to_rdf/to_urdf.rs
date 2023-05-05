//! The infrastructure to describe a `Robot` in the Universal Robot Description Format (URDF).
//!
//! TODO: EXPAND

use std::io::Cursor;

use quick_xml::{
	events::{BytesDecl, Event},
	Writer,
};

use super::{make_xml_writer, XMLMode};
use crate::cluster_objects::KinematicInterface;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
/// FIXME: FIX CONFIG, MAYBE MAKE AN INTERNAL CONFIG TYPE
/// A configuration struct to configure the
pub struct URDFConfig {
	pub material_references: URDFMaterialReferences,
	/// Cannot make pub(crate), because of externa; default //TO
	pub direct_material_ref: URDFMaterialMode,
	pub urdf_target: URDFTarget,
	pub xml_mode: XMLMode,
}

/// TODO: ADD FULL MATERIALS EVERYWHERE MODE
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

/// A trait to allow parts of a `Robot` to be described in the URDF format.
///
/// TODO: EXPAND?
pub trait ToURDF {
	/// Represents the element as in URDF format.
	fn to_urdf(
		&self,
		writer: &mut Writer<Cursor<Vec<u8>>>,
		urdf_config: &URDFConfig,
	) -> Result<(), quick_xml::Error>;
}

/// A function to represent a `KinematicInterface` implementor in the URDF format.
///
/// TODO: EXPAND
pub fn to_urdf(
	tree: &(impl KinematicInterface + ToURDF),
	urdf_config: URDFConfig,
) -> Result<Writer<Cursor<Vec<u8>>>, quick_xml::Error> {
	let mut writer = make_xml_writer(urdf_config.xml_mode);

	writer.write_bom()?;
	writer.write_event(&Event::Decl(BytesDecl::new("1.0", None, None)))?;
	tree.to_urdf(&mut writer, &urdf_config)?;
	Ok(writer)
}

// This does not work due to ElementWriter.write_inner() expecting a closure that returns `quick_xml::Error`
// /// TODO: DOCS
// /// TODO: DOES THIS COMPLY WITH THE NAMING CONVENTION
// /// THIS DOES NOT WORK DO TO CLOSURES
// #[derive(Debug, Error)]
// pub enum ToURDFError {
// 	#[error(transparent)]
// 	XML(#[from] quick_xml::Error),
// }
