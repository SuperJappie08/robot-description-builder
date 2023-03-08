use std::sync::Arc;

use crate::{material::Material, ArcLock};

use super::geometry::GeometryInterface;

#[derive(Debug)]
pub struct Visual {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	reference: Option<TMPLocationThing>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub material: Option<ArcLock<Material>>,
}

impl Visual {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		reference: Option<TMPLocationThing>,
		geometry: Box<dyn GeometryInterface + Sync + Send>,
		material: Option<ArcLock<Material>>,
	) -> Self {
		Self {
			name,
			reference,
			geometry,
			material,
		}
	}
}

impl PartialEq for Visual {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.reference == other.reference
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
			reference: self.reference.clone(),
			geometry: self.geometry.boxed_clone(),
			material: self.material.clone(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
/// In URDF this is known as an `origin`
pub struct TMPLocationThing;
