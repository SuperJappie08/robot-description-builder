use robot_description_builder::{
	link_data::geometry::BoxGeometry, linkbuilding::VisualBuilder, KinematicInterface, Link,
	SmartJointBuilder, Transform,
};

fn main() {
	let fixed_builder = SmartJointBuilder::new("fixed")
		.fixed()
		.add_dynamic_transform(|data| {
			let bounding_box = &data.main_geometry.bounding_box();
			let extend = |x: f32, bound: f32| {
				if x == 0. {
					0.
				} else {
					x / x * bound
				}
			};

			data.main_geometry.transform.translation.map(|(x, y, z)| {
				(
					extend(x, bounding_box.0),
					extend(y, bounding_box.1),
					extend(z, bounding_box.2),
				)
			});

			Transform {
				translation: data.main_geometry.transform.translation.map(|(x, y, z)| {
					(
						extend(x, bounding_box.0),
						extend(y, bounding_box.1),
						extend(z, bounding_box.2),
					)
				}),
				rotation: data.main_geometry.transform.rotation,
			}
		});

	println!("{:#?}", fixed_builder);

	let tree = Link::builder("root")
		.add_visual(VisualBuilder::new_full(
			Some("root_vis".into()),
			Transform::new_translation(1.0, 0., 0.).into(),
			BoxGeometry::new(2., 1., 1.),
			None,
		))
		.build_tree();

	tree.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(fixed_builder, Link::builder("child_fixed"))
		.unwrap();

	println!("{:#?}", tree);

	let revolute_builder = SmartJointBuilder::new("rev")
		.revolute()
		.with_axis((3., 2., 4.))
		.with_dynamics()
		.set_friction(400.);

	println!("{:#?}", revolute_builder);

	// let continous_joint_builder = SmartJointBuilder::new_continuous("continous");
}
