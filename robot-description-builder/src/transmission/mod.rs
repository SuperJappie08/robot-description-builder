//! TODO: MODULE DOC
//! This needs a rewrite, since the system is very different then on first inspection.
//!
//! struct Transmission {
//!     name: String
//!     data: TransmissionType/dyn impl TransmissionInterface
//! }
//!

mod build_transmission_error;
mod hardware_interface;
mod transmission_actuator;
mod transmission_joint;
mod transmission_type;

use std::sync::Weak;

use crate::{cluster_objects::kinematic_data_tree::KinematicDataTree, identifiers::GroupID};

use itertools::Itertools;

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

pub use build_transmission_error::BuildTransmissionError;
pub use hardware_interface::TransmissionHardwareInterface;
pub use transmission_actuator::TransmissionActuator;
pub use transmission_joint::{TransmissionJoint, TransmissionJointBuilder};
pub use transmission_type::TransmissionType;

pub(crate) mod transmission_builder_state {
	/// Trait for the [`TransmissionJointBuilder`](super::TransmissionJointBuilder) container state structs.
	pub trait TransmissionJointTrait {
		/// Returns the current vector of `TransmissionJointBuilder`, creates a new one if it was empty.
		fn into_inner(self) -> Vec<super::TransmissionJointBuilder>;
	}

	#[derive(Debug, PartialEq, Clone)]
	pub struct NoJoints;
	impl TransmissionJointTrait for NoJoints {
		fn into_inner(self) -> Vec<super::TransmissionJointBuilder> {
			Vec::new()
		}
	}

	#[derive(Debug, PartialEq, Clone)]
	pub struct WithJoints(pub(super) Vec<super::TransmissionJointBuilder>);
	impl TransmissionJointTrait for WithJoints {
		fn into_inner(self) -> Vec<super::TransmissionJointBuilder> {
			self.0
		}
	}

	/// Trait for the [`TransmissionActuator`](super::TransmissionActuator) container state structs.
	pub trait TransmissionActuatorTrait {
		/// Returns the current vector of `TransmissionActuator`, creates a new one if it was empty.
		fn into_inner(self) -> Vec<super::TransmissionActuator>;
	}

	#[derive(Debug, PartialEq, Clone)]
	pub struct NoActuator;
	impl TransmissionActuatorTrait for NoActuator {
		fn into_inner(self) -> Vec<super::TransmissionActuator> {
			Vec::new()
		}
	}

	#[derive(Debug, PartialEq, Clone)]
	pub struct WithActuator(pub(super) Vec<super::TransmissionActuator>);
	impl TransmissionActuatorTrait for WithActuator {
		fn into_inner(self) -> Vec<super::TransmissionActuator> {
			self.0
		}
	}
}

#[cfg(feature = "wrapper")]
pub use transmission_builder_state::{NoActuator, NoJoints, WithActuator, WithJoints};

#[cfg(not(feature = "wrapper"))]
use transmission_builder_state::{NoActuator, NoJoints, WithActuator, WithJoints};

use transmission_builder_state::{TransmissionActuatorTrait, TransmissionJointTrait};

#[derive(Debug, PartialEq, Clone)]
pub struct TransmissionBuilder<Joints, Actuators>
where
	Joints: TransmissionJointTrait,
	Actuators: TransmissionActuatorTrait,
{
	name: String,
	transmission_type: TransmissionType,
	joints: Joints,
	actuators: Actuators,
}

impl TransmissionBuilder<NoJoints, NoActuator> {
	pub fn new(name: impl Into<String>, transmission_type: TransmissionType) -> Self {
		Self {
			name: name.into(),
			transmission_type,
			joints: NoJoints,
			actuators: NoActuator,
		}
	}
}

