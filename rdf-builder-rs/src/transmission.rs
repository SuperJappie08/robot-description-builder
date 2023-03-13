#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(any(feature = "urdf", feature = "sdf"))]
use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};

#[derive(Debug, PartialEq, Eq)]
/// TODO: IMPLEMENT PROPPERLY, THIS IS TEMPORARY
pub struct Transmission {
	pub name: String,
}

#[cfg(feature = "urdf")]
impl ToURDF for Transmission {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("transmission")
			.with_attribute(Attribute {
				key: QName(b"name"),
				value: self.name.clone().as_bytes().into(),
			})
			.write_text_content(BytesText::new("<!-- TODO: TRANSMISSIONS -->"))?;
		Ok(())
	}
}
