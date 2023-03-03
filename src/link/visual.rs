use std::{cell::RefCell, rc::Rc};

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
	pub material: Option<Rc<RefCell<Material>>>,
}

impl Visual {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		reference: Option<TMPLocationThing>,
		geometry: Box<dyn GeometryInterface>,
		material: Option<Rc<RefCell<Material>>>,
	) -> Self {
		Self {
			name,
			reference: reference,
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
			&& self.material == other.material
	}
}

#[derive(Debug, PartialEq, Clone)]
/// In URDF this is known as an `origin`
pub struct TMPLocationThing;
