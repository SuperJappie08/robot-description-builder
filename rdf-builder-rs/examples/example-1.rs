// run example with:
//    cargo run --example example-1

use rdf_builder_rs::{Joint, JointType, KinematicInterface, Link};

fn main() {
	let link = Link::new("name".into());
	link.get_newest_link()
		.try_write()
		.unwrap()
		.try_attach_child(
			Box::new(Link::new("name2".into())),
			Joint::new("dave".into(), JointType::Fixed),
		)
		.unwrap();
	todo!()
}
