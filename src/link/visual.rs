use std::sync::{Arc, RwLock};

use crate::material::Material;

use super::geometry::GeometryInterface;

#[derive(Debug, Clone)]
pub struct Visual {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	reference: Option<TMPLocationThing>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface>,
	/// Not sure about refCell
	pub material: Option<Arc<RwLock<Material>>>,
}

impl Visual {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		reference: Option<TMPLocationThing>,
		geometry: Box<dyn GeometryInterface>,
		material: Option<Arc<RwLock<Material>>>,
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
					Arc::ptr_eq(&own_material, &other_material)
				}
				_ => false,
			}
	}
}

#[derive(Debug, PartialEq, Clone)]
/// In URDF this is known as an `origin`
pub struct TMPLocationThing;
