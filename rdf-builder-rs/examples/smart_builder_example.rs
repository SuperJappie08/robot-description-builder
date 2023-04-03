use rdf_builder_rs::{OffsetMode, SmartJointBuilder};

fn main() {
	let fixed_builder =
		SmartJointBuilder::new("fixed")
			.fixed()
			.add_offset(OffsetMode::FigureItOut(
				rdf_builder_rs::link_data::ConnectionPoint::End,
			));

	println!("{:#?}", fixed_builder);

	let revolute_builder = SmartJointBuilder::new("rev")
		.revolute()
		.with_axis((3., 2., 4.))
		.with_dynamics()
		.set_friction(400.);

	println!("{:#?}", revolute_builder);
}
