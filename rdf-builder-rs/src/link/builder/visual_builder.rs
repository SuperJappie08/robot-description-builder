use crate::{
	link::geometry::GeometryInterface, link::visual::Visual,
	link_data::geometry::GeometryShapeData, material_mod::MaterialBuilder,
	transform_data::Transform,
};

#[derive(Debug)]
pub struct VisualBuilder {
	/// TODO: Figure out if I want to keep the name optional?.
	pub(crate) name: Option<String>,
	pub(crate) origin: Option<Transform>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub(crate) material_description: Option<MaterialBuilder>,
}

impl VisualBuilder {
	pub fn new<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		geometry: Geometry,
	) -> Self {
		Self {
			name: None,
			origin: None,
			geometry: geometry.into(),
			material_description: None,
		}
	}

	/// TODO: Figure out if this will be kept [Added for easier transistion]
	pub fn new_full<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		name: Option<String>,
		origin: Option<Transform>,
		geometry: Geometry,
		material_description: Option<MaterialBuilder>,
	) -> Self {
		Self {
			name,
			origin,
			geometry: geometry.into(),
			material_description,
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

	pub fn material(mut self, material_description: MaterialBuilder) -> Self {
		self.material_description = Some(material_description);
		self
	}

	/// FIXME: Propper Error
	pub(crate) fn build(self) -> Result<Visual, String> {
		let material = self
			.material_description
			.map(|description| description.build());

		Ok(Visual {
			name: self.name,
			origin: self.origin,
			geometry: self.geometry,
			material,
		})
	}

	pub(crate) fn get_geometry_data(&self) -> GeometryShapeData {
		GeometryShapeData {
			origin: self.origin.unwrap_or_default(),
			geometry: self.geometry.try_get_shape().unwrap(), // FIXME: Is unwrap OK?, for now Ok until Mesh gets supported
		}
	}
}

impl PartialEq for VisualBuilder {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& *self.geometry == *other.geometry
			&& self.material_description == other.material_description
	}
}

impl Clone for VisualBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin,
			geometry: self.geometry.boxed_clone(),
			material_description: self.material_description.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	// TODO: Write tests
}
