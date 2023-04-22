use std::sync::{Arc, RwLock};

#[cfg(feature = "xml")]
use quick_xml::events::attributes::Attribute;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, material_mod::MaterialData,
	to_rdf::to_urdf::URDFMaterialMode, ArcLock,
};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;

use super::MaterialBuilder;

/// TODO: Name is subject to change
#[derive(Debug, PartialEq)]
pub enum Material {
	Named { name: String, data: MaterialStage },
	Unamed(MaterialData),
}

impl Material {
	/// TODO: PROPPER ERRORS
	pub(crate) fn initialize(&mut self, tree: &KinematicDataTree) -> Result<(), String> {
		match self {
			Material::Unamed(_) => Ok(()),
			Material::Named { name, data } => {
				let material_data = match data {
					MaterialStage::PreInit(data) => {
						let material_data_index = Arc::clone(&tree.material_index);

						let other_material = material_data_index
							.read()
							.unwrap() //FIXME: Unwrap not OK
							.get(name)
							.map(Arc::clone);
						match other_material {
							Some(other_material) => {
								if *other_material.read().unwrap() == *data {
									//FIXME: Is unwrap Ok?
									other_material
								} else {
									return Err("Conflict".into());
								}
							}
							None => {
								let material_data = Arc::new(RwLock::new(data.clone()));
								assert!(material_data_index
									.write()
									.unwrap() //FIXME: Is unwrap Ok?
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

	pub fn get_material_data(&self) -> MaterialDataRefereceWrapper {
		match self {
			Material::Named { name: _, data } => data.get_data(),
			Material::Unamed(data) => data.into(),
		}
	}

	pub fn rebuild(&self) -> MaterialBuilder {
		let builder = MaterialBuilder::new_data(self.get_material_data().into());
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

/// FIXME: TEMP PUB
#[derive(Debug)]
pub enum MaterialStage {
	PreInit(MaterialData),
	Initialized(ArcLock<MaterialData>),
}

impl MaterialStage {
	/// Gets the Strong count of the `MaterialData`,
	/// returns 0 if the `LocalMaterial` is not fully initialized yet.
	fn get_used_count(&self) -> usize {
		match self {
			MaterialStage::PreInit(_) => 0,
			MaterialStage::Initialized(arc_data) => Arc::strong_count(arc_data),
		}
	}

	pub(crate) fn initialize(&mut self, material_data: ArcLock<MaterialData>) {
		match self {
			MaterialStage::PreInit(_) => *self = MaterialStage::Initialized(material_data),
			MaterialStage::Initialized(data) => {
				debug_assert!(Arc::ptr_eq(data, &material_data));
			}
		}
	}

	pub(crate) fn get_data(&self) -> MaterialDataRefereceWrapper {
		match self {
			MaterialStage::PreInit(data) => data.into(),
			MaterialStage::Initialized(arc_data) => Arc::clone(arc_data).into(), //Unwrap not Ok
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for MaterialStage {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		match self {
			MaterialStage::PreInit(data) => data.to_urdf(writer, urdf_config),
			MaterialStage::Initialized(arc_data) => {
				arc_data.read().unwrap().to_urdf(writer, urdf_config) // FIXME: UNWRAP NOT OK
			}
		}
	}
}

impl PartialEq for MaterialStage {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::PreInit(l0), Self::PreInit(r0)) => l0 == r0,
			(Self::Initialized(l0), Self::Initialized(r0)) => Arc::ptr_eq(l0, r0),
			_ => false,
		}
	}
}

impl Clone for MaterialStage {
	fn clone(&self) -> Self {
		match self {
			Self::PreInit(arg0) => Self::PreInit(arg0.clone()),
			Self::Initialized(arg0) => Self::Initialized(Arc::clone(arg0)),
		}
	}
}

impl From<MaterialData> for MaterialStage {
	fn from(value: MaterialData) -> Self {
		Self::PreInit(value)
	}
}

#[derive(Debug)]
pub enum MaterialDataRefereceWrapper<'a> {
	Direct(&'a MaterialData),
	Global(ArcLock<MaterialData>),
}

impl<'a> MaterialDataRefereceWrapper<'a> {
	pub fn same_material_data(&self, other: &MaterialDataRefereceWrapper) -> bool {
		match (self, other) {
			(
				MaterialDataRefereceWrapper::Direct(left),
				MaterialDataRefereceWrapper::Direct(right),
			) => left == right,
			(
				MaterialDataRefereceWrapper::Direct(left),
				MaterialDataRefereceWrapper::Global(right),
			) => (*left).clone() == right.read().unwrap().clone(), // FIXME: Unwrap not OK
			(
				MaterialDataRefereceWrapper::Global(left),
				MaterialDataRefereceWrapper::Direct(right),
			) => (*right).clone() == left.read().unwrap().clone(), // FIXME: Unwrap not OK
			(
				MaterialDataRefereceWrapper::Global(left),
				MaterialDataRefereceWrapper::Global(right),
			) => left.read().unwrap().clone() == right.read().unwrap().clone(), // FIXME: Unwrap not OK
		}
	}
}

impl<'a> PartialEq for MaterialDataRefereceWrapper<'a> {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Direct(l0), Self::Direct(r0)) => l0 == r0,
			(Self::Global(l0), Self::Global(r0)) => Arc::ptr_eq(l0, r0),
			_ => false,
		}
	}
}

impl<'a> From<&'a MaterialData> for MaterialDataRefereceWrapper<'a> {
	fn from(value: &'a MaterialData) -> Self {
		Self::Direct(value)
	}
}

impl<'a> From<ArcLock<MaterialData>> for MaterialDataRefereceWrapper<'a> {
	fn from(value: ArcLock<MaterialData>) -> Self {
		MaterialDataRefereceWrapper::Global(value)
	}
}

impl<'a> From<MaterialDataRefereceWrapper<'a>> for MaterialData {
	fn from(value: MaterialDataRefereceWrapper) -> Self {
		match value {
			MaterialDataRefereceWrapper::Direct(data) => data.clone(),
			MaterialDataRefereceWrapper::Global(arc_data) => arc_data.read().unwrap().clone(), // FIXME: NOT OK
		}
	}
}
