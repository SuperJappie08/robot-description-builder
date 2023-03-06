use crate::link::{geometry::GeometryInterface, visual::TMPLocationThing};

#[derive(Debug)]
pub struct Collision {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	reference: Option<TMPLocationThing>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl Collision {
	/// Maybe temp
	pub fn new(
		name: Option<String>,
		reference: Option<TMPLocationThing>,
		geometry: Box<dyn GeometryInterface + Sync + Send>,
	) -> Self {
		Self {
			name,
			reference,
			geometry,
		}
	}
}

impl PartialEq for Collision {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.reference == other.reference
			&& (&self.geometry == &other.geometry)
	}
}

impl Clone for Collision {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			reference: self.reference.clone(),
			geometry: self.geometry.boxed_clone(),
		}
	}
}
