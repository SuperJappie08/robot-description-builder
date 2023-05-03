//! The infrastructure to describe a `Robot` in a robot description format.
//!
//! TODO: EXPAND

#[cfg(feature = "xml")]
use quick_xml::Writer;
#[cfg(feature = "xml")]
use std::io::Cursor;

#[cfg(feature = "urdf")]
pub mod to_urdf;

#[cfg(feature = "sdf")]
pub mod to_sdf;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum XMLMode {
	#[default]
	NoIndent,
	Indent(char, usize),
}

/// Create a XML-writer with the specified `XMLMode`
#[cfg(feature = "xml")]
fn make_xml_writer(xml_mode: XMLMode) -> Writer<Cursor<Vec<u8>>> {
	match xml_mode {
		XMLMode::NoIndent => Writer::new(Cursor::new(Vec::new())),
		XMLMode::Indent(c, depth) => {
			Writer::new_with_indent(Cursor::new(Vec::new()), c as u8, depth)
		}
	}
}
