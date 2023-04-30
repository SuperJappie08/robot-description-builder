// use crate::{WeakLock, Joint};
#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};

// #[derive(Debug)]
// pub struct TransmissionBuilder{
// 	name: String,
// 	transmission_type: String,
// 	joints: Vec<TransmissionJoint>
// }

// #[derive(Debug)]
// pub struct TransmissionJoint {
// 	joint: WeakLock<Joint>,
// 	/// TODO:
// 	hardware_interface: TransmissionHardwareInterface,
// }

// #[derive(Debug)]
// pub enum TransmissionHardwareInterface {

// }

#[derive(Debug, PartialEq, Eq)]
/// TODO: IMPLEMENT PROPPERLY, THIS IS TEMPORARY
pub struct Transmission {
	pub name: String,
	/// TODO: Figure out if this is constant, probably give the ability to change it via builder
	pub(crate) transmission_type: String,
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
