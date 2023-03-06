use crate::{cluster_objects::KinematicTree, KinematicInterface};

use crate::link::{
	geometry::{BoxGeometry, GeometryInterface},
	Collision, Link, Visual,
};

use super::geometry::{CylinderGeometry, SphereGeometry};

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_quick_link(
	link_name: String,
	geometry: Box<dyn GeometryInterface + Sync + Send>,
) -> KinematicTree {
	let tree = Link::new(link_name.clone());

	let binding = tree.get_newest_link();
	let mut link = binding.try_write().unwrap(); // FIXME: Might not be ok to unwrap()

	let mut visual_name = link_name.clone();
	visual_name.push_str("_visual");
	link.try_add_visual(Visual::new(
		visual_name.into(),
		None, // TODO: NOT HOW I WANT IT
		geometry.boxed_clone(),
		None, // TODO: TEMP
	))
	.unwrap();

	let mut collision_name = link_name;
	collision_name.push_str("_collision");
	link.add_collider(Collision::new(collision_name.into(), None, geometry));

	tree
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_box_link(link_name: String, side1: f32, side2: f32, side3: f32) -> KinematicTree {
	let geometry = BoxGeometry::new(side1, side2, side3);

	new_quick_link(link_name, geometry.into())
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
/// TODO: Orientation??
pub fn new_cylinder_link(link_name: String, radius: f32, length: f32) -> KinematicTree {
	let geometry = CylinderGeometry::new(radius, length);

	new_quick_link(link_name, geometry.into())
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_sphere_link(link_name: String, radius: f32) -> KinematicTree {
	let geometry = SphereGeometry::new(radius);

	new_quick_link(link_name, geometry.into())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_box_link() {
		let tree = new_box_link("Zelda".to_owned(), 2f32, 3f32, 5f32);

		assert_eq!(tree.get_links().try_read().unwrap().len(), 1);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().get_name(),
			"Zelda"
		);
		assert_eq!(tree.get_newest_link().try_read().unwrap().visuals.len(), 1);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().visuals[0].name,
			Some("Zelda_visual".into())
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().visuals[0]
				.geometry
				.volume(),
			30f32
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().visuals[0]
				.geometry
				.surface_area(),
			62f32
		);

		assert_eq!(
			tree.get_newest_link().try_read().unwrap().colliders.len(),
			1
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().colliders[0].name,
			Some("Zelda_collision".into())
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().colliders[0]
				.geometry
				.volume(),
			30f32
		);
		assert_eq!(
			tree.get_newest_link().try_read().unwrap().colliders[0]
				.geometry
				.surface_area(),
			62f32
		);
		// TODO: UPDATE WHEN FUNCTION IS FINALIZED
		// TODO: TEST INERTIAL DATA
	}
}
