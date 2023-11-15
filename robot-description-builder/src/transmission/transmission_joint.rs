use std::sync::{Arc, Weak};

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	identifiers::GroupID,
	joint::Joint,
	utils::{ArcRW, WeakLock},
};

#[cfg(feature = "urdf")]
use crate::to_rdf::to_urdf::ToURDF;
#[cfg(feature = "xml")]
use itertools::Itertools;
#[cfg(feature = "xml")]
use quick_xml::{events::attributes::Attribute, name::QName};

use super::{BuildTransmissionError, TransmissionHardwareInterface};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct TransmissionJointBuilder {
	joint_name: String,
	hardware_interfaces: Vec<TransmissionHardwareInterface>,
}

impl TransmissionJointBuilder {
	/// TODO: DOC
	///
	/// The minum hardware interfaces is 1 so I require one at creation
	pub fn new(
		joint_name: impl Into<String>,
		hardware_interface: TransmissionHardwareInterface,
	) -> Self {
		Self {
			joint_name: joint_name.into(),
			hardware_interfaces: vec![hardware_interface],
		}
	}

	pub fn add_hw_interface(&mut self, hardware_interface: TransmissionHardwareInterface) {
		self.hardware_interfaces.push(hardware_interface)
	}

	pub fn with_hw_inteface(mut self, hardware_interface: TransmissionHardwareInterface) -> Self {
		self.hardware_interfaces.push(hardware_interface);

		self
	}

	pub fn name(&self) -> &String {
		&self.joint_name
	}

	pub fn hw_interfaces(&self) -> &Vec<TransmissionHardwareInterface> {
		&self.hardware_interfaces
	}

	pub fn hw_interfaces_mut(&mut self) -> &mut Vec<TransmissionHardwareInterface> {
		&mut self.hardware_interfaces
	}

	pub(super) fn build(
		self,
		tree: &Arc<KinematicDataTree>,
	) -> Result<TransmissionJoint, BuildTransmissionError> {
		let joint = match tree.joints.mread()?.get(self.name()).map(Weak::clone) {
			Some(joint) => joint,
			None => return Err(BuildTransmissionError::InvalidJoint(self.joint_name)),
		};

		Ok(TransmissionJoint {
			joint,
			hardware_interfaces: self.hardware_interfaces,
		})
	}
}

impl<Name> From<(Name, TransmissionHardwareInterface)> for TransmissionJointBuilder
where
	Name: Into<String>,
{
	fn from(value: (Name, TransmissionHardwareInterface)) -> Self {
		let (name, hardware_interface) = value;

		Self::new(name, hardware_interface)
	}
}

impl<Name> From<(Name, Vec<TransmissionHardwareInterface>)> for TransmissionJointBuilder
where
	Name: Into<String>,
{
	fn from(value: (Name, Vec<TransmissionHardwareInterface>)) -> Self {
		let (name, hardware_interfaces) = value;

		Self {
			joint_name: name.into(),
			hardware_interfaces,
		}
	}
}

#[derive(Debug)]
// pub(super)
pub struct TransmissionJoint {
	/// TODO: This is not the way for the builder since it is not transmutable to other groups
	joint: WeakLock<Joint>,
	/// TODO:
	hardware_interfaces: Vec<TransmissionHardwareInterface>,
}

impl TransmissionJoint {
	pub fn joint(&self) -> WeakLock<Joint> {
		Weak::clone(&self.joint)
	}

	pub fn hardware_interfaces(&self) -> &Vec<TransmissionHardwareInterface> {
		&self.hardware_interfaces
	}

	pub fn rebuild(&self) -> TransmissionJointBuilder {
		TransmissionJointBuilder {
			joint_name: self
				.joint
				.upgrade()
				.unwrap() // This unwrap is Ok
				.read()
				.unwrap() // FIXME: This unwrap is not Ok
				.name()
				.clone(),
			hardware_interfaces: self.hardware_interfaces.clone(),
		}
	}
}

#[cfg(feature = "urdf")]
impl ToURDF for TransmissionJoint {
	fn to_urdf(
		&self,
		writer: &mut quick_xml::Writer<std::io::Cursor<Vec<u8>>>,
		urdf_config: &crate::to_rdf::to_urdf::URDFConfig,
	) -> Result<(), quick_xml::Error> {
		writer
			.create_element("joint")
			.with_attribute(Attribute {
				key: QName(b"name"),
				value: self
					.joint
					.upgrade()
					.unwrap() // FIXME: Is UNWRAP OK?
					.read()
					.unwrap() // FIXME: Is UNWRAP OK?
					.name()
					.display()
					.as_bytes()
					.into(),
			})
			.write_inner_content(|writer| -> quick_xml::Result<()> {
				self.hardware_interfaces
					.iter()
					.map(|hw_interface| hw_interface.to_urdf(writer, urdf_config))
					.process_results(|iter| iter.collect::<Vec<_>>())?;
				Ok(())
			})?;

		Ok(())
	}
}

impl PartialEq for TransmissionJoint {
	fn eq(&self, other: &Self) -> bool {
		Weak::ptr_eq(&self.joint, &other.joint)
			&& self.hardware_interfaces == other.hardware_interfaces
	}
}

impl Clone for TransmissionJoint {
	fn clone(&self) -> Self {
		Self {
			joint: Weak::clone(&self.joint),
			hardware_interfaces: self.hardware_interfaces.clone(),
		}
	}
}
