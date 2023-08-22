mod box_geometry;
mod cylinder_geometry;
mod geometry_shape_data;
mod mesh_geometry;
use nalgebra::Matrix3;
mod sphere_geometry;

pub use box_geometry::BoxGeometry;
pub use cylinder_geometry::CylinderGeometry;
pub use mesh_geometry::MeshGeometry;
pub use sphere_geometry::SphereGeometry;

pub use geometry_shape_data::GeometryShapeData;

// TODO: Maybe only exported for `wrapping` feature
pub use geometry_shape_data::GeometryShapeContainer;

use std::fmt::Debug;

use crate::transform::Mirror;

// use self::geometry_shape_data::GeometryShapeContainer;

/// A trait to mirror items inside of a `Box<T>`.
pub trait BoxedMirror {
	/// Performs a `Mirror::mirrored` on a Boxed Implementor.
	fn boxed_mirrored(
		&self,
		mirror_matrix: &Matrix3<f32>,
	) -> Box<dyn GeometryInterface + Sync + Send>;
}

impl<Geometry> BoxedMirror for Geometry
where
	Geometry: GeometryInterface + Mirror + Sync + Send,
{
	fn boxed_mirrored(
		&self,
		mirror_matrix: &Matrix3<f32>,
	) -> Box<dyn GeometryInterface + Sync + Send> {
		self.mirrored(mirror_matrix).boxed_clone()
	}
}

/// An interface for working with `Geometry`s generically.
// LONGTERM-TODO: DECIDE IF `Box<dyn dyn GeometryInterface + Sync + Send>` should be replaced with [`GeometryShapeContainer`]
pub trait GeometryInterface: Debug + BoxedMirror {
	/// Provides the volume of a `Geometry`.
	fn volume(&self) -> f32;
	/// Provides the surface area of a `Geometry`.
	fn surface_area(&self) -> f32;
	/// Allows for Cloning of Boxed Geometries.
	///
	/// This has similiar functionality to [`Clone::clone`] except that it allows items to be [`Box`ed](Box).
	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send>;

	/// Get's the untransformed boundingbox size of the geometry from it's center. (X, Y, Z).
	fn bounding_box(&self) -> (f32, f32, f32);

	/// Gets a `GeometryShapeContainer` of the current Shape.
	fn shape_container(&self) -> GeometryShapeContainer;
}

impl Mirror for Box<dyn GeometryInterface + Sync + Send> {
	fn mirrored(&self, mirror_matrix: &Matrix3<f32>) -> Self {
		self.boxed_mirrored(mirror_matrix)
	}
}

impl PartialEq for (dyn GeometryInterface + Sync + Send) {
	fn eq(&self, other: &Self) -> bool {
		// Should probably just get shape data
		self.volume() == other.volume()
			&& self.surface_area() == other.surface_area()
			&& self.bounding_box() == other.bounding_box()
	}
}

impl From<&(dyn GeometryInterface + Sync + Send)> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: &(dyn GeometryInterface + Sync + Send)) -> Self {
		value.boxed_clone()
	}
}
