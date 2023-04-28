use std::sync::{Arc, RwLock};

#[cfg(feature = "xml")]
use quick_xml::events::attributes::Attribute;

use crate::{
	cluster_objects::{
		kinematic_data_errors::{errored_read_lock, errored_write_lock, AddMaterialError},
		kinematic_data_tree::KinematicDataTree,
	},
	material_mod::MaterialData,
	to_rdf::to_urdf::URDFMaterialMode,
	ArcLock,
};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

use super::{
	material_data_reference::MaterialDataReferenceWrapper, material_stage::MaterialStage,
	MaterialBuilder,
};

/// TODO: Name is subject to change
/// Need to rebuild this like [`ParseIntError`](https://doc.rust-lang.org/std/num/struct.ParseIntError.html) in order to allow privitized fields
#[derive(Debug, PartialEq)]
pub enum Material {
	Named { name: String, data: MaterialStage },
	Unamed(MaterialData),
}

impl Material {
	/// TODO: DOCS
	pub(crate) fn initialize(&mut self, tree: &KinematicDataTree) -> Result<(), AddMaterialError> {
		match self {
			Material::Unamed(_) => Ok(()),
			Material::Named { name, data } => {
				let material_data = match data {
					MaterialStage::PreInit(data) => {
						let material_data_index = Arc::clone(&tree.material_index);

						let other_material = material_data_index
							.read()
							.map_err(|_| errored_read_lock(&material_data_index))?
							.get(name)
							.map(Arc::clone);

						match other_material {
							Some(other_material) => {
								if *other_material
									.read()
									.map_err(|_| errored_read_lock(&other_material))?
									== *data
								{
									other_material
								} else {
									return Err(AddMaterialError::Conflict(name.clone()));
								}
							}
							None => {
								let material_data = Arc::new(RwLock::new(data.clone()));
								assert!(material_data_index
									.write()
									.map_err(|_| errored_write_lock(&material_data_index))?
									.insert(name.clone(), Arc::clone(&material_data))
									.is_none());
								material_data
							}
						}
					}
					MaterialStage::Initialized(data) => Arc::clone(data),
				};
				data.initialize(material_data);
				Ok(())
			}
		}
	}

	pub fn get_name(&self) -> Option<&String> {
		match self {
			Material::Named { name, data: _ } => Some(name),
			Material::Unamed(_) => None,
		}
	}

	pub fn get_material_data(&self) -> MaterialDataReferenceWrapper {
		match self {
			Material::Named { name: _, data } => data.get_data(),
			Material::Unamed(data) => data.into(),
		}
	}

	pub fn rebuild(&self) -> MaterialBuilder {
		let builder = MaterialBuilder::new_data(self.get_material_data().try_into().unwrap()); //FIXME: Unwrap not OK
		match self {
			Material::Named { name, data: _ } => builder.named(name),
			Material::Unamed(_) => builder,
		}
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

		match self {
			Material::Named { name, data } => {
				element = element.with_attribute(Attribute {
					key: quick_xml::name::QName(b"name"),
					value: name.as_bytes().into(),
				});
				match (urdf_config.direct_material_ref, dbg!(data.get_used_count())) {
					(URDFMaterialMode::Referenced, 2..) => element.write_empty()?,
					(URDFMaterialMode::FullMaterial, _) | (URDFMaterialMode::Referenced, _) => {
						element.write_inner_content(|writer| data.to_urdf(writer, urdf_config))?
					}
				}
			}
			Material::Unamed(data) => {
				element.write_inner_content(|writer| data.to_urdf(writer, urdf_config))?
			}
		};
		Ok(())
	}
}

impl Clone for Material {
	fn clone(&self) -> Self {
		match self {
			Self::Named { name, data } => Self::Named {
				name: name.clone(),
				data: data.clone(),
			},
			Self::Unamed(arg0) => Self::Unamed(arg0.clone()),
		}
	}
}

impl From<(String, ArcLock<MaterialData>)> for Material {
	fn from(value: (String, ArcLock<MaterialData>)) -> Self {
		let name = value.0;
		let data = value.1;

		Self::Named {
			name,
			data: MaterialStage::Initialized(data),
		}
	}
}
