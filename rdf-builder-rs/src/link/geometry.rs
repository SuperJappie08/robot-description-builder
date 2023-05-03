mod box_geometry;
mod cylinder_geometry;
mod geometry_shape_data;
mod sphere_geometry;

pub use box_geometry::BoxGeometry;
pub use cylinder_geometry::CylinderGeometry;
pub use sphere_geometry::SphereGeometry;

pub use geometry_shape_data::GeometryShapeData;

use std::fmt::Debug;

use self::geometry_shape_data::GeometryShapeContainer;

/// An interface for working with `Geometry`s generically.
///
/// LONGTERM-TODO: DECIDE IF `Box<dyn dyn GeometryInterface + Sync + Send>` shoudl be replaced with [`GeometryShapeContainer`]
pub trait GeometryInterface: Debug {
	fn volume(&self) -> f32;
	fn surface_area(&self) -> f32;
	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send>;

	/// Get's the untransformed boundingbox size of the geometry from it's center. (X, Y, Z)
	fn bounding_box(&self) -> (f32, f32, f32);

	/// Attemps to get a `GeometryShapeConatainer`
	///
	/// This fails when the data is not representable as a shape container.
	fn try_get_shape(&self) -> Result<GeometryShapeContainer, ()>;
}

impl PartialEq for (dyn GeometryInterface + Sync + Send) {
	fn eq(&self, other: &Self) -> bool {
		// Should probably just get shape data
		self.volume() == other.volume()
			&& self.surface_area() == other.surface_area()
			&& self.bounding_box() == other.bounding_box()
	}
}

// TODO: Is this ever used?
impl From<&(dyn GeometryInterface + Sync + Send)> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: &(dyn GeometryInterface + Sync + Send)) -> Self {
		value.boxed_clone()
	}
}
