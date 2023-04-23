// run example with:
//    cargo run --example example-1 --features urdf
#[cfg(feature = "urdf")]
use std::io::prelude::*;

#[cfg(feature = "urdf")]
use rdf_builder_rs::to_rdf::{
	to_urdf::{to_urdf, URDFConfig},
	XMLMode,
};
use rdf_builder_rs::{
	link_data::{
		geometry::{BoxGeometry, CylinderGeometry, SphereGeometry},
		Collision, Visual,
	},
	linkbuilding::LinkBuilder,
	JointBuilder, JointType, KinematicInterface, Link, TransformData,
};

fn main() {
	let link = Link::builder("Leg_[R1]_l0").build_tree();
	link.get_root_link()
		.write()
		.unwrap()
		.add_visual(Visual::new(
			Some("Leg_[R1]_l0_vis".into()),
			Some(TransformData {
				translation: Some((1.0, 0.0, 0.0)),
				..Default::default()
			}),
			BoxGeometry::new(2.0, 0.5, 0.5),
			None,
		))
		.add_collider(Collision::new(
			Some("Leg_[R1]_l0_col".into()),
			Some(TransformData {
				translation: Some((1.0, 0.0, 0.0)),
				..Default::default()
			}),
			CylinderGeometry::new(0.24, 2.0),
		));

	link.get_newest_link()
		.try_write()
		.unwrap()
		.try_attach_child(
			LinkBuilder::new("Leg_[R1]_l1"),
			JointBuilder::new("Leg_[R1]_j1", JointType::Fixed)
				.add_origin_offset((2.0, 0., 0.))
				.to_owned(),
		)
		.unwrap();

	link.get_newest_link()
		.write()
		.unwrap()
		.add_visual(Visual::new(
			Some("Leg_[R1]_l1_vis".into()),
			Some(TransformData {
				translation: Some((1.0, 0., 0.)),
				..Default::default()
			}),
			BoxGeometry::new(0.5, 0.1, 0.1),
			None,
		))
		.add_collider(Collision::new(
			Some("Leg_[R1]_l1_col".into()),
			Some(TransformData {
				translation: Some((0.25, 0., 0.)),
				..Default::default()
			}),
			BoxGeometry::new(0.25, 0.1, 0.1),
		));

	let robot_root = LinkBuilder::new("robot_root").build_tree();
	let robot = robot_root.to_robot("my_robot");
	robot
		.get_root_link()
		.write()
		.unwrap()
		.add_visual(Visual::new(
			Some("robot_root_vis".into()),
			None,
			SphereGeometry::new(0.3),
			None,
		))
		.add_collider(Collision::new(
			Some("robot_root_col".into()),
			None,
			SphereGeometry::new(0.35),
		))
		.try_attach_child(
			link,
			JointBuilder::new("Leg_[R1]_j0", JointType::Fixed)
				.add_origin_offset((0.4, 0., 0.))
				.to_owned(),
		)
		.unwrap();
	#[cfg(feature = "urdf")]
	{
		let mut buffer = to_urdf(
			robot,
			URDFConfig {
				xml_mode: XMLMode::Indent(' ', 4),
				..Default::default()
			},
		)
		.unwrap()
		.inner()
		.to_owned();

		let mut out = String::new();
		buffer.rewind().unwrap();
		buffer.read_to_string(&mut out).unwrap();

		println!("{}", out);
	}
	#[cfg(not(feature = "urdf"))]
	println!("{:?}", robot)
}
