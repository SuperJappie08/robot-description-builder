use crate::link::{
	builder::{CollisionBuilder, LinkBuilder, VisualBuilder},
	geometry::{BoxGeometry, CylinderGeometry, GeometryInterface, SphereGeometry},
};

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_quick_link(link_name: impl Into<String>, visual: VisualBuilder) -> LinkBuilder {
	let link_name = link_name.into();
	let mut link = LinkBuilder::new(&link_name);

	let mut collision_name = link_name.clone();
	collision_name.push_str("_collision");
	link = link.add_collider(visual.to_collision().named(collision_name));

	let mut visual_name = link_name.clone();
	visual_name.push_str("_visual");
	link = link.add_visual(visual.named(visual_name));

	link
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
/// TODO: Deprecate?
fn new_quick_link_old(
	link_name: impl Into<String>,
	geometry: Box<dyn GeometryInterface + Sync + Send>,
) -> LinkBuilder {
	let link_name = link_name.into();
	let mut link = LinkBuilder::new(link_name.clone());

	let mut visual_name = link_name.clone();
	visual_name.push_str("_visual");
	link = link.add_visual(VisualBuilder::new_full(
		visual_name.into(),
		None, // TODO: NOT HOW I WANT IT
		geometry.boxed_clone(),
		None, // TODO: TEMP
	));

	let mut collision_name = link_name;
	collision_name.push_str("_collision");
	link = link.add_collider(CollisionBuilder::new_full(
		collision_name.into(),
		None,
		geometry,
	));

	link
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_box_link(
	link_name: impl Into<String>,
	side1: f32,
	side2: f32,
	side3: f32,
) -> LinkBuilder {
	let geometry = BoxGeometry::new(side1, side2, side3);

	new_quick_link_old(link_name, geometry.into())
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
/// TODO: Orientation??
pub fn new_cylinder_link(link_name: impl Into<String>, radius: f32, length: f32) -> LinkBuilder {
	let geometry = CylinderGeometry::new(radius, length);

	new_quick_link_old(link_name, geometry.into())
}

/// TODO: Finalize, this is temp
/// TODO: ADD NAMED CHOICE for Vis & Col
/// TODO: Add material Specifierer
/// TODO: Add Inertial data options
/// TODO: ADD TEST?
pub fn new_sphere_link(link_name: impl Into<String>, radius: f32) -> LinkBuilder {
	let geometry = SphereGeometry::new(radius);

	new_quick_link_old(link_name, geometry.into())
}

#[cfg(test)]
mod tests {
	use crate::{link::helper_functions::*, KinematicInterface};

	#[test]
	fn test_new_box_link() {
		let tree = new_box_link("Zelda", 2f32, 3f32, 5f32).build_tree();

		assert_eq!(tree.get_links().try_read().unwrap().len(), 1);
		assert_eq!(tree.get_newest_link().try_read().unwrap().name(), "Zelda");
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
