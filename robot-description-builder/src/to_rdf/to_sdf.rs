//! The infrastructure to describe a `Robot` in SDFormat (SDF).
use std::io::Cursor;

use quick_xml::{
	events::{BytesDecl, Event},
	Writer,
};

use super::{make_xml_writer, XMLMode};
use crate::cluster_objects::KinematicInterface;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SDFConfig {
	pub xml_mode: XMLMode,
}

/// A trait to allow parts of a `Robot` to be described in the SDFormat.
pub trait ToSDF {
	/// Represents the element as in SDFormat.
	fn to_sdf(
		&self,
		writer: &mut Writer<Cursor<Vec<u8>>>,
		sdf_config: &SDFConfig,
	) -> Result<(), quick_xml::Error>;
}

/// A function to represent a `KinematicInterface` implementor in the SDFormat.
///
/// TODO: EXPAND
pub fn to_sdf(
	tree: &(impl KinematicInterface + ToSDF),
	sdf_config: SDFConfig,
) -> Result<Writer<Cursor<Vec<u8>>>, quick_xml::Error> {
	let mut writer = make_xml_writer(self.xml_mode);

	writer.write_bom()?;
	writer.write_event(&Event::Decl(BytesDecl::new("1.0", None, None)))?;
	tree.to_sdf(&mut writer, &sdf_config)?;
	Ok(writer)
}
