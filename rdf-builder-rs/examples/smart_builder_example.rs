use rdf_builder_rs::{OffsetMode, SmartJointBuilder};

fn main() {
	let fixed_builder = SmartJointBuilder::new("fixed".into())
		.fixed()
		.add_offset(OffsetMode::FigureItOut);

	println!("{:#?}", fixed_builder);

	let revolute_builder = SmartJointBuilder::new("rev".into())
		.revolute()
		.set_axis((3., 2., 4.))
		.set_friction(400.);

	println!("{:#?}", revolute_builder);
}
