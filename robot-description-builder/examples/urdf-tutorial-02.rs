use std::io::{Read, Seek};

use robot_description_builder::{
	link_data::geometry::{BoxGeometry, CylinderGeometry},
	linkbuilding::{LinkBuilder, VisualBuilder},
	prelude::*,
	to_rdf::{
		to_urdf::{to_urdf, URDFConfig},
		XMLMode,
	},
	Robot, SmartJointBuilder,
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
	/* Step 1 */
	let base_link = LinkBuilder::new("base_link")
		.add_visual(VisualBuilder::new(CylinderGeometry::new(0.2, 0.6)));

	let tree = base_link.build_tree();

	/* UNCOMMENT FOR 1 */
	// let out = to_urdf_string(
	//     &tree.to_robot("my_first")
	// );

	let right_leg_link = LinkBuilder::new("right_leg")
		.add_visual(VisualBuilder::new(BoxGeometry::new(0.6, 0.1, 0.2)));

	let base_right_leg_joint = SmartJointBuilder::new_fixed("base_to_right_leg");

	tree.get_root_link()
		.write()
		.unwrap()
		.try_attach_child(base_right_leg_joint, right_leg_link)
		.unwrap();

	// /* UNCOMMENT FOR 2 */
	let out = to_urdf_string(&tree.to_robot("multipleshapes"));

	println!("{}", out);
}
