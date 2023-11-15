//! The `Material` System
//!
//! TODO: MODULE DOC
// DOCS TODO:
//  - Module
//  - Material
//  - MaterialDescriptor
mod descriptor;
pub(crate) mod stage;

pub mod data;
pub use descriptor::MaterialDescriptor;

use std::sync::{Arc, RwLock};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::{ToURDF, URDFMaterialMode};
#[cfg(feature = "xml")]
use quick_xml::events::attributes::Attribute;

#[cfg(feature = "wrapper")]
use crate::utils::ArcLock;

use crate::{
	cluster_objects::{
		kinematic_data_errors::AddMaterialError, kinematic_data_tree::KinematicDataTree,
	},
	identifiers::GroupID,
	utils::{errored_read_lock, ArcRW},
};

use data::{MaterialData, MaterialDataReference};
use stage::MaterialStage;

/// A struct to represents a `Material` of a `Visual` geometry.
///
/// A [`Material`] can be constructed via the [`MaterialDescriptor`].
///
/// A [`Material`] can contain either:
///  - a RGBA color
///  - a Texture
///
/// See [`MaterialDescriptor`] for more information.
#[derive(Debug, PartialEq, Clone)]
pub struct Material(MaterialKind);

impl Material {
	/// Creates a new unnamed `Material` from a `MaterialData`.
	pub(crate) fn new_unnamed(data: MaterialData) -> Self {
		Self(MaterialKind::Unnamed(data))
	}

	/// Creates a new named `Material` which still has to be initilized.
	pub(crate) fn new_named_uninited(name: impl Into<String>, data: MaterialData) -> Self {
		Self(MaterialKind::Named {
			name: name.into(),
			data: MaterialStage::PreInit(data),
		})
	}

	/// Creates a new named `Material`, which is already initilized.
	pub(crate) fn new_named_inited(
		name: impl Into<String>,
		data: Arc<RwLock<MaterialData>>,
	) -> Self {
		Self(MaterialKind::Named {
			name: name.into(),
			data: MaterialStage::Initialized(data),
		})
	}

	/// Register the `Material` in the `KinematicDataTree`.
	// TODO: Safe poisoned Locks when `mutex_unpoison` #96469 becomes stable.
	pub(crate) fn initialize(&mut self, tree: &KinematicDataTree) -> Result<(), AddMaterialError> {
		match &mut self.0 {
			// An unnamed Material does not have to be initialized.
			MaterialKind::Unnamed(_) => Ok(()),
			MaterialKind::Named { name, data } => {
				let material_data = match data {
					MaterialStage::PreInit(data) => {
						let material_data_index = Arc::clone(&tree.material_index);

						// Check if there already exists a `Material` with the same name
						let other_material = material_data_index.mread()?.get(name).map(Arc::clone);

						match other_material {
							Some(other_material) => {
								if *other_material
									.read()
									/* In the future the lock could be saved but waiting for
									"This is a nightly-only experimental API. (mutex_unpoison #96469)" */
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
									.mwrite()?
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

	/// The `name` of the `Material` if any.
	///
	/// Returns the `Some(name)` for a named [`Material`] and `None` for an unnamed [`Material`].
	pub fn name(&self) -> Option<&String> {
		match &self.0 {
			MaterialKind::Named { name, data: _ } => Some(name),
			MaterialKind::Unnamed(_) => None,
		}
	}

	/// Get a reference to the `MaterialData` as a [`MaterialDataReference`].
	// TODO: EXPAND docs
	pub fn material_data(&self) -> MaterialDataReference {
		match &self.0 {
			MaterialKind::Named { name: _, data } => data.data(),
			MaterialKind::Unnamed(data) => data.into(),
		}
	}

	/// Describes the `Material` to reform a [`MaterialDescriptor`].
	///
	/// This can be used to clone the `Material` for use in a different `Link`.
	pub fn describe(&self) -> MaterialDescriptor {
		let descriptor = MaterialDescriptor::new_data(self.material_data().try_into().unwrap()); //FIXME: Unwrap not OK
		match &self.0 {
			MaterialKind::Named { name, data: _ } => descriptor.named(name),
			MaterialKind::Unnamed(_) => descriptor,
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

		match &self.0 {
			MaterialKind::Named { name, data } => {
				element = element.with_attribute(Attribute {
					key: quick_xml::name::QName(b"name"),
					value: name.display().as_bytes().into(),
				});
				match (urdf_config.direct_material_ref, data.used_count()) {
					(URDFMaterialMode::Referenced, 2..) => element.write_empty()?,
					(URDFMaterialMode::FullMaterial, _) | (URDFMaterialMode::Referenced, _) => {
						element.write_inner_content(|writer| data.to_urdf(writer, urdf_config))?
					}
				}
			}
			MaterialKind::Unnamed(data) => {
				element.write_inner_content(|writer| data.to_urdf(writer, urdf_config))?
			}
		};
		Ok(())
	}
}

#[cfg(feature = "wrapper")]
impl From<(String, ArcLock<MaterialData>)> for Material {
	fn from(value: (String, ArcLock<MaterialData>)) -> Self {
		let name = value.0;
		let data = value.1;

		Self::new_named_inited(name, data)
	}
}

/// An enum to unify named and unnamed `Material` into a single type.
#[derive(Debug, PartialEq)]
enum MaterialKind {
	/// A variant to represent a named [`Material`].
	///
	/// It keeps track of the `name` of the [`Material`] and its data which needs to be initialized.
	/// This is ensured via [`MaterialStage`].
	Named { name: String, data: MaterialStage },
	/// A variant to represent unnamed [`Material`].
	///
	/// Tt holds [`MaterialData`].
	Unnamed(MaterialData),
}

impl From<MaterialKind> for Material {
	fn from(value: MaterialKind) -> Self {
		Self(value)
	}
}

impl Clone for MaterialKind {
	fn clone(&self) -> Self {
		match self {
			Self::Named { name, data } => Self::Named {
				name: name.clone(),
				data: data.clone(),
			},
			Self::Unnamed(arg0) => Self::Unnamed(arg0.clone()),
		}
	}
}

#[cfg(test)]
mod tests {
	// use crate::material::Material;
	use crate::material::MaterialDescriptor;
	use test_log::test;

	// #[test]
	// fn rebuild() {
	// 	// assert_eq!(MaterialDescriptor::new_color(9., 1., 2., 1.).build().rebuild(), );
	// }

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{test, MaterialDescriptor};
		use crate::{
			link::builder::{LinkBuilder, VisualBuilder},
			link_data::geometry::BoxGeometry,
			to_rdf::to_urdf::{ToURDF, URDFConfig, URDFMaterialMode},
			KinematicInterface,
		};
		use std::io::Seek;

		fn test_to_urdf_material(
			material_builder: MaterialDescriptor,
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
				MaterialDescriptor::new_color(0.2, 0.4, 0.6, 0.8),
				String::from(r#"<material><color rgba="0.2 0.4 0.6 0.8"/></material>"#),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn color_name_full() {
			test_to_urdf_material(
				MaterialDescriptor::new_color(0.2, 0.4, 0.6, 0.8).named("test_material"),
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
					MaterialDescriptor::new_color(0.2, 0.4, 0.6, 0.8)
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
				MaterialDescriptor::new_texture("package://robot_description/..."),
				String::from(
					r#"<material><texture filename="package://robot_description/..."/></material>"#,
				),
				&URDFConfig::default(),
			);
		}

		#[test]
		fn texture_name_full() {
			test_to_urdf_material(
				MaterialDescriptor::new_texture("package://robot_description/...")
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
					MaterialDescriptor::new_texture("package://robot_description/...")
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
