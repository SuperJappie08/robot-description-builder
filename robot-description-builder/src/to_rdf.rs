//! The infrastructure to describe a `Robot` in a robot description format.
//!
//! TODO: EXPAND
// DOCS TODO:
//  - Module
//  - to_urdf
//  - to_sdf

#[cfg(feature = "xml")]
use quick_xml::Writer;
#[cfg(feature = "xml")]
use std::io::Cursor;

#[cfg(feature = "urdf")]
pub mod to_urdf;

#[cfg(feature = "sdf")]
pub mod to_sdf;

/// A setting for configuring the style of the generated XML representation.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum XMLMode {
	/// No indentation in the output XML
	#[default]
	NoIndent,
	/// Indentation as specified in the output XML
	///
	/// The indentation level will increase with every opening XML element and decreases when a XML element is closed.
	/// - `char` is the character will be used to indent the elements.
	/// - `usize` is the amount of character which will be used per indent level.
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
