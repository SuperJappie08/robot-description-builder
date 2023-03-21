use std::io::Cursor;

use quick_xml::{Writer, events::{Event, BytesDecl}};

use crate::{to_rdf::XMLMode, cluster_objects::KinematicInterface};


#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SDFConfig {
    pub xml_mode: XMLMode,
}

pub trait ToSDF {
    fn to_sdf(&self, writer: &mut Writer<Cursor<Vec<u8>>>, sdf_config: &SDFConfig) -> Result<(), quick_xml::Error>;
}

pub fn to_sdf(
	tree: impl KinematicInterface + ToSDF,
	sdf_config: SDFConfig,
) -> Result<Writer<Cursor<Vec<u8>>>, quick_xml::Error> {
	let mut writer = match sdf_config.xml_mode {
		XMLMode::NoIndent => Writer::new(Cursor::new(Vec::new())),
		XMLMode::Indent(c, depth) => {
			Writer::new_with_indent(Cursor::new(Vec::new()), c as u8, depth)
		}
	};
	writer.write_bom()?;
	writer.write_event(&Event::Decl(BytesDecl::new("1.0", None, None)))?;
	tree.to_sdf(&mut writer, &sdf_config)?;
	Ok(writer)
}