/// If an argument is supplied no meshes are used.
///
/// This has been done to prevent allow for the use of an online viewer and the usage of WSL without setting some OpenGL settings.
use std::f32::consts::FRAC_PI_2;

use robot_description_builder::{
	link_data::{geometry::*, Visual},
	material::MaterialDescriptor,
	prelude::*,
	to_rdf::{
		to_urdf::{to_urdf, URDFConfig},
		xml_writer_to_string, XMLMode,
	},
	JointBuilder, JointType, Link, MirrorAxis, Robot, SmartJointBuilder, Transform,
};

fn to_urdf_string(robot: &Robot) -> String {
	xml_writer_to_string(
		to_urdf(
			robot,
			URDFConfig {
				xml_mode: XMLMode::Indent(' ', 2),
				..Default::default()
			},
		)
		.unwrap(),
	)
}

fn main() {
	// Check if we are using meshes, by default meshes are used
	let args: Vec<String> = std::env::args().collect();

	let use_meshes = args.get(1).is_none();

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
			.transformed(Transform::new((0., 0., -0.3), (0., FRAC_PI_2, 0.))),
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
		.try_attach_child(right_base_joint, right_base_link)
		.unwrap();

	let right_front_wheel_link = Link::builder("[\\[right]\\]_[[front]]_wheel").add_visual(
		Visual::builder(CylinderGeometry::new(0.035, 0.1))
			.transformed(Transform::new_rotation(FRAC_PI_2, 0., 0.))
			.materialized(black_material.clone()),
	);

	let right_front_wheel_joint =
		SmartJointBuilder::new_fixed("[\\[right]\\]_[[front]]_wheel_joint")
			.add_transform(Transform::new_translation(0.133333333333, 0., -0.085));

	right_leg
		.get_newest_link()
		.write()
		.unwrap()
		.try_attach_child(right_front_wheel_joint, right_front_wheel_link)
		.unwrap();

	let mut right_back_wheel = right_leg
		.get_joint("[\\[right]\\]_[[front]]_wheel_joint")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
		.unwrap()
		.mirror(MirrorAxis::X);
	right_back_wheel.change_group_id("back").unwrap();

	right_leg
		.get_link("[\\[right]\\]_base")
		.unwrap()
		.write()
		.unwrap()
		.attach_joint_chain(right_back_wheel)
		.unwrap();

	let mut right_leg = right_leg.yank_root().unwrap();
	right_leg.apply_group_id();

	let base_right_leg_joint = SmartJointBuilder::new_fixed("base_to_[[right]]_leg")
		.add_transform(Transform::new_translation(0., -0.22, 0.25));

	/* ==== Attaching right leg ==== */

	model
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(base_right_leg_joint, right_leg)
		.unwrap();

	/* ==== Attaching left leg ===== */

	let mut left_leg = model
		.get_joint("base_to_[[right]]_leg")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
		.unwrap()
		.mirror(MirrorAxis::Y);
	left_leg.change_group_id("left").unwrap();

	model
		.get_root_link()
		.write()
		.unwrap()
		.attach_joint_chain(left_leg)
		.unwrap();

	/* === Defining the gripper ==== */
	let gripper_pole = Link::builder("gripper_pole")
		.add_visual(
			Visual::builder(CylinderGeometry::new(0.01, 0.2))
				.transformed(Transform::new((0.1, 0., 0.), (0., FRAC_PI_2, 0.))),
		)
		.build_tree();

	let left_gripper = Link::builder("[[left]]_gripper")
		.add_visual(Visual::builder(match use_meshes {
			true => MeshGeometry::new(
				"package://urdf_tutorial/meshes/l_finger.dae",
				(0.1, 0.05, 0.06),
				None,
			)
			.boxed_clone(),
			false => BoxGeometry::new(0.1, 0.05, 0.06).boxed_clone(),
		}))
		.build_tree();

	left_gripper
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			SmartJointBuilder::new_fixed("[[left]]_tip_joint"),
			Link::builder("[[left]]_tip").add_visual(
				Visual::builder(match use_meshes {
					true => MeshGeometry::new(
						"package://urdf_tutorial/meshes/l_finger_tip.dae",
						(0.06, 0.04, 0.02),
						None,
					)
					.boxed_clone(),
					false => BoxGeometry::new(0.06, 0.04, 0.02).boxed_clone(),
				})
				.transformed(Transform::new_translation(0.09137, 0.00495, 0.)),
			),
		)
		.unwrap();

	gripper_pole
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			SmartJointBuilder::new_fixed("[[left]]_gripper_joint")
				.add_transform(Transform::new_translation(0.2, 0.01, 0.)),
			left_gripper.yank_root().unwrap(),
		)
		.unwrap();

	let mut right_gripper = gripper_pole
		.get_joint("[[left]]_gripper_joint")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
		.unwrap()
		.mirror(MirrorAxis::Y);

	right_gripper.change_group_id("right").unwrap();

	gripper_pole
		.get_root_link()
		.write()
		.unwrap()
		.attach_joint_chain(right_gripper)
		.unwrap();

	model
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			SmartJointBuilder::new_fixed("gripper_extension")
				.add_transform(Transform::new_translation(0.19, 0., 0.2)),
			gripper_pole.yank_root().unwrap(),
		)
		.unwrap();

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
		.try_attach_child(head_swivel_joint, head_link)
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
		.try_attach_child(to_box_joint, box_link)
		.unwrap();

	let out = to_urdf_string(&model);

	println!("{}", out);
}
