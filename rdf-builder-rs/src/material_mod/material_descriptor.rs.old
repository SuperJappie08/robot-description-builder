use std::sync::{Arc, RwLock, Weak};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, material::Material,
	material::MaterialData, ArcLock,
};

/// A `MaterialDescriptor` describes a `Material`
///
/// When a `MaterialDescriptor` is constructed for a specific `KinematicDataTee`, the following steps happen:
///  1. Check if the description of the `MaterialDescriptor` matches a pre-existing `Material` already in the tree.
///     - If the a `Material` matches the description, the reference to that material is returned.
///     - If no `Material` matches the desctiption, a new `Material` is constructed and inserted to the `material_index` of the `KinematicDataTree` and the reference is returned.
///     - If only the `name` of the `Material` matches, an error is raised.
#[derive(Debug, PartialEq, Clone)]
pub struct MaterialDescriptor {
	pub(crate) name: Option<String>,
	pub(crate) data: MaterialData,
}

impl MaterialDescriptor {
	pub fn new_color(
		name: Option<String>,
		red: f32,
		green: f32,
		blue: f32,
		alpha: f32,
	) -> MaterialDescriptor {
		MaterialDescriptor {
			name,
			data: MaterialData::Color(red, green, blue, alpha),
		}
	}

	/// FIXME: Define an error
	pub(crate) fn construct(
		self,
		tree: &Weak<KinematicDataTree>,
	) -> Result<ArcLock<Material>, String> {
		let material_index = Arc::clone(
			&tree
				.upgrade()
				.expect("Expected an initialized KinematicDataTree")
				.material_index,
		); // This unwrap is Ok
		match self.name.as_ref() {
			Some(name) => match material_index.read().unwrap().get(name) {
				Some(other_material) => {
					match other_material.read().unwrap().data == self.data {
						true => Ok(Arc::clone(other_material)),
						// FIXME: ERROR
						false => Err("Material Name Collision".into()),
					}
				}
				None => {
					let material = Arc::new(RwLock::new(Material::new_data(
						self.name.clone(),
						self.data,
					)));
					debug_assert!(material_index
						.write()
						.unwrap()
						.insert(name.clone(), Arc::clone(&material))
						.is_none());
					Ok(material)
				}
			},
			// TODO: Consider moving, RwLock over data
			None => Ok(Arc::new(RwLock::new(Material::new_data(
				self.name, self.data,
			)))),
		}
	}
}
