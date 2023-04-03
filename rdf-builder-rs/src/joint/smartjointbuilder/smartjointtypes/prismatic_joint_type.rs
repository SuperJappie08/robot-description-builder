use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::{
		jointbuilder::{BuildJoint, JointBuilder},
		smartjointbuilder::{
			smartparams::{
				smart_joint_datatraits::{self, LimitDataType},
				smart_joint_specification, WithLimit,
			},
			SmartJointBuilder,
		},
		Joint, JointType,
	},
	link::Link,
	ArcLock, WeakLock,
};

#[derive(Debug, Default, Clone)]
pub struct PrismaticType;

impl From<PrismaticType> for JointType {
	fn from(_value: PrismaticType) -> Self {
		JointType::Continuous
	}
}

impl smart_joint_specification::AxisAllowed for PrismaticType {}
impl smart_joint_specification::CalibrationAllowed for PrismaticType {}
impl smart_joint_specification::DynamicsAllowed for PrismaticType {}
impl smart_joint_specification::LimitAllowed for PrismaticType {}
impl smart_joint_specification::MimicAllowed for PrismaticType {}
impl smart_joint_specification::SafetyControllerAllowed for PrismaticType {}

impl<Axis, Calibration, Dynamics, Mimic, SafetyController> BuildJoint
	for SmartJointBuilder<
		PrismaticType,
		Axis,
		Calibration,
		Dynamics,
		WithLimit,
		Mimic,
		SafetyController,
	> where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn build(
		self,
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
	) -> ArcLock<Joint> {
		let mut joint_builder = JointBuilder::new(self.name, self.joint_type.into());

		if self.offset.is_some() || self.rotation.is_some() {
			todo!("Build Prismatic Joint")
		}

		self.axis.simplify(&mut joint_builder);
		self.calibration.simplify(&mut joint_builder);
		self.dynamics.simplify(&mut joint_builder);
		self.limit.simplify(&mut joint_builder, false);
		self.mimic.simplify(&mut joint_builder);
		self.safety_controller.simplify(&mut joint_builder);

		joint_builder.build(tree, parent_link, child_link)
	}
}
