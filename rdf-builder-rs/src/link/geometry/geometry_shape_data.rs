use crate::{
	link::geometry::{BoxGeometry, CylinderGeometry, GeometryInterface, SphereGeometry},
	transform_data::TransformData,
};

#[derive(Debug, PartialEq, Clone)]
pub struct GeometryShapeData {
	pub origin: TransformData,
	pub geometry: GeometryShapeContainer,
}

impl GeometryShapeData {
	/// X Y Z Bounding box sizes from center of the origin of the shape
	pub fn bounding_box(&self) -> (f32, f32, f32) {
		match &self.geometry {
			GeometryShapeContainer::Box(g) => g.bounding_box(),
			GeometryShapeContainer::Cylinder(g) => g.bounding_box(),
			GeometryShapeContainer::Sphere(g) => g.bounding_box(),
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
pub enum GeometryShapeContainer {
	Box(BoxGeometry),
	Cylinder(CylinderGeometry),
	Sphere(SphereGeometry),
	// Capsule(String),
}

impl From<BoxGeometry> for GeometryShapeContainer {
	fn from(value: BoxGeometry) -> Self {
		Self::Box(value)
	}
}

impl From<CylinderGeometry> for GeometryShapeContainer {
	fn from(value: CylinderGeometry) -> Self {
		Self::Cylinder(value)
	}
}

impl From<SphereGeometry> for GeometryShapeContainer {
	fn from(value: SphereGeometry) -> Self {
		Self::Sphere(value)
	}
}