impl<Actuator, Joints> TransmissionBuilder<Joints, Actuator>
where
	Joints: TransmissionJointTrait,
	Actuator: TransmissionActuatorTrait,
{
	pub fn add_joint(
		self,
		transmission_joint: impl Into<TransmissionJointBuilder>,
	) -> TransmissionBuilder<WithJoints, Actuator> {
		let mut joints = self.joints.into_inner();

		joints.push(transmission_joint.into());

		TransmissionBuilder {
			name: self.name,
			transmission_type: self.transmission_type,
			joints: WithJoints(joints),
			actuators: self.actuators,
		}
	}

	pub fn add_actuator(
		self,
		transmission_actuator: TransmissionActuator,
	) -> TransmissionBuilder<Joints, WithActuator> {
		let mut actuators = self.actuators.into_inner();

		actuators.push(transmission_actuator);

		TransmissionBuilder {
			name: self.name,
			transmission_type: self.transmission_type,
			joints: self.joints,
			actuators: WithActuator(actuators),
		}
	}

	/// Gets a reference to the name of the current `TransmissionBuilder`.
	pub fn name(&self) -> &String {
		&self.name
	}

	pub fn transmission_type(&self) -> &TransmissionType {
		&self.transmission_type
	}
}

impl<Actuators> TransmissionBuilder<WithJoints, Actuators>
where
	Actuators: TransmissionActuatorTrait,
{
	pub fn joints(&self) -> Option<&Vec<TransmissionJointBuilder>> {
		Some(&self.joints.0)
	}
}

impl<Actuators> TransmissionBuilder<NoJoints, Actuators>
where
	Actuators: TransmissionActuatorTrait,
{
	pub fn joints(&self) -> Option<&Vec<TransmissionJointBuilder>> {
		None
	}
}

impl<Joints> TransmissionBuilder<Joints, WithActuator>
where
	Joints: TransmissionJointTrait,
{
	pub fn actuators(&self) -> Option<&Vec<TransmissionActuator>> {
		Some(&self.actuators.0)
	}
}

impl<Joints> TransmissionBuilder<Joints, NoActuator>
where
	Joints: TransmissionJointTrait,
{
	pub fn actuators(&self) -> Option<&Vec<TransmissionActuator>> {
		None
	}
}

impl TransmissionBuilder<WithJoints, WithActuator> {
	pub(crate) fn build(
		self,
		tree: &Weak<KinematicDataTree>,
	) -> Result<Transmission, BuildTransmissionError> {
		// Unwrap Ok because called from tree
		let tree = Weak::upgrade(tree)
			.expect("KinematicDataTree should be initilized before registering Transmissions");

		Ok(Transmission {
			name: self.name,
			transmission_type: self.transmission_type,
			joints: self
				.joints
				.0
				.into_iter()
				.map(|transmission_joint_builder| transmission_joint_builder.build(&tree))
				.process_results(|iter| iter.collect())?,
			actuators: self.actuators.0,
		})
	}
}

#[derive(Debug, PartialEq)]
/// Represents a transmission between a `Joint` and a actuator.
///
/// TODO: DOCS
/// # TODO: DOCS
///  - Link `Joint`
///  - Link ROS WIKI
pub struct Transmission {
	name: String,
	transmission_type: TransmissionType,
	joints: Vec<TransmissionJoint>,
	actuators: Vec<TransmissionActuator>,
}

impl Transmission {
	/// Gets a reference to the name of the current `Transmission`.
	pub fn name(&self) -> &String {
		&self.name
	}

	/// Gets the `TransmissionType` of the current `Transmission`.
	///
	/// See [`TransmissionType`] for more details about the possible transmissiontypes.
	pub fn transmission_type(&self) -> TransmissionType {
		self.transmission_type
	}

	/// Gets a refence to the vector containning all the joint data of this `Transmission`.
	///
	/// TODO: EXPLAIN SOMETHING ABOUT TRANSMISSIONJOINT
	pub fn joints(&self) -> &Vec<TransmissionJoint> {
		&self.joints
	}

	/// Gets a refence to the vector containning all the actuators of this `Transmission`.
	///
	/// TODO: EXPLAIN SOMETHING ABOUT [`TransmissionActuator`]
	pub fn actuators(&self) -> &Vec<TransmissionActuator> {
		&self.actuators
	}

