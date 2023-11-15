// run example with:
//    cargo run --example example-1 --features urdf
#[cfg(feature = "urdf")]
use std::io::prelude::*;

use robot_description_builder as rdb;

#[cfg(feature = "urdf")]
use rdb::to_rdf::{
	to_urdf::{to_urdf, URDFConfig},
	XMLMode,
};

use rdb::{
	link_data::{
		geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
		Collision, Visual,
	},
	JointBuilder, JointType, KinematicInterface, Link, Transform,
};

fn main() {
	let link = Link::builder("Leg_[[R1]]_l0")
		.add_visual(
			Visual::builder(BoxGeometry::new(2.0, 0.5, 0.5))
				.named("Leg_[[R1]]_l0_vis")
				.transformed(Transform::new_translation(1.0, 0.0, 0.0)),
		)
		.add_collider(
			Collision::builder(CylinderGeometry::new(0.24, 2.0))
				.named("Leg_[[R1]]_l0_col")
				.transformed(Transform::new_translation(1.0, 0.0, 0.0)),
		)
		.build_tree();

	link.get_newest_link()
		.try_write()
		.unwrap()
		.try_attach_child(
			JointBuilder::new("Leg_[[R1]]_j1", JointType::Fixed)
				.add_origin_offset((2.0, 0., 0.))
				.to_owned(),
			Link::builder("Leg_[[R1]]_l1")
				.add_visual(
					Visual::builder(BoxGeometry::new(0.5, 0.1, 0.1))
						.named("Leg_[[R1]]_l1_vis")
						.transformed(Transform::new_translation(1., 0., 0.)),
				)
				.add_collider(
					Collision::builder(BoxGeometry::new(0.25, 0.1, 0.1))
						.named("Leg_[[R1]]_l1_col")
						.transformed(Transform::new_translation(0.25, 0., 0.)),
				),
		)
		.unwrap();

	let robot_root = Link::builder("robot_root")
		.add_visual(Visual::builder(SphereGeometry::new(0.3)).named("robot_root_vis"))
		.add_collider(Collision::builder(SphereGeometry::new(0.35)).named("robot_root_col"))
		.build_tree();

	let robot = robot_root.to_robot("my_robot");

	robot
		.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(
			JointBuilder::new("Leg_[[R1]]_j0", JointType::Fixed)
				.add_origin_offset((0.4, 0., 0.))
				.to_owned(),
			link,
		)
		.unwrap();

	#[cfg(feature = "urdf")]
	{
		let mut buffer = to_urdf(
			&robot,
			URDFConfig {
				xml_mode: XMLMode::Indent(' ', 4),
				..Default::default()
			},
		)
		.unwrap()
		.into_inner()
		.to_owned();

		let mut out = String::new();
		buffer.rewind().unwrap();
		buffer.read_to_string(&mut out).unwrap();

		println!("{}", out);
	}

	#[cfg(not(feature = "urdf"))]
	println!("{:?}", robot)
}
