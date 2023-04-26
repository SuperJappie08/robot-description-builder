mod material;
mod material_builder;
pub(crate) mod material_data_reference;
pub(crate) mod material_stage;

#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

pub use material::Material;
pub use material_builder::MaterialBuilder;

#[derive(Debug, PartialEq, Clone)]
pub enum MaterialData {
	/// Color as RGBA.
	///
	/// The fields need to be between 0 and 1 (for most simulators). (Not enforced)
	Color(f32, f32, f32, f32),
	/// Texture, containing the texture path as a valid package path (e.g. `"package://robot_description/textures/{texture}"`). You are on your own here.
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

#[cfg(test)]
mod tests {
	// use crate::material::Material;
	use crate::material_mod::MaterialBuilder;
	use test_log::test;

	// #[test]
	// fn rebuild() {
	// 	// assert_eq!(MaterialBuilder::new_color(9., 1., 2., 1.).build().rebuild(), );
	// }

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, MaterialBuilder};
		use crate::{
			link::builder::{LinkBuilder, VisualBuilder},
			link_data::geometry::BoxGeometry,
			to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode},
			KinematicInterface,
		};
		use std::io::Seek;

		fn test_to_urdf_material(
			material_builder: MaterialBuilder,
			result: String,
			urdf_config: &URDFConfig,
		) {
			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(material_builder
				.build()
				.to_urdf(&mut writer, urdf_config)
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				result
			)
		}

		#[test]
		fn color_no_name_full() {
			test_to_urdf_material(
				MaterialBuilder::new_color(0.2, 0.4, 0.6, 0.8),
				String::from(r#"<material><color rgba="0.2 0.4 0.6 0.8"/></material>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn color_name_full() {
			test_to_urdf_material(
				MaterialBuilder::new_color(0.2, 0.4, 0.6, 0.8).named("test_material"),
				String::from(
					r#"<material name="test_material"><color rgba="0.2 0.4 0.6 0.8"/></material>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn color_name_ref() {
			let tree = LinkBuilder::new("link")
				.add_visual(VisualBuilder::new_full(
					None,
					None,
					BoxGeometry::new(1., 1., 1.),
					MaterialBuilder::new_color(0.2, 0.4, 0.6, 0.8)
						.named("test_material")
						.into(),
				))
				.build_tree();

			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(tree
				.get_material("test_material")
				.unwrap()
				.to_urdf(
					&mut writer,
					&URDFConfig {
						direct_material_ref: URDFMaterialMode::Referenced,
						..Default::default()
					}
				)
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<material name="test_material"/>"#)
			)
		}

		#[test]
		fn texture_no_name_full() {
			test_to_urdf_material(
				MaterialBuilder::new_texture("package://robot_description/..."),
				String::from(
					r#"<material><texture filename="package://robot_description/..."/></material>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn texture_name_full() {
			test_to_urdf_material(
				MaterialBuilder::new_texture("package://robot_description/...")
					.named("texture_material"),
				String::from(
					r#"<material name="texture_material"><texture filename="package://robot_description/..."/></material>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn texture_name_ref() {
			let tree = LinkBuilder::new("link")
				.add_visual(VisualBuilder::new_full(
					None,
					None,
					BoxGeometry::new(1., 1., 1.),
					MaterialBuilder::new_texture("package://robot_description/...")
						.named("texture_material")
						.into(),
				))
				.build_tree();

			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));
			assert!(tree
				.get_material("texture_material")
				.unwrap()
				.to_urdf(
					&mut writer,
					&URDFConfig {
						direct_material_ref: URDFMaterialMode::Referenced,
						..Default::default()
					}
				)
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				String::from(r#"<material name="texture_material"/>"#)
			)
		}
	}
}
