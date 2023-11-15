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
use std::io::{Cursor, Read, Seek};

#[cfg(feature = "urdf")]
pub mod to_urdf;

#[cfg(feature = "sdf")]
pub mod to_sdf;

/// A setting for configuring the style of the generated XML representation.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum XMLMode {
	/// No indentation in the output XML.
	#[default]
	NoIndent,
	/// Indentation as specified in the output XML.
	///
	/// The indentation level will increase with every opening XML element and decreases when a XML element is closed.
	/// - `char` is the character will be used to indent the elements.
	/// - `usize` is the amount of character which will be used per indent level.
	Indent(char, usize),
}

/// Create a XML-writer with the specified [`XMLMode`].
#[cfg(feature = "xml")]
fn make_xml_writer(xml_mode: XMLMode) -> Writer<Cursor<Vec<u8>>> {
	match xml_mode {
		XMLMode::NoIndent => Writer::new(Cursor::new(Vec::new())),
		XMLMode::Indent(c, depth) => {
			Writer::new_with_indent(Cursor::new(Vec::new()), c as u8, depth)
		}
	}
}

#[cfg(feature = "xml")]
/// Convert a [`quick_xml::Writer`] to a [`String`].
///
/// # Example
/// ```rust
/// # use robot_description_builder::{Link, link_data::{Visual, geometry::SphereGeometry}, to_rdf::{xml_writer_to_string, to_urdf::{to_urdf, URDFConfig}}};
/// let robot = Link::builder("my-link")
///     .add_visual(
///         Visual::builder(SphereGeometry::new(4.0))
///     )
///     .build_tree()
///     .to_robot("my-robot");
///
/// let writer = to_urdf(&robot, URDFConfig::default()).unwrap();
/// let urdf_description: String = xml_writer_to_string(writer);
///
/// assert_eq!(
///     urdf_description,
///     String::from(
///         "\u{feff}<?xml version=\"1.0\"?><robot name=\"my-robot\"><link name=\"my-link\"><visual><geometry><sphere radius=\"4\"/></geometry></visual></link></robot>"
///     )
/// );
/// ```
pub fn xml_writer_to_string(writer: Writer<Cursor<Vec<u8>>>) -> String {
	let mut buffer = writer.into_inner();

	let mut out = String::new();
	buffer.rewind().unwrap();
	buffer.read_to_string(&mut out).unwrap();

	out
}
