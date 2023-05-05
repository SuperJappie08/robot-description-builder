use crate::identifiers::GroupID;
#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransmissionActuator {
	name: String,
	/// Specifies a mechanical reduction at the joint/actuator transmission. This tag may not be needed for all transmissions.
	mechanical_reduction: Option<f32>,
}

impl TransmissionActuator {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			..Default::default()
		}
	}

	pub fn new_with_reduction(name: impl Into<String>, mechanical_reduction: f32) -> Self {
		Self {
			name: name.into(),
			mechanical_reduction: Some(mechanical_reduction),
		}
	}

	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn mechanically_reduced(mut self, mechanical_reduction: f32) -> Self {
		self.mechanical_reduction = Some(mechanical_reduction);
		self
	}

	pub fn mechanical_reduction(&self) -> Option<&f32> {
		self.mechanical_reduction.as_ref()
	}

	/// TODO: Maybe remove because of immutability
	#[deprecated]
	pub fn set_mechanical_reduction(&mut self, mechanical_reduction: f32) {
		self.mechanical_reduction = Some(mechanical_reduction);
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for TransmissionActuator {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let element = writer.create_element("actuator").with_attribute(Attribute {
			key: QName(b"name"),
			value: self.name().display().as_bytes().into(),
		});

		match self.mechanical_reduction() {
			Some(reduction) => element.write_inner_content(|writer| {
				writer
					.create_element("mechanicalReduction")
					.write_text_content(BytesText::new(&format!("{}", reduction)))?;
				Ok(())
			}),
			None => element.write_empty(),
		}?;

		Ok(())
	}
}