	/// Recreates a `TransmissionBuilder` that would reconstruct this `Transmission`
	pub fn rebuild(&self) -> TransmissionBuilder<WithJoints, WithActuator> {
		TransmissionBuilder {
			name: self.name.clone(),
			transmission_type: self.transmission_type,
			joints: WithJoints(
				self.joints
					.iter()
					.map(|transmission_joint| transmission_joint.rebuild())
					.collect(),
			),
			actuators: WithActuator(self.actuators.to_vec()),
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for Transmission {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("transmission")
			.with_attribute(Attribute {
				key: QName(b"name"),
				value: self.name().display().as_bytes().into(),
			})
			.write_inner_content(|writer| -> quick_xml::Result<()> {
				self.transmission_type().to_urdf(writer, urdf_config)?;

				self.joints()
					.iter()
					.map(|transmission_joint| transmission_joint.to_urdf(writer, urdf_config))
					.process_results(|iter| iter.collect_vec())?;

				self.actuators
					.iter()
					.map(|transmission_actuator| transmission_actuator.to_urdf(writer, urdf_config))
					.process_results(|iter| iter.collect_vec())?;

				Ok(())
			})?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{
		transmission_builder_state, TransmissionActuator, TransmissionBuilder,
		TransmissionHardwareInterface, TransmissionJointBuilder, TransmissionType,
	}; // use super::Transmission;
	use test_log::test;

	// TODO: ADD MORE TESTS

	#[test]
	fn add_joint() {
		let transmission_builder = TransmissionBuilder::new(
			"I'm on a mission, a transmission",
			TransmissionType::FourBarLinkageTransmission,
		);

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::NoJoints,
				actuators: transmission_builder_state::NoActuator,
			}
		);

		let mut transmission_builder =
			transmission_builder.add_joint(TransmissionJointBuilder::new(
				"joint-1",
				TransmissionHardwareInterface::PosVelAccJointInterface,
			));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![(
					"joint-1",
					TransmissionHardwareInterface::PosVelAccJointInterface
				)
					.into()]),
				actuators: transmission_builder_state::NoActuator,
			}
		);

		transmission_builder = transmission_builder.add_joint(
			TransmissionJointBuilder::new(
				"joint-z",
				TransmissionHardwareInterface::IMUSensorInterface,
			)
			.with_hw_inteface(TransmissionHardwareInterface::VelocityActuatorInterface),
		);

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![
					(
						"joint-1",
						TransmissionHardwareInterface::PosVelAccJointInterface
					)
						.into(),
					(
						"joint-z",
						vec![
							TransmissionHardwareInterface::IMUSensorInterface,
							TransmissionHardwareInterface::VelocityActuatorInterface
						]
					)
						.into()
				]),
				actuators: transmission_builder_state::NoActuator,
			}
		);

		let mut transmission_builder =
			transmission_builder.add_actuator(TransmissionActuator::new("actuator"));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![
					(
						"joint-1",
						TransmissionHardwareInterface::PosVelAccJointInterface
					)
						.into(),
					(
						"joint-z",
						vec![
							TransmissionHardwareInterface::IMUSensorInterface,
							TransmissionHardwareInterface::VelocityActuatorInterface
						]
					)
						.into()
				]),
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator")
				]),
			}
		);

		transmission_builder = transmission_builder.add_joint(TransmissionJointBuilder::new(
			"joint-99",
			TransmissionHardwareInterface::ForceTorqueSensorInterface,
		));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![
					(
						"joint-1",
						TransmissionHardwareInterface::PosVelAccJointInterface
					)
						.into(),
					(
						"joint-z",
						vec![
							TransmissionHardwareInterface::IMUSensorInterface,
							TransmissionHardwareInterface::VelocityActuatorInterface
						]
					)
						.into(),
					(
						"joint-99",
						TransmissionHardwareInterface::ForceTorqueSensorInterface
					)
						.into()
				]),
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator")
				]),
			}
		);
	}

	#[test]
	fn add_actuator() {
		let transmission_builder = TransmissionBuilder::new(
			"I'm on a mission, a transmission",
			TransmissionType::FourBarLinkageTransmission,
		);

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::NoJoints,
				actuators: transmission_builder_state::NoActuator,
			}
		);

		let mut transmission_builder =
			transmission_builder.add_actuator(TransmissionActuator::new("actuator-1"));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::NoJoints,
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator-1")
				]),
			}
		);

		transmission_builder = transmission_builder
			.add_actuator(TransmissionActuator::new_with_reduction("actuator-z", -99.));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::NoJoints,
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator-1"),
					TransmissionActuator::new_with_reduction("actuator-z", -99.),
				]),
			}
		);

		let mut transmission_builder =
			transmission_builder.add_joint(TransmissionJointBuilder::new(
				"joint-x",
				TransmissionHardwareInterface::VelocityJointInterface,
			));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![(
					"joint-x",
					TransmissionHardwareInterface::VelocityJointInterface
				)
					.into()]),
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator-1"),
					TransmissionActuator::new_with_reduction("actuator-z", -99.),
				]),
			}
		);

		transmission_builder =
			transmission_builder.add_actuator(TransmissionActuator::new("actuator-bob"));

		assert_eq!(
			transmission_builder,
			TransmissionBuilder {
				name: "I'm on a mission, a transmission".into(),
				transmission_type: TransmissionType::FourBarLinkageTransmission,
				joints: transmission_builder_state::WithJoints(vec![(
					"joint-x",
					TransmissionHardwareInterface::VelocityJointInterface
				)
					.into()]),
				actuators: transmission_builder_state::WithActuator(vec![
					TransmissionActuator::new("actuator-1"),
					TransmissionActuator::new_with_reduction("actuator-z", -99.),
					TransmissionActuator::new("actuator-bob")
				]),
			}
		);
	}

	#[cfg(feature = "urdf")]
	mod to_urdf {
		use super::{
			test, TransmissionActuator, TransmissionBuilder, TransmissionHardwareInterface,
			TransmissionType,
		};

		use std::io::Seek;

		use crate::to_rdf::to_urdf::{ToURDF, URDFConfig};
		use crate::transmission::TransmissionJointBuilder;
		use crate::{cluster_objects::KinematicInterface, joint::SmartJointBuilder, link::Link};

		#[test]
		fn to_urdf() {
			let tree = Link::builder("root").build_tree();

			tree.get_root_link()
				.try_write()
				.unwrap()
				.try_attach_child(
					SmartJointBuilder::new_continuous("Jointy"),
					Link::builder("child"),
				)
				.unwrap();

			let transmission_builder =
				TransmissionBuilder::new("test", TransmissionType::SimpleTransmission)
					.add_joint(TransmissionJointBuilder::new(
						"Jointy",
						TransmissionHardwareInterface::EffortJointInterface,
					))
					.add_actuator(TransmissionActuator::new("dave").mechanically_reduced(5000000.));

			tree.try_add_transmission(transmission_builder).unwrap();

			let mut writer = quick_xml::Writer::new(std::io::Cursor::new(Vec::new()));

			assert!(tree
				.get_transmission("test")
				.unwrap()
				.try_read()
				.unwrap()
				.to_urdf(&mut writer, &URDFConfig::default())
				.is_ok());

			writer.get_mut().rewind().unwrap();

			assert_eq!(
				std::io::read_to_string(writer.into_inner()).unwrap(),
				r#"<transmission name="test"><type>transmission_interface/SimpleTransmission</type><joint name="Jointy"><hardwareInterface>hardware_interface/EffortJointInterface</hardwareInterface></joint><actuator name="dave"><mechanicalReduction>5000000</mechanicalReduction></actuator></transmission>"#
			)
		}
	}
}
