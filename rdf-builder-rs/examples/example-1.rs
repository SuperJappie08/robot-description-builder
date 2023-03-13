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
	JointBuilder, JointType, KinematicInterface, Link, TransformData,
};

fn main() {
	let link = Link::new("Leg_[R1]_l0".into());
	link.get_root_link()
		.write()
		.unwrap()
		.add_visual(Visual::new(
			Some("Leg_[R1]_l0_vis".into()),
			Some(TransformData {
				translation: Some((1.0, 0.0, 0.0)),
				..Default::default()
			}),
			BoxGeometry::new(2.0, 0.5, 0.5).into(),
			None,
		))
		.add_collider(Collision::new(
			Some("Leg_[R1]_l0_col".into()),
			Some(TransformData {
				translation: Some((1.0, 0.0, 0.0)),
				..Default::default()
			}),
			CylinderGeometry::new(0.24, 2.0).into(),
		));

	link.get_newest_link()
		.try_write()
		.unwrap()
		.try_attach_child(
			Box::new(Link::new("Leg_[R1]_l1".into())),
			JointBuilder::new("Leg_[R1]_j1".into(), JointType::Fixed)
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
			BoxGeometry::new(0.5, 0.1, 0.1).into(),
			None,
		))
		.add_collider(Collision::new(
			Some("Leg_[R1]_l1_col".into()),
			Some(TransformData {
				translation: Some((0.25, 0., 0.)),
				..Default::default()
			}),
			BoxGeometry::new(0.25, 0.1, 0.1).into(),
		));

	let robot_root = Link::new("robot_root".into());
	let robot = robot_root.to_robot("my_robot".into());
	robot
		.get_root_link()
		.write()
		.unwrap()
		.add_visual(Visual::new(
			Some("robot_root_vis".into()),
			None,
			SphereGeometry::new(0.3).into(),
			None,
		))
		.add_collider(Collision::new(
			Some("robot_root_col".into()),
			None,
			SphereGeometry::new(0.35).into(),
		))
		.try_attach_child(
			link.into(),
			JointBuilder::new("Leg_[R1]_j0".into(), JointType::Fixed)
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
