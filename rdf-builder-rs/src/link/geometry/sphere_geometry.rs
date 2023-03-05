use std::f32::consts::{FRAC_PI_3, PI};

use crate::link::geometry::GeometryInterface;

#[derive(Debug, PartialEq, Clone)]
pub struct SphereGeometry {
	radius: f32,
}

impl SphereGeometry {
	pub fn new(radius: f32) -> Self {
		Self { radius }
	}
}

impl GeometryInterface for SphereGeometry {
	fn volume(&self) -> f32 {
		4f32 * FRAC_PI_3 * self.radius * self.radius * self.radius
	}

	fn surface_area(&self) -> f32 {
		4f32 * PI * self.radius * self.radius
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface + Sync + Send> {
		Box::new(self.clone())
	}
}

impl From<SphereGeometry> for Box<dyn GeometryInterface + Sync + Send> {
	fn from(value: SphereGeometry) -> Self {
		Box::new(value)
	}
}
