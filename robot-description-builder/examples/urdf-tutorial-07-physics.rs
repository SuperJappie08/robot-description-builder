/// If an argument is supplied no meshes are used.
///
/// This has been done to prevent allow for the use of an online viewer and the usage of WSL without setting some OpenGL settings.
use std::{
	f32::consts::FRAC_PI_2,
	io::{Read, Seek},
};

use robot_description_builder as rdb;

use rdb::{
	link_data::{geometry::*, Collision, InertialData, Visual},
	material::MaterialDescriptor,
	prelude::*,
	to_rdf::{
		to_urdf::{to_urdf, URDFConfig},
		XMLMode,
	},
	Link, MirrorAxis, Robot, SmartJointBuilder, Transform,
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

/// [Adding Physical and Collision Properties to a URDF Model](http://wiki.ros.org/urdf/Tutorials/Adding%20Physical%20and%20Collision%20Properties%20to%20a%20URDF%20Model)
/// Using ROS 2 as reference, since it is the most [up-to-date](https://github.com/ros/urdf_tutorial/blob/ros2/urdf/07-physics.urdf).
fn main() {
	// Check if we are using meshes, by default meshes are used
	let args: Vec<String> = std::env::args().collect();

	let use_meshes = args.get(1).is_none();

	/* === Material Descriptions === */
	let blue_material = MaterialDescriptor::new_rgb(0., 0., 0.8).named("blue");
	let black_material = MaterialDescriptor::new_rgb(0., 0., 0.).named("black");
	let white_material = MaterialDescriptor::new_rgb(1., 1., 1.).named("white");

	/* Step 1 */
	let base_link = Link::builder("base_link")
		.add_visual(
			Visual::builder(CylinderGeometry::new(0.2, 0.6)).materialized(blue_material.clone()),
		)
		.add_collider(Collision::builder(CylinderGeometry::new(0.2, 0.6)))
		.add_intertial(InertialData {
			mass: 10.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

	let model = base_link.build_tree().to_robot("physics");

	/* ====== Start right leg ====== */
	let right_leg_link = Link::builder("[\\[right]\\]_leg")
		.add_visual(
			Visual::builder(BoxGeometry::new(0.6, 0.1, 0.2))
				.materialized(white_material.clone())
				.tranformed(Transform::new((0., 0., -0.3), (0., FRAC_PI_2, 0.))),
		)
		.add_collider(
			Collision::builder(BoxGeometry::new(0.6, 0.1, 0.2))
				.tranformed(Transform::new((0., 0., -0.3), (0., FRAC_PI_2, 0.))),
		)
		.add_intertial(InertialData {
			mass: 10.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

	let right_leg = right_leg_link.build_tree();

	let right_base_link = Link::builder("[\\[right]\\]_base")
		.add_visual(Visual::builder(BoxGeometry::new(0.4, 0.1, 0.1)).materialized(white_material))
		.add_collider(Collision::builder(BoxGeometry::new(0.4, 0.1, 0.1)))
		.add_intertial(InertialData {
			mass: 10.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

	let right_base_joint = SmartJointBuilder::new_fixed("[\\[right]\\]_base_joint")
		.add_transform(Transform::new_translation(0., 0., -0.6));

	right_leg
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(right_base_link, right_base_joint)
		.unwrap();

	let right_front_wheel_link = Link::builder("[\\[right]\\]_[[front]]_wheel")
		.add_visual(
			Visual::builder(CylinderGeometry::new(0.035, 0.1))
				.tranformed(Transform::new_rotation(FRAC_PI_2, 0., 0.))
				.materialized(black_material.clone()),
		)
		.add_collider(
			Collision::builder(CylinderGeometry::new(0.035, 0.1))
				.tranformed(Transform::new_rotation(FRAC_PI_2, 0., 0.)),
		)
		.add_intertial(InertialData {
			mass: 1.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

	let right_front_wheel_joint =
		SmartJointBuilder::new_continuous("[\\[right]\\]_[[front]]_wheel_joint")
			.with_axis((0., 1., 0.))
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
	// Need to de-mirror the rotation axis.
	right_back_wheel.with_axis((0., 1., 0.));

	right_leg
		.get_link("[\\[right]\\]_base")
		.unwrap()
		.write()
		.unwrap()
		.attach_joint_chain(right_back_wheel)
		.unwrap();

	let mut right_leg = right_leg.yank_root();
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
	let gripper_pole = Link::builder("gripper_pole")
		.add_visual(
			Visual::builder(CylinderGeometry::new(0.01, 0.2))
				.tranformed(Transform::new((0.1, 0., 0.), (0., FRAC_PI_2, 0.))),
		)
		.add_collider(
			Collision::builder(CylinderGeometry::new(0.01, 0.2))
				.tranformed(Transform::new((0.1, 0., 0.), (0., FRAC_PI_2, 0.))),
		)
		.add_intertial(InertialData {
			mass: 0.05,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		})
		.build_tree();

	let left_gripper_geometry = match use_meshes {
		true => MeshGeometry::new(
			"package://urdf_tutorial/meshes/l_finger.dae",
			(0.1, 0.05, 0.06),
			None,
		)
		.boxed_clone(),
		false => BoxGeometry::new(0.1, 0.05, 0.06).boxed_clone(),
	};

	let left_gripper = Link::builder("[[left]]_gripper")
		.add_visual(Visual::builder(left_gripper_geometry.boxed_clone()))
		.add_collider(Collision::builder(left_gripper_geometry))
		.add_intertial(InertialData {
			mass: 0.05,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		})
		.build_tree();

	let left_tip_geometry = match use_meshes {
		true => MeshGeometry::new(
			"package://urdf_tutorial/meshes/l_finger_tip.dae",
			(0.06, 0.04, 0.02),
			None,
		)
		.boxed_clone(),
		false => BoxGeometry::new(0.06, 0.04, 0.02).boxed_clone(),
	};

	left_gripper
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			Link::builder("[[left]]_tip")
				.add_visual(
					Visual::builder(left_tip_geometry.boxed_clone())
						.tranformed(Transform::new_translation(0.09137, 0.00495, 0.)),
				)
				.add_collider(
					Collision::builder(left_tip_geometry)
						.tranformed(Transform::new_translation(0.09137, 0.00495, 0.)),
				)
				.add_intertial(InertialData {
					mass: 0.05,
					ixx: 1e-3,
					iyy: 1e-3,
					izz: 1e-3,
					..Default::default()
				}),
			SmartJointBuilder::new_fixed("[[left]]_tip_joint"),
		)
		.unwrap();

	gripper_pole
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			left_gripper.yank_root(),
			SmartJointBuilder::new_revolute("[[left]]_gripper_joint")
				.with_axis((0., 0., 1.))
				.with_limit(1000., 0.5)
				.set_upper_limit(0.548)
				.set_lower_limit(0.)
				.add_transform(Transform::new_translation(0.2, 0.01, 0.)),
		)
		.unwrap();

	let mut right_gripper = gripper_pole
		.get_joint("[[left]]_gripper_joint")
		.unwrap()
		.read()
		.unwrap()
		.rebuild_branch()
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
			gripper_pole.yank_root(),
			SmartJointBuilder::new_prismatic("gripper_extension")
				.with_limit(1000., 0.5)
				.set_lower_limit(-0.38)
				.set_upper_limit(0.)
				.add_transform(Transform::new_translation(0.19, 0., 0.2)),
		)
		.unwrap();

	/* ===== Defining the head ===== */
	let head_link = Link::builder("head")
		.add_visual(
			Visual::builder(SphereGeometry::new(0.2))
				.materialized(MaterialDescriptor::new_rgb(1., 1., 1.).named("white")),
		)
		.add_collider(Collision::builder(SphereGeometry::new(0.2)))
		.add_intertial(InertialData {
			mass: 2.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

	let head_swivel_joint = SmartJointBuilder::new_continuous("head_swivel")
		.with_axis((0., 0., 1.))
		.add_transform(Transform::new_translation(0., 0., 0.3));

	model
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(head_link, head_swivel_joint)
		.unwrap();

	//  The URDF tutorial is inconsistent here, out of nowhere translates visual, but not collision.
	let box_link = Link::builder("box")
		.add_visual(
			Visual::builder(BoxGeometry::new(0.08, 0.08, 0.08))
				.materialized(blue_material.clone())
				.tranformed(Transform::new_translation(-0.04, 0., 0.)),
		)
		.add_collider(
			Collision::builder(BoxGeometry::new(0.08, 0.08, 0.08))
				.tranformed(Transform::new_translation(-0.04, 0., 0.)),
		)
		.add_intertial(InertialData {
			mass: 1.,
			ixx: 1e-3,
			iyy: 1e-3,
			izz: 1e-3,
			..Default::default()
		});

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
