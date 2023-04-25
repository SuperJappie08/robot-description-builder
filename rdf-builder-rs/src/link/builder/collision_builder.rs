use crate::{
	link::geometry::GeometryInterface, link::geometry::GeometryShapeData, link::Collision,
	transform_data::Transform,
};

#[derive(Debug)]
pub struct CollisionBuilder {
	pub(crate) name: Option<String>,
	pub(crate) origin: Option<Transform>,
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
}

impl CollisionBuilder {
	pub fn new<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		geometry: Geometry,
	) -> Self {
		Self {
			name: None,
			origin: None,
			geometry: geometry.into(),
		}
	}

	pub fn new_full<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		name: Option<String>,
		origin: Option<Transform>,
		geometry: Geometry,
	) -> Self {
		Self {
			name,
			origin,
			geometry: geometry.into(),
		}
	}

	pub fn named(mut self, name: impl Into<String>) -> Self {
		self.name = Some(name.into());
		self
	}

	pub fn tranformed(mut self, transform: Transform) -> Self {
		self.origin = Some(transform);
		self
	}

	pub(crate) fn build(self) -> Collision {
		Collision {
			name: self.name,
			origin: self.origin,
			geometry: self.geometry,
		}
	}

	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			origin: self.origin.unwrap_or_default(),
			geometry: self.geometry.try_get_shape().unwrap(), // FIXME: Is unwrap OK?, for now Ok until Mesh gets supported
		}
	}
}

impl PartialEq for CollisionBuilder {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.origin == other.origin && *self.geometry == *other.geometry
	}
}

impl Clone for CollisionBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
		}
	}
}

/// TODO: Decide if this is ok?
impl From<CollisionBuilder> for Collision {
	fn from(value: CollisionBuilder) -> Self {
		value.build()
	}
}

#[cfg(test)]
mod tests {
	// use test_log::test;
	// TODO: Write tests
}
