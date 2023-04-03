use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree, link::geometry::GeometryInterface,
	link::Visual, material::MaterialDescriptor, transform_data::TransformData,
};

#[derive(Debug)]
pub struct VisualBuilder {
	/// TODO: Figure out if I want to keep the name optional?.
	pub name: Option<String>,
	pub(crate) origin: Option<TransformData>,

	/// Figure out if this needs to be public or not
	pub(crate) geometry: Box<dyn GeometryInterface + Sync + Send>,
	/// Not sure about refCell
	pub material_description: Option<MaterialDescriptor>,
}

impl VisualBuilder {
	/// TODO: Figure out if this will be kept [Added for easier transistion]
	pub fn new<Geometry: Into<Box<dyn GeometryInterface + Sync + Send>>>(
		name: Option<String>,
		origin: Option<TransformData>,
		geometry: Geometry,
		material_description: Option<MaterialDescriptor>,
	) -> Self {
		Self {
			name,
			origin,
			geometry: geometry.into(),
			material_description,
		}
	}

	/// FIXME: Propper Error
	pub(crate) fn build(self, tree: &Weak<KinematicDataTree>) -> Result<Visual, String> {
		let material = match self.material_description {
			Some(description) => Some(description.construct(tree)?),
			None => None,
		};

		Ok(Visual {
			name: self.name,
			origin: self.origin,
			geometry: self.geometry,
			material,
		})
	}
}

impl PartialEq for VisualBuilder {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
			&& self.origin == other.origin
			&& (&self.geometry == &other.geometry)
			&& self.material_description == other.material_description
	}
}

impl Clone for VisualBuilder {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			origin: self.origin.clone(),
			geometry: self.geometry.boxed_clone(),
			material_description: self.material_description.clone(),
		}
	}
}
