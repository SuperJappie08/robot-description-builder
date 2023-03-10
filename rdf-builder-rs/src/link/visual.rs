use std::sync::Arc;

use quick_xml::{events::attributes::Attribute, name::QName};

use crate::{
	material::Material,
	to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode, URDFMaterialReferences},
	transform_data::TransformData,
	ArcLock,
};

use super::geometry::GeometryInterface;

#[derive(Debug)]
pub struct Visual {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	origin: Option<TransformData>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub material: Option<ArcLock<Material>>,
}

impl Visual {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		origin: Option<TransformData>,
		geometry: Box<dyn GeometryInterface + Sync + Send>,
		material: Option<ArcLock<Material>>,
	) -> Self {
		Self {
			name,
			origin,
			geometry,
			material,
		}
	}
}

impl ToURDF for Visual {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		let mut element = writer.create_element("visual");
		if let Some(name) = self.name.clone() {
			element = element.with_attribute(Attribute {
				key: QName(b"name"),
				value: name.clone().as_bytes().into(),
			});
		}
		element.write_inner_content(|writer| {
			if let Some(origin) = self.origin.clone() {
				origin.to_urdf(writer, urdf_config)?
			}

			self.geometry.to_urdf(writer, urdf_config)?;

			if let Some(material) = self.material.clone() {
				let material_config = URDFConfig {
					direct_material_ref: match urdf_config.material_references {
						URDFMaterialReferences::AllNamedMaterialOnTop => {
							if material.read().unwrap().name.is_some()
							// FIXME: Check if unwrap is ok here?
							{
								URDFMaterialMode::Referenced
							} else {
								URDFMaterialMode::FullMaterial
							}
						}
						URDFMaterialReferences::OnlyMultiUseMaterials => {
							if Arc::strong_count(&material) > 2 {
								URDFMaterialMode::Referenced
							} else {
								URDFMaterialMode::FullMaterial
							}
						}
					},
					..urdf_config.clone()
				};
				material
					.read()
					.unwrap() // FIXME: Don't know if unwrap is ok here?
					.to_urdf(writer, &material_config)?
			}
			Ok(())
		})?;

		Ok(())
	}
}

impl PartialEq for Visual {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& (&self.geometry == &other.geometry)
			&& match (&self.material, &other.material) {
				(None, None) => true,
				(Some(own_material), Some(other_material)) => {
					Arc::ptr_eq(own_material, other_material)
				}
				_ => false,
			}
	}
}

impl Clone for Visual {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin.clone(),
			geometry: self.geometry.boxed_clone(),
			material: self.material.clone(),
		}
	}
}
