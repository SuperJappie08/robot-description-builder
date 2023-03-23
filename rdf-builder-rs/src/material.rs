use std::sync::{Arc, RwLock};

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFMaterialMode};
use crate::ArcLock;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
	pub name: Option<String>,
	material: MaterialData,
}

impl Material {
	pub fn new_color(
		name: Option<String>,
		red: f32,
		green: f32,
		blue: f32,
		alpha: f32,
	) -> Material {
		Material {
			name,
			material: MaterialData::Color(red, green, blue, alpha),
		}
	}

	pub fn new_texture(name: Option<String>, texture_path: String) -> Material {
		Material {
			name,
			material: MaterialData::Texture(texture_path),
		}
	}

	/// Returns a Reference to the optional material name
	/// TODO: Maybe Make the name a reference only
	pub fn get_name(&self) -> &Option<String> {
		&self.name
	}

	pub fn get_material_data(&self) -> &MaterialData {
		&self.material
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Material {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("material");
		if let Some(name) = self.get_name() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.as_bytes().into(),
			});
			match urdf_config.direct_material_ref {
				URDFMaterialMode::Referenced => {
					element.write_empty()?;
				}
				URDFMaterialMode::FullMaterial => {
					element.write_inner_content(|writer| {
						self.get_material_data().to_urdf(writer, urdf_config)
					})?;
				}
			};
			Ok(())
		} else {
			element.write_inner_content(|writer| {
				self.get_material_data().to_urdf(writer, urdf_config)
			})?;
			Ok(())
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA
	Color(f32, f32, f32, f32),
	/// TODO: TO TEXTURE OR NOT?
	Texture(String),
}

#[cfg(feature = "urdf")]
impl ToURDF for MaterialData {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		_urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		match self {
			MaterialData::Color(red, green, blue, alpha) => {
				writer
					.create_element("color")
					.with_attribute(Attribute {
						key: QName(b"rgba"),
						value: format!("{} {} {} {}", red, green, blue, alpha)
							.as_bytes()
							.into(),
					})
					.write_empty()?;
				Ok(())
			}
			MaterialData::Texture(texture_path) => {
				writer
					.create_element("texture")
					.with_attribute(Attribute {
						key: QName(b"filename"),
						value: texture_path.clone().as_bytes().into(),
					})
					.write_empty()?;
				Ok(())
			}
		}
	}
}

impl From<Material> for ArcLock<Material> {
	fn from(value: Material) -> Self {
		Arc::new(RwLock::new(value))
	}
}

#[cfg(test)]
mod tests {

	use crate::material::Material;

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::*;
		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode};
		use std::io::Seek;

		#[test]
		fn color_no_name_full() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Material::new_color(None, 0.2, 0.4, 0.6, 0.8)
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<material><color rgba="0.2 0.4 0.6 0.8"/></material>"#)
			)
		}

		#[test]
		fn color_name_full() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(
				Material::new_color(Some("test_material".into()), 0.2, 0.4, 0.6, 0.8)
					.to_urdf(&mut writer, &URDFConfig::default())
					.is_ok()
			);

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<material name="test_material"><color rgba="0.2 0.4 0.6 0.8"/></material>"#
				)
			)
		}

		#[test]
		fn color_name_ref() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(
				Material::new_color(Some("test_material".into()), 0.2, 0.4, 0.6, 0.8)
					.to_urdf(
						&mut writer,
						&URDFConfig {
							direct_material_ref: URDFMaterialMode::Referenced,
							..Default::default()
						}
					)
					.is_ok()
			);

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<material name="test_material"/>"#)
			)
		}

		#[test]
		fn texture_no_name_full() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(
				Material::new_texture(None, "package://robot_description/...".into())
					.to_urdf(&mut writer, &URDFConfig::default())
					.is_ok()
			);

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<material><texture filename="package://robot_description/..."/></material>"#
				)
			)
		}

		#[test]
		fn texture_name_full() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Material::new_texture(
				Some("texture_material".into()),
				"package://robot_description/...".into()
			)
			.to_urdf(&mut writer, &URDFConfig::default())
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(
					r#"<material name="texture_material"><texture filename="package://robot_description/..."/></material>"#
				)
			)
		}

		#[test]
		fn texture_name_ref() {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(Material::new_texture(
				Some("texture_material".into()),
				"package://robot_description/...".into()
			)
			.to_urdf(
				&mut writer,
				&URDFConfig {
					direct_material_ref: URDFMaterialMode::Referenced,
					..Default::default()
				}
			)
			.is_ok());

			writer.inner().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.inner()).unwrap(),
				String::from(r#"<material name="texture_material"/>"#)
			)
		}
	}
}
