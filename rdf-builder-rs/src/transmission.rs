#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};

#[derive(Debug, PartialEq, Eq)]
/// TODO: IMPLEMENT PROPPERLY, THIS IS TEMPORARY
pub struct Transmission {
	pub name: String,
}

impl Transmission {
	pub fn get_name(&self) -> &String {
		&self.name
	}
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
				value: self.get_name().as_bytes().into(),
			})
			.write_text_content(BytesText::new("<!-- TODO: TRANSMISSIONS -->"))?;
		Ok(())
	}
}
