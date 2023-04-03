use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::JointType,
	joint::{
		jointbuilder::BuildJoint,
		smartjointbuilder::smartparams::{smart_joint_datatraits, smart_joint_specification},
		Joint, SmartJointBuilder,
	},
	link::Link,
	ArcLock, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct ContinuousType;

impl From<ContinuousType> for JointType {
	fn from(_value: ContinuousType) -> Self {
		JointType::Continuous
	}
}

impl smart_joint_specification::AxisAllowed for ContinuousType {}
impl smart_joint_specification::CalibrationAllowed for ContinuousType {}
impl smart_joint_specification::DynamicsAllowed for ContinuousType {}
impl smart_joint_specification::LimitAllowed for ContinuousType {}
impl smart_joint_specification::MimicAllowed for ContinuousType {}
impl smart_joint_specification::SafetyControllerAllowed for ContinuousType {}

impl<Axis, Calibration, Dynamics, Limit, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<ContinuousType, Axis, Calibration, Dynamics, Limit, Mimic, SafetyController>
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Limit: smart_joint_datatraits::LimitDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let mut joint_builder =
			crate::joint::jointbuilder::JointBuilder::new(self.name, self.joint_type.into());

		// TODO:OFFSET
		if let Some(_mode) = self.offset {
			todo!("DO OFFSET")
		}
		if let Some(_rotation) = self.rotation {
			todo!("DO ROTATION")
		}

		self.axis.simplify(&mut joint_builder);
		self.calibration.simplify(&mut joint_builder);
		self.dynamics.simplify(&mut joint_builder);
		self.limit.simplify(&mut joint_builder, true);
		self.mimic.simplify(&mut joint_builder);
		self.safety_controller.simplify(&mut joint_builder);

		joint_builder.build(tree, parent_link, child_link)
	}
}
