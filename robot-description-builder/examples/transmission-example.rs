use robot_description_builder::{
	transmission::{
		TransmissionActuator, TransmissionBuilder, TransmissionHardwareInterface, TransmissionType,
	},
	KinematicInterface, Link, SmartJointBuilder,
};

fn main() {
	let transmission_builder =
		TransmissionBuilder::new("test", TransmissionType::SimpleTransmission)
			.add_joint((
				"Jointy",
				TransmissionHardwareInterface::ActuatorStateInterface,
			))
			.add_actuator(TransmissionActuator::new("dave").mechanically_reduced(5000000.));

	let tree = Link::builder("root").build_tree();

	tree.get_root_link()
		.try_write()
		.unwrap()
		.try_attach_child(
			SmartJointBuilder::new_continuous("Jointy"),
			Link::builder("child"),
		)
		.unwrap();

	tree.try_add_transmission(transmission_builder).unwrap();

	println!("{:#?}", &tree);
}
