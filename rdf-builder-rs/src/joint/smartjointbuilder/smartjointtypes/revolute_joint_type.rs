use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_tree_data::KinematicTreeData,
	joint::{
		smartjointbuilder::{
			smartparams::{smart_joint_datatraits, smart_joint_specification, WithLimit},
			SmartJointBuilder,
		},
		BuildJoint, Joint, JointBuilder,
	},
	link::Link,
	ArcLock, JointType, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct RevoluteType;

impl smart_joint_specification::AxisAllowed for RevoluteType {}
impl smart_joint_specification::CalibrationAllowed for RevoluteType {}
impl smart_joint_specification::DynamicsAllowed for RevoluteType {}
impl smart_joint_specification::LimitAllowed for RevoluteType {}
impl smart_joint_specification::MimicAllowed for RevoluteType {}
impl smart_joint_specification::SafetyControllerAllowed for RevoluteType {}

impl<Axis, Calibration, Dynamics, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<RevoluteType, Axis, Calibration, Dynamics, WithLimit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn build(
		self,
		_tree: Weak<KinematicTreeData>,
		_parent_link: WeakLock<Link>,
		_child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let mut _joint_builder = JointBuilder::new(self.name, JointType::Revolute);

		if let Some(_mode) = self.offset {
			todo!("BUILD FUNCTIOn")
		}

		todo!("Create a `RevoluteJoint`")
		// crate::Joint { name: , tree: (), parent_link: (), child_link: (), joint_type: () }
	}
}
