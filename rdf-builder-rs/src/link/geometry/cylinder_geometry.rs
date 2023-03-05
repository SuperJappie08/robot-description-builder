use std::f32::consts::{PI, TAU};

use crate::link::geometry::GeometryInterface;

#[derive(Debug, PartialEq, Clone)]
pub struct CylinderGeometry {
	radius: f32,
	length: f32,
}

impl CylinderGeometry {
	pub fn new(radius: f32, length: f32) -> Self {
		Self { radius, length }
	}
}

impl GeometryInterface for CylinderGeometry {
	fn volume(&self) -> f32 {
		self.radius * self.radius * PI * self.length
	}

	fn surface_area(&self) -> f32 {
		2f32 * (self.radius * self.radius * PI) + self.length * self.radius * TAU
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}
}

impl From<CylinderGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: CylinderGeometry) -> Self {
		Box::new(value)
	}
}
