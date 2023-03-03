use crate::link::geometry::GeometryInterface;

#[derive(Debug, PartialEq, Clone)]
pub struct BoxGeometry {
	/// TODO: Figure out correct field names
	side1: f32,
	side2: f32,
	side3: f32,
}

impl BoxGeometry {
	/// TODO: REPLACE PARAMETER NAMES AND MAYBE NOT PUBLIC
	pub fn new(side1: f32, side2: f32, side3: f32) -> Self {
		Self {
			side1,
			side2,
			side3,
		}
	}
}

impl GeometryInterface for BoxGeometry {
	fn volume(&self) -> f32 {
		self.side1 * self.side2 * self.side3
	}

	fn surface_area(&self) -> f32 {
		2f32 * (self.side1 * self.side2 + self.side1 * self.side3 + self.side2 * self.side3)
	}

	fn boxed_clone(&self) -> Box<dyn GeometryInterface> {
		Box::new(self.clone())
	}
}

impl Into<Box<dyn GeometryInterface>> for BoxGeometry {
	fn into(self) -> Box<dyn GeometryInterface> {
		Box::new(self)
	}
}
