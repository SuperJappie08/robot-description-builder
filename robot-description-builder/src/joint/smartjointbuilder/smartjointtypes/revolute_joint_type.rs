use std::sync::Weak;

use crate::{
	cluster_objects::kinematic_data_tree::KinematicDataTree,
	joint::JointType,
	joint::{
		smartjointbuilder::{
			smartparams::{
				smart_joint_datatraits::{self, JointTypeTrait, LimitDataType},
				smart_joint_specification, WithLimit,
			},
			SmartJointBuilder,
		},
		BuildJoint, Joint, JointBuilder,
	},
	link::Link,
	utils::{ArcLock, WeakLock},
};

// TODO: Maybe flip the JointType Doc order
/// A representation of a revolute joint (`JointType::Revolute`) for the `SmartJointBuilder`.
///
/// See [`JointType::Revolute`] for more details.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct RevoluteType;

impl_jointtype_traits!(RevoluteType, false);

impl From<RevoluteType> for JointType {
	fn from(_value: RevoluteType) -> Self {
		JointType::Revolute
	}
}

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
		tree: Weak<KinematicDataTree>,
		parent_link: WeakLock<Link>,
		child_link: ArcLock<Link>,
		parent_shape_data: crate::link::LinkShapeData,
	) -> ArcLock<Joint> {
		Into::<JointBuilder>::into(self).build(tree, parent_link, child_link, parent_shape_data)
	}
}

impl<Axis, Calibration, Dynamics, Mimic, SafetyController>
	From<
		SmartJointBuilder<
			RevoluteType,
			Axis,
			Calibration,
			Dynamics,
			WithLimit,
			Mimic,
			SafetyController,
		>,
	> for JointBuilder
where
	Axis: smart_joint_datatraits::AxisDataType,
	Calibration: smart_joint_datatraits::CalibrationDataType,
	Dynamics: smart_joint_datatraits::DynamicsDataType,
	Mimic: smart_joint_datatraits::MimicDataType,
	SafetyController: smart_joint_datatraits::SafetyControllerDataType,
{
	fn from(
		value: SmartJointBuilder<
			RevoluteType,
			Axis,
			Calibration,
			Dynamics,
			WithLimit,
			Mimic,
			SafetyController,
		>,
	) -> Self {
		let mut joint_builder = JointBuilder::new(value.name, value.joint_type.into());

		joint_builder.with_transform(value.transform.unwrap_or_default());

		value.axis.simplify(&mut joint_builder);
		value.calibration.simplify(&mut joint_builder);
		value.dynamics.simplify(&mut joint_builder);
		value
			.limit
			.simplify(&mut joint_builder, RevoluteType::IS_CONTINOUS);
		value.mimic.simplify(&mut joint_builder);
		value.safety_controller.simplify(&mut joint_builder);

		joint_builder
	}
}
