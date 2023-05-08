use std::{
	f32::consts::FRAC_PI_2,
	io::{Read, Seek},
};

use robot_description_builder::{
	link_data::{geometry::*, Visual},
	material::MaterialDescriptor,
	prelude::*,
	to_rdf::{
		to_urdf::{to_urdf, URDFConfig},
		XMLMode,
	},
	JointBuilder, JointType, Link, MirrorAxis, Robot, SmartJointBuilder, Transform,
};

fn to_urdf_string(robot: &Robot) -> String {
	let mut buffer = to_urdf(
		robot,
		URDFConfig {
			xml_mode: XMLMode::Indent(' ', 2),
			..Default::default()
		},
	)
	.unwrap()
	.into_inner();

	let mut out = String::new();
	buffer.rewind().unwrap();
	buffer.read_to_string(&mut out).unwrap();

	out
}

fn main() {
	/* === Material Descriptions === */
	let blue_material = MaterialDescriptor::new_rgb(0., 0., 0.8).named("blue");
	let black_material = MaterialDescriptor::new_rgb(0., 0., 0.).named("black");
	let white_material = MaterialDescriptor::new_rgb(1., 1., 1.).named("white");

	/* Step 1 */
	let base_link = Link::builder("base_link").add_visual(
		Visual::builder(CylinderGeometry::new(0.2, 0.6)).materialized(blue_material.clone()),
	);

	let model = base_link.build_tree().to_robot("visual");

	/* ====== Start right leg ====== */
	let right_leg_link = Link::builder("[\\[right]\\]_leg").add_visual(
		Visual::builder(BoxGeometry::new(0.6, 0.1, 0.2))
			.materialized(white_material.clone())
			.tranformed(Transform::new((0., 0., -0.3), (0., FRAC_PI_2, 0.))),
	);

	let right_leg = right_leg_link.build_tree();

	let right_base_link = Link::builder("[\\[right]\\]_base")
		.add_visual(Visual::builder(BoxGeometry::new(0.4, 0.1, 0.1)).materialized(white_material));

	let right_base_joint = SmartJointBuilder::new_fixed("[\\[right]\\]_base_joint")
		.add_transform(Transform::new_translation(0., 0., -0.6));

	right_leg
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(right_base_link, right_base_joint)
		.unwrap();

	let right_front_wheel_link = Link::builder("[\\[right]\\]_[[front]]_wheel").add_visual(
		Visual::builder(CylinderGeometry::new(0.035, 0.1))
			.tranformed(Transform::new_rotation(FRAC_PI_2, 0., 0.))
			.materialized(black_material.clone()),
	);

	let right_front_wheel_joint =
		SmartJointBuilder::new_fixed("[\\[right]\\]_[[front]]_wheel_joint")
			.add_transform(Transform::new_translation(0.133333333333, 0., -0.085));

	right_leg
		.get_newest_link()
		.write()
		.unwrap()
		.try_attach_child(right_front_wheel_link, right_front_wheel_joint)
		.unwrap();

	let mut right_back_wheel = right_leg
		.get_joint("[\\[right]\\]_[[front]]_wheel_joint")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
		.mirror(MirrorAxis::X);
	right_back_wheel.change_group_id("back").unwrap();

	right_leg
		.get_link("[\\[right]\\]_base")
		.unwrap()
		.write()
		.unwrap()
		.attach_joint_chain(right_back_wheel)
		.unwrap();

	let mut right_leg = right_leg.yank_link("[\\[right]\\]_leg").unwrap();
	right_leg.apply_group_id();

	let base_right_leg_joint = SmartJointBuilder::new_fixed("base_to_[[right]]_leg")
		.add_transform(Transform::new_translation(0., -0.22, 0.25));

	/* ==== Attaching right leg ==== */

	model
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(right_leg, base_right_leg_joint)
		.unwrap();

	/* ==== Attaching left leg ===== */

	let mut left_leg = model
		.get_joint("base_to_[[right]]_leg")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
		.mirror(MirrorAxis::Y);
	left_leg.change_group_id("left").unwrap();

	model
		.get_root_link()
		.write()
		.unwrap()
		.attach_joint_chain(left_leg)
		.unwrap();

	/* === Defining the gripper ==== */
	// TODO: Meshes

	/* ===== Defining the head ===== */
	let head_link = Link::builder("head").add_visual(
		Visual::builder(SphereGeometry::new(0.2))
			.materialized(MaterialDescriptor::new_rgb(1., 1., 1.).named("white")),
	);

	let head_swivel_joint =
		JointBuilder::new("head_swivel", JointType::Fixed).add_origin_offset((0., 0., 0.3));

	model
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(head_link, head_swivel_joint)
		.unwrap();

	let box_link = Link::builder("box").add_visual(
		Visual::builder(BoxGeometry::new(0.08, 0.08, 0.08)).materialized(blue_material.clone()),
	);

	let to_box_joint = SmartJointBuilder::new_fixed("tobox")
		.add_transform(Transform::new_translation(0.1814, 0., 0.1414));

	model
		.get_newest_link()
		.write()
		.unwrap()
		.try_attach_child(box_link, to_box_joint)
		.unwrap();

	let out = to_urdf_string(&model);

	println!("{}", out);
}
