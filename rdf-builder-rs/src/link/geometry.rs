mod box_geometry;
mod cylinder_geometry;
mod sphere_geometry;

pub use box_geometry::BoxGeometry;
pub use cylinder_geometry::CylinderGeometry;
pub use sphere_geometry::SphereGeometry;

use std::fmt::Debug;

pub trait GeometryInterface: Debug {
	fn volume(&self) -> f32;
	fn surface_area(&self) -> f32;
	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send>;
}

impl PartialEq for Box<dyn GeometryInterface + Sync + Send> {
	fn eq(&self, other: &Self) -> bool {
		self.volume() == other.volume() && self.surface_area() == other.surface_area()
	}
}

// impl Clone for Box<dyn GeometryInterface> {
// 	fn clone(&self) -> Self {
// 		self.boxed_clone()
// 	}
// }
