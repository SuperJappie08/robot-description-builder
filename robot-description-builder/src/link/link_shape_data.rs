use super::geometry::{GeometryShapeData, SphereGeometry};
use crate::transform::Transform;

// TODO: IMPROVE DOCS
/// Contains the main geometry (first occurance). and the rest for use in the closure.
#[derive(Debug, PartialEq, Clone)]
pub struct LinkShapeData {
	pub main_geometry: GeometryShapeData,
	pub geometries: Vec<GeometryShapeData>,
}

impl LinkShapeData {
	pub(crate) fn new<I>(iter: I) -> Self
	where
		I: Iterator<Item = GeometryShapeData>,
	{
		let geometries: Vec<GeometryShapeData> = (iter).collect();

		// TODO: For now only use the first Viz. It is a a lot of work to use them all, and most of the time it isn't necessary
		// This would fix a lot of short term issues.

		if geometries.is_empty() {
			let main_geometry = GeometryShapeData {
				transform: Transform::default(),
				geometry: SphereGeometry::new(0.).into(),
			};

			Self {
				main_geometry: main_geometry.clone(),
				geometries: vec![main_geometry],
			}
		} else {
			Self {
				main_geometry: geometries.first().unwrap().clone(),
				geometries,
			}
		}
	}
}
