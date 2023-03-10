use quick_xml::{
	events::{attributes::Attribute, BytesText},
	name::QName,
};

use crate::to_rdf::to_urdf::{ToURDF, URDFMaterialMode};

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
	pub name: Option<String>,
	material: MaterialData,
}

impl Material {
	/// TODO: FIGURE OUT IF I WANT THIS
	pub fn get_name(&self) -> Option<String> {
		self.name.clone()
	}
}

impl ToURDF for Material {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("material");
		if self.name.is_some() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: self.name.clone().unwrap().as_bytes().into(),
			});
			match urdf_config.direct_material_ref {
				URDFMaterialMode::Referenced => {
					element.write_empty()?;
				}
				URDFMaterialMode::FullMaterial => {
					element.write_text_content(BytesText::new("<!--This is temp-->"))?;
				}
			};
			Ok(())
		} else {
			// TODO: FIX THIGNS
			element.write_text_content(BytesText::new("<!--This is temp-->"))?;
			Ok(())
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA
	Color(f32, f32, f32, f32),
	/// TODO: TO TEXTURE OR NOT?
	Texture,
}
