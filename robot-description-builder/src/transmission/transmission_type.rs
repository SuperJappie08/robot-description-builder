#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::events::BytesText;

// TODO: INTRO SENTENCE
// TODO: DOC
// <http://wiki.ros.org/ros_control#Transmission_Interfaces>
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TransmissionType {
	// TODO: DOC
	// FROM [`ros_control`](http://wiki.ros.org/ros_control#Transmission_Interfaces)
	SimpleTransmission,
	// TODO: DOC
	// FROM [`ros_control`](http://wiki.ros.org/ros_control#Transmission_Interfaces)
	DifferentialTransmission,
	// TODO: DOC
	// FROM [`ros_control`](http://wiki.ros.org/ros_control#Transmission_Interfaces)
	FourBarLinkageTransmission,
}

impl TransmissionType {
	/// Gets the URDF String identifier for this `TransmissionType`
	///
	/// TODO: Might not be named inline with [convention](https://rust-lang.github.io/api-guidelines/naming.html#ad-hoc-conversions-follow-as_-to_-into_-conventions-c-conv)
	#[cfg(feature = "urdf")]
	fn as_urdf_transmission_type(&self) -> String {
		let mut result = String::from(match true {
			true => "transmission_interface/",
			false => "",
		});

		match self {
			Self::SimpleTransmission => result.push_str("SimpleTransmission"),
			Self::DifferentialTransmission => result.push_str("DifferentialTransmission"),
			Self::FourBarLinkageTransmission => result.push_str("FourBarLinkageTransmission"),
		}

		result
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for TransmissionType {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("type")
			.write_text_content(BytesText::new(self.as_urdf_transmission_type().as_str()))?;

		Ok(())
	}
}
